use p_hash::p_hash;

#[test]
fn digest_len_is_128_hex() {
    assert_eq!(p_hash(b"").len(), 128);
}

#[test]
fn repeatability() {
    let x1 = p_hash(b"syntaxos");
    let x2 = p_hash(b"syntaxos");
    assert_eq!(x1, x2);
}

#[test]
fn different_inputs_differ() {
    let a = p_hash(b"hello");
    let b = p_hash(b"world");
    assert_ne!(a, b);
}
