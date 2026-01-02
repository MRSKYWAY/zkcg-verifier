# ZK-Verified Computation Gateway (ZKCG)

ZK-Verified Computation Gateway (ZKCG) is a Rust-based protocol node that enables **trustless verification of off-chain computation** using zero-knowledge proofs.

The system allows clients to submit proofs that a computation was executed correctly **and** satisfies protocol-defined policies, without revealing private inputs or requiring the verifier to re-execute the computation.

---

## Motivation

Modern systems increasingly rely on off-chain computation for performance, scalability, and privacy reasons. However, verifiers currently face a difficult tradeoff:

- Trust the computation provider ❌
- Re-execute the computation ❌
- Centralize computation ❌

ZKCG resolves this by verifying **zero-knowledge proofs of correct computation**, allowing results to be accepted without trust or recomputation.

---

## Design Goals

- **Protocol-first**: Explicit state machine and deterministic transitions
- **Trustless verification**: No trust in the prover or execution environment
- **Privacy-preserving**: Private inputs are never revealed
- **Production-oriented**: Long-running verifier node, not a demo
- **Extensible**: Supports multiple proof backends (circuits, zkVMs)

---

## High-Level Architecture



Prover (Client)
|
|-- private computation
|-- ZK proof generation
v
ZKCG Verifier Node (Rust)
|
|-- proof verification
|-- policy enforcement
|-- state transition
v
Persistent Protocol State


---

## Core Concepts

### Actors
- **Prover**: Performs computation and generates a ZK proof
- **Verifier Node**: Validates proofs and enforces protocol rules
- **Observer**: Reads public protocol state (optional)

### State
The protocol maintains a deterministic state consisting of:
- Merkle commitment root
- Monotonically increasing nonce
- Epoch/version identifier

### Policy Enforcement
A proof is accepted only if:
- The ZK proof verifies successfully
- Protocol-defined policy constraints are satisfied
- State transition rules are respected

A valid proof alone is **not sufficient** to update state.

---

## Use Case (Phase 1)

**Private Risk / Score Verification**

A prover demonstrates that a score computed from private data satisfies a public threshold, without revealing the underlying data or intermediate values.

This pattern applies to:
- credit or risk checks
- compliance validation
- private eligibility proofs

---

## Roadmap

### Phase 1 (Core Protocol)
- Deterministic state machine
- Circuit-level ZK proof verification
- Policy enforcement
- Replay protection
- Persistent state storage

### Phase 2 (Power Move)
- Pluggable proof backends
- zkVM integration
- Proof backend abstraction
- Comparative benchmarks

---

## Repository Structure



zk-compute-gateway/
├── SPEC.md # Protocol specification
├── SECURITY.md # Threat model and assumptions
├── verifier/ # Rust verifier node
├── circuits/ # ZK circuits
├── zkvm/ # zkVM integrations (Phase 2)
├── tests/
├── benches/
└── docs/


---

## Status

This project is under active development and is currently **pre-release**.
Interfaces and specifications may evolve.

---

## License

This project is licensed under the Apache License, Version 2.0.

The Apache-2.0 license was chosen to allow broad use in both open-source and commercial systems, while providing an explicit patent grant.
