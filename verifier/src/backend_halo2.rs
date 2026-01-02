#![cfg(feature = "zk-halo2")]

use zkcg_common::errors::ProtocolError;
use crate::{
    backend::ProofBackend,
    engine::PublicInputs,
};

use halo2_proofs::{
    plonk::{verify_proof, VerifyingKey, SingleVerifier},
    poly::commitment::Params,
    transcript::{Blake2bRead, Challenge255},
};

use halo2curves::bn256::{Fr, G1Affine};

/// Real Halo2 verifier backend (runtime keys, KZG implicit)
pub struct Halo2Backend {
    pub vk: VerifyingKey<G1Affine>,
    pub params: Params<G1Affine>,
}

impl ProofBackend for Halo2Backend {
    fn verify(
        &self,
        proof_bytes: &[u8],
        public_inputs: &PublicInputs,
    ) -> Result<(), ProtocolError> {
        // --- public inputs (instance columns)
        let threshold = Fr::from(public_inputs.threshold as u64);

        let instance_values = vec![vec![threshold]];
        let instance_slices: Vec<&[Fr]> =
            instance_values.iter().map(|v| v.as_slice()).collect();
        let all_instances: Vec<&[&[Fr]]> =
            vec![instance_slices.as_slice()];

        // --- transcript
        let mut transcript =
            Blake2bRead::<_, G1Affine, Challenge255<G1Affine>>::init(proof_bytes);

        // --- verification strategy
        let strategy = SingleVerifier::new(&self.params);
        println!("Starting proof verification...");
        println!("Public inputs: {:?}", all_instances);
        println!("Proof bytes length: {}", proof_bytes.len());
        // println!("Params: {:?}", self.params);
        // println!("Transcript state: {:?}", transcript);
        println!("Using SingleVerifier strategy.");
        // --- verify
        verify_proof(
            &self.params,
            &self.vk,
            strategy,
            &all_instances,
            &mut transcript,
        )
        .map_err(|_| ProtocolError::InvalidProof)?;

        Ok(())
    }
}

