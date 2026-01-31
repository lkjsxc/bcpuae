# System Contracts

Cross-cutting invariants that make the system cohesive and debuggable.

Treat these as "build-breaking" constraints: if code contradicts a contract, code is wrong (or a decision record is required).

## Contracts

- **[Mode Contract](mode.md)** — Mode state invariants
- **[Buffer Contract](buffer.md)** — Text storage invariants
- **[Error Handling](errors.md)** — Error propagation rules
- **[Event Loop](event_loop.md)** — Main loop invariants
- **[Rendering](rendering.md)** — Draw loop constraints
