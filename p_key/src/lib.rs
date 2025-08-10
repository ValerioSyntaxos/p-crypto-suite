use hex::encode as hex_encode;
use rand::{rngs::OsRng, RngCore};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const PRIME_TABLE: [u64; 168] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547,
    557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659,
    661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797,
    809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929,
    937, 941, 947, 953, 967, 971, 977, 983, 991, 997,
];
const N_ITERATIONS: u64 = 1_000_000;
const TRACE_LEN: usize = 1_000_000;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicKey(pub String); // "PKEY:1:<hex>"

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PrivateKey(pub Vec<u64>);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EncryptedPackage {
    pub nonce: u64,
    pub ciphertext: Vec<u8>,
}

fn sga_engine(seed: u64, iterations: u64) -> Vec<u64> {
    let mut promoters = Vec::new();
    let mut queue: BinaryHeap<Reverse<(u64, u64)>> = BinaryHeap::new();
    let mut current = 2u64;

    for i in 0..iterations {
        if queue.is_empty() {
            promoters.push(current);
            let job = current.saturating_mul(current) + seed % current;
            queue.push(Reverse((job, current)));
            current += 1;
            continue;
        }

        if let Some(Reverse((next_comp, _))) = queue.peek() {
            if current < *next_comp {
                promoters.push(current);
                let job = current.saturating_mul(current) + seed % current;
                queue.push(Reverse((job, current)));
                current += 1;
            } else {
                while let Some(Reverse((comp, prom_ref))) = queue.peek() {
                    if *comp != current {
                        break;
                    }
                    let prom = *prom_ref;
                    queue.pop();
                    let nxt = current
                        .saturating_add(prom)
                        .saturating_add(seed % (prom + i + 1));
                    queue.push(Reverse((nxt, prom)));
                }
                current += 1;
            }
        }
    }
    promoters
}

fn build_trace(pass: &str) -> Vec<u64> {
    let mut seed = 1u64;
    for b in pass.bytes() {
        let p = PRIME_TABLE[(b as usize) % PRIME_TABLE.len()];
        seed = (seed.wrapping_mul(31).wrapping_add(p)) % 1_000_000_007;
    }
    sga_engine(seed, N_ITERATIONS)
        .into_iter()
        .take(TRACE_LEN)
        .collect()
}

pub fn generate_keypair(pass: &str) -> (PublicKey, PrivateKey) {
    let mut trace = build_trace(pass);
    let mut salt = [0u8; 8];
    OsRng.fill_bytes(&mut salt);
    trace[0] ^= u64::from_le_bytes(salt);

    let mut h = Sha3_256::new();
    for t in &trace {
        h.update(t.to_le_bytes());
    }
    let pub_hex = format!("PKEY:1:{}", hex_encode(h.finalize()));
    (PublicKey(pub_hex), PrivateKey(trace))
}

pub fn encrypt(pub_key: &PublicKey, message: &str) -> EncryptedPackage {
    let mut nonce_bytes = [0u8; 8];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = u64::from_le_bytes(nonce_bytes);

    let mut h = Sha3_256::new();
    h.update(pub_key.0.as_bytes());
    h.update(nonce.to_le_bytes());
    let mut keystream = h.finalize().to_vec();
    while keystream.len() < message.len() {
        let last = Sha3_256::digest(&keystream[keystream.len() - 32..]);
        keystream.extend_from_slice(&last);
    }

    let ciphertext: Vec<u8> = message
        .as_bytes()
        .iter()
        .zip(keystream)
        .map(|(b, k)| b ^ k)
        .collect();

    EncryptedPackage { nonce, ciphertext }
}

pub fn decrypt(pub_key: &PublicKey, pkg: &EncryptedPackage) -> String {
    let mut h = Sha3_256::new();
    h.update(pub_key.0.as_bytes());
    h.update(pkg.nonce.to_le_bytes());
    let mut keystream = h.finalize().to_vec();
    while keystream.len() < pkg.ciphertext.len() {
        let last = Sha3_256::digest(&keystream[keystream.len() - 32..]);
        keystream.extend_from_slice(&last);
    }

    let plain: Vec<u8> = pkg
        .ciphertext
        .iter()
        .zip(keystream)
        .map(|(b, k)| b ^ k)
        .collect();

    String::from_utf8_lossy(&plain).into_owned()
}
