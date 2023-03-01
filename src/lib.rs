use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreditcoinCommand {
    verb: String,
    args: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CCBlock {
    pub header: CCBlockHeader,
    pub header_signature: String,
    pub batches: Vec<CCBatch>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CCBlockHeader {
    pub number: u64,
    pub parent: String,
    pub miner: String,

    pub batch_ids: Vec<String>,

    pub consensus: String,

    pub state_root_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CCBatch {
    pub header: CCBatchHeader,
    pub header_signature: String,
    pub transactions: Vec<CCTransaction>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CCBatchHeader {
    pub signer: String,
    pub transaction_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum TxPayload {
    Creditcoin(CreditcoinCommand),
    Settings(SettingsPayload),
    Other(Vec<u8>),
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SettingsPayload {
    pub action: SettingsAction,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum SettingsAction {
    Unset,
    Propose {
        setting: String,
        value: String,
        nonce: String,
    },
    Vote,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SettingVote {
    pub kind: VoteKind,
    pub proposal_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum VoteKind {
    Unset,
    Accept,
    Reject,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CCTransaction {
    pub header: CCTransactionHeader,
    pub header_signature: String,
    pub payload: TxPayload,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CCTransactionHeader {
    pub batcher: String,
    pub dependencies: Vec<String>,
    pub family_name: String,
    pub family_version: String,
    pub inputs: Vec<String>,
    pub nonce: String,
    pub outputs: Vec<String>,
    pub payload_hash: String,
    pub signer: String,
}

pub fn decode_blocks(bytes: &[u8]) -> Result<Vec<CCBlock>, bincode::Error> {
    bincode::deserialize(bytes)
}
