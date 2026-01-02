use zkcg_common::errors::ProtocolError;
use crate::engine::PublicInputs;

pub trait ProofBackend: Send + Sync {
    fn verify(
        &self,
        proof_bytes: &[u8],
        public_inputs: &PublicInputs,
    ) -> Result<(), ProtocolError>;
}
