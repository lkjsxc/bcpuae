# Mode Contract

Invariants for the modal editing system.

## Invariants

### INV-MODE-1: Exactly One Mode

The editor is always in exactly one mode:
- Normal
- Insert
- Command

No ambiguous or transitional states.

### INV-MODE-2: Mode is Always Visible

The current mode is always displayed in the status bar.

### INV-MODE-3: Esc Returns to Normal

Pressing `Esc` in any mode returns to Normal mode (or cancels current operation).

### INV-MODE-4: Normal is Default

On startup and after command execution, mode is Normal.

## State Transitions

Valid transitions:
```
Normal ──i──▶ Insert
Normal ──:──▶ Command
Insert ──Esc──▶ Normal
Command ──Esc──▶ Normal
Command ──Enter──▶ Normal
```

Invalid transitions (must be rejected or no-op):
```
Insert ──i──▶ X (already in Insert)
Insert ──:──▶ X (must go through Normal)
Command ──i──▶ X (must execute or cancel first)
```

## Mode-Specific Behavior

| Capability | Normal | Insert | Command |
|------------|--------|--------|---------|
| Navigation keys | Yes | No | No |
| Text insertion | No | Yes | Yes (in buffer) |
| Command execution | Yes | No | On Enter |
| Save/Quit | Yes | No | Yes |

## Enforcement

- Mode stored in `EditorState.mode` enum
- `CommandMapper::map()` checks mode before returning Action
- UI rendering uses mode for status bar styling
