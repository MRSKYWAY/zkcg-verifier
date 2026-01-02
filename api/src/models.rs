use serde::{Deserialize, Serialize};
use zkcg_common::types::Hash;

#[derive(Debug, Deserialize)]
pub struct SubmitProofRequest {
    pub proof: String,
    pub public_inputs: PublicInputsDto,
    pub new_state_commitment: Hash,
}

#[derive(Debug, Deserialize)]
pub struct PublicInputsDto {
    pub threshold: u64,
    pub old_state_root: Hash,
    pub nonce: u64,
}

#[derive(Debug, Serialize)]
pub struct SubmitProofResponse {
    pub status: String,
}
