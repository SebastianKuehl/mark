# P-016 — B-004: Completion Suggests Subcommands After FILE

## Prompt ID
P-016

## Linked Item
B-004 — Shell completion suggests subcommands after the positional `FILE` argument is already present

## Task Objective
Fix the shell-completion behavior so `mark some-file.md` does not keep offering root-level subcommands like `cleanup-home` or `config` after the `FILE` positional has already been supplied, and add regression coverage for the fix.

---

## Worktree Instructions

```sh
cd /Users/sebastian/Documents/repos/mark
git worktree add .worktrees/B-004-completion-after-file -b fix/B-004-completion-after-file main
cd .worktrees/B-004-completion-after-file
```

All work must happen inside `.worktrees/B-004-completion-after-file`.

---

## Exact Scope

Investigate and fix the repo-side completion bug documented in `docs/bugs/B-004-completion-subcommands-after-file.md`.

The reported/reproduced behavior is:

```sh
mark some-file.md<TAB><TAB>
```

and completion still offers root-level subcommands such as:

- `cleanup-home`
- `config`
- `completions`
- `help`

### Required work

1. Inspect the CLI structure in `src/cli.rs` and completion generation path in `src/main.rs` / generated shell scripts.
2. Implement a fix so that once `FILE` is already present, top-level subcommands are not suggested for that invocation.
3. Preserve legitimate flag and subcommand completion behavior in other contexts.
4. Add regression coverage in `tests/completions.rs` (and additional tests if needed).

### Allowed implementation direction

Use the smallest robust fix that aligns with the codebase. That may be:

- a CLI-definition adjustment that yields correct generated completions, or
- a targeted completion-generation workaround if the clap-generated output cannot express the desired behavior directly.

If the problem turns out to be an unavoidable upstream `clap_complete` limitation, document that clearly in your handoff with evidence from the generated script rather than silently working around it.

---

## Files to Inspect

- `src/cli.rs`
- `src/main.rs`
- `tests/completions.rs`
- `README.md` (read-only; do not edit)
- `docs/bugs/B-004-completion-subcommands-after-file.md`

---

## Implementation Constraints

- Do not edit `README.md`.
- Do not merge or tag releases.
- Do not change unrelated CLI behavior.
- Prefer a behavior fix over documenting the issue as permanent, if a local fix is feasible and safe.
- Add regression coverage for the reproduced case.

---

## Acceptance Criteria

1. The reproduced completion case no longer offers top-level subcommands after `FILE` is already present.
2. Root-level completion still offers valid subcommands when no file has been supplied.
3. Existing completion generation for supported shells still works.
4. `cargo fmt --all`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test` all pass.

---

## Testing Requirements

```sh
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

All must pass.

---

## Forbidden Actions

- Do **not** edit `README.md`
- Do **not** merge or push to `main`
- Do **not** create or move git tags
- Do **not** change approved scope documents
- Do **not** declare work complete without the full handoff
- Do **not** continue through a blocking API rate-limit condition

---

## Rate-Limit Stop and Notify Instructions

If an API rate limit, token cap, service throttling response, or similar limitation prevents reliable continuation:

1. Stop work immediately
2. Do not keep retrying blindly
3. Preserve the current worktree state
4. Notify the master / Product Owner Agent via output text using this format:

```text
Item ID:        B-004
Prompt ID:      P-016
Worktree path:  .worktrees/B-004-completion-after-file
Branch name:    fix/B-004-completion-after-file
Completed:      <what was finished before stopping>
Blocked:        <what remains blocked>
Rate limit:     <exact limitation, if known>
Resume point:   <safe next step once limits clear>
```

Do not report the task as complete if rate limiting blocked reliable completion.

---

## Completion Report Format

```text
Item ID:        B-004
Prompt ID:      P-016
Worktree path:  .worktrees/B-004-completion-after-file
Branch name:    fix/B-004-completion-after-file
Summary:        <what was implemented>
Changed files:  <list>
Checks run:     cargo fmt --all, cargo clippy --all-targets --all-features -- -D warnings, cargo test
Check results:  <pass or describe failures>
Known issues:   <caveats>
README update needed: <yes / no, and why>
```
