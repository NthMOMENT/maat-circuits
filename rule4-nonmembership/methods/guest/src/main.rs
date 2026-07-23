#![no_main]
risc0_zkvm::guest::entry!(main);

use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
pub struct NonMembershipInput {
    /// Hash of the entity being checked
    pub entity_hash: [u8; 32],

    /// The watchlist — hashes of flagged entities
    /// Examples:
    ///   - NCLT/bankruptcy filings
    ///   - Inter-creditor dispute register
    ///   - OFAC/sanctions list
    ///   - RBI defaulter list
    ///   - IBBI insolvency register
    pub watchlist: Vec<[u8; 32]>,

    /// What watchlist is being checked
    pub label: String,
}

#[derive(Serialize, Deserialize)]
pub struct NonMembershipOutput {
    /// True = entity is NOT on the watchlist (clean)
    pub is_clean: bool,

    /// Hash of the watchlist snapshot
    pub watchlist_snapshot_hash: [u8; 32],

    /// What was checked
    pub label: String,
}

fn main() {
    let input: NonMembershipInput = env::read();

    // Entity must NOT be in the watchlist
    let is_clean = !input.watchlist.contains(&input.entity_hash);

    let watchlist_snapshot_hash = hash_watchlist(&input.watchlist);

    env::commit(&NonMembershipOutput {
        is_clean,
        watchlist_snapshot_hash,
        label: input.label,
    });
}

fn hash_watchlist(watchlist: &[[u8; 32]]) -> [u8; 32] {
    let mut h = Sha256::new();
    for entry in watchlist {
        h.update(entry);
    }
    h.finalize().into()
}
