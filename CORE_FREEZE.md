# ZKCG Core Freeze — v0.1

This document defines the frozen invariants of the ZKCG protocol core.

## Frozen Components

- PublicInputs schema
- State transition semantics
- Halo2 circuit constraints
- zkVM guest logic and commit order
- ProofBackend interface

## Invariants

1. A proof is valid iff:
   - score <= threshold
   - public inputs match the committed values
   - backend cryptography verifies

2. Backends must be observationally equivalent:
   - Halo2
   - zkVM

3. State transitions are deterministic:
   - nonce monotonic
   - state_root binding enforced outside proof

## Non-Goals

- Generic proving SDK
- Production-ready CLI
- Performance guarantees

## Change Policy

Breaking changes are not allowed without:
- Version bump
- New freeze document

Status: FROZEN
Version: v0.1
