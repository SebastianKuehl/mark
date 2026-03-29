# Worker Agent Rules

## Delegated Implementation Agent

All implementation work is delegated to the Copilot CLI agent named **`anvil`**.
The Product Owner Agent orchestrates; anvil agents implement.

## Worktree Requirement

Every anvil agent **must** create a dedicated git worktree before making any changes.

```sh
git worktree add .worktrees/<item-id>-<short-slug> -b <branch-name> main
```

- Worktree path: `.worktrees/<item-id>-<short-slug>`
- Feature branches: `feat/<item-id>-<short-slug>`
- Bug-fix branches: `fix/<item-id>-<short-slug>`
- Chore branches: `chore/<item-id>-<short-slug>`

All work must happen inside that worktree. Never modify the main checkout directly.

## Branch Naming

| Type    | Pattern                        |
|---------|--------------------------------|
| Feature | `feat/<item-id>-<short-slug>`  |
| Bug fix | `fix/<item-id>-<short-slug>`   |
| Chore   | `chore/<item-id>-<short-slug>` |

## Forbidden Actions

Anvil agents must **never**:

- Edit `README.md`
- Commit or merge directly to `main`
- Create or move git tags
- Change scope, milestone, feature, or bug status documents unilaterally
- Declare their own work complete or close items independently

## Testing Expectations

Before reporting completion, every anvil agent must run:

```sh
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

All checks must pass with no errors. Warnings that are not suppressible must be reported.

## Completion Handoff Format

When work is done, report to the Product Owner Agent with:

```
Item ID:        <e.g. F-010>
Prompt ID:      <e.g. P-010-F-010>
Worktree path:  .worktrees/<path>
Branch name:    <branch>
Summary:        <short description of changes>
Changed files:  <list>
Checks run:     cargo fmt, cargo clippy, cargo test
Check results:  <pass / fail details>
Known issues:   <any follow-ups or caveats>
README update needed: <yes / no, and why>
```

Do not summarise as "done" — provide the full handoff so the Product Owner Agent can review.

## Background Execution

Anvil agents are always launched as background tasks. The Product Owner Agent does not block waiting for completion. Agents notify the Product Owner Agent upon completion using the handoff format above.
