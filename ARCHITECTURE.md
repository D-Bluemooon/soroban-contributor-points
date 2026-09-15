# Architecture

## Overview

This contract is intentionally the simplest of the three: a registry with two related
mappings and one aggregate counter, all keyed by content rather than a growing list of
structs.

```
DataKey::Maintainer                  -> Address          (single trusted admin)
DataKey::IssuePoints(issue_id)       -> u32               (points for a registered issue)
DataKey::IssueResolvedBy(issue_id)   -> Address            (who resolved it, once)
DataKey::ContributorPoints(address)  -> u32               (running total per contributor)
```

Using `Symbol` as the issue identifier (rather than an auto-incrementing `u64` like the
other two repos in this set) is deliberate: it lets a maintainer use a stable, external
identifier — e.g. a short slug like `stellar-wave-9-issue-42` — that maps naturally onto
a GitHub issue reference, without needing an extra off-chain lookup table to translate
between an on-chain id and the issue it corresponds to.

## Complexity tiers

```rust
pub enum Complexity {
    Trivial, // 100 points
    Medium,  // 150 points
    High,    // 200 points
}
```

These map directly onto the tiers described in the Drips Wave maintainer docs
("Trivial: 100 Points (Base)... Medium: 150 Points... High: 200 Points"), so a
maintainer already running a Wave-style program has a mental model that transfers
directly — the on-chain registry is meant to complement that workflow, not introduce a
new one.

## Trust model

There is exactly one privileged address, `maintainer`, checked via
`maintainer.require_auth()` in `add_issue` and `resolve_issue`. This is a deliberate
starting point, not an endorsement of centralization as a permanent design — see
`ISSUES.md` for a scoped issue on moving to a multi-maintainer or DAO-governed model.

## Why no automatic PR verification

Verifying "this contributor actually merged a PR that closed this GitHub issue" from
inside a Soroban contract would require an oracle bridging GitHub state on-chain — a
much larger design surface (which oracle, how is it kept honest, what if GitHub is
temporarily unreachable) that doesn't belong in a minimal reference contract. The
current design assumes the maintainer verifies off-chain (exactly as they would in
Wave's own review flow) and submits the on-chain `resolve_issue` call as the
attestation. A future version could plug in an oracle without changing this contract's
external interface.

## Read-only accessors

`points_of` and `resolved_by` are read-only and require no authorization — the whole
point of an on-chain ledger is that anyone can verify it without needing permission.
