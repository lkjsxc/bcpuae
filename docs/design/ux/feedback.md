# Feedback

User feedback mechanisms.

## Notification System

Transient messages for:
- Operation success/failure
- Warnings
- Errors

### Priority

Higher priority messages should be more noticeable:
1. Fatal (Red) — Highest
2. Warning (Yellow)
3. Success (Green)
4. Info (Blue) — Lowest

## Status Bar Updates

Real-time indicators:
- Mode change: immediate visual update
- Dirty state: `[+]` appears on first edit
- Filename: updates on save-as

## Error Messages

Should be:
- Actionable (tell user what to do)
- Concise (fit in notification area)
- Persistent (stay until acknowledged for errors)

Examples:
```
Good:  "No file name (use :w <filename>)"
Bad:   "Error 42"

Good:  "No write since last change (add ! to override)"
Bad:   "Unsaved changes"
```

## Progress Indication

For async operations:
- Show "Loading..." in status bar
- Update to "Done" or error on completion
- Notifications for final result
