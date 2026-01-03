# ZKCG Verifier

[![crates.io](https://img.shields.io/crates/v/zkcg-verifier.svg)](https://crates.io/crates/zkcg-verifier)
[![crates.io](https://img.shields.io/crates/v/zkcg-common.svg)](https://crates.io/crates/zkcg-common)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Sponsor](https://img.shields.io/badge/Sponsor-%E2%9D%A4-brightgreen)](https://github.com/sponsors/MRSKYWAY)

Public verifier and protocol library for **ZKCG** — a trustless, privacy-preserving protocol for off-chain computation verification using zero-knowledge proofs.

- **Phase 1**: Halo2-based zk-SNARKs
- **Phase 2** (planned): zkVM integration

The verifier is fully open-source and auditable so anyone can independently verify proofs and run verifier nodes.

## Important: Public Verifier Only

This repository contains **only the public components**:

- Verification logic
- Shared types and protocol definitions
- API interfaces
- Frozen parameters and specification

The **proving circuits**, proof generation code, and zkVM guest programs are kept in a private repository to protect core intellectual property while developing as a solo maintainer.

Anyone can:
- Audit the verification logic
- Run a verifier node
- Independently verify published proofs

Proof generation requires access to the private components — contact [@MRSKYWAY](https://github.com/MRSKYWAY) for collaboration or sponsored access.

## Repository Structure
```
zkcg-verifier/
├── common/         # Shared types, errors, and protocol utilities (zkcg-common crate)
├── verifier/       # Core verifier node and proof verification logic (zkcg-verifier crate)
├── api/            # API interfaces for proof submission and client interaction
├── SPEC.md         # Full protocol specification
├── CORE_FREEZE.md  # Frozen circuit parameters and commitments
├── SECURITY.md     # Security assumptions and reporting
├── LICENSE         # Apache-2.0
└── README.md       # This file
```

## Installation

Add the crates to your project:

```bash
cargo add zkcg-verifier zkcg-common
```
Or manually in Cargo.toml:
```
toml[dependencies]
zkcg-verifier = "0.1.0"
zkcg-common   = "0.1.0"
```

## Features

zk-halo2: Enable Halo2 proof verification backend (requires halo2_proofs, ff, halo2curves)
zk-vm: Enable RISC0 zkVM verification support

Example:
```
zkcg-verifier = { version = "0.1.0", features = 
"zk-halo2"] }
```

## Basic Usage
```Rust
// Example coming soon — loading a verification key and verifying a proof
// See examples/ directory (to be added) for full workflows
```

Documentation

Protocol Specification
Core Freeze Commitments
Security Model

## Contributing
Contributions to the verifier, documentation, tests, and examples are welcome!
Please open an issue first for major changes. See CONTRIBUTING.md (to be added).
## License
Licensed under the Apache License, Version 2.0.
See LICENSE for details.
## Support the Project
ZKCG is built and maintained by a single developer. Sponsorship helps dedicate more time to development, documentation, and community support instead of contract work.
Sponsor
Thank you for supporting independent zk development! 🚀