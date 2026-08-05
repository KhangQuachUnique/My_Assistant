# Implementation Log

This file records selected project history by date and issue.

It is not a replacement for Git history. Use it to capture context that future work should remember.

## When To Update

Update this log when:

- an issue changes architecture, workflow, or project rules
- a module boundary or dependency direction changes
- a contract is created or revised
- a notable decision is made, including a decision to defer work
- a blocker or manual step should be remembered
- a follow-up issue is created from the current work

Do not update this log for typo fixes, formatting, small local refactors, or changes that are fully obvious from the commit alone.

## Entry Format

Use this shape:

```md
### ISSUE-ID / KEY - Title

- Changed: what changed.
- Decision: what was decided, if anything.
- Validation: what was checked.
- Follow-up: what remains, if anything.
```

Keep entries short. Prefer useful context over completeness.

## 2026-08-05

### AI-35 / CORE-001-04 - Design AGENTS.md and docs routing for AI flow

- Changed: created `AGENTS.md` as a short router and added minimal docs homes for workflows, architecture, and implementation history.
- Decision: keep detailed workflow rules in `docs/`; keep `AGENTS.md` small; document project architecture by runtime boundary instead of splitting frontend/backend docs early.
- Validation: docs review only; no runtime code changed.
- Follow-up: add focused architecture docs when a code issue changes durable boundaries.
