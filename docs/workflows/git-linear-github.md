# Git, Linear, And GitHub Workflow

## When To Read This

Read this document before:

- creating or renaming a branch
- creating, updating, or closing a Linear issue
- deciding which milestone an issue belongs to
- committing or opening a PR
- judging whether a local change finishes a Linear issue
- splitting work into a follow-up issue

## Milestones

Use milestones to group work by product capability, not by temporary implementation detail.

Milestone names should follow this shape:

```text
AREA-### - Capability Name
```

Examples:

```text
CORE-001 - Application Foundation
AVT-001 - Avatar Process Lifecycle
```

Use a new milestone when the work represents a coherent capability with multiple issues, such as app foundation, avatar process lifecycle, wake word runtime, or assistant session flow.

Do not create a milestone for a one-off cleanup, tiny refactor, small bug, or documentation-only adjustment unless it supports a broader capability.

## Issues Outside A Milestone

An issue can stay outside a milestone when:

- it is exploratory and the owning capability is not clear yet
- it is a small chore that does not belong to a product capability
- it is temporary project hygiene
- it is a duplicate, canceled, or legacy item kept only for history
- it is documentation that spans multiple milestones

Once the issue has a clear capability owner, assign it to the matching milestone.

## Creating Issues

Create issues around one clear deliverable. The issue should be small enough to finish in one focused branch and PR.

Issue titles should follow this shape when tied to a milestone:

```text
AREA-###-NN - Imperative issue title
```

Examples:

```text
CORE-001-04 - Design AGENTS.md and docs routing for AI flow
AVT-001-02 - Implement Windows avatar process runner
```

Each issue should include:

- Goal
- Scope
- Out of scope when useful
- Acceptance criteria
- Git convention

Use `blockedBy` when the next issue depends on an unfinished contract, design, or implementation.

## Branches

Prefer the branch name written in the issue Git convention.

Branch names should be lowercase and descriptive:

```text
core-001-04-agents-docs-router
avt-001-02-windows-avatar-process-runner
```

Do not work directly on `main`.

Do not reuse old branches for unrelated issues.

## Commits

Use the issue key at the start of the commit message:

```text
CORE-001-04 design AGENTS docs router for AI flow
AVT-001-02 implement Windows avatar process runner
```

Commit only files that belong to the active issue.

If the worktree has unrelated files, leave them unstaged.

## Pull Requests

PR titles should follow:

```text
AREA-###-NN: Short summary
```

PR body should include:

- Summary
- Changes
- Validation
- Linear issue ID

Default to draft PRs unless the user asks for ready-for-review.

## Closing Issues

An issue is ready to close only when:

- all acceptance criteria are satisfied
- relevant tests/checks pass or the missing validation is clearly explained
- docs are updated when boundaries, contracts, or workflow rules changed
- code is committed and pushed
- a PR exists, or the reason it cannot be created is recorded in Linear

Move the issue to `Done` only after the PR is merged or the user explicitly accepts closing it before merge.

If the current work satisfies only part of the issue, leave it open and add a Linear comment with what remains.

## Follow-Up Issues

Create a follow-up issue when the next work is real but outside the current issue scope.

Good follow-up examples:

- implementing a process runner after a contract issue
- exposing Tauri commands after backend lifecycle works
- adding UI controls after IPC is available

Avoid follow-up issues for vague future ideas. Put those in notes or docs until they become actionable.
