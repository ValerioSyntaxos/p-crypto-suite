//! p_hash_core — Core Rust implementation of the P-Hash-512 algorithm

use sha2::{Digest, Sha512};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

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

const GRAMMATICAL_SALT: u64 = 7 * 19 * 3 * 41 * 2;
const N_ITERATIONS: u64 = 1_000_000;
const FINAL_PROM_CNT: usize = 32;

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

#[derive(Debug)]
struct MorphogeneticSignature {
    delta: u64,
    nu: u64,
    phi: u64,
    rho: u64,
    omega: u64,
}

fn calculate_signature(prom: &[u64]) -> MorphogeneticSignature {
    if prom.is_empty() {
        return MorphogeneticSignature {
            delta: 0,
            nu: 0,
            phi: 0,
            rho: 0,
            omega: 0,
        };
    }
    let delta = prom.len() as u64;
    let nu = prom.iter().collect::<HashSet<_>>().len() as u64;
    let phi = *prom.iter().max().unwrap();
    let rho = phi - *prom.iter().min().unwrap();
    let omega = prom.iter().sum::<u64>() % 999_999_937;
    MorphogeneticSignature {
        delta,
        nu,
        phi,
        rho,
        omega,
    }
}

pub fn p_hash(data: &[u8]) -> String {
    let mut input_primes: Vec<u64> = data
        .iter()
        .map(|b| PRIME_TABLE[(*b as usize) % PRIME_TABLE.len()])
        .collect();
    if input_primes.is_empty() {
        input_primes.push(2);
    }

    let salted: Vec<u64> = input_primes.iter().map(|p| p * GRAMMATICAL_SALT).collect();

    let mut seed = 1u64;
    for p in salted {
        seed = (seed.wrapping_mul(31).wrapping_add(p)) % 1_000_000_007;
    }

    let promoters = sga_engine(seed, N_ITERATIONS);

    let filler_iter = [2u64, 3, 5, 7].iter().copied().cycle();
    let final_prom: Vec<u64> = promoters
        .iter()
        .rev()
        .copied()
        .chain(filler_iter)
        .take(FINAL_PROM_CNT)
        .collect();

    let sign = calculate_signature(&promoters);
    let final_string = format!(
        "{}:{},{},{},{},{}",
        final_prom
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(","),
        sign.delta,
        sign.nu,
        sign.phi,
        sign.rho,
        sign.omega
    );

    let mut h = Sha512::new();
    h.update(final_string.as_bytes());
    format!("{:x}", h.finalize())
}
