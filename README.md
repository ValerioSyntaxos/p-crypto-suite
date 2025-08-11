# P‑CryptoSuite
> **🚨 Bug bounty paused (Aug 10, 2025).**  
> We are addressing a critical report in `p_key`. No new submissions will be accepted until **v0.1.1** is released.  
> Details: https://github.com/ValerioSyntaxos/p-crypto-suite/issues/1
Open‑source cryptography suite with two Rust crates:

* **p\_hash/** – P‑Hash‑512 (morphogenetic 512‑bit hash finalised with SHA‑512)
* **p\_key/**  – P‑Key (deterministic public‑key crypto built on the same SGA engine)

## Repository layout

```text
P-CryptoSuite/
├─ Cargo.toml            # workspace
├─ p_hash/
│  ├─ Cargo.toml
│  └─ src/
│     └─ lib.rs
└─ p_key/
   ├─ Cargo.toml
   └─ src/
      └─ lib.rs
```

## Build & test (local)

```bash
cargo build --workspace --release
cargo test  --workspace --release
```

## Minimal usage

### P‑Hash

```rust
use p_hash::p_hash;

fn main() {
    let digest_hex = p_hash(b"hello");
    assert_eq!(digest_hex.len(), 128); // 512-bit (128 hex chars)
    println!("{}", digest_hex);
}
```

### P‑Key

```rust
use p_key::{generate_keypair, encrypt, decrypt};

fn main() {
    let (pubk, _privk) = generate_keypair("my passphrase");
    let pkg = encrypt(&pubk, "hello world");
    let clear = decrypt(&pubk, &pkg);
    assert_eq!(clear, "hello world");
}
```

## Security & bug bounty
- Active bounty → [P-Crypto Bug Bounty — Phase 1](https://github.com/ValerioSyntaxos/p-crypto-suite/issues/1)
See [SECURITY.md](./SECURITY.md).

## License

Apache‑2.0 — see [LICENSE](./LICENSE).
