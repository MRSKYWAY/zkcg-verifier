#![cfg(feature = "zk-halo2")]

use rand::rngs::OsRng;

use halo2_proofs::{
    circuit::Value,
    plonk::{create_proof, keygen_pk, keygen_vk},
    poly::commitment::Params,
    transcript::{Blake2bWrite, Challenge255, TranscriptWrite},
};
use halo2_proofs::arithmetic::Field;
use halo2curves::bn256::{Fr, G1Affine};

use circuits::score_circuit::ScoreCircuit;
use crate::{
    backend::ProofBackend,
    backend_halo2::Halo2Backend,
    engine::PublicInputs,
};

/// Generate a valid Halo2 proof using fresh params
fn generate_valid_proof(score: u64, threshold: u64) -> Vec<u8> {
    let circuit = ScoreCircuit::<Fr> {
        score: Value::known(Fr::from(score)),
        threshold: Value::known(Fr::from(threshold)),
    };

    let k = 9;
    let params: Params<G1Affine> = Params::new(k);

    let vk = keygen_vk(&params, &circuit).unwrap();
    let pk = keygen_pk(&params, vk, &circuit).unwrap();

    let public_inputs = vec![vec![Fr::from(threshold)]];
    let instance_slices: Vec<&[Fr]> =
        public_inputs.iter().map(|v| v.as_slice()).collect();
    let all_instances: Vec<&[&[Fr]]> =
        vec![instance_slices.as_slice()];

    let mut transcript =
        Blake2bWrite::<_, G1Affine, Challenge255<G1Affine>>::init(Vec::new());

    create_proof(
        &params,
        &pk,
        &[circuit],
        &all_instances,
        OsRng,
        &mut transcript,
    )
    .unwrap();

    transcript.finalize()
}

/// Generate a valid Halo2 proof using caller-supplied params
fn generate_valid_proof_with_params(
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
    let all_instances: Vec<&[&[Fr]]> =
        vec![instance_slices.as_slice()];

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

/// Construct a Halo2 verifier backend from params
fn backend(params: Params<G1Affine>) -> Halo2Backend {
    let dummy = ScoreCircuit::<Fr> {
        score: Value::known(Fr::ZERO),
        threshold: Value::known(Fr::ZERO),
    };

    let vk = keygen_vk(&params, &dummy).unwrap();
    Halo2Backend { vk, params }
}

#[test]
fn valid_halo2_proof_is_accepted() {
    let k = 9;
    let params: Params<G1Affine> = Params::new(k);

    let proof = generate_valid_proof_with_params(39, 40, &params);
    let backend = backend(params);

    let inputs = PublicInputs {
        threshold: 40,
        old_state_root: [0u8; 32],
        nonce: 1,
    };
    assert!(backend.verify(&proof, &inputs).is_ok());
}

#[test]
fn modified_proof_is_rejected() {
    let k = 9;
    let params: Params<G1Affine> = Params::new(k);

    let mut proof = generate_valid_proof_with_params(39, 40, &params);
    proof[10] ^= 0xFF;

    let backend = backend(params);

    let inputs = PublicInputs {
        threshold: 40,
        old_state_root: [0u8; 32],
        nonce: 1,
    };

    assert!(backend.verify(&proof, &inputs).is_err());
}

#[test]
fn wrong_public_input_is_rejected() {
    let k = 9;
    let params: Params<G1Affine> = Params::new(k);

    let proof = generate_valid_proof_with_params(39, 40, &params);
    let backend = backend(params);

    let wrong_inputs = PublicInputs {
        threshold: 41, // WRONG
        old_state_root: [0u8; 32],
        nonce: 1,
    };

    assert!(backend.verify(&proof, &wrong_inputs).is_err());
}

#[test]
fn empty_proof_is_rejected() {
    let k = 9;
    let params: Params<G1Affine> = Params::new(k);

    let backend = backend(params);

    let inputs = PublicInputs {
        threshold: 40,
        old_state_root: [0u8; 32],
        nonce: 1,
    };

    assert!(backend.verify(&[], &inputs).is_err());
}