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
use crate::models::{SubmitProofRequest, SubmitProofResponse, ProveRequest, ProveResponse, ProvePublicInputs};
use std::sync::{Arc, Mutex};
use zkcg_verifier::backend::ProofBackend;
#[cfg(feature = "zk-vm")]
use zkcg_zkvm_host::{prove as zkvm_prove, ZkVmProverError};

#[derive(Clone)]
pub struct AppState {
    pub engine: Arc<Mutex<VerifierEngine>>
}

pub async fn submit_proof(
    State(state): State<AppState>,
    Json(req): Json<SubmitProofRequest>,
) -> Result<Json<SubmitProofResponse>, (StatusCode, String)> {
    println!("================ ZKCG =================");
    println!("📥 Received /v1/submit-proof request");
    println!("• threshold   : {}", req.public_inputs.threshold);
    println!("• nonce       : {}", req.public_inputs.nonce);
    println!("• commitment  : {:?}", req.new_state_commitment);
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
    println!("✅ Proof accepted");
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

fn map_prover_error(err: ZkVmProverError) -> (StatusCode, String) {
    match err {
        ZkVmProverError::PolicyViolation => (
            StatusCode::UNPROCESSABLE_ENTITY,
            "policy violation".to_string(),
        ),

        ZkVmProverError::ExecutionFailed => (
            StatusCode::BAD_REQUEST,
            "zkvm execution failed".to_string(),
        ),

    }
}


#[cfg(feature = "zk-vm")]
pub async fn prove(
    State(_state): State<AppState>, // backend NOT needed here
    Json(req): Json<ProveRequest>,
) -> Result<Json<ProveResponse>, (StatusCode, String)> {

    // DEV / DEMO SAFETY
    if std::env::var("ZKCG_ENABLE_PROVER").is_err() {
        return Err((StatusCode::FORBIDDEN, "prover disabled".into()));
    }

    println!("🧪 zkVM prover request received");
    println!("• secret_value: {}", req.secret_value);
    println!("• threshold   : {}", req.threshold);

    // ---- IMPORTANT ----
    // For demo purposes we always prove against GENESIS
    let old_state_root = [0u8; 32];
    let nonce = 1;

    let proof = zkvm_prove(
        req.secret_value,
        req.threshold,
        old_state_root,
        nonce,
    ).map_err(|e| map_prover_error(e))?;

    Ok(Json(ProveResponse {
        proof: STANDARD.encode(&proof),
        public_inputs: ProvePublicInputs {
            threshold: req.threshold,
        },
        commitment: {
            let mut c = [0u8; 32];
            c[0] = (req.secret_value % 256) as u8;
            c
        },
    }))
}
