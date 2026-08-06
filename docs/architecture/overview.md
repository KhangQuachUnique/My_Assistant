# Architecture Overview

This project is a desktop application. Treat the boundary as frontend UI plus Tauri runtime, not as a traditional web frontend/backend split.

## Top-Level Shape

```text
apps/
  desktop/
    src/          # frontend UI
    src-tauri/    # desktop runtime
```

The frontend should own presentation and user interaction.

The Tauri runtime should own application lifecycle, local runtime modules, platform adapters, and IPC boundaries.

## Frontend Layers

```text
apps/desktop/src/
  app/       # React app composition, providers, and router creation
  routes/    # route-level layouts and screens
  features/  # user-facing workflows with local UI state and feature adapters
  shared/    # reusable UI, utilities, and typed Tauri API adapters
  styles/    # global styles and Tailwind entrypoint
```

Rules:

- `app` wires router and provider composition; it should not own feature workflow logic.
- `routes` owns route modules, route layouts, redirects, and not-found handling.
- `features` is created only for real user workflows; it owns local UI state and feature-specific adapters.
- `shared/api` owns typed frontend adapters for Tauri commands/events once those contracts exist, so React components avoid scattered direct IPC calls.
- Use `api` or `client` for frontend boundary adapters; avoid a broad `services` folder unless it has a narrower owner such as `features/<name>/api`.
- `shared/ui` is for genuinely reusable primitives, not one-off feature markup.

## Tauri Runtime Layers

```text
src-tauri/src/
  app/        # composition root, AppState, startup/shutdown lifecycle
  interface/  # Tauri commands, events, and IPC adapters
  modules/    # business/runtime modules
  platform/   # OS-specific implementations
  shared/     # shared config, error, and cross-cutting types
```

## Dependency Direction

Preferred flow:

```text
frontend UI
  -> interface/commands
    -> app/AppState
      -> modules
        -> platform
```

Rules:

- `interface` maps IPC input and output; it should not own business logic.
- `app` wires dependencies and orchestrates startup/shutdown; it should not own module internals.
- `modules` owns business and runtime rules; it should not depend directly on Tauri or OS APIs.
- `platform` owns OS-specific behavior and external process details.
- `shared` holds genuinely cross-cutting types only.

## Runtime Modules

A runtime module is a module that owns a live resource such as a process, listener, stream, worker, or background task.

Runtime modules may implement `RuntimeModule`.

Plain modules that only transform data should not implement lifecycle traits.

## Frontend And Backend Docs

Do not split architecture docs into frontend/backend files yet.

Add focused docs later only when the area becomes large enough to need its own durable context, for example:

- `frontend.md` for complex UI state or frontend architecture
- `tauri-runtime.md` for runtime composition and IPC rules
- a focused module contract such as `avatar-process-lifecycle.md`

## Job-Specific Guides

Do not create `docs/skills/` yet.

Use `docs/workflows/` for repeated jobs until there are enough stable, specialized guides to justify a separate skills area.
