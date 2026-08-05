# Assistant Feature Vision

## Product Direction

Teo is not just a desktop app with a chat box. Teo is a 2D desktop AI companion that lives on top of the Windows desktop, listens and talks through voice, reacts through an animated avatar, understands user context, and helps with everyday computer tasks.

The product should feel like a small assistant system:

- visible as a 2D character overlay
- interactive through voice, clicks, and lightweight UI
- capable of doing useful work, not only answering text
- personal enough to remember preferences and recent context
- safe enough to confirm risky actions before acting

## MVP Definition

The first useful MVP should prove this loop:

```text
User asks or clicks
  -> Teo understands intent
  -> Teo performs a small useful action
  -> Teo responds through UI, voice, and avatar state
```

The MVP should not try to be a full autonomous agent yet. It should be a reliable assistant that can be called, understood, controlled, and trusted for small tasks.

## Core Experience

### 1. Desktop Avatar Overlay

Teo should appear as a 2D character on the desktop.

Expected capabilities:

- stay above normal windows when active
- be draggable around the screen
- support click-through when the user needs to interact with apps behind it
- show clear states: idle, listening, thinking, speaking, success, error, sleeping
- avoid blocking the user's workspace

Why it matters:

The avatar is the product's identity. It turns the assistant from a hidden background service into something users can naturally interact with.

MVP version:

- always-on-top avatar process
- start, stop, restart, and status control
- basic idle/listening/thinking/speaking visual states

### 2. Voice Interaction

Teo should support natural voice input and voice output.

Expected capabilities:

- push-to-talk for the first MVP
- later wake word support, such as "Hey Teo"
- speech-to-text for user commands
- text-to-speech for assistant responses
- clear listening and speaking avatar states

Why it matters:

Voice makes Teo feel like a companion instead of another app window.

MVP version:

- push-to-talk first
- cloud STT/TTS preferred at first to keep local resource use low
- wake word can come later when the assistant loop is stable

### 3. Screen Understanding

Teo should be able to understand what is on screen when the user asks.

Example user requests:

- "Look at this error and explain what is wrong."
- "Summarize this window."
- "What should I click next?"
- "Turn this text into a note."
- "Compare these two visible windows."

Why it matters:

This is a stronger differentiator than opening apps. It makes Teo useful inside the user's current workflow.

MVP version:

- capture screenshot on demand
- send screenshot to a vision-capable model
- answer with explanation or suggested next step
- require user confirmation before any click or typing action

### 4. Notes And Lightweight Memory

Teo should remember small things that make future interactions easier.

Expected capabilities:

- create notes from voice or selected/screen text
- search recent notes
- summarize notes
- remember simple preferences
- remember current project context when explicitly saved

Example user requests:

- "Remember that this project uses Tauri and Godot."
- "Make a note from this screen."
- "What did we decide about avatar lifecycle?"
- "Save this as a todo list."

Why it matters:

Memory makes the assistant feel personal without requiring risky automation.

MVP version:

- local notes storage
- simple note title/body/timestamp
- manual save and search
- no hidden long-term memory until privacy rules are clear

### 5. Desktop Actions

Teo should perform small desktop actions that save time.

Expected capabilities:

- open apps
- open websites
- open project folders
- create files or notes
- run safe predefined commands
- start focused workflows

Example user requests:

- "Open VS Code in this project."
- "Open my notes."
- "Create a note called Avatar ideas."
- "Start the dev server."

Why it matters:

Actions make Teo more than a chatbot.

MVP version:

- open app or URL
- create/read notes
- run only allowlisted local commands
- ask for confirmation before commands that modify files or system state

### 6. Workflow Assistance

Teo should help with multi-step work, especially development and planning tasks.

Expected capabilities:

- summarize current task
- read terminal/compiler errors
- propose next steps
- track a small checklist
- watch a command and report when it completes

Example user requests:

- "What should I do next for this issue?"
- "Explain this cargo test failure."
- "Turn this Linear issue into a checklist."
- "Watch the build and tell me if it fails."

Why it matters:

This creates real assistant value before full desktop automation is ready.

MVP version:

- task checklist panel
- command/test result explanation
- issue summary from local docs or connected tools

### 7. Reactive Personality

Teo should react to events through animation and behavior.

Expected capabilities:

- idle animation when waiting
- listening animation when recording
- thinking animation during model calls
- speaking animation during TTS
- success animation when a task finishes
- confused/error animation when something fails
- sleep animation after long inactivity

Why it matters:

Small reactions make the assistant feel alive and help users understand current state without reading logs.

MVP version:

- 6 core animation states: idle, listening, thinking, speaking, success, error
- state changes driven by assistant events
- no complex emotion engine yet

### 8. Safe Automation

Teo can eventually click, type, and operate apps, but this should be introduced carefully.

Expected capabilities:

- identify UI elements from screenshots
- suggest where to click
- type into focused fields after confirmation
- execute multi-step desktop workflows
- recover when the UI changes

Why it matters:

This is powerful but risky. Bad automation can click the wrong thing, lose work, or confuse the user.

MVP version:

- explain and suggest actions first
- require explicit confirmation before clicking or typing
- avoid autonomous destructive actions

## Capability Roadmap

### Phase 1: Avatar Control

Goal:

Make Teo visible, controllable, and stable.

Features:

- avatar process lifecycle
- Tauri commands for start, stop, restart, status
- minimal frontend control panel
- always-on-top and click-through behavior
- clean shutdown

### Phase 2: Basic Assistant Loop

Goal:

Make Teo respond to the user.

Features:

- push-to-talk
- speech-to-text
- LLM response
- text-to-speech
- avatar state changes for listening, thinking, speaking

### Phase 3: Useful Personal Tasks

Goal:

Make Teo do simple work.

Features:

- notes
- reminders
- open apps and URLs
- local preferences
- recent conversation/task context

### Phase 4: Screen-Aware Help

Goal:

Make Teo useful inside the current desktop workflow.

Features:

- screenshot on demand
- screen explanation
- error reading
- summarize visible content
- create notes from screen content

### Phase 5: Safer Automation

Goal:

Let Teo act on the desktop with user control.

Features:

- suggested clicks
- confirmed typing
- allowlisted workflows
- command execution with confirmation
- progress monitoring

### Phase 6: Background Companion

Goal:

Make Teo feel present without being annoying.

Features:

- wake word
- tray controls
- passive reminders
- focus-aware behavior
- richer personality and animations

## Resource Strategy

To keep the MVP realistic:

- keep the desktop shell local
- keep avatar rendering local through Godot
- keep notes and settings local
- use cloud AI APIs first for LLM, vision, STT, and TTS
- avoid local AI models until the product loop is proven

Local-first features are cheap and reliable. Heavy AI features should start cloud-backed so the app works on normal laptops.

## What Not To Build First

Avoid these early:

- full autonomous desktop control
- local offline LLM or vision model
- complex plugin marketplace
- large settings dashboard
- multi-agent planning
- hidden long-term memory without clear privacy rules

These can come later, but they should not block the first useful version.

## Near-Term MVP Slice

The best first "worth using" slice is:

```text
2D avatar overlay
  + start/stop/restart/status controls
  + push-to-talk
  + simple voice response
  + notes
  + screenshot question answering
```

This gives users something memorable:

- Teo is visible.
- Teo can listen.
- Teo can answer.
- Teo can save useful notes.
- Teo can look at the screen when asked.

That is enough to feel like an assistant system, not just a launcher.
