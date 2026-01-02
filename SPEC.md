# Protocol Specification — ZK-Verified Computation Gateway (ZKCG)

This document specifies the core protocol, state machine, proof interfaces, and transition rules of the **ZK-Verified Computation Gateway (ZKCG)**.

It is designed to be:

- **Precise** — deterministic in behavior  
- **Auditable** — comprehensible by other engineers  
- **Robust** — covers edge cases and error conditions  

---

## Table of Contents

1. Protocol Overview  
2. Actors  
3. Core Concepts  
4. State Definition  
5. Message Formats  
6. Valid State Transition Rules  
7. Policy Constraints  
8. Verifier Semantics  
9. Error Codes & Rejections  
10. Extensions (Phase 2)  

---

## 1. Protocol Overview

ZKCG is a verifier protocol that enables clients (provers) to submit zero-knowledge proofs attesting that a computation was executed correctly and adheres to specific policy constraints.

A verifier node validates the proof and updates the protocol state when all checks pass.

---

## 2. Actors

- **Prover (Client)** — Executes computation off-chain and produces a ZK proof  
- **Verifier Node** — Validates proofs, enforces policies, updates state  
- **Observer** — Optional read-only entity monitoring public state  

All actors may be real machines in a distributed system.

---

## 3. Core Concepts

### 3.1 Proof

A zero-knowledge proof attesting to the correctness of a computation with respect to given public inputs.

### 3.2 Public Inputs

Data included in each proof and required for verification, such as:

- protocol version  
- threshold values  
- previous state commitment  

### 3.3 Private Inputs

Data used by the prover but not revealed to the verifier.

### 3.4 Commitment

A cryptographic commitment (e.g., Merkle root) representing the post-computation state.

---

## 4. State Definition

The verifier maintains a deterministic state:

```rust
struct ProtocolState {
    state_root: Hash,
    nonce: u64,
    epoch: u64,
}
```

- `state_root`: Merkle commitment representing current state  
- `nonce`: Strictly increasing counter  
- `epoch`: Version or generation identifier  

---

## 5. Message Formats

### 5.1 Proof Submission

```json
{
  "proof": "<base64-encoded proof>",
  "public_inputs": {
    "threshold": "<uint64>",
    "old_state_root": "<hash>",
    "nonce": "<uint64>"
  },
  "new_state_commitment": "<hash>"
}
```

---

## 6. Valid State Transition Rules

A transition is valid if **all** of the following hold:

1. `public_inputs.old_state_root == current.state_root`  
2. `public_inputs.nonce == current.nonce + 1`  
3. The ZK proof is valid  
4. The computed result satisfies all policy constraints  
5. `new_state_commitment` correctly reflects the post-computation state  

If any rule fails, the submission is rejected.

---

## 7. Policy Constraints

### Phase 1 Constraint

A private risk or score check is enforced:

```
computed_score ≤ threshold
```

This constraint **must be embedded in the proof** and cannot be bypassed by the prover.

---

## 8. Verifier Semantics

Upon receiving a proof submission, the verifier performs the following steps:

1. Parse the message  
2. Validate message format  
3. Check that `old_state_root` and `nonce` match current state  
4. Verify the ZK proof using the provided public inputs  
5. Enforce policy constraints  
6. Compute and persist the new state  
7. Emit an event or log entry  

All steps are deterministic.

---

## 9. Error Codes & Rejections

| Code | Meaning |
|----|----|
| `ERR_INVALID_FORMAT` | Bad message structure |
| `ERR_STATE_MISMATCH` | Old state does not match current |
| `ERR_NONCE_INVALID` | Invalid nonce |
| `ERR_PROOF_INVALID` | Proof verification failed |
| `ERR_POLICY_VIOLATION` | Policy constraint not satisfied |
| `ERR_COMMITMENT_MISMATCH` | New commitment does not match |

Each error must be returned to the client and logged by the verifier.

---

## 10. Extensions (Phase 2)

### 10.1 Pluggable Proof Backends

ZKCG supports multiple proof systems:

- Circuit-based proofs (e.g., Halo2)  
- zkVM proofs (e.g., RISC Zero, SP1)  

The verifier interface remains stable; only backend verification logic differs.

---

### 10.2 Versioning

The `epoch` field enables protocol upgrades and routes verification logic to the correct version.

---

## Provenance Statement

This specification is designed to be:

- Unambiguous  
- Machine-verifiable  
- Extensible  

All state transitions and policy checks are deterministic.
