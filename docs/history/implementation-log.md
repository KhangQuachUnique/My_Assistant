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

## 2026-08-06

### AI-39 / AVT-002-01 - Define avatar animation control contract

- Changed: added the avatar control system architecture contract for `set_animation`, readiness, JSON-lines loopback transport, and frontend-safe command responses/errors.
- Decision: use stable app-level animation names `idle`, `listening`, `thinking`, `speaking`, `success`, and `error`; Godot may fallback to current `idle`/`speaking` assets until richer animations exist.
- Validation: docs review only; no runtime code changed.
- Follow-up: implement the Tauri-to-Godot `set_animation` path, then add frontend debug controls.

## 2026-08-05

### AI-37 / AVT-001-03 - Expose avatar lifecycle Tauri commands

- Changed: added Tauri command adapters for avatar `start`, `stop`, `restart`, and `status`; registered them in the Tauri builder.
- Decision: keep command response and error DTOs in `interface/commands/avatar.rs`; commands call `AvatarModule` through managed `AppState` and do not manage processes directly.
- Validation: `cargo test` from `apps/desktop/src-tauri`.
- Follow-up: frontend controls and Tauri-to-Godot IPC remain outside this issue.

### AI-36 / AVT-001-02 - Implement Windows avatar process runner

- Changed: added the Windows `AvatarProcessRunner` adapter and wired app composition to use it for avatar lifecycle startup and shutdown.
- Decision: keep `std::process::Child` inside `platform/windows/avatar`; expose the Windows platform module only on Windows; keep lifecycle tests on the in-memory runner so `cargo test` does not spawn `avatar.exe`.
- Validation: `cargo test` from `apps/desktop/src-tauri`, including runner-level duplicate-start coverage.
- Follow-up: Tauri commands, IPC, readiness handshakes, and UI controls remain outside this issue.

### AI-35 / CORE-001-04 - Design AGENTS.md and docs routing for AI flow

- Changed: created `AGENTS.md` as a short router and added minimal docs homes for workflows, architecture, and implementation history.
- Decision: keep detailed workflow rules in `docs/`; keep `AGENTS.md` small; document project architecture by runtime boundary instead of splitting frontend/backend docs early.
- Validation: docs review only; no runtime code changed.
- Follow-up: add focused architecture docs when a code issue changes durable boundaries.
