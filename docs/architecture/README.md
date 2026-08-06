# Architecture Docs

This folder stores durable architecture context.

## When To Read This

Read architecture docs before changing:

- the `src-tauri/src` layer structure
- adding a new runtime module
- module boundaries
- dependency direction
- runtime lifecycle behavior
- platform adapters
- IPC or interface contracts
- resource locations used by runtime code

## When To Update This

Update architecture docs when a change creates or changes a durable boundary, contract, dependency direction, or runtime ownership rule.

Do not update architecture docs for local implementation details that are obvious from the code.

## Current Docs

- `overview.md` - top-level project architecture and Tauri runtime layers
- `avatar-process-lifecycle.md` - process lifecycle contract for `avatar.exe`
- `avatar-control-system.md` - animation control contract between frontend, Tauri, and Godot
