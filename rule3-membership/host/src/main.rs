use anyhow::Result;
use methods::MEMBERSHIP_GUEST_ELF;
use risc0_zkvm::{ExecutorEnv, ExternalProver};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
pub struct MembershipInput {
    pub entity_hash: [u8; 32],
    pub approved_set: Vec<[u8; 32]>,
    pub label: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MembershipOutput {
    pub is_member: bool,
    pub set_snapshot_hash: [u8; 32],
    pub label: String,
}

fn sha256_str(s: &str) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(s.as_bytes());
    h.finalize().into()
}

fn prove_membership(
    label: &str,
    entity: &str,
    approved: Vec<&str>,
) -> Result<MembershipOutput> {
    let entity_hash = sha256_str(entity);
    let approved_set: Vec<[u8; 32]> = approved.iter().map(|e| sha256_str(e)).collect();

    let input = MembershipInput {
        entity_hash,
        approved_set,
        label: label.to_string(),
    };

    let env = ExecutorEnv::builder().write(&input)?.build()?;
    let prover = ExternalProver::new("gpu", "/home/rammint/.cargo/bin/r0vm");
    let receipt = prover.prove(env, MEMBERSHIP_GUEST_ELF)?.receipt;
    receipt.verify(methods::MEMBERSHIP_GUEST_ID)?;
    let output: MembershipOutput = receipt.journal.decode()?;
    Ok(output)
}

fn print_result(output: &MembershipOutput) {
    println!(
        "  {} → {}",
        output.label,
        if output.is_member { "✓ PASS — entity is in approved set" } 
        else { "✗ FAIL — entity not found in approved set" }
    );
}

fn main() -> Result<()> {
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║   Maat · Rule 3 · Membership Proof                      ║");
    println!("║   NTH MOMENT · DUNS 772435720                           ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // ── Test 1: Anchor is KYC-verified ───────────────────────────────────────
    // Anchor EIN: 13-9876543 — is it in the KYC-cleared set?
    println!("Test 1: Anchor KYC verification");
    let r1 = prove_membership(
        "anchor_kyc_cleared",
        "13-9876543",  // the anchor being checked
        vec![
            "13-9876543",  // Apple Inc (example)
            "91-1144442",  // Microsoft (example)
            "94-3203813",  // Google (example)
        ],
    )?;
    print_result(&r1);

    // ── Test 2: Borrower is in approved supplier registry ────────────────────
    println!("\nTest 2: Supplier in approved registry");
    let r2 = prove_membership(
        "supplier_registry",
        "82-1234567",  // the supplier being checked
        vec![
            "82-1234567",  // our supplier — should pass
            "45-6789012",
            "33-1122334",
        ],
    )?;
    print_result(&r2);

    // ── Test 3: Unknown entity — should fail ─────────────────────────────────
    println!("\nTest 3: Unknown entity not in approved set — should FAIL");
    let r3 = prove_membership(
        "anchor_kyc_cleared",
        "99-0000001",  // unknown entity
        vec![
            "13-9876543",
            "91-1144442",
            "94-3203813",
        ],
    )?;
    print_result(&r3);

    // ── Test 4: Wallet address in KYC registry (DeFi use case) ───────────────
    println!("\nTest 4: Wallet address in on-chain KYC registry");
    let r4 = prove_membership(
        "wallet_kyc_registry",
        "0xabc123def456",  // wallet being checked
        vec![
            "0xabc123def456",  // registered wallet — should pass
            "0xdeadbeef1234",
            "0xcafe00001111",
        ],
    )?;
    print_result(&r4);

    println!("\n✓ All membership proofs complete.");
    println!("  Entity identities were never revealed.");
    println!("  Only membership status and set snapshot are public.\n");

    Ok(())
}
