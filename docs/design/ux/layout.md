# Layout

Screen region definitions and geometry.

## Regions

```
┌─────────────────────────────────────┐
│                                     │
│           Editor Area               │  Min height: 1
│        (line numbers +              │
│         text content)               │
│                                     │
├─────────────────────────────────────┤
│ [INFO] Message one                  │  Height: 3
│ [WARN] Message two                  │
│ [OK]   File saved                   │
├─────────────────────────────────────┤
│ NORMAL  filename.txt  [+]           │  Height: 1
└─────────────────────────────────────┘
```

## Editor Area

Contains:
- Line number gutter (6 chars: " 123 │ ")
- Text content (remaining width)

Line number format:
```
{:>4} │    # Right-aligned, 4 digits max visible
```

Example:
```
   1 │ First line
   2 │ Second line
  10 │ Tenth line
 100 │ Hundredth line
```

## Notification Area

Fixed 3 rows at bottom:
- Shows last 3 notifications
- Oldest at top
- Newest at bottom
- Auto-expires based on TTL

## Status Bar

Single row at very bottom:
```
[MODE]  [filename]  [dirty]  [position]
```

Components:
- Mode badge (colored background)
- Filename (or "[No Name]")
- Dirty indicator `[+]` (if modified)
- Optional: line:column position

## Constraints

- Minimum terminal size: 40x10
- Below minimum: show error message and exit
