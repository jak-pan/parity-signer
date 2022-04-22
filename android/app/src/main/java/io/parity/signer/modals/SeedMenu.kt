package io.parity.signer.modals

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.ShieldAlert
import io.parity.signer.alerts.AndroidCalledConfirm
import io.parity.signer.components.BigButton
import io.parity.signer.components.HeaderBar
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton
import io.parity.signer.models.removeSeed
import io.parity.signer.ui.theme.Bg000
import io.parity.signer.ui.theme.modal
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.MSeedMenu

@Composable
fun SeedMenu(seedMenu: MSeedMenu, signerDataModel: SignerDataModel) {
	var confirm by remember { mutableStateOf(false) }

	Column {
		Spacer(Modifier.weight(1f))
		Surface(
			color = MaterialTheme.colors.Bg000,
			shape = MaterialTheme.shapes.modal
		) {
			Column(
				modifier = Modifier.padding(20.dp)
			) {
				HeaderBar(line1 = "SEED MENU", line2 = "Select action")
				BigButton(
					text = "Backup",
					action = {
						if (signerDataModel.alertState.value == ShieldAlert.None)
							signerDataModel.pushButton(Action.BACKUP_SEED)
						else
							signerDataModel.pushButton(Action.SHIELD)
					})
				BigButton(
					text = "Derive new key",
					action = {
						if (signerDataModel.alertState.value == ShieldAlert.None)
							signerDataModel.pushButton(Action.NEW_KEY)
						else
							signerDataModel.pushButton(Action.SHIELD)
					},
					isShaded = true,
					isCrypto = true
				)
				BigButton(
					text = "Forget this seed forever",
					isShaded = true,
					isDangerous = true,
					action = {
						val seedName = seedMenu.seed
						signerDataModel.removeSeed(seedName)
					}
				)
			}
		}
	}

	AndroidCalledConfirm(
		show = confirm,
		header = "Forget this seed forever?",
		text = "This seed will be removed for all networks. This is not reversible. Are you sure?",
		back = { confirm = false },
		forward = {
			seedMenu.seed.let {
				if (it.isNotBlank()) signerDataModel.removeSeed(it)
			}
		},
		backText = "Cancel",
		forwardText = "Remove seed"
	)
}
