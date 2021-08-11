//
//  CameraService.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 20.7.2021.
//

import AVKit
import Vision
import UIKit

public class CameraService: UIViewController, AVCaptureVideoDataOutputSampleBufferDelegate {
    @Published public var isCameraUnavailable = true
    @Published public var payload: String?
    @Published public var total: Int?
    @Published public var captured: Int?
    
    public let session = AVCaptureSession()
    var isSessionRunning = false
    var isConfigured = false
    var setupResult: SessionSetupResult = .success
    
    private let sessionQueue = DispatchQueue(label: "session queue")
    private let stitcherQueue = DispatchQueue(label: "stitcher queue")
    
    @objc dynamic var videoDeviceInput: AVCaptureDeviceInput!
    
    private let videoDataOutput = AVCaptureVideoDataOutput()
    private let videoDataOutputQueue = DispatchQueue(label: "qr code detection queue")
    
    private var detectionRequests: [VNDetectBarcodesRequest] = [VNDetectBarcodesRequest(completionHandler: { (request, error) in
        if error != nil {
            print("QR code detection error: \(String(describing: error))")
        }
        
        guard let barcodeDetectionRequest = request as? VNDetectBarcodesRequest,
              let results = barcodeDetectionRequest.results as? [VNBarcodeObservation] else {
            return
        }
        barcodeDetectionRequest.symbologies = [VNBarcodeSymbology.QR]
    })]
    
    private var bucketCount = 0
    
    private var bucket: [[UInt8]] = []
    
    public func configure() {
        sessionQueue.async {
            self.configureSession()
        }
    }
    
    public func checkForPermissions() {
        switch AVCaptureDevice.authorizationStatus(for: .video) {
        case .authorized:
            break
        case .notDetermined:
            sessionQueue.suspend()
            AVCaptureDevice.requestAccess(for: .video, completionHandler: {
                granted in
                if !granted {
                    self.setupResult = .notAuthorized
                }
                self.sessionQueue.resume()
            })
        default:
            setupResult = .notAuthorized
            
            DispatchQueue.main.async {
                self.isCameraUnavailable = true
            }
        }
    }
    
    public func start() {
        self.bucket = []
        sessionQueue.async {
            if !self.isSessionRunning && self.isConfigured {
                switch self.setupResult {
                case .success:
                    self.session.startRunning()
                    self.isSessionRunning = self.session.isRunning
                    
                    if self.session.isRunning {
                        DispatchQueue.main.async {
                            self.isCameraUnavailable = false
                        }
                    }
                case .configurationFailed, .notAuthorized:
                    print("Camera configuration invalid")
                    
                    DispatchQueue.main.sync {
                        self.isCameraUnavailable = true
                    }
                }
            }
        }
    }
    
    public func stop(completion: (() -> ())? = nil) {
        sessionQueue.async {
            if self.isSessionRunning {
                if self.setupResult == .success {
                    self.session.stopRunning()
                    self.isSessionRunning = self.session.isRunning
                    
                    if !self.session.isRunning {
                        DispatchQueue.main.async {
                            self.isCameraUnavailable = true
                            completion?()
                        }
                    }
                }
            }
        }
    }
    
    private func configureSession() {
        if setupResult != .success {
            return
        }
        
        session.beginConfiguration()
        
        session.sessionPreset = .photo
        
        do {
            guard let videoDevice = AVCaptureDevice.default(.builtInWideAngleCamera, for: .video, position: .back) else {
                print("Default camera is unavailable")
                setupResult = .configurationFailed
                session.commitConfiguration()
                return
            }
            
            let videoDeviceInput = try AVCaptureDeviceInput(device: videoDevice)
            
            if session.canAddInput(videoDeviceInput) {
                session.addInput(videoDeviceInput)
                self.videoDeviceInput = videoDeviceInput
            } else {
                print("Couldn't add camera input to the session")
                setupResult = .configurationFailed
                session.commitConfiguration()
                return
            }
        } catch {
            print("Couldn't create video device input: \(error)")
            setupResult = .configurationFailed
            session.commitConfiguration()
            return
        }
        
        videoDataOutput.alwaysDiscardsLateVideoFrames = true
        videoDataOutput.setSampleBufferDelegate(self, queue: videoDataOutputQueue)
        
        if session.canAddOutput(videoDataOutput) {
            session.addOutput(videoDataOutput)
            
            videoDataOutput.connection(with: .video)?.isEnabled = true
        } else {
            print("Could not add metadata output to the session")
            setupResult = .configurationFailed
            session.commitConfiguration()
            return
        }
        
        session.commitConfiguration()
        
        self.isConfigured = true
        self.start()
    }
    
    // Callback for receiving buffer - payload assembly should eventually be fed from here
    public func captureOutput(_ output: AVCaptureOutput, didOutput sampleBuffer: CMSampleBuffer, from connection: AVCaptureConnection) {
        guard let pixelBuffer = CMSampleBufferGetImageBuffer(sampleBuffer) else {
            print("Failed to obtain pixelbuffer for this frame")
            return
        }
        
        let imageRequestHandler = VNImageRequestHandler(cvPixelBuffer: pixelBuffer, options: [:])
        
        do {
            try imageRequestHandler.perform(detectionRequests)
        } catch {
            print("Failed to handle \(error)")
        }
        
        if let result = detectionRequests[0].results as? [VNBarcodeObservation] {
            if result.count != 0 {
                if result.count>1 {
                    print("lagging!")
                    print(result.count)
                }
                if let descriptor = result[0].barcodeDescriptor as? CIQRCodeDescriptor {
                    let payloadArray = descriptor.errorCorrectedPayload.map{$0}
                    let payloadStr = descriptor.errorCorrectedPayload.map{String(format: "%02x", $0)}.joined()
                    if !bucket.contains(payloadArray) {
                        if total == nil {
                            DispatchQueue.main.async {
                                self.total = 30
                            }
                        }
                        bucket.append(payloadArray)
                        print(bucket.count)
                        DispatchQueue.main.async {
                            self.captured = self.bucket.count
                            self.payload = payloadStr
                        }
                    }
                }
            }
        }
    }
}

extension CameraService {
    enum SessionSetupResult {
        case success
        case notAuthorized
        case configurationFailed
    }
}