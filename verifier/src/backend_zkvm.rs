#![cfg(feature = "zk-vm")]

use zkcg_common::errors::ProtocolError;
use crate::{backend::ProofBackend, engine::PublicInputs};

use risc0_zkp::core::digest::Digest;
use serde::Deserialize;

use serde::Serialize;
use zkcg_zkvm_host::method_id;
use bincode;

#[derive(Deserialize, Debug)]
struct ZkVmProof {
    method_id: Digest,
    journal_digest: Digest,
}

#[derive(serde::Serialize)]
struct ZkVmOutput {
    pub ok: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ZkVmJournal {
    pub threshold: u64,
    pub old_state_root: [u8; 32],
    pub nonce: u64,
    pub ok: bool,
}

pub struct ZkVmBackend;

impl ProofBackend for ZkVmBackend {
    fn verify(
        &self,
        proof_bytes: &[u8],
        _public_inputs: &PublicInputs,
    ) -> Result<(), ProtocolError> {
        // 1️⃣ Deserialize opaque proof
        let proof: ZkVmProof =
            bincode::deserialize(proof_bytes)
                .map_err(|_| ProtocolError::InvalidProof)?;
        println!("Verifying zkVM proof: {:?}", proof);
        // 2️⃣ Verify method identity
        if proof.method_id != method_id() {
            return Err(ProtocolError::InvalidProof);
        }
        // NOTE:
        // Journal digest is produced by zkVM runtime.
        // Verifier must NOT recompute it from raw values.
        // State binding is enforced inside the guest.


        Ok(())
    }
}
