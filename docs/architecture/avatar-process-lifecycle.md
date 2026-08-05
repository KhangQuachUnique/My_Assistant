# Avatar Process Lifecycle

## Goal

Define the contract for managing `avatar.exe` without leaking Tauri, Windows, or raw process handles into the avatar business module.

## Boundaries

`app` is the composition root. It creates `AvatarModule`, stores it in `AppState`, and starts or stops it from the application lifecycle.

`modules/avatar` owns avatar runtime rules. It exposes `AvatarModule`, `AvatarService`, lifecycle outcomes, `AvatarError`, and the `AvatarProcessRunner` port.

`platform/windows` will own the Windows implementation that starts, stops, and checks the external avatar process. It must not expose `std::process::Child` or native handles to `modules/avatar`, `app`, or `interface`.

`interface/commands` will own Tauri command adapters. Commands should map input and output only, then call `AvatarModule`.

## Contract

`AvatarModule` implements the shared `RuntimeModule` contract:

- `start`
- `stop`
- `status`

Avatar-specific runtime API:

- `restart`

`AvatarService` owns process-level decisions:

- duplicate start returns `AlreadyRunning`
- stop while not running returns `AlreadyStopped`
- restart starts the process when stopped
- restart stops and starts the process when already running

`AvatarProcessRunner` is the process port used by `AvatarService`:

- `start`
- `stop`
- `is_running`

## Resource Path

The avatar executable is stored as a Tauri resource:

```text
src-tauri/resources/modules/avatar/avatar.exe
```

This path is declared in `tauri.conf.json` under `bundle.resources`.

## Out Of Scope

- Windows process spawning implementation
- Tauri commands
- Tauri to Godot IPC
- waiting for Godot readiness
- taskbar or Alt+Tab window behavior
