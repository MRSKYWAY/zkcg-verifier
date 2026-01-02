use zkcg_common::errors::ProtocolError;
use crate::engine::PublicInputs;

pub fn enforce(inputs: &PublicInputs) -> Result<(), ProtocolError> {
    // Phase 1: placeholder
    // Real constraint will be enforced inside ZK proof later

    if inputs.threshold == 0 {
        return Err(ProtocolError::PolicyViolation);
    }

    Ok(())
}
