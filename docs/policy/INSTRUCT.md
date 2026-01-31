# INSTRUCT.md — Always Read First

This repository is developed with LLM assistance. Optimize for deterministic iteration, architectural coherence, and correctness. Human-facing ergonomics are secondary; **clarity and invariants are primary**.

## 0. Operating Contract

### 0.1 Mandatory Two-Phase Execution

1. **Plan & Structure First**
   - Clarify target behavior and invariants
   - Update/reshape documentation and directory structure as needed
2. **Implement Second**
   - Write/modify source code following the plan
   - Add tests and validate

Never jump directly into code without first stabilizing the plan and docs for the touched area.

### 0.2 Response Format (Every PR-Sized Change)

In your output, use these headings in order:

1. **Intent**
2. **Plan**
3. **Repo Tree Impact**
4. **Docs Impact**
5. **Code Impact**
6. **Tests**
7. **Commits (messages, in order)**
8. **Risks / Follow-ups**

Keep it crisp and explicit. Prefer checklists over prose.

---

## 1. Hard Repository Invariants

### 1.1 Root Layout (Strict Allowlist)

Project root may contain only:
- `README.md`
- `LICENSE`
- `.gitignore`
- `Cargo.toml`
- `Cargo.lock`
- `Dockerfile`
- `docker-compose.yml`
- `Makefile`
- `config/`
- `docs/`
- `scripts/`
- `src/`
- `.github/`
- hidden files/dirs (e.g., `.env.example`, `.vscode/`)

Anything else at root is a regression. Move it under appropriate directory.

### 1.2 Documentation Topology (Recursive TOC Discipline)

- **Every directory** must contain **exactly one** `README.md` (acts as that directory's TOC)
- All other docs live as:
  - additional `.md` files in the directory, and/or
  - subdirectories (each with their own `README.md`)
- Delete documentation that is no longer used

### 1.2.1 Unfinished Work Is Isolated

- Do not leave "TODO/TBD/not implemented" placeholders in production docs/code
- The only acceptable place for unfinished work checklists is `docs/implementation/todo/`

### 1.3 Code Topology (Short Files; Deep Trees)

Prefer many small files and deep directories over large files.

**File Size Guidelines** (soft, but strongly preferred):
- Source code: target ≤ 200 lines
- Docs: target ≤ 300 lines
- If you must exceed, split the file instead of stretching the limit
- Treat `mod.rs` as thin entrypoints; split logic into modules

### 1.4 Runtime Constraint (Single Container)

The runtime environment is **one Docker container** that runs:
- The Rust editor binary
- All terminal I/O via crossterm

### 1.5 Docker-First Builds and Tests

- Compilation and tests must run via Docker
- Do not rely on host toolchains as the canonical path

---

## 2. Project Identity

### 2.1 Editor Type

A modal text editor (Vim-inspired) with:
- Async I/O via Tokio
- Terminal UI via ratatui/crossterm
- MPSC notification system
- Three modes: Normal, Insert, Command

### 2.2 Core Requirements

- **Modal editing** is non-negotiable (Normal/Insert/Command)
- **Async file I/O** — never block the UI thread
- **Graceful degradation** — simple mode when no TTY available
- **Panic safety** — terminal always restored to canonical mode

---

## 3. Agent Workflow Rules

### 3.1 Default Workflow Checklist

- [ ] Read closest directory `README.md` files for touched areas
- [ ] Identify invariants (file limits, mode behavior, error handling)
- [ ] Update docs / TOCs to reflect intended structure
- [ ] Implement code in small cohesive slices
- [ ] Add/adjust tests
- [ ] Update docs and TOCs
- [ ] Commit with clean messages

### 3.2 Determinism Discipline

Prefer:
- Explicit state machines (EditorMode enum)
- Fixed tick loop for notifications
- Bounded queues / capped growth
- Defensive bounds checking

Avoid:
- Unbounded memory growth
- Ad-hoc patches
- "Smart" but fragile implicit behavior

### 3.3 "Fixes Must Improve Structure"

When fixing bugs:
- Clarify invariants and failure modes
- Improve types/structures to prevent recurrence
- Add tests that fail before the fix

No duct tape.

### 3.4 Git Discipline

Commit frequently:
- Behavior changes and feature slices
- Refactors/restructures
- Docs-only updates

Commit message format:
- `feat(<area>): <summary>`
- `fix(<area>): <summary>`
- `refactor(<area>): <summary>`
- `docs(<area>): <summary>`
- `test(<area>): <summary>`
- `chore(<area>): <summary>`

Area examples: `core`, `ui`, `system`, `operations`, `docker`, `docs`

---

## 4. Quality Bar (Definition of Done)

A slice is done only when:
- Feature works in TTY mode
- Feature degrades gracefully in simple mode (if applicable)
- Error handling is explicit (no silent failures)
- Tests exist for critical logic
- File size limits respected
- Docs and TOCs are updated
- Commits are clean and ordered

If you cannot reach full DoD, explicitly list what is missing and why.

---

## 5. If You Are Uncertain

Do not stall. Make the best coherent decision consistent with this document and:
- Append the decision into `docs/implementation/decision_log.md`
- Implement accordingly
- Leave a follow-up note

This repo favors forward motion with explicit decisions over ambiguity.
