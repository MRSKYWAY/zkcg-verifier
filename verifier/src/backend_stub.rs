use zkcg_common::errors::ProtocolError;
use crate::{
    backend::ProofBackend,
    engine::PublicInputs,
};

#[derive(Default)]
pub struct StubBackend;

impl ProofBackend for StubBackend {
    fn verify(
        &self,
        _proof_bytes: &[u8],
        _public_inputs: &PublicInputs,
    ) -> Result<(), ProtocolError> {
        // Rust-only mode: always accept
        Ok(())
    }
}
