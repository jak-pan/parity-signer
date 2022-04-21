use crate::{crypto::Encryption, history::Event, keyring::NetworkSpecsKey};

#[derive(PartialEq, Clone)]
pub struct SeedNameWithIdenticon {
    pub seed_name: String,
    pub identicon: String,
}

/// Enum containing card sets for three different outcomes:
/// signing (Sign), accepting (Stub) and reading, for example, in case of an error (Read)
#[derive(PartialEq, Debug, Clone)]
pub enum TransactionAction {
    Derivations {
        content: String,
        network_info: String,
        checksum: u32,
        network_specs_key: NetworkSpecsKey,
    },
    Sign {
        content: String,
        checksum: u32,
        has_pwd: bool,
        author_info: String,
        network_info: String,
    },
    Stub {
        s: String,
        u: u32,
        stub: StubNav,
    },
    Read {
        r: String,
    },
}

/// Enum describing Stub content.
/// Is used for proper navigation. Variants:
/// AddSpecs (with associated NetworkSpecsKey), LoadMeta (with associated
/// NetworkSpecsKey for the first by order network using those metadata),
/// and LoadTypes
#[derive(PartialEq, Debug, Clone)]
pub enum StubNav {
    AddSpecs { n: NetworkSpecsKey },
    LoadMeta { l: NetworkSpecsKey },
    LoadTypes,
}

#[derive(Clone, PartialEq)]
pub struct ActionResult {
    pub screen: Option<String>,
    pub screen_label: String,
    pub back: bool,
    pub footer: bool,
    pub footer_button: String,
    pub right_button: String,
    pub screen_name_type: String,
    pub modal: String,
    pub alert: String,
    pub screen_data: ScreenData,
    pub modal_data: String,
    pub alert_data: String,
}

#[derive(Clone, PartialEq)]
pub struct LogScreenEntry {
    pub order: u32,
    pub timestamp: String,
    pub events: Vec<Event>,
}

#[derive(Clone, PartialEq)]
pub enum ScreenData {
    Scan,
    Keys { f: MKeys },
    Settings { f: MSettings },
    Log { f: MLog },
    LogDetails { f: MLogDetails },
    Transaction { f: MTransaction },
    SeedSelector { f: MSeeds },
    KeyDetails { f: MKeyDetails },
    NewSeed { f: MNewSeed },
    RecoverSeedName { f: MRecoverSeedName },
    RecoverSeedPhrase { f: MRecoverSeedPhrase },
    DeriveKey { f: MDeriveKey },
    VVerifier { f: MVerifierDetails },
    ManageNetworks { f: MManageNetworks },
    NNetworkDetails { f: MNetworkDetails },
    SignSufficientCrypto { f: MSignSufficientCrypto },
    SelectSeedForBackup { f: MSeeds },
    Documents,
    KeyDetailsMulti { f: MKeyDetailsMulti },
}

#[derive(Clone, PartialEq)]
pub struct Identity {
    pub seed_name: String,
    pub address_key: String,
    pub public_key: String,
    pub identicon: String,
    pub has_pwd: bool,
    pub path: String,
    pub is_multiselect: bool,
    pub base58: String,
}

#[derive(Clone, PartialEq)]
pub struct MKeysCard {
    pub address_key: String,
    pub base58: String,
    pub identicon: String,
    pub has_pwd: bool,
    pub path: String,
    pub swiped: bool,
    pub multiselect: bool,
}

#[derive(Clone, Default, PartialEq)]
pub struct MNetworkCard {
    pub title: String,
    pub logo: String,
}

// TODO: This has to have a custom default.
#[derive(Clone, Default, PartialEq)]
pub struct MSeedKeyCard {
    pub seed_name: String,
    pub identicon: String,
    pub address_key: String,
    pub base58: String,
    pub swiped: bool,
    pub multiselect: bool,
}

#[derive(Clone, PartialEq)]
pub struct MKeys {
    pub set: Vec<MKeysCard>,
    pub root: MSeedKeyCard,
    pub network: MNetworkCard,
    pub multiselect_mode: bool,
    pub multiselect_count: String,
}

#[derive(Clone, PartialEq)]
pub struct MSettings {
    pub public_key: Option<String>,
    pub identicon: Option<String>,
    pub encryption: Option<String>,
    pub error: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct History {
    pub order: u32,
    pub timestamp: String,
    pub events: Vec<Event>,
}

#[derive(Clone, PartialEq)]
pub struct MLog {
    pub log: Vec<History>,
    pub total_entries: u32,
}

#[derive(Clone, PartialEq)]
pub struct MLogDetails {
    pub timestamp: String,
    pub events: Vec<Event>,
}

#[derive(Clone, PartialEq)]
pub enum TransactionType {
    Sign,
    Stub,
    Read,
    ImportDerivations,
    Done,
}

#[derive(Clone, PartialEq)]
pub struct TransactionNetworkInfo {
    pub network_title: String,
    pub network_logo: String,
}

#[derive(Clone, PartialEq)]
pub struct TransactionAuthor {
    pub base58: String,
    pub identicon: String,
    pub seed: String,
    pub derivation_path: String,
}

#[derive(Clone, PartialEq)]
pub struct TransactionCard {
    pub content: String,
}

#[derive(Clone, PartialEq)]
pub struct TransactionCardSet {
    pub author: Option<Vec<TransactionCard>>,
    pub error: Option<Vec<TransactionCard>>,
    pub extensions: Option<Vec<TransactionCard>>,
    pub importing_derivations: Option<Vec<TransactionCard>>,
    pub message: Option<Vec<TransactionCard>>,
    pub meta: Option<Vec<TransactionCard>>,
    pub method: Option<Vec<TransactionCard>>,
    pub new_specs: Option<Vec<TransactionCard>>,
    pub verifier: Option<Vec<TransactionCard>>,
    pub warning: Option<Vec<TransactionCard>>,
    pub types_info: Option<Vec<TransactionCard>>,
}

#[derive(Clone, PartialEq)]
pub struct MTransaction {
    pub content: TransactionCardSet,
    pub ttype: TransactionType,
    pub author_info: Option<TransactionAuthor>,
    pub network_info: Option<TransactionNetworkInfo>,
}

#[derive(Clone, PartialEq)]
pub struct SeedNameCard {
    pub seed_name: String,
    pub identicon: String,
}

#[derive(Clone, PartialEq)]
pub struct MSeeds {
    pub seed_name_cards: Vec<SeedNameCard>,
}

#[derive(Clone, PartialEq)]
pub struct MKeyDetails {
    pub qr: String,
    pub pubkey: String,
    pub base58: String,
    pub identicon: String,
    pub seed_name: String,
    pub path: String,
    pub network_title: String,
    pub network_logo: String,
}

#[derive(Clone, PartialEq)]
pub struct MNewSeed {
    pub keyboard: bool,
}

#[derive(Clone, PartialEq)]
pub struct MRecoverSeedName {
    pub keyboard: bool,
    pub seed_name: String,
}

#[derive(Clone, PartialEq)]
pub struct MRecoverSeedPhrase {
    pub keyboard: bool,
    pub seed_name: String,
    pub user_input: String,
    pub guess_set: Vec<String>,
    pub draft: Vec<SeedWord>,
    pub ready_seed: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct SeedWord {
    pub order: u32,
    pub content: String,
}

#[derive(Clone, PartialEq)]
pub struct DerivationCheck {
    pub button_good: Option<bool>,
    pub where_to: Option<DerivationDestination>,
    pub collision: Option<Address>,
    pub error: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct Address {
    pub base58: String,
    pub path: String,
    pub has_pwd: bool,
    pub identicon: String,
    pub seed_name: String,
    pub multiselect: Option<bool>,
}

#[derive(Clone, PartialEq)]
pub enum DerivationDestination {
    Pwd,
    Pin,
}

#[derive(Clone, PartialEq)]
pub struct MDeriveKey {
    pub seed_name: String,
    pub network_title: String,
    pub network_logo: String,
    pub network_specs_key: String,
    pub suggested_derivation: String,
    pub keyboard: bool,
    pub derivation_check: Option<DerivationCheck>,
}

#[derive(Clone, Default, PartialEq)]
pub struct MVerifierDetails {
    pub public_key: String,
    pub identicon: String,
    pub encryption: String,
}

#[derive(Clone, Default, PartialEq)]
pub struct MVerifier {
    pub ttype: String,
    pub details: MVerifierDetails,
}

#[derive(Clone, PartialEq)]
pub struct MMetadataRecord {
    pub specs_version: String,
    pub meta_hash: String,
    pub meta_id_pic: String,
}

#[derive(Clone, PartialEq)]
pub struct MNetworkDetails {
    pub base58prefix: u16,
    pub color: String,
    pub decimals: u8,
    pub encryption: Encryption,
    pub genesis_hash: String,
    pub logo: String,
    pub name: String,
    pub order: String,
    pub path_id: String,
    pub secondary_color: String,
    pub title: String,
    pub unit: String,
    pub current_verifier: MVerifier,
    pub meta: Vec<MMetadataRecord>,
}

#[derive(Clone, PartialEq)]
pub struct MRawKey {
    pub seed_name: String,
    pub address_key: String,
    pub public_key: String,
    pub identicon: String,
    pub has_pwd: bool,
    pub path: String,
}

#[derive(Clone, PartialEq)]
pub struct MSignSufficientCrypto {
    pub identities: Vec<MRawKey>,
}

#[derive(Clone, PartialEq)]
pub struct MKeyDetailsMulti {
    pub key_details: MKeyDetails,
    pub current_number: String,
    pub out_of: String,
}

#[derive(Clone, PartialEq)]
pub struct MMNetwork {
    pub key: String,
    pub title: String,
    pub logo: String,
    pub order: u8,
}

#[derive(Clone, PartialEq)]
pub struct MManageNetworks {
    pub networks: Vec<MMNetwork>,
}