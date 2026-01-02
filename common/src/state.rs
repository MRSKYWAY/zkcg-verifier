use crate::types::Hash;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolState {
    pub state_root: Hash,
    pub nonce: u64,
    pub epoch: u64,
}

impl ProtocolState {
    pub fn genesis() -> Self {
        Self {
            state_root: [0u8; 32],
            nonce: 0,
            epoch: 0,
        }
    }
}
