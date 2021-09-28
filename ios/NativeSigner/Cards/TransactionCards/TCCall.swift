//
//  TCCall.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCCall: View {
    let value: Call
    @State private var showDoc = false
    var body: some View {
        VStack {
            Button (action: {
                self.showDoc.toggle()
            }) {
                HStack {
                    Text(value.method)
                        .foregroundColor(Color("textMainColor"))
                    Text(" from ")
                        .foregroundColor(Color("AccentColor"))
                    Text(value.pallet)
                        .foregroundColor(Color("textMainColor"))
                    Spacer()
                    Text("?")
                        .foregroundColor(Color("AccentColor"))
                }
            }
            if showDoc {
                Text(String(decoding: Data(fromHexEncodedString: value.docs) ?? Data(), as: UTF8.self))
                    .foregroundColor(Color("textMainColor"))
                    .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
            }
        }
    }
}

/*
 struct TCCall_Previews: PreviewProvider {
 static var previews: some View {
 TCCall()
 }
 }
 */
