# P-CryptoSuite

Open-source cryptography suite with two Rust crates:
- **p_hash/** – P-Hash-512 (morphogenetic 512-bit hash finalised with SHA-512)
- **p_key/**  – P-Key (deterministic public-key crypto built on the same SGA engine)

## Repository layout

    P-CryptoSuite/
    ├─ p_hash/
    │  ├─ Cargo.toml
    │  └─ src/lib.rs
    └─ p_key/
       ├─ Cargo.toml
       └─ src/lib.rs

## Build & test (local)

    cd p_hash && cargo build --release && cargo test --release
    cd ../p_key && cargo build --release && cargo test --release

## Security & bug bounty

See [SECURITY.md](./SECURITY.md).
