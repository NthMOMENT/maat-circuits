use anyhow::Result;
use methods::RANGE_GUEST_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RangeInput {
    pub value: u64,
    pub min: u64,
    pub max: u64,
    pub label: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RangeOutput {
    pub in_range: bool,
    pub min: u64,
    pub max: u64,
    pub label: String,
}

fn prove_range(label: &str, value: u64, min: u64, max: u64) -> Result<RangeOutput> {
    let input = RangeInput {
        value,
        min,
        max,
        label: label.to_string(),
    };

    let env = ExecutorEnv::builder().write(&input)?.build()?;
    let prover = default_prover();
    let receipt = prover.prove(env, RANGE_GUEST_ELF)?.receipt;
    receipt.verify(methods::RANGE_GUEST_ID)?;
    let output: RangeOutput = receipt.journal.decode()?;
    Ok(output)
}

fn print_result(output: &RangeOutput) {
    println!(
        "  {} → range [{}, {}] → {}",
        output.label,
        output.min,
        output.max,
        if output.in_range { "✓ PASS" } else { "✗ FAIL" }
    );
}

fn main() -> Result<()> {
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║   Maat · Rule 2 · Range Proof                           ║");
    println!("║   NTH MOMENT · DUNS 772435720                           ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // ── Test case 1: Anchor credit rating >= BBB+ ─────────────────────────────
    // Credit rating scale: BBB- = 300, BBB = 325, BBB+ = 350, A- = 375 ...
    // We prove the anchor's rating is within [350, 500] (BBB+ to AAA)
    // without revealing the exact rating.
    println!("Test 1: Anchor credit rating (BBB+ minimum)");
    let r1 = prove_range("anchor_credit_rating", 375, 350, 500)?;
    print_result(&r1);

    // ── Test case 2: Invoice amount within approved facility ───────────────────
    // Facility limit: $500k to $5M
    // Invoice: $1.5M — should pass
    println!("\nTest 2: Invoice amount within facility limits ($500k–$5M)");
    let r2 = prove_range("invoice_amount_cents", 150_000_000, 50_000_000, 500_000_000)?;
    print_result(&r2);

    // ── Test case 3: Loan-to-value ratio (mortgage) ───────────────────────────
    // Max LTV: 80% (8000 basis points)
    // Applicant LTV: 75% (7500 basis points) — should pass
    println!("\nTest 3: Mortgage LTV ratio (max 80%)");
    let r3 = prove_range("ltv_ratio_bps", 7500, 0, 8000)?;
    print_result(&r3);

    // ── Test case 4: LTV too high — should fail ───────────────────────────────
    println!("\nTest 4: Mortgage LTV ratio (max 80%) — should FAIL");
    let r4 = prove_range("ltv_ratio_bps", 9000, 0, 8000)?;
    print_result(&r4);

    println!("\n✓ All range proofs complete.");
    println!("  The private values were never revealed.");
    println!("  Only pass/fail and the bounds are public.\n");

    Ok(())
}
