# Contributing

Thanks for considering a contribution. This project is part of the Stellar open-source
ecosystem and participates in [Drips Wave](https://www.drips.network/wave/stellar).

## Ground rules

- This contract is deliberately minimal — see `ARCHITECTURE.md` for what's in scope and
  what's intentionally left out. New features should come with a clear rationale for
  why they belong in the base contract rather than a layer on top of it.
- Every change needs a corresponding test in `src/test.rs`, including failure-path tests
  where relevant (this contract has several `assert!`/`expect` guards worth testing).
- Run `cargo fmt` and `cargo clippy --all-targets` before opening a PR.
- Follow [Conventional Commits](https://www.conventionalcommits.org/) for commit
  messages (`feat:`, `fix:`, `docs:`, `test:`, `chore:`).

## Picking up an issue

1. Check the [open issues](./ISSUES.md) or the repo's Issues tab.
2. Comment on the issue to claim it before starting work.
3. Issues are labeled by complexity, matching the Drips Wave points system:
   - `complexity:trivial` — small, well-scoped fixes (100 pts)
   - `complexity:medium` — a standard feature or non-trivial bug fix (150 pts)
   - `complexity:high` — a new capability or trust-model change (200 pts)
4. Open a draft PR early for feedback on direction, especially for anything touching
   the maintainer/authorization model.

## PR checklist

- [ ] `cargo build --target wasm32-unknown-unknown --release` succeeds
- [ ] `cargo test` passes, including new tests for the change
- [ ] `cargo clippy --all-targets -- -D warnings` is clean
- [ ] `ARCHITECTURE.md` updated if storage layout or trust model changed
- [ ] `README.md` updated if the public interface changed

## Reporting bugs or proposing features

Open an issue describing the current behavior, expected behavior, and — for anything
touching the maintainer's authorization or point accounting — why the change is safe
(what stops a compromised or malicious maintainer key from abusing it, if relevant).

## Code of conduct

Be respectful, assume good faith, and keep discussion focused on the work.
