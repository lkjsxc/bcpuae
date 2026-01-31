# Mode Transitions

## State Machine

```
┌─────────┐    i/a/o     ┌─────────┐
│ NORMAL  │─────────────▶│ INSERT  │
│         │◀─────────────│         │
└────┬────┘     Esc      └─────────┘
     │
     │ :
     ▼
┌─────────┐
│ COMMAND │
│  (cmd)  │◀─── Any char appends
└────┬────┘
     │
     │ Enter or Esc
     ▼
┌─────────┐
│ NORMAL  │
└─────────┘
```

## Transition Table

| From | Trigger | To | Notes |
|------|---------|-----|-------|
| Normal | `i` | Insert | Insert before cursor |
| Normal | `a` | Insert | Insert after cursor |
| Normal | `o` | Insert | Open new line below, then Insert |
| Normal | `:` | Command | Show command line |
| Insert | `Esc` | Normal | Return to navigation |
| Command | `Esc` | Normal | Cancel command |
| Command | `Enter` | Normal | Execute command |

## Visual Indicators

Mode change is immediately visible:

- Status bar background color changes
- Mode text updates (NORMAL/INSERT/COMMAND)
- Command line appears/disappears (Command mode)

## Invariants

1. **Always in exactly one mode** — No ambiguous states
2. **Mode is always visible** — User never guesses current mode
3. **Esc returns to Normal** — Universal "get me out" key
4. **Commands execute in Normal** — Most actions require Normal mode
