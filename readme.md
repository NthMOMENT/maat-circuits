# Maat Circuits

**ZK circuit primitives for real-world financial compliance.**

Part of **NTH MOMENT** · DUNS 772435720

---

## What this is

Maat is a universal ZK proof layer that validates financial instrument compliance across any asset class, any jurisdiction.

The circuits in this repository are the open-source primitives that power Maat's proof layer, the mathematical building blocks that any financial compliance system can be assembled from:

- **Uniqueness proofs** — prove an instrument has never been registered before
- **Range proofs** — prove a value is within bounds without revealing it
- **Membership proofs** — prove an entity belongs to a verified set
- **Non-membership proofs** — prove an entity is absent from a watchlist
- **Consistency proofs** — prove two datasets agree without revealing either

These primitives compose into compliance rulesets for any financial instrument: invoices, mortgages, auto loans, trade finance, SME credit, insurance claims, pension assets.

MIT licensed. Auditable. Composable. Build on them.

---

## The problem

Every financial instrument today relies on trust at the compliance layer.

A bank says an invoice hasn't been financed before. You trust the bank.  
A borrower says they have no active insolvency proceedings. You trust the borrower.  
A mortgage applicant says their financials match their filed returns. You trust the applicant.  
A home buyer says the title is clean. You trust the title company.  
A protocol says its underwriting ran correctly. You trust the protocol.

Trust fails — across every asset class, every jurisdiction, every counterparty.

Double financing the same invoice at multiple lenders. Concealed bankruptcy filings. Inflated appraisals. Fabricated income statements. These are not edge cases. They are systematic failures of a compliance layer built entirely on human attestation and institutional trust.

The financial system has no shared, verifiable, privacy-preserving layer that proves an instrument is what it claims to be — without revealing the underlying details to the world.

Maat is that layer.

---

## How it works

```
Any financial instrument submitted
        ↓
Deterministic compliance ruleset applied
        ↓
ZK proof generated (RISC Zero / SP1)
        ↓
Proof verified on-chain
        ↓
Any smart contract consumes the proof — no human discretion
```

The proof reveals nothing about the instrument's details. It proves only that the rules ran correctly and the instrument passed or failed. The underlying data — amounts, identities, credit scores, property values — stays private.

The same architecture works for every asset class. The primitives are universal. The rulesets are specific.

---

## Primitives

### Uniqueness (`rule1-uniqueness`)
Proves a financial instrument has never been seen before in a registry. Prevents double financing of invoices, duplicate mortgage applications, re-pledging of collateral. Works for any instrument that can be hashed.

### Range Proof *(coming)*
Proves a value — income, collateral, credit score, loan-to-value ratio — is within an approved range without revealing the exact figure. Applicable to mortgages, SME credit, margin requirements, insurance underwriting.

### Membership Proof *(coming)*
Proves an entity — borrower, anchor, counterparty — exists in a verified set (KYC-cleared, credit-rated, licensed) without revealing which set or the entity's position within it.

### Non-Membership Proof *(coming)*
Proves an entity is absent from a watchlist — bankruptcy registry, sanctions list, inter-creditor dispute register — without revealing the watchlist contents.

### Consistency Proof *(coming)*
Proves two datasets agree — submitted financials match filed returns, appraisal matches market data — without revealing either dataset.

---

## Asset classes

Each asset class is a composed ruleset built from the primitives above:

```
Invoice Discounting V1 (US)     ← in progress
Mortgage V1 (US)                ← next
SME Credit V1 (US)
Auto Loan V1 (US)
Invoice Discounting V1 (India)
Trade Finance V1 (cross-border)
```

Rulesets are proprietary. Primitives are open. See licensing below.

---

## Technical stack

- **zkVM:** RISC Zero 3.0.6 / SP1 6.3.1
- **Hash:** SHA-256 / Poseidon (ZK-optimised)
- **Verification:** Groth16 on-chain verifier (Arbitrum One)
- **Proving infrastructure:** [theprover](https://github.com/NthMOMENT/theprover)
- **Benchmarks:** [risc-zero-merkle-proofs](https://github.com/NthMOMENT/risc-zero-merkle-proofs)

---

## Run the uniqueness primitive

```bash
# Dev mode no GPU required
RISC0_DEV_MODE=1 cargo run --release -p host

# Production — GPU proving (RTX 4090 / A100)
cargo run --release -p host
```

---

## Repository structure

```
maat-circuits/
  rule1-uniqueness/         ← Instrument uniqueness proof
    methods/guest/src/      ← ZK program (runs inside zkVM)
    host/src/               ← Orchestrator
  rule2-range/              ← coming
  rule3-membership/         ← coming
  rule4-nonmembership/      ← coming
  rule5-consistency/        ← coming
```

---

## Part of the NTH MOMENT ZK Stack

- [theprover](https://github.com/NthMOMENT/theprover) — Python ZK proving harness (to be released soon)
- [risc-zero-merkle-proofs](https://github.com/NthMOMENT/risc-zero-merkle-proofs) — Merkle membership benchmarks
- maat-circuits — this repo, universal ZK financial compliance primitives

---

## License

Primitives: MIT the math is open. Build on it.

Compliance rulesets (assembled rule logic per asset class and jurisdiction) are proprietary to NTH MOMENT. Contact [@0xfourier](https://x.com/0xfourier) for licensing.

---

*Built by [GhostProver](https://x.com/0xfourier) · NTH MOMENT · DUNS 772435720*
