#![cfg(all(feature = "zk-halo2", feature = "zk-vm"))]

use crate::{
    backend_halo2::Halo2Backend,
    engine::PublicInputs,
};
use crate::backend_zkvm::ZkVmBackend;
use crate::backend::ProofBackend;
use zkcg_common::errors::ProtocolError;
use rand::rngs::OsRng;
use halo2_proofs::{
    circuit::Value,
    plonk::{create_proof, keygen_pk, keygen_vk},
    poly::commitment::Params,
    transcript::{Blake2bWrite, Challenge255},
};
use halo2_proofs::arithmetic::Field;
use halo2curves::bn256::{Fr, G1Affine};
use circuits::score_circuit::ScoreCircuit;
use zkcg_zkvm_host::prove;
use zkcg_common::state::ProtocolState;
/* ---------------- Expectations ---------------- */

#[derive(Copy, Clone)]
enum Expectation {
    Accept,
    Reject,
}

struct TestScenario {
    score: u64,
    threshold: u64,
    expected: Expectation,
    desc: &'static str,
}

fn scenarios() -> Vec<TestScenario> {
    vec![
        TestScenario {
            score: 39,
            threshold: 40,
            expected: Expectation::Accept,
            desc: "Valid transition",
        },
        TestScenario {
            score: 41,
            threshold: 40,
            expected: Expectation::Reject,
            desc: "Policy violation",
        },
        TestScenario {
            score: 0,
            threshold: 0,
            expected: Expectation::Accept,
            desc: "Zero boundary",
        },
    ]
}

/* ---------------- Helpers ---------------- */

fn matches_expectation(
    result: Result<(), ProtocolError>,
    expected: Expectation,
) -> bool {
    match expected {
        Expectation::Accept => result.is_ok(),
        Expectation::Reject => result.is_err(),
    }
}

/* ---------------- Halo2 ---------------- */

fn halo2_prove(score: u64, threshold: u64, params: &Params<G1Affine>) -> Vec<u8> {
    let circuit = ScoreCircuit::<Fr> {
        score: Value::known(Fr::from(score)),
        threshold: Value::known(Fr::from(threshold)),
    };

    let vk = keygen_vk(params, &circuit).unwrap();
    let pk = keygen_pk(params, vk, &circuit).unwrap();

    let instances = vec![vec![Fr::from(threshold)]];
    let instance_refs: Vec<&[Fr]> = instances.iter().map(|v| v.as_slice()).collect();
    let all_instances = vec![instance_refs.as_slice()];

    let mut transcript =
        Blake2bWrite::<_, G1Affine, Challenge255<G1Affine>>::init(Vec::new());

    create_proof(
        params,
        &pk,
        &[circuit],
        &all_instances,
        OsRng,
        &mut transcript,
    )
    .unwrap();

    transcript.finalize()
}

fn halo2_backend() -> Halo2Backend {
    let params = Params::new(9);
    let dummy = ScoreCircuit::<Fr> {
        score: Value::known(Fr::ZERO),
        threshold: Value::known(Fr::ZERO),
    };
    let vk = keygen_vk(&params, &dummy).unwrap();
    Halo2Backend { vk, params }
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
/* ---------------- zkVM ---------------- */

fn zkvm_prove(score: u64, threshold: u64) -> Result<Vec<u8>, ProtocolError> {
    let inputs = test_inputs();
    prove(score, threshold, inputs.old_state_root, inputs.nonce).map_err(|_| ProtocolError::InvalidProof)
}

/* ---------------- Rust baseline ---------------- */

fn rust_only(score: u64, threshold: u64) -> Result<(), ProtocolError> {
    if score > threshold {
        Err(ProtocolError::PolicyViolation)
    } else {
        Ok(())
    }
}

/* ---------------- Test ---------------- */

#[test]
fn cross_backend_equivalence() {
    for s in scenarios() {
        let inputs = PublicInputs {
            threshold: s.threshold,
            old_state_root: [0u8; 32],
            nonce: 1,
        };

        // Halo2
        let params = Params::new(9);
        let halo2_proof = halo2_prove(s.score, s.threshold, &params);
        let halo2 = halo2_backend();
        let halo2_result = halo2.verify(&halo2_proof, &inputs);

        assert!(
            matches_expectation(halo2_result, s.expected),
            "Halo2 failed: {}",
            s.desc
        );

        // zkVM
        let zkvm = ZkVmBackend;
        let zkvm_result = zkvm_prove(s.score, s.threshold)
            .and_then(|p| zkvm.verify(&p, &inputs));

        assert!(
            matches_expectation(zkvm_result, s.expected),
            "zkVM failed: {}",
            s.desc
        );

        // Rust-only baseline
        let rust_result = rust_only(s.score, s.threshold);
        assert!(
            matches_expectation(rust_result, s.expected),
            "Rust failed: {}",
            s.desc
        );

        println!("✅ {}", s.desc);
    }
}
