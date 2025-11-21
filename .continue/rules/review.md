---
invokable: true
---

Review this code for potential issues, including:
- **Missing dependency declarations**: Currently `Cargo.toml` has no dependencies. Ensure any external crates used are added.
- **Module layout**: When new modules (`journal.rs`, `ui.rs`, `state.rs`) arrive, confirm they are imported correctly and follow Rust’s idioms.
- **State switching logic**: Verify that screen changes or state updates don’t panic or create race conditions.
- **Error handling**: Results should be propagated and user feedback should be clear.
- **Testing**: Once tests exist, cover corner cases, I/O reliability, and potential concurrency errors.
- **Linting**: Run `cargo fmt` and `cargo clippy` to catch style issues and possible bugs.
- **Documentation**: Public types and functions should have doc comments for better readability.
- **Dead code**: Keep an eye on unused or orphaned code blocks.

Provide specific, actionable recommendations for improvement.
