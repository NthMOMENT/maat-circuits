#![no_main]
risc0_zkvm::guest::entry!(main);

use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
pub struct ConsistencyInput {
    /// Hash of dataset A (e.g. borrower-submitted financials)
    pub dataset_a_hash: [u8; 32],

    /// Hash of dataset B (e.g. MCA/IRS filed returns)
    pub dataset_b_hash: [u8; 32],

    /// Tolerance in basis points (0 = exact match required)
    /// e.g. 100 bps = within 1% is acceptable
    /// For hash comparison this must be 0 — exact match only
    pub tolerance_bps: u32,

    /// What is being compared
    pub label: String,
}

#[derive(Serialize, Deserialize)]
pub struct ConsistencyOutput {
    /// True = datasets are consistent
    pub is_consistent: bool,

    /// Hash of dataset A (public reference)
    pub dataset_a_hash: [u8; 32],

    /// Hash of dataset B (public reference)
    pub dataset_b_hash: [u8; 32],

    /// What was compared
    pub label: String,
}

fn main() {
    let input: ConsistencyInput = env::read();

    // Consistency check — hashes must match exactly
    // Neither dataset is revealed — only whether they agree
    let is_consistent = input.dataset_a_hash == input.dataset_b_hash;

    env::commit(&ConsistencyOutput {
        is_consistent,
        dataset_a_hash: input.dataset_a_hash,
        dataset_b_hash: input.dataset_b_hash,
        label: input.label,
    });
}

fn sha256_bytes(data: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(data);
    h.finalize().into()
}
