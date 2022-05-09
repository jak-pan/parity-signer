package io.parity.signer.modals

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.*
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.unit.dp
import io.parity.signer.components.HeaderBar
import io.parity.signer.components.KeyCard
import io.parity.signer.models.intoImageBitmap
import io.parity.signer.ui.theme.Bg000
import io.parity.signer.ui.theme.modal
import io.parity.signer.uniffi.MSufficientCryptoReady

@Composable
fun SufficientCryptoReady(
	sufficientCrypto: MSufficientCryptoReady,
) {
	Surface(
		shape = MaterialTheme.shapes.modal,
		color = MaterialTheme.colors.Bg000
	) {
		Column(
			modifier = Modifier
				.fillMaxSize()
				.padding(20.dp)
		) {
			HeaderBar("Your signature", "Scan this into your application")
			Image(
				bitmap = sufficientCrypto.sufficient.intoImageBitmap(),
				contentDescription = "Signed update",
				contentScale = ContentScale.FillWidth,
				modifier = Modifier.fillMaxWidth()
			)
			KeyCard(
				identity = sufficientCrypto.authorInfo,
			)
			Text(
				"Payload: " + sufficientCrypto.content.ttype
			)
		}
	}
}
