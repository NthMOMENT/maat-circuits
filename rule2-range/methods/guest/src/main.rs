#![no_main]
risc0_zkvm::guest::entry!(main);

use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};

// ── Input: the value and the range bounds ─────────────────────────────────────

#[derive(Serialize, Deserialize)]
pub struct RangeInput {
    /// The private value being checked (never revealed publicly)
    /// Examples:
    ///   - Credit rating score (e.g. BBB+ = 350, AAA = 500)
    ///   - Invoice amount in cents
    ///   - Loan-to-value ratio in basis points
    ///   - Income in cents
    ///   - Collateral value in cents
    pub value: u64,

    /// Minimum allowed value (inclusive). Public.
    pub min: u64,

    /// Maximum allowed value (inclusive). Public.
    pub max: u64,

    /// Human-readable label for what is being checked.
    /// e.g. "anchor_credit_rating", "invoice_amount", "ltv_ratio"
    pub label: String,
}

// ── Output: committed to journal (public) ─────────────────────────────────────

#[derive(Serialize, Deserialize)]
pub struct RangeOutput {
    /// True = value is within [min, max]
    pub in_range: bool,

    /// The bounds that were checked (public)
    pub min: u64,
    pub max: u64,

    /// What was checked
    pub label: String,
}

fn main() {
    let input: RangeInput = env::read();

    // The core check — value stays private, result is public
    let in_range = input.value >= input.min && input.value <= input.max;

    env::commit(&RangeOutput {
        in_range,
        min: input.min,
        max: input.max,
        label: input.label,
    });
}
