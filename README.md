# soroban-contributor-points

A minimal **on-chain contributor points registry** for [Soroban](https://soroban.stellar.org),
Stellar's smart contract platform.

## Why this exists

Programs like Drips Wave track contributor points off-chain (in a web app / database)
and settle rewards in USDC at the end of each cycle. That works well, but it means the
contribution history itself — who resolved what, worth how many points — lives in one
platform's database rather than being independently verifiable on-chain.

This repo is a small, single-maintainer-oriented primitive for the opposite approach:
a maintainer registers issues with a complexity tier (mirroring the Trivial/Medium/High
= 100/150/200 point structure already used by Wave), records resolutions on-chain, and
anyone can independently verify a contributor's accumulated points without trusting an
off-chain database. It's meant as a small building block — e.g. for a single project or
small org that wants a transparent, self-hosted contribution ledger — not a replacement
for a full platform like Wave.

## What it does

- `init(maintainer)` — one-time setup, sets the address allowed to manage issues.
- `add_issue(issue_id, complexity)` — maintainer-only. Registers an issue worth
  100 (Trivial), 150 (Medium), or 200 (High) points.
- `resolve_issue(issue_id, contributor)` — maintainer-only. Records that `contributor`
  resolved the issue and credits their point total. Each issue can only be resolved once.
- `points_of(contributor)` — read-only. Total accumulated points.
- `resolved_by(issue_id)` — read-only. Who resolved a given issue, if anyone.

## What it deliberately does *not* do (yet)

No GitHub integration, no automated verification that a PR was actually merged, no
payout/reward logic, and a single trusted maintainer rather than multi-sig or DAO
governance over who can register/resolve issues. See [`ISSUES.md`](./ISSUES.md) for
scoped next steps.

## Getting started

```bash
# build
cargo build --target wasm32-unknown-unknown --release

# run tests
cargo test
```

## Docs

- [`ARCHITECTURE.md`](./ARCHITECTURE.md) — storage layout and design rationale
- [`CONTRIBUTING.md`](./CONTRIBUTING.md) — how to propose changes and pick up an issue
- [`ISSUES.md`](./ISSUES.md) — current open issues, mirrored here for convenience

## License

MIT — see [`LICENSE`](./LICENSE).
