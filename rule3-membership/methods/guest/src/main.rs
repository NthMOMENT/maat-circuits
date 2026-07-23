#![no_main]
risc0_zkvm::guest::entry!(main);

use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
pub struct MembershipInput {
    /// Hash of the entity being checked (e.g. hashed EIN, wallet, DID)
    /// Never reveals the actual identity — only the hash is used
    pub entity_hash: [u8; 32],

    /// The verified set — hashes of all approved entities
    /// Examples:
    ///   - KYC-cleared borrowers
    ///   - BBB+ rated anchors
    ///   - Licensed counterparties
    ///   - Approved trade corridors
    pub approved_set: Vec<[u8; 32]>,

    /// What set is being checked against
    pub label: String,
}

#[derive(Serialize, Deserialize)]
pub struct MembershipOutput {
    /// True = entity exists in the approved set
    pub is_member: bool,

    /// Hash of the approved set snapshot — proves which set was checked
    pub set_snapshot_hash: [u8; 32],

    /// What was checked
    pub label: String,
}

fn main() {
    let input: MembershipInput = env::read();

    // Check membership — entity hash must exist in approved set
    let is_member = input.approved_set.contains(&input.entity_hash);

    // Hash the set snapshot for provenance
    let set_snapshot_hash = hash_set(&input.approved_set);

    env::commit(&MembershipOutput {
        is_member,
        set_snapshot_hash,
        label: input.label,
    });
}

fn hash_set(set: &[[u8; 32]]) -> [u8; 32] {
    let mut h = Sha256::new();
    for entry in set {
        h.update(entry);
    }
    h.finalize().into()
}
