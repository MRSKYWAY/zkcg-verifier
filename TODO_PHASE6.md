# Phase 6 — Real Proof Verification (Halo2)

## Status: IN PROGRESS

### Tasks

#### Section 6.1 — Proof Generation (Prover-Side)
- [x] `halo2/prover/src/main.rs` uses real `create_proof` with KZG
- [x] Witness values come from real inputs (score, threshold)
- [x] Proof output is opaque bytes (Vec<u8>)

#### Section 6.2 — Proof Format & Transport
- [x] Proof represented as raw bytes (`Halo2Proof` struct)
- [x] Public inputs supplied separately via PublicInputs
- [x] No curve types leak into engine or API
- [x] Serialization/deserialization is deterministic

#### Section 6.3 — Real Halo2 Verification Backend
- [x] `verifier/src/backend_halo2.rs` exists with `verify_proof`
- [ ] Update `verifier/src/proof.rs` to use `Halo2Backend` instead of MockProver
- [ ] Remove MockProver usage from production path

#### Section 6.4 — Feature-Gated Production Path
- [x] Feature gating exists (`zk-halo2` in verifier/Cargo.toml)
- [x] Rust-only mode still works (StubBackend)
- [ ] Ensure heavy deps don't leak when feature disabled

#### Section 6.5 — Cryptographic Tests (Minimal, Real)
- [ ] Create tests demonstrating valid Halo2 proof is accepted
- [ ] Create tests demonstrating modified proof bytes are rejected
- [ ] Create tests demonstrating wrong public input is rejected
- [ ] Keep circuit unit tests with MockProver (development only)

### Completion Criteria
- [ ] No MockProver exists anywhere in verifier production code
- [ ] Real Halo2 proof can be generated
- [ ] That proof is verified using `verify_proof`
- [ ] Engine and API remain unchanged
- [ ] Tests demonstrate cryptographic rejection

### Files to Modify
1. `verifier/src/proof.rs` - Replace MockProver with Halo2Backend
2. `verifier/src/lib.rs` - Export Halo2Backend
3. `verifier/src/tests.rs` - Add cryptographic tests
4. `halo2/prover/src/main.rs` - Export params and vk for verifier use

### Files to Create
- `verifier/src/tests_halo2_crypto.rs` - Cryptographic tests for Halo2

