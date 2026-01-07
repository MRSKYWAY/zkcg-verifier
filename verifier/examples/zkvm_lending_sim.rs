use zkcg_common::{
    errors::ProtocolError,
    state::ProtocolState,
    types::Commitment,
};

use zkcg_verifier::{
    engine::{PublicInputs, VerifierEngine},
    backend_zkvm::ZkVmBackend,
};

use zkcg_zkvm_host::{prove, ZkVmProverError};

fn main() {
    println!("zkVM DeFi Lending Simulation");

    // ----------------------------------
    // 1. Initial protocol state
    // ----------------------------------
    let mut state = ProtocolState::genesis();

    let mut engine = VerifierEngine::new(
        state.clone(),
        Box::new(ZkVmBackend),
    );

    let threshold = 600u64;
    let score = 550u64; // borrower score (private to prover)

    // ----------------------------------
    // 2. Public inputs (verifier-controlled)
    // ----------------------------------
    let inputs = PublicInputs {
        threshold,
        old_state_root: state.state_root,
        nonce: state.nonce + 1,
    };

    let commitment = Commitment([42u8; 32]);

    // ----------------------------------
    // 3. zkVM proof generation (prover side)
    // ----------------------------------
    //
    // In production this runs:
    // - in a separate service
    // - in a private repo
    // - or on a sequencer
    //
    // Here we call it directly for demonstration.
    let proof = match prove(
        score,
        inputs.threshold,
        inputs.old_state_root,
        inputs.nonce,
    ) {
        Ok(p) => p,
        Err(ZkVmProverError::PolicyViolation) => {
            println!("❌ Loan rejected: policy violation");
            return;
        }
        Err(e) => {
            println!("❌ Prover error: {:?}", e);
            return;
        }
    };

    // ----------------------------------
    // 4. Verifier applies transition
    // ----------------------------------
    match engine.process_transition(
        &proof,
        inputs,
        commitment,
    ) {
        Ok(new_state) => {
            println!("✅ zkVM proof accepted");
            println!("New protocol state: {:?}", new_state);
        }
        Err(ProtocolError::InvalidProof) => {
            println!("❌ Invalid zkVM proof");
        }
        Err(e) => {
            println!("❌ Verification failed: {:?}", e);
        }
    }
}
