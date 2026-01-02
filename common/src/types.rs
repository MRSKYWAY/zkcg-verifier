use serde::{Deserialize, Serialize};

pub type Hash = [u8; 32];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commitment(pub Hash);

#[derive(Serialize, Deserialize)]
pub struct ZkVmInput {
    pub score: u64,
    pub threshold: u64,
    pub old_state_root: [u8; 32],
    pub nonce: u64,
}