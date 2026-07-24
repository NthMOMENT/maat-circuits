use anyhow::Result;
use methods::NONMEMBERSHIP_GUEST_ELF;
use risc0_zkvm::{ExecutorEnv, ExternalProver};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
pub struct NonMembershipInput {
    pub entity_hash: [u8; 32],
    pub watchlist: Vec<[u8; 32]>,
    pub label: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NonMembershipOutput {
    pub is_clean: bool,
    pub watchlist_snapshot_hash: [u8; 32],
    pub label: String,
}

fn sha256_str(s: &str) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(s.as_bytes());
    h.finalize().into()
}

fn prove_nonmembership(
    label: &str,
    entity: &str,
    watchlist: Vec<&str>,
) -> Result<NonMembershipOutput> {
    let entity_hash = sha256_str(entity);
    let watchlist: Vec<[u8; 32]> = watchlist.iter().map(|e| sha256_str(e)).collect();

    let input = NonMembershipInput {
        entity_hash,
        watchlist,
        label: label.to_string(),
    };

    let env = ExecutorEnv::builder().write(&input)?.build()?;
    let prover = ExternalProver::new("gpu", "/home/rammint/.cargo/bin/r0vm");
    let receipt = prover.prove(env, NONMEMBERSHIP_GUEST_ELF)?.receipt;
    receipt.verify(methods::NONMEMBERSHIP_GUEST_ID)?;
    let output: NonMembershipOutput = receipt.journal.decode()?;
    Ok(output)
}

fn print_result(output: &NonMembershipOutput) {
    println!(
        "  {} → {}",
        output.label,
        if output.is_clean { "✓ PASS — entity is clean" }
        else { "✗ FAIL — entity found on watchlist" }
    );
}

fn main() -> Result<()> {
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║   Maat · Rule 4 · Non-Membership Proof                  ║");
    println!("║   NTH MOMENT · DUNS 772435720                           ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // ── Test 1: Borrower not in NCLT/bankruptcy register ─────────────────────
    println!("Test 1: Borrower not in bankruptcy register (NCLT/PACER)");
    let r1 = prove_nonmembership(
        "bankruptcy_register",
        "82-1234567",  // borrower EIN — clean
        vec![
            "11-1111111",  // defaulted entity 1
            "22-2222222",  // defaulted entity 2
            "33-3333333",  // defaulted entity 3
        ],
    )?;
    print_result(&r1);

    // ── Test 2: Anchor not in inter-creditor dispute register ─────────────────
    println!("\nTest 2: Anchor not in inter-creditor dispute register");
    let r2 = prove_nonmembership(
        "intercreditor_dispute_register",
        "13-9876543",  // anchor EIN — clean
        vec![
            "44-4444444",
            "55-5555555",
            "66-6666666",
        ],
    )?;
    print_result(&r2);

    // ── Test 3: Entity ON the watchlist — should FAIL ─────────────────────────
    println!("\nTest 3: Entity on sanctions list — should FAIL");
    let r3 = prove_nonmembership(
        "sanctions_list",
        "99-9999999",  // flagged entity
        vec![
            "99-9999999",  // this entity is sanctioned
            "88-8888888",
            "77-7777777",
        ],
    )?;
    print_result(&r3);

    // ── Test 4: Borrower not on RBI defaulter list ────────────────────────────
    println!("\nTest 4: Borrower not on RBI defaulter list");
    let r4 = prove_nonmembership(
        "rbi_defaulter_list",
        "82-1234567",  // borrower — clean
        vec![
            "11-1111111",
            "22-2222222",
        ],
    )?;
    print_result(&r4);

    println!("\n✓ All non-membership proofs complete.");
    println!("  Watchlist contents were never revealed.");
    println!("  Only clean/flagged status and watchlist snapshot are public.\n");

    Ok(())
}
