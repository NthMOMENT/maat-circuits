use anyhow::Result;
use methods::CONSISTENCY_GUEST_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
pub struct ConsistencyInput {
    pub dataset_a_hash: [u8; 32],
    pub dataset_b_hash: [u8; 32],
    pub tolerance_bps: u32,
    pub label: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConsistencyOutput {
    pub is_consistent: bool,
    pub dataset_a_hash: [u8; 32],
    pub dataset_b_hash: [u8; 32],
    pub label: String,
}

fn sha256_bytes(data: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(data);
    h.finalize().into()
}

fn prove_consistency(
    label: &str,
    data_a: &[u8],
    data_b: &[u8],
) -> Result<ConsistencyOutput> {
    let input = ConsistencyInput {
        dataset_a_hash: sha256_bytes(data_a),
        dataset_b_hash: sha256_bytes(data_b),
        tolerance_bps: 0,
        label: label.to_string(),
    };

    let env = ExecutorEnv::builder().write(&input)?.build()?;
    let prover = default_prover();
    let receipt = prover.prove(env, CONSISTENCY_GUEST_ELF)?.receipt;
    receipt.verify(methods::CONSISTENCY_GUEST_ID)?;
    let output: ConsistencyOutput = receipt.journal.decode()?;
    Ok(output)
}

fn print_result(output: &ConsistencyOutput) {
    println!(
        "  {} → {}",
        output.label,
        if output.is_consistent { "✓ PASS — datasets are consistent" }
        else { "✗ FAIL — datasets do not match" }
    );
}

fn main() -> Result<()> {
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║   Maat · Rule 5 · Consistency Proof                     ║");
    println!("║   NTH MOMENT · DUNS 772435720                           ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // ── Test 1: Submitted financials match MCA/IRS filed returns ─────────────
    // Both sides hash the same underlying financial data
    // Neither reveals the actual figures
    println!("Test 1: Submitted financials match filed returns (MCA/IRS)");
    let financial_data = b"revenue:1500000,ebitda:350000,debt:200000,filing_year:2025";
    let r1 = prove_consistency(
        "financials_vs_filed_returns",
        financial_data,  // borrower submitted
        financial_data,  // MCA/IRS filed — same data, should match
    )?;
    print_result(&r1);

    // ── Test 2: Property appraisal consistent with market data ───────────────
    println!("\nTest 2: Property appraisal matches market data");
    let appraisal_data = b"property:0x123MainSt,value_usd:850000,date:2026-07-18";
    let r2 = prove_consistency(
        "appraisal_vs_market_data",
        appraisal_data,
        appraisal_data,
    )?;
    print_result(&r2);

    // ── Test 3: Tampered financials — should FAIL ─────────────────────────────
    println!("\nTest 3: Tampered financials — should FAIL");
    let submitted = b"revenue:5000000,ebitda:1200000,debt:200000,filing_year:2025";
    let filed     = b"revenue:1500000,ebitda:350000,debt:200000,filing_year:2025";
    let r3 = prove_consistency(
        "financials_vs_filed_returns",
        submitted,  // inflated figures submitted by borrower
        filed,      // actual IRS filing — mismatch
    )?;
    print_result(&r3);

    // ── Test 4: Invoice hash matches anchor-confirmed PO ─────────────────────
    println!("\nTest 4: Invoice details match anchor-confirmed PO");
    let invoice_data = b"po:PO-2026-0042,amount:1500000,seller:82-1234567,buyer:13-9876543";
    let r4 = prove_consistency(
        "invoice_vs_anchor_po",
        invoice_data,
        invoice_data,
    )?;
    print_result(&r4);

    println!("\n✓ All consistency proofs complete.");
    println!("  Neither dataset was revealed.");
    println!("  Only consistency status and dataset hashes are public.\n");

    Ok(())
}
