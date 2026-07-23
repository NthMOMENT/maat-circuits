use anyhow::Result;
use methods::UNIQUENESS_GUEST_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
pub struct UniquenessInput {
    pub seller_ein_hash: [u8; 32],
    pub buyer_ein_hash: [u8; 32],
    pub po_number_hash: [u8; 32],
    pub invoice_amount_cents: u64,
    pub invoice_due_date: u32,
    pub anchor_signature: Vec<u8>,
    pub registry: Vec<[u8; 32]>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UniquenessOutput {
    pub invoice_hash: [u8; 32],
    pub is_unique: bool,
    pub registry_snapshot_hash: [u8; 32],
}

fn sha256_str(s: &str) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(s.as_bytes());
    h.finalize().into()
}

fn main() -> Result<()> {
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║   Maat · Rule 1 · Invoice Uniqueness Proof              ║");
    println!("║   NTH MOMENT · DUNS 772435720                           ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // ── Build a test invoice ──────────────────────────────────────────────────
    let input = UniquenessInput {
        seller_ein_hash:       sha256_str("82-1234567"),  // seller EIN
        buyer_ein_hash:        sha256_str("13-9876543"),  // anchor EIN
        po_number_hash:        sha256_str("PO-2026-0042"),// anchor-issued PO
        invoice_amount_cents:  1_500_000_00,              // $1,500,000.00
        invoice_due_date:      1_753_920_000,             // 2025-07-31
        anchor_signature:      vec![0u8; 64],                 // placeholder
        registry:              vec![],                    // empty = first ever
    };

    println!("Invoice details:");
    println!("  Seller EIN hash : {}", hex::encode(input.seller_ein_hash));
    println!("  Buyer EIN hash  : {}", hex::encode(input.buyer_ein_hash));
    println!("  PO number hash  : {}", hex::encode(input.po_number_hash));
    println!("  Amount          : ${:.2}", input.invoice_amount_cents as f64 / 100.0);
    println!("  Registry size   : {} entries\n", input.registry.len());

    // ── Prove ────────────────────────────────────────────────────────────────
    println!("Generating ZK proof...");
    let env = ExecutorEnv::builder().write(&input)?.build()?;
    let prover = default_prover();
    let receipt = prover.prove(env, UNIQUENESS_GUEST_ELF)?.receipt;

    // ── Verify ───────────────────────────────────────────────────────────────
    receipt.verify(methods::UNIQUENESS_GUEST_ID)?;
    println!("✓ Proof verified\n");

    // ── Decode output ─────────────────────────────────────────────────────────
    let output: UniquenessOutput = receipt.journal.decode()?;

    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║   RESULT                                                 ║");
    println!("╠══════════════════════════════════════════════════════════╣");
    println!("║  Invoice hash : {}...  ║", &hex::encode(output.invoice_hash)[..20]);
    println!("║  Unique       : {}                                    ║",
        if output.is_unique { "✓ YES — safe to register" } else { "✗ NO  — DUPLICATE DETECTED" });
    println!("║  Registry     : {}... ║", &hex::encode(output.registry_snapshot_hash)[..20]);
    println!("╚══════════════════════════════════════════════════════════╝\n");

    if output.is_unique {
        println!("→ Safe to proceed. Register invoice hash on-chain.");
        println!("→ Hash: 0x{}", hex::encode(output.invoice_hash));
    } else {
        println!("→ REJECT. This invoice has already been financed.");
        println!("→ Potential double-financing fraud detected.");
    }

    Ok(())
}
