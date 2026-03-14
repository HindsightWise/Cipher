# The Company (Aion__Prime) - Structural Memory Engravings

This file is automatically tracked by the structural orchestrator loop. It serves as permanent cross-iteration memory for the Gemini Aion__Prime substrate.

## Codebase Patterns

- The engine operates purely on `tokio` async loops. Avoid bridging synchronous blocking tasks unless strictly wrapped in unblocking spawn threads.
- All structural changes must strictly produce verifiable `ExecutionReceipts`.
- The `eframe` GUI lives cleanly separated on the main thread.
- Neural monologues write safely to `sensory_cortex/monologue.log`.

## Gotchas & Defenses

- macOS severely restricts editing `Cargo.lock` during auto-builds (the Sandbox permission wall). Do NOT run `cargo add` or auto-trigger `cargo build` through agent subprocess commands; yield to the Operator for host-side compilation to unblock dependency changes.
- Beware infinite macro-recursion limit traps when routing `ui_log!`.
- **Sovereign Stringency:** NEVER use powerful, sweeping terminal text-replacement commands like `sed` willy-nilly on the core Rust engine. If a file needs repairing or modification, use precise, localized file manipulation tools. Do not take shortcuts; you are computationally fast enough to read, verify, and write exact, simple replacements.

> *Sovereign Directive: Update this file aggressively with architectural invariants as they manifest.*

## Communication & Interaction Directives

- **Execution Options:** When providing commands or execution paths to the Operator, only offer the single, guaranteed working option by default. If multiple options must be presented, explicitly explain the precise reasoning, context, and tradeoffs for each option.
