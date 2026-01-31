# Buffer Contract

Invariants for text storage.

## Invariants

### INV-BUF-1: Always At Least One Line

A buffer always contains at least one line. Empty file = `[""]` (one empty string).

### INV-BUF-2: No Trailing Newlines in Storage

Newlines are separators, not terminators. Lines are stored without their trailing `\n`.

Example:
```
File: "line1\nline2\n"
Storage: ["line1", "line2", ""]

File: "line1\nline2"
Storage: ["line1", "line2"]
```

### INV-BUF-3: Valid UTF-8 Only

All text is valid UTF-8. Invalid sequences are replaced or rejected.

### INV-BUF-4: Character-Aware Operations

All operations respect Unicode character boundaries (not byte indices).

Example:
```rust
// CORRECT: use char_indices()
let byte_idx = line.char_indices().nth(x).map(|(i, _)| i);

// WRONG: byte indexing
let byte_idx = x; // Assumes 1 byte per char
```

## Line Boundaries

- `y` index: 0 to `line_count - 1`
- `x` index: 0 to `line_length` (inclusive, cursor can be after last char)

## Operations

### Insert Character

```rust
// Precondition: y < line_count, x <= line_length
// Postcondition: line_length increased by 1
```

### Delete Character

```rust
// Precondition: y < line_count
// Postcondition:
//   - If x > 0: line_length decreased by 1
//   - If x = 0, y > 0: line merged with previous
//   - If x = 0, y = 0: no change
```

### Insert Newline

```rust
// Precondition: y < line_count, x <= line_length
// Postcondition:
//   - Current line split at x
//   - New line created with content after x
//   - Line count increased by 1
```

## String Conversion

```rust
// To string: join with "\n", add final "\n" if original had one
buffer.to_string() -> lines.join("\n")

// From string: split on "\n", handle trailing newline
Buffer::from_str(s) -> lines
```
