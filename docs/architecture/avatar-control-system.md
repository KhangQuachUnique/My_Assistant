# Avatar Control System

## Goal

Define the first control contract for changing the running Godot avatar from the Tauri app.

`AVT-001` made the avatar process manageable. `AVT-002` makes the running avatar controllable.

## Boundaries

`frontend` owns debug controls and user interaction. It calls Tauri commands and renders success or error states.

`interface/commands` owns Tauri command adapters. Commands validate IPC input shape, call application state, and map responses or errors for the frontend.

`app` owns dependency wiring. It stores avatar lifecycle and control dependencies in managed state.

`modules/avatar` owns avatar lifecycle and control rules. It validates requested animation names and calls an avatar control transport port.

`platform` owns concrete transport details when they are OS-specific or process-adjacent.

`apps/avatar` owns Godot-side command handling and maps accepted control states to available Godot animations.

## Dependency Direction

```text
frontend UI
  -> interface/commands
    -> app/AppState
      -> modules/avatar
        -> avatar control transport port
          -> platform or transport adapter
            -> Godot avatar process
```

Commands must not know how the transport works. Godot must not know about Tauri command DTOs.

## MVP Animation Contract

The app-level animation names are stable lower-case strings:

```text
idle
listening
thinking
speaking
success
error
```

Meanings:

- `idle`: avatar is waiting.
- `listening`: user audio is being captured or a push-to-talk session is active.
- `thinking`: assistant is processing a request.
- `speaking`: assistant is responding through voice or text.
- `success`: a task finished successfully.
- `error`: a task failed or needs user attention.

These names are the contract between frontend, Tauri, and Godot. Godot may map a contract name to whatever local animation asset is currently available.

Current Godot state:

- available scene animations: `idle`, `speaking`
- current controller states: `IDLE`, `SPEAKING`

Until more assets exist, Godot should accept every MVP contract animation and may fallback like this:

```text
idle      -> idle
listening -> idle
thinking  -> idle
speaking  -> speaking
success   -> idle
error     -> idle
```

Unknown animation names must be rejected before they are sent to Godot when possible.

## Tauri Command Contract

Command name:

```text
avatar_set_animation
```

Request:

```json
{
  "animation": "thinking"
}
```

Success response:

```json
{
  "animation": "thinking",
  "accepted": true
}
```

Error response:

```json
{
  "code": "unknownAnimation",
  "message": "unknown avatar animation: dance"
}
```

Error codes:

```text
unknownAnimation
avatarNotRunning
avatarNotReady
transportUnavailable
commandTimeout
commandRejected
internal
```

`avatarNotRunning` means the avatar lifecycle module does not currently report `running`.

`avatarNotReady` means the process may be running but the Godot control endpoint has not accepted a readiness check yet.

`transportUnavailable` means the configured control channel cannot be opened or used.

`commandTimeout` means Godot did not respond before the command timeout.

`commandRejected` means Godot received the command but refused it.

## Transport Contract

Use a local-only JSON-lines control protocol for the MVP.

Transport shape:

- Godot listens on loopback only: `127.0.0.1`
- Tauri connects as the client.
- Each request and response is one JSON object followed by a newline.
- Every request includes an `id`.
- Every response includes the same `id`.
- The MVP may use a fixed app constant for the port; move it into config when port collisions or multi-avatar support become real.

Readiness check:

```json
{
  "id": "request-1",
  "type": "ping"
}
```

Readiness response:

```json
{
  "id": "request-1",
  "ok": true,
  "type": "pong"
}
```

Set animation request:

```json
{
  "id": "request-2",
  "type": "set_animation",
  "animation": "thinking"
}
```

Set animation response:

```json
{
  "id": "request-2",
  "ok": true,
  "animation": "thinking"
}
```

Rejected response:

```json
{
  "id": "request-2",
  "ok": false,
  "error": {
    "code": "unknownAnimation",
    "message": "unknown avatar animation: dance"
  }
}
```

## Ownership Decisions

Rust validates animation names before sending transport commands.

Godot also validates animation names because transport input is an external boundary.

Rust command DTOs stay in `interface/commands`.

Avatar control request, response, animation, and error domain types should live in `modules/avatar`.

Transport request/response serialization should live next to the concrete transport adapter, not inside the Tauri command layer.

For MVP, command queueing is out of scope. If the avatar is not ready, return `avatarNotReady`.

## Out Of Scope

- frontend debug controls
- automatic animation changes from assistant events
- voice, TTS, or LLM state integration
- complex behavior trees or emotion engines
- multiple avatar instances
