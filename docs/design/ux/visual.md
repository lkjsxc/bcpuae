# Visual Design

Color schemes and styling.

## Default Theme

### Colors

| Element | Color | Usage |
|---------|-------|-------|
| Background | Black | Editor background |
| Foreground | White | Text color |
| Accent | Cyan | Mode badges, highlights |
| Status BG | DarkGray | Status bar background |
| Status FG | White | Status bar text |
| Line Numbers | Gray | Gutter text |

### Notification Colors

| Level | Color |
|-------|-------|
| Info | Blue |
| Success | Green |
| Warning | Yellow |
| Fatal | Red |

## Mode Badges

Badge styling in status bar:

```
┌─────────┐
│ NORMAL  │  Cyan background, black text, bold
└─────────┘

┌─────────┐
│ INSERT  │  Green background, black text, bold
└─────────┘

┌─────────┐
│ COMMAND │  Yellow background, black text, bold
└─────────┘
```

## Typography

- Monospace font required
- Box-drawing character for gutter separator: `│`
- No ligatures (can confuse cursor positioning)

## Accessibility

- High contrast default
- Color not the only indicator (mode text always visible)
- Clear visual hierarchy
