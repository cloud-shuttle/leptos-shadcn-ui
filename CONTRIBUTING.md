# Contributing to Leptos ShadCN UI

Welcome to the Leptos ShadCN UI project! This guide will help you get started as a contributor.

## 🚀 Quick Start for Contributors

### Prerequisites

- **Rust 1.75+** with Cargo
- **Node.js 18+** (for frontend tooling)
- **Git** for version control
- **Basic knowledge** of Leptos v0.8+ and Rust

### 1. Clone and Setup

```bash
# Clone the repository
git clone https://github.com/cloud-shuttle/leptos-shadcn-ui.git
cd leptos-shadcn-ui

# Install dependencies
cargo build --workspace

# Verify setup with TDD tests
cargo nextest run --package leptos-shadcn-contract-testing
```

### 2. Development Workflow

#### TDD-First Development
This project follows Test-Driven Development (TDD) principles:

```bash
# 1. Run contract tests before making changes
cargo nextest run --package leptos-shadcn-contract-testing

# 2. Make your changes
# 3. Run tests again to ensure nothing broke
cargo nextest run --package leptos-shadcn-contract-testing

# 4. Run full workspace check
cargo check --workspace
```

#### Component Development
```bash
# Test individual component
cargo build --package leptos-shadcn-button

# Test main package with specific features
cargo build --package leptos-shadcn-ui --features button,input,card,dialog
```

### 3. Project Structure

```
leptos-shadcn-ui/
├── packages/
│   ├── leptos/              # Individual component packages
│   │   ├── button/          # Button component
│   │   ├── input/           # Input component
│   │   └── ...
│   ├── contract-testing/    # TDD framework
│   ├── signal-management/   # Signal lifecycle management
│   └── tailwind-rs-core/    # Type-safe Tailwind CSS
├── examples/leptos/         # Example applications
├── docs/                    # Documentation
└── scripts/                 # Automation scripts
```

### 4. Adding New Components

#### Step 1: Create Component Package
```bash
# Use the component generator
cargo run --package leptos-shadcn-component-generator -- --name my-component
```

#### Step 2: Implement TDD Tests
```rust
// In your component's tests/
#[cfg(test)]
mod tests {
    use super::*;
    use leptos_shadcn_contract_testing::*;

    #[test]
    fn test_component_contracts() {
        let tester = ContractTester::new();
        tester.validate_component_contracts("my-component");
    }
}
```

#### Step 3: Update Workspace
Add your component to `Cargo.toml` workspace members and dependencies.

### 5. Performance Requirements

All components must meet these performance contracts:

- **Bundle Size**: < 500KB per component
- **Render Time**: < 16ms initial render
- **WASM Compatibility**: Full compatibility with WebAssembly targets

```bash
# Validate performance contracts
cargo run --package leptos-shadcn-contract-testing --bin validate_performance
```

### 6. Code Quality Standards

#### Rust Standards
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Follow Rust naming conventions
- Document all public APIs

#### Component Standards
- Implement both `default` and `new_york` variants
- Include comprehensive prop documentation
- Support signal-based state management
- Include accessibility features

### 7. Testing Strategy

#### Contract Testing
```bash
# Run all contract tests
cargo nextest run --package leptos-shadcn-contract-testing

# Run specific test categories
cargo nextest run --package leptos-shadcn-contract-testing --test-threads 1
```

#### Integration Testing
```bash
# Test example applications
cd examples/leptos
cargo run
```

### 8. Dependency Management

#### Automated Dependency Fixing
```bash
# Fix dependency issues automatically
cargo run --package leptos-shadcn-contract-testing --bin fix_dependencies
```

#### Version Consistency
All packages must use version `0.8.0` and workspace dependencies.

### 9. Pull Request Process

1. **Fork** the repository
2. **Create** a feature branch: `git checkout -b feature/my-feature`
3. **Implement** your changes with tests
4. **Run** the full test suite: `cargo nextest run --workspace`
5. **Update** documentation if needed
6. **Submit** a pull request with a clear description

### 10. Common Issues and Solutions

#### Build Failures
```bash
# Clean and rebuild
cargo clean
cargo build --workspace

# Check for dependency issues
cargo run --package leptos-shadcn-contract-testing --bin fix_dependencies
```

#### Test Failures
```bash
# Run tests with verbose output
cargo nextest run --package leptos-shadcn-contract-testing -- --nocapture

# Check specific test
cargo test --package leptos-shadcn-contract-testing test_name
```

#### Performance Issues
```bash
# Profile bundle size
cargo run --package leptos-shadcn-contract-testing --bin profile_bundle_size

# Check render performance
cargo run --package leptos-shadcn-contract-testing --bin profile_render_time
```

### 11. Getting Help

- **Documentation**: Check `docs/` directory
- **Issues**: Open a GitHub issue
- **Discussions**: Use GitHub Discussions
- **Examples**: Look at `examples/leptos/` for usage patterns

### 12. Release Process

Releases are automated through CI/CD. To trigger a release:

1. Update version numbers in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create a release tag
4. CI/CD will handle publishing

## 🎯 Development Goals

- **Quality**: Maintain 100% test pass rate
- **Performance**: Meet all performance contracts
- **Compatibility**: Full Leptos v0.8+ compatibility
- **Documentation**: Comprehensive API documentation
- **Accessibility**: WCAG 2.1 AA compliance

## 📚 Additional Resources

- [Leptos Documentation](https://leptos.dev/)
- [ShadCN UI Design System](https://ui.shadcn.com/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Project Architecture](docs/architecture/)

---

**Happy Contributing!** 🚀

For questions or support, please open an issue or start a discussion.
