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


#[derive(Debug, Serialize, Deserialize)]
pub struct ProveRequest {
    // demo inputs (can evolve later)
    pub secret_value: u64,
    pub threshold: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProveResponse {
    pub proof: String,                 // base64
    pub public_inputs: ProvePublicInputs,
    pub commitment: [u8; 32],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvePublicInputs {
    pub threshold: u64,
}
