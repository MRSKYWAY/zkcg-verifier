use std::time::Instant;

use rayon::prelude::*;

use zkcg_common::{state::ProtocolState, types::Commitment};
use zkcg_verifier::{
    backend::ProofBackend,
    backend_halo2::Halo2Backend,
    backend_zkvm::ZkVmBackend,
    engine::{PublicInputs, VerifierEngine},
};

use zkcg_zkvm_host::prove as zkvm_prove;

// ---------------- Halo2 imports ----------------
use circuits::score_circuit::ScoreCircuit;
use halo2_proofs::{
    arithmetic::Field,
    circuit::Value,
    plonk::{create_proof, keygen_pk, keygen_vk},
    poly::commitment::Params,
    transcript::{Blake2bWrite, Challenge255},
};
use halo2curves::bn256::{Fr, G1Affine};
use rand::rngs::OsRng;

fn main() {
    println!("Halo2 → zkVM Attestation Pipeline\n");

    let threshold = 600u64;
    let scores = vec![550, 720, 480, 810, 590];

    // =========================================================
    // Phase 1: Halo2 policy verification
    // =========================================================
    println!("Phase 1: Halo2 policy verification");

    let phase1_start = Instant::now();

    let k = 9;
    let params: Params<G1Affine> = Params::new(k);

    let dummy = ScoreCircuit::<Fr> {
        score: Value::known(Fr::ZERO),
        threshold: Value::known(Fr::ZERO),
    };
    let vk = keygen_vk(&params, &dummy).unwrap();

    let halo2_backend = Halo2Backend {
        vk,
        params: params.clone(),
    };

    let approved: Vec<u64> = scores
        .par_iter()
        .copied()
        .filter(|&score| {
            let proof = halo2_prove(score, threshold, &params);
            let inputs = PublicInputs {
                threshold,
                old_state_root: [0u8; 32], // unused by Halo2 backend
                nonce: 1,
            };
            halo2_backend.verify(&proof, &inputs).is_ok()
        })
        .collect();

    let phase1_time = phase1_start.elapsed();

    println!(
        "✓ Halo2 approved {} / {} loans in {:.2?}",
        approved.len(),
        scores.len(),
        phase1_time
    );

    // =========================================================
    // Phase 2a: zkVM proof generation (parallel)
    // =========================================================
    println!("\nPhase 2a: zkVM proof generation (parallel)");

    let phase2a_start = Instant::now();

    let zkvm_proofs: Vec<Vec<u8>> = approved
        .par_iter()
        .map(|&score| {
            zkvm_prove(
                score,
                threshold,
                [0u8; 32], // placeholder (validated later)
                0,
            )
            .expect("zkVM proof generation failed")
        })
        .collect();

    let phase2a_time = phase2a_start.elapsed();

    println!(
        "✓ Generated {} zkVM proofs in {:.2?} ({:.2} proofs/sec)",
        zkvm_proofs.len(),
        phase2a_time,
        zkvm_proofs.len() as f64 / phase2a_time.as_secs_f64()
    );

    // =========================================================
    // Phase 2b: zkVM protocol attestation (sequential)
    // =========================================================
    println!("\nPhase 2b: zkVM protocol attestation (sequential)");

    let phase2b_start = Instant::now();

    let mut engine = VerifierEngine::new(
        ProtocolState::genesis(),
        Box::new(ZkVmBackend),
    );

    for (i, proof) in zkvm_proofs.iter().enumerate() {
        let state = engine.state().clone();

        let inputs = PublicInputs {
            threshold,
            old_state_root: state.state_root,
            nonce: state.nonce + 1,
        };

        engine
            .process_transition(
                proof,
                inputs,
                Commitment([i as u8; 32]),
            )
            .expect("zkVM attestation failed");
    }

    let phase2b_time = phase2b_start.elapsed();

    println!(
        "✓ Attested {} loans in {:.2?} ({:.2} tx/sec)",
        zkvm_proofs.len(),
        phase2b_time,
        zkvm_proofs.len() as f64 / phase2b_time.as_secs_f64()
    );

    // =========================================================
    // Final Summary
    // =========================================================
    let total_time = phase1_time + phase2a_time + phase2b_time;

    println!("\n========== SUMMARY ==========");
    println!("Total loans evaluated: {}", scores.len());
    println!("Loans approved:       {}", approved.len());
    println!("Halo2 phase:          {:.2?}", phase1_time);
    println!("zkVM prove phase:     {:.2?}", phase2a_time);
    println!("zkVM attest phase:    {:.2?}", phase2b_time);
    println!("-----------------------------");
    println!("Total pipeline time:  {:.2?}", total_time);
    println!(
        "End-to-end throughput: {:.2} loans/sec",
        approved.len() as f64 / total_time.as_secs_f64()
    );

    println!("\n✅ Halo2 → zkVM attestation pipeline completed successfully");
}

// ------------------------------------------------------------
// Halo2 proof helper (identical semantics to tests_halo2.rs)
// ------------------------------------------------------------
fn halo2_prove(
    score: u64,
    threshold: u64,
    params: &Params<G1Affine>,
) -> Vec<u8> {
    let circuit = ScoreCircuit::<Fr> {
        score: Value::known(Fr::from(score)),
        threshold: Value::known(Fr::from(threshold)),
    };

    let vk = keygen_vk(params, &circuit).unwrap();
    let pk = keygen_pk(params, vk, &circuit).unwrap();

    let public_inputs = vec![vec![Fr::from(threshold)]];
    let instance_slices: Vec<&[Fr]> =
        public_inputs.iter().map(|v| v.as_slice()).collect();

    let mut transcript =
        Blake2bWrite::<_, G1Affine, Challenge255<G1Affine>>::init(Vec::new());

    create_proof(
        params,
        &pk,
        &[circuit],
        &[instance_slices.as_slice()],
        OsRng,
        &mut transcript,
    )
    .unwrap();

    transcript.finalize()
}
