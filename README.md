# ZKCG Verifier

[![crates.io](https://img.shields.io/crates/v/zkcg-verifier.svg)](https://crates.io/crates/zkcg-verifier)
[![crates.io](https://img.shields.io/crates/v/zkcg-common.svg)](https://crates.io/crates/zkcg-common)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Sponsor](https://img.shields.io/badge/Sponsor-%E2%9D%A4-brightgreen)](https://github.com/sponsors/MRSKYWAY)

---

## Overview

**ZKCG Verifier** is the public, auditable verification layer of the ZKCG protocol.
 
* **Phase 1**: Halo2-based zk-SNARK verification
* **Phase 2 (planned)**: zkVM-based verification (RISC0)

This repository is intentionally **verifier-only**.
Anyone can independently verify proofs, audit the logic, and run verifier nodes.

---

## Important: Public Verifier Only

This repository contains **only public components**:

* Verification logic
* Shared protocol types and errors
* API interfaces
* Frozen parameters and specifications

The following are **intentionally excluded**:

* Proving circuits
* Proof generation code
* zkVM guest programs

Those components are maintained in a **private repository** while the project is developed by a solo maintainer.

Anyone can:

* Audit the verifier
* Run a verifier node
* Independently verify published proofs

Proof generation requires access to private components —
contact [@MRSKYWAY](https://github.com/MRSKYWAY) for collaboration or sponsored access.

---

## Repository Structure

```text
zkcg-verifier/
├── common/         # Shared types, errors, and protocol utilities (zkcg-common crate)
├── verifier/       # Core verifier logic (zkcg-verifier crate)
├── api/            # HTTP API for proof submission
├── SPEC.md         # Full protocol specification
├── CORE_FREEZE.md  # Frozen circuit parameters and commitments
├── SECURITY.md     # Security assumptions and reporting
├── LICENSE         # Apache-2.0
└── README.md       # This file
```

---

## Installation

Add the crates to your project:

```bash
cargo add zkcg-verifier zkcg-common
```

Or manually in `Cargo.toml`:

```toml
[dependencies]
zkcg-verifier = "0.1.0"
zkcg-common   = "0.1.0"
```

---

## Features

* `zk-halo2` — Enable Halo2 proof verification backend
* `zk-vm` — Enable zkVM (RISC0) verification support

Example:

```toml
zkcg-verifier = { version = "0.1.0", features = ["zk-halo2"] }
```

---

## 🐳 Docker Setup (Optional)

Docker is **optional**.

* Halo2 verification runs natively
* zkVM verification can run natively or in Docker
* Docker is recommended for **reproducible environments and CI**

### Install Docker (Ubuntu / WSL2)

```bash
sudo apt update
sudo apt install docker.io -y
sudo usermod -aG docker $USER
newgrp docker
```

Verify:

```bash
docker --version
```

---

### Build Docker Image

From the repository root:

```bash
docker build -t zkcg-verifier .
```


## 📊 Benchmarks

> **Environment**
>
> * Platform: Windows (WSL2, Ubuntu)
> * CPU: Intel i5 (10th Gen)
> * RAM: 16 GB
> * Build: Release
> * Parallelism: Default (no tuning)

---

### Halo2 (BN254, k = 9)

**Use case:** Interactive / near-real-time ZK policy verification

* **Prove:** ~306–316 ms
* **Verify:** ~9–10 ms
* **End-to-End:** ~317–351 ms

---

### zkVM (RISC0)

**Use case:** Audit-grade execution proofs

* **Prove:** ~13.7 seconds
* **Verify:** ~41–42 ns

---

### Summary

| Backend | Prove Time | Verify Time | Intended Use            |
| ------- | ---------- | ----------- | ----------------------- |
| Halo2   | ~310 ms    | ~9 ms       | Interactive ZK policies |
| zkVM    | ~13–17 s   | ~40 ns      | Audit / attestation     |

---

<!-- ## 🧪 Running Benchmarks

### Without Docker

```bash
cargo bench --bench halo2_prove
cargo bench --bench halo2_verify
cargo bench --bench halo2_prove_and_verify
```

### With Docker

```bash
docker run --rm zkcg-verifier cargo bench --bench zkvm_prove
docker run --rm zkcg-verifier cargo bench --bench zkvm_verify
```

--- -->
### End-to-End Simulation Results

#### Sequential Halo2 Simulation (1000 proofs)

```
Loans evaluated: 1000
Approvals: 128 (12.8%)

Prove total:   ~482.1 s
Verify total:  ~7.7 s
Throughput:    ~2.0 TPS
```

#### Parallel Halo2 Simulation (8 threads)

```
Loans evaluated: 1000
CPU threads: 8

Approvals: 130 (13.0%)

Prove total:   ~127.4 s
Verify total:  ~5.5 s
Throughput:    ~7.5 TPS
```

---

### Summary

| Backend | Prove Cost | Verify Cost | Throughput | Intended Use |
|------|-----------|------------|-----------|--------------|
| Halo2 (seq) | ~480 ms | ~7 ms | ~2 TPS | Interactive ZK policies |
| Halo2 (8-core) | ~127 ms | ~5 ms | ~7.5 TPS | Batch / off-chain proving |
| zkVM | ~13–17 s | ~40 ns | Prove-bound | Audit & attestation |

---

## Real-World Integration Example

ZKCG can be integrated into DeFi protocols for privacy-preserving verifications (e.g., credit score checks without revealing scores). See this demo in the [collateral_vault repository](https://github.com/MRSKYWAY/collateral_vault/blob/master/scripts/collateral_demo.ts), which shows the full on-chain + off-chain pipeline:

- **Off-Chain Proof Generation**: Generate a ZK proof using ZKCG's prover (Halo2 or zkVM) for conditions like "credit score > threshold".
- **Off-Chain Verification**: Call ZKCG's API (/v1/submit-proof) to verify the proof trustlessly.
- **On-Chain Settlement**: If verified, anchor the new state commitment on-chain (Solana program in collateral_vault) to approve loans or unlock collateral.

Run the demo: `ts-node collateral_demo.ts` (requires ZKCG API running locally).

This pipeline ensures fast off-chain processing (~340ms E2E for Halo2) with on-chain immutability.

## Contact

For questions, collaborations, or sponsorships, reach out:
- X (Twitter): [@sujyot]([https://x.com/sujyot](https://x.com/Sujyot10))
- GitHub Issues: Open in this repo for verifier discussions, or in [ZKCG private repo](https://github.com/MRSKYWAY/ZKCG) for prover/circuits.

---
## License

Apache-2.0

---

## Support the Project

ZKCG is built and maintained by a single developer.

👉 Sponsor: [https://github.com/sponsors/MRSKYWAY](https://github.com/sponsors/MRSKYWAY)


