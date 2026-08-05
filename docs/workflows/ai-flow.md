# AI Flow

## When To Read This

Read this before starting a Linear-backed coding or documentation task.

## Default Flow

1. Identify the active Linear issue.
2. Read `AGENTS.md`.
3. Read the docs linked from `AGENTS.md` that match the task.
4. Check the current Git branch and worktree.
5. Keep changes scoped to the active issue.
6. Run the smallest useful validation.
7. Update docs when the work changes a boundary, workflow, or durable project rule.
8. Update `docs/history/implementation-log.md` when the change should be remembered.
9. Commit and push using the issue Git convention.
10. Open or prepare a PR.
11. Comment in Linear when something is blocked, partially done, or cannot be automated.

## Scope Control

Do not mix unrelated issues in one branch, commit, or PR.

Create a follow-up issue when useful work is real but outside the current issue scope.

Leave unrelated worktree changes unstaged.

## Docs Updates

Update docs when:

- a module boundary changes
- a dependency direction changes
- a workflow rule changes
- a recurring decision should be remembered
- a future agent would otherwise need to rediscover the same context
- a blocker or follow-up issue should be remembered

Do not update docs for tiny implementation details that are obvious from the code.

## Validation

Prefer focused checks first. Broaden validation when shared code, lifecycle behavior, or cross-layer contracts changed.

If validation cannot run, record the reason in the final response and, when relevant, in Linear.
