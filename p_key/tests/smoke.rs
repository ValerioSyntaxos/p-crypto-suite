use p_key::{decrypt, encrypt, generate_keypair, EncryptedPackage};

#[test]
fn pubkey_prefix_is_correct() {
    let (pk, _sk) = generate_keypair("syntaxos");
    assert!(pk.0.starts_with("PKEY:1:"));
    assert_eq!(pk.0.len(), "PKEY:1:".len() + 64); // sha3-256 hex = 64
}

#[test]
fn encrypt_decrypt_roundtrip() {
    let (pk, _sk) = generate_keypair("syntaxos");
    let msg = "hello, world!";
    let pkg: EncryptedPackage = encrypt(&pk, msg);
    let out = decrypt(&pk, &pkg);
    assert_eq!(out, msg);
}
