# Open Issues

Mirrors the issues filed on the repo's Issues tab, with labels. Labels follow a **type**
label plus a **Drips Wave complexity** label (`complexity:trivial` = 100 pts,
`complexity:medium` = 150 pts, `complexity:high` = 200 pts).

---

### 1. Support multiple authorized maintainers instead of a single admin address

**Labels:** `enhancement`, `complexity:high`

**Description**
Right now `init` sets exactly one `maintainer` address, and it can never be changed or
extended. For any real project, that's a single point of failure (lost key = registry
frozen forever) and doesn't reflect how most repos are actually run (multiple
maintainers, not one). Replace the single `Maintainer` key with a set of authorized
addresses, and add:
- `add_maintainer(new_maintainer)` — callable by an existing maintainer
- `remove_maintainer(maintainer_to_remove)` — callable by an existing maintainer, with a
  guard against removing the last remaining maintainer

**Acceptance criteria**
- `add_issue` / `resolve_issue` succeed when called by *any* authorized maintainer, not
  just the original one.
- Removing the last maintainer is rejected with a clear error.
- Existing single-maintainer behavior (from `init`) still works as the starting state.
- Tests cover: add a second maintainer and use it, remove a maintainer, attempt to
  remove the last one.

---

### 2. Add a `total_issues_resolved` counter and a way to list a contributor's resolved issues

**Labels:** `enhancement`, `complexity:medium`

**Description**
Currently you can look up a contributor's total points (`points_of`) or who resolved a
specific issue (`resolved_by`), but there's no way to go the other direction — "show me
every issue this contributor has resolved." Add a way to track and retrieve that,
e.g. a `Vec<Symbol>` per contributor, plus a global resolved-issue counter for basic
program-wide stats.

**Acceptance criteria**
- New read-only method, e.g. `issues_resolved_by(contributor) -> Vec<Symbol>`.
- New read-only method, e.g. `total_resolved() -> u32`.
- Existing storage layout for points/resolution isn't broken — this should be additive.
- Tests confirm the list and counter update correctly across multiple resolutions.

---

### 3. Improve panic messages and add inline doc comments for all public methods

**Labels:** `documentation`, `complexity:trivial`

**Description**
A few of the `expect()` calls (e.g. in `require_maintainer`) use short messages that
don't explain what the caller should do differently. Improve these, and add `///` doc
comments above every `#[contractimpl]` method summarizing its purpose, required
authorization, and any panics it can raise — this repo currently only documents that at
the module level (top of `lib.rs`) rather than per-method.

**Acceptance criteria**
- Every public method in `ContributorPointsContract` has a doc comment covering: what it
  does, who must authorize it, and what conditions cause it to panic.
- `contract not initialized` and similar messages are specific enough that a caller
  knows exactly what step they skipped.
