use std::convert::TryInto;

const P: u64 = 2_147_483_647;
const STATE_SIZE: usize = 3;
const RATE: usize = 3;
const ROUNDS: usize = 8;

const ROUND_CONSTANTS: [u64; ROUNDS * STATE_SIZE] = [
    5, 7, 11, 3, 13, 17, 19, 23, 29,
    31, 37, 41, 43, 47, 53, 59, 61, 67,
    71, 73, 79, 83, 89, 97,
];
const MDS: [[u64; STATE_SIZE]; STATE_SIZE] = [
    [2, 1, 1],
    [1, 2, 1],
    [1, 1, 2],
];

fn add_mod(a: u64, b: u64) -> u64 {
    let s = a.wrapping_add(b);
    if s >= P { s - P } else { s }
}
fn mul_mod(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % P as u128) as u64
}
fn pow_mod(mut x: u64, mut e: u64) -> u64 {
    let mut res = 1;
    x %= P;
    while e > 0 {
        if e & 1 == 1 { res = mul_mod(res, x); }
        x = mul_mod(x, x);
        e >>= 1;
    }
    res
}
fn sbox(x: u64) -> u64 { pow_mod(x, 5) }
fn mds_multiply(state: &[u64; STATE_SIZE]) -> [u64; STATE_SIZE] {
    let mut out = [0; STATE_SIZE];
    for i in 0..STATE_SIZE {
        let mut acc = 0;
        for j in 0..STATE_SIZE {
            acc = add_mod(acc, mul_mod(MDS[i][j], state[j]));
        }
        out[i] = acc;
    }
    out
}




fn poseidon_permutation(mut state: [u64; STATE_SIZE]) -> [u64; STATE_SIZE] {
    for r in 0..ROUNDS {
        for i in 0..STATE_SIZE {
            state[i] = add_mod(state[i], ROUND_CONSTANTS[r * STATE_SIZE + i] % P);
        }
        for i in 0..STATE_SIZE {
            state[i] = sbox(state[i]);
        }
        state = mds_multiply(&state);
    }
    state
}


fn poseidon_hash(message: &[u64]) -> Vec<u64> {
    let mut state = [0; STATE_SIZE];
    let mut idx = 0;
    while idx < message.len() {
        for i in 0..RATE {
            if idx + i < message.len() {
                state[i] = add_mod(state[i], message[idx + i] % P);
            }
        }
        state = poseidon_permutation(state);
        idx += RATE;
    }
    state[0..RATE].to_vec()
}


fn bytes_to_field_elements(bytes: &[u8]) -> Vec<u64> {
    let mut elems = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let take = std::cmp::min(8, bytes.len() - i);
        println!("take: {}", take);
        let mut buf = [0u8; 8];
        buf[..take].copy_from_slice(&bytes[i..i + take]);
        elems.push(u64::from_le_bytes(buf) % P);
        i += take;
    }
    elems
}

fn main() {
    let input = b"t";
    let elems = bytes_to_field_elements(input);

    let digest1 = poseidon_hash(&elems);
    let digest2 = poseidon_hash(&elems);

    println!("Input: {:?}", input);
    println!("Digest1: {:?}", digest1);
    println!("Digest2: {:?}", digest2);

    assert_eq!(digest1, digest2, "Poseidon hash mismatch!");
    println!("✅ Toy Poseidon hashes match!");
}
