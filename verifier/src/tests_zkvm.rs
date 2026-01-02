#![cfg(feature = "zk-vm")]

use crate::{
    engine::{PublicInputs, VerifierEngine},
    backend_zkvm::ZkVmBackend,
};
use zkcg_common::{
    errors::ProtocolError,
    state::ProtocolState,
    types::Commitment,
};
use zkcg_zkvm_host::{prove, ZkVmProverError};

fn commitment() -> Commitment {
    Commitment([42u8; 32])
}
fn valid_inputs() -> PublicInputs {
    PublicInputs {
        threshold: 10,
        old_state_root: [9u8; 32],
        nonce: 7,
    }
}
// Consistent inputs (override genesis for matching)
fn test_inputs() -> PublicInputs {
    PublicInputs {
        threshold: 10,
        old_state_root: [0u8; 32], // Match genesis root for simplicity
        nonce: 1, // state.nonce + 1
    }
}

// Helper: Mock state to match inputs (avoids genesis mismatch)
fn mock_state(inputs: &PublicInputs) -> ProtocolState {
    ProtocolState {
        state_root: inputs.old_state_root,
        nonce: inputs.nonce - 1, // Pre-transition
        ..ProtocolState::genesis()
    }
}

#[test]
fn zkvm_valid_transition_succeeds() {
    let inputs = test_inputs();
    let state = mock_state(&inputs);

    let mut engine = VerifierEngine::new(
        state.clone(),
        Box::new(ZkVmBackend),
    );

    // Prove with matching inputs (score=5 <=10)
    let proof = prove(5, inputs.threshold, inputs.old_state_root, inputs.nonce)
        .expect("valid proof generated");
    println!("Generated proof: {:?}", proof);
    let result = engine.process_transition(
        &proof,
        inputs,
        commitment(),
    );
    println!("Result: {:?}", result);

    assert!(result.is_ok(), "Valid transition should succeed");
}

#[test]
fn zkvm_policy_violation_is_rejected() {
    let mut inputs = valid_inputs();
    let result = prove(20, 10, inputs.old_state_root, inputs.nonce);

    assert!(matches!(
        result,
        Err(ZkVmProverError::PolicyViolation)
    ));
}

#[test]
fn zkvm_tampered_proof_is_rejected() {
    let mut inputs = valid_inputs();
    let mut proof = prove(5, 10, inputs.old_state_root, inputs.nonce).unwrap();

    proof[0] ^= 0xFF; // corrupt method id

    let state = ProtocolState::genesis();
    let mut engine = VerifierEngine::new(
        state.clone(),
        Box::new(ZkVmBackend),
    );

    let inputs = PublicInputs {
        threshold: 10,
        old_state_root: state.state_root,
        nonce: state.nonce + 1,
    };

    let result = engine.process_transition(
        &proof,
        inputs,
        commitment(),
    );

    assert!(matches!(result, Err(ProtocolError::InvalidProof)));
}

#[test]
fn zkvm_empty_proof_is_rejected() {
    let state = ProtocolState::genesis();
    let mut engine = VerifierEngine::new(
        state.clone(),
        Box::new(ZkVmBackend),
    );

    let inputs = PublicInputs {
        threshold: 10,
        old_state_root: state.state_root,
        nonce: state.nonce + 1,
    };

    let result = engine.process_transition(
        &[],
        inputs,
        commitment(),
    );

    assert!(result.is_err());
}

#[test]
fn zkvm_overflow_inputs_rejected() {
    let mut inputs = valid_inputs();
    let result = prove(u64::MAX, u64::MAX - 1, inputs.old_state_root, inputs.nonce);
    assert!(result.is_err());
}
