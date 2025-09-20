# AGENTS.md - Development Guide for leptos-shadcn-ui

## Build/Test Commands
- **Build**: `cargo build --workspace` (or `make build`)
- **Test workspace**: `cargo test --workspace --lib` (or `make test-rust`)
- **Test single component**: `cargo test -p leptos-shadcn-<component>` (e.g., `cargo test -p leptos-shadcn-button`)
- **Test with verbose**: `RUST_LOG=debug cargo test --workspace --lib -- --nocapture`
- **E2E tests**: `make test-e2e` or `pnpm playwright test`
- **Lint**: `cargo clippy --workspace -- -D warnings` (or `make lint`)
- **Format**: `cargo fmt --all` (or `make fmt`)
- **Check**: `cargo check --workspace` (or `make check`)

## Architecture
- **Workspace**: Monorepo with packages in `packages/leptos/` for components
- **Components**: 46 ShadCN UI components for Leptos v0.8+
- **Structure**: Each component has lib.rs, default.rs, new_york.rs, signal_managed.rs, tests.rs
- **Testing**: TDD-focused with separate test modules in `src/tdd_tests/`
- **Performance**: Includes performance-audit system and benchmarking

## Code Style
- **Imports**: Leptos imports first (`use leptos::prelude::*;`), then utilities, external crates, internal
- **Naming**: kebab-case packages, PascalCase components, snake_case functions, SCREAMING_SNAKE_CASE constants
- **Components**: Use `#[component]` with optional props via `MaybeProp<T>`, `Option<Callback<T>>` for events
- **Props**: Standard props are `class`, `id`, `style`, `children`, with `#[prop(into, optional)]`
- **Errors**: Use validation system with `ValidationRule`, comprehensive error boundaries
- **Derive**: Standard order `#[derive(Debug, Clone, PartialEq)]` for most types
- **CSS**: Define class constants, use Signal::derive for dynamic classes
- **Documentation**: Module docs with `//!`, inline comments for accessibility and TDD enhancements
