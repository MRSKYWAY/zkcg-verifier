# Security Model

This document outlines the threat model, security assumptions, and non-goals of the ZK-Verified Computation Gateway (ZKCG).

---

## Threat Model

### Adversary Capabilities
An adversary may:
- Submit malformed or adversarial proofs
- Attempt to replay previously valid proofs
- Attempt to submit proofs that violate protocol policy
- Attempt to cause verifier denial-of-service via invalid inputs
- Attempt to infer private inputs from public data

The adversary **cannot**:
- Break standard cryptographic assumptions
- Compromise the verifier host environment (out of scope)

---

## Security Properties

ZKCG is designed to provide the following guarantees:

### 1. Correctness
Only computations that satisfy the protocol-defined constraints and policies can produce valid state transitions.

### 2. Soundness
Invalid computations cannot produce valid proofs under the assumed security of the underlying proof system.

### 3. Replay Protection
Each state transition requires a strictly increasing nonce, preventing replay of previously accepted proofs.

### 4. Privacy
Private inputs used during computation are never revealed to the verifier. Only public inputs and commitments are exposed.

### 5. Deterministic State Transitions
Given the same prior state and inputs, the verifier produces the same next state.

---

## Assumptions

ZKCG assumes:
- The cryptographic soundness of the underlying zero-knowledge proof system
- Correct implementation of cryptographic primitives
- Correctness of the verifier node implementation
- Secure key management by provers

---

## Out of Scope

The following are explicitly **out of scope**:
- Compromise of the verifier host or operating system
- Side-channel attacks on prover environments
- Economic or incentive-layer attacks
- Network-level censorship or availability guarantees

---

## Reporting Vulnerabilities

Security vulnerabilities should be reported **privately**.

Please contact:
- Email: security@zkcg.local (placeholder)

Do not open public issues for security-sensitive bugs.
