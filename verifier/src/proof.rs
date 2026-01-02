use zkcg_common::errors::ProtocolError;
use crate::{
    engine::PublicInputs,
    backend::ProofBackend,
};


pub struct ProofInput<'a> {
    pub proof_bytes: &'a [u8],
    pub public_inputs: &'a PublicInputs,
}

pub fn verify(
    proof: ProofInput<'_>,
) -> Result<(), ProtocolError> {
    #[cfg(feature = "zk-halo2")]
    {
        use crate::backend_halo2::Halo2Backend;
        

        let artifacts = circuits::halo2_artifacts::verifier_artifacts();


        let backend = Halo2Backend {
            vk: artifacts.vk,
            params: artifacts.params,
        };

        return backend.verify(proof.proof_bytes, proof.public_inputs);
    }

    #[cfg(not(feature = "zk-halo2"))]
    {
        Ok(())
    }
}
