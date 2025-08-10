P‑CryptoSuite

<!-- NOTE: the CI badge below works only if .github/workflows/ci.yml exists -->


Open‑source cryptography suite with two Rust crates:
	•	p_hash/ – P‑Hash‑512 (morphogenetic 512‑bit hash finalised with SHA‑512)
	•	p_key/  – P‑Key (deterministic public‑key crypto built on the same SGA engine)

Repository layout

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

Build & test (local)

cargo build --workspace --release
cargo test  --workspace --release

Minimal usage

P‑Hash

use p_hash::p_hash;

fn main() {
    let digest_hex = p_hash(b"hello");
    assert_eq!(digest_hex.len(), 128); // 512-bit (128 hex chars)
    println!("{}", digest_hex);
}

P‑Key

use p_key::{generate_keypair, encrypt, decrypt};

fn main() {
    let (pubk, _privk) = generate_keypair("my passphrase");
    let pkg = encrypt(&pubk, "hello world");
    let clear = decrypt(&pubk, &pkg);
    assert_eq!(clear, "hello world");
}

Security & bug bounty

See SECURITY.md.

License

Apache‑2.0 — see LICENSE.
