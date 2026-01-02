use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use zkcg_verifier::engine::{PublicInputs, VerifierEngine};
use zkcg_common::{
    errors::ProtocolError,
    types::Commitment,
};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use crate::models::{SubmitProofRequest, SubmitProofResponse};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub engine: Arc<Mutex<VerifierEngine>>,
}

pub async fn submit_proof(
    State(state): State<AppState>,
    Json(req): Json<SubmitProofRequest>,
) -> Result<Json<SubmitProofResponse>, (StatusCode, String)> {
    let mut engine = state.engine.lock().unwrap();
    let proof_bytes = STANDARD
    .decode(&req.proof)
    .map_err(|_| (StatusCode::BAD_REQUEST, "invalid base64 proof".to_string()))?;
    let inputs = PublicInputs {
        threshold: req.public_inputs.threshold,
        old_state_root: req.public_inputs.old_state_root,
        nonce: req.public_inputs.nonce,
    };

    let commitment = Commitment(req.new_state_commitment);

    engine
        .process_transition(&proof_bytes, inputs, commitment)
        .map_err(map_error)?;

    Ok(Json(SubmitProofResponse {
        status: "accepted".to_string(),
    }))
}

fn map_error(err: ProtocolError) -> (StatusCode, String) {
    use ProtocolError::*;

    match err {
        InvalidFormat => (StatusCode::BAD_REQUEST, err.to_string()),
        InvalidNonce => (StatusCode::CONFLICT, err.to_string()),
        StateMismatch => (StatusCode::CONFLICT, err.to_string()),
        PolicyViolation => (StatusCode::UNPROCESSABLE_ENTITY, err.to_string()),
        InvalidProof => (StatusCode::BAD_REQUEST, err.to_string()),
        CommitmentMismatch => (StatusCode::BAD_REQUEST, err.to_string()),
    }
}
