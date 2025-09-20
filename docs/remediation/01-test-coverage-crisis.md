# Test Coverage Crisis Remediation

## Issue Summary
**Severity**: 🔴 CRITICAL  
**Effort**: 40-60 hours  
**Priority**: P0 (Block all other work)

## Problem Description
Repository claims "100% test coverage" but analysis reveals:
- ~170 actual test assertions across entire codebase
- Majority are `assert!(true, "message")` placeholders
- No coverage tooling configured (tarpaulin, llvm-cov)
- Tests don't mount components in DOM
- No WASM test execution in CI

## Root Cause Analysis
1. **Test-Driven Development Theater**: Tests written to satisfy CI without validating functionality
2. **Missing Test Infrastructure**: No proper testing harness for Leptos components
3. **No Coverage Enforcement**: No gates preventing regression
4. **Copy-Paste Testing**: Same placeholder patterns across all components

## Remediation Steps

### Step 1: Audit Current Test Reality (4 hours)
```bash
# Count real vs placeholder tests
find packages/leptos -name "*.rs" -type f -exec grep -l "assert!(true" {} \; | wc -l
find packages/leptos -name "*.rs" -type f -exec grep -l "assert_eq!\|assert_ne!" {} \; | wc -l

# Generate coverage baseline
cargo install cargo-llvm-cov
cargo llvm-cov --html --output-dir coverage-report/
```

### Step 2: Fix Core Component Tests (20-30 hours)
Priority components to fix first:
1. **Button** - Most critical, used everywhere
2. **Input** - Form foundation
3. **Card** - Layout foundation  
4. **Badge** - Simple but essential
5. **Label** - Accessibility critical

**Example Real Test (Button)**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn button_renders_with_text() {
        mount_to_body(|| {
            view! {
                <Button>"Click me"</Button>
            }
        });
        
        let button = document()
            .query_selector("button")
            .unwrap()
            .unwrap();
            
        assert_eq!(button.text_content().unwrap(), "Click me");
        assert!(button.class_list().contains("bg-primary"));
    }
    
    #[wasm_bindgen_test] 
    fn button_handles_click_events() {
        let clicked = create_rw_signal(false);
        
        mount_to_body(|| {
            view! {
                <Button on_click=move |_| clicked.set(true)>
                    "Click me"
                </Button>
            }
        });
        
        let button = document()
            .query_selector("button")
            .unwrap()
            .unwrap();
            
        button.click();
        assert!(clicked.get());
    }
}
```

### Step 3: Add Coverage Infrastructure (8 hours)
```toml
# Add to Cargo.toml [dev-dependencies]
wasm-bindgen-test = "0.3"
web-sys = "0.3"

# Add coverage config
[toolchain]
channel = "nightly"

[env]
RUSTFLAGS = "-C instrument-coverage"
```

### Step 4: CI Integration (4-6 hours)
```yaml
# Add to CI pipeline
- name: Generate Coverage
  run: |
    cargo install cargo-llvm-cov
    cargo llvm-cov --workspace --lcov --output-path lcov.info
    
- name: Upload Coverage 
  uses: codecov/codecov-action@v3
  with:
    file: lcov.info
    
- name: Coverage Gate
  run: |
    coverage=$(cargo llvm-cov --workspace --summary-only | grep "TOTAL" | awk '{print $10}' | tr -d '%')
    if [ $(echo "$coverage < 80" | bc -l) -eq 1 ]; then
      echo "Coverage $coverage% below 80% threshold"
      exit 1
    fi
```

### Step 5: WASM Test Execution (6 hours)
```yaml
- name: Install wasm-pack
  run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

- name: Run WASM Tests
  run: |
    for package in packages/leptos/*/; do
      cd "$package"
      wasm-pack test --headless --chrome
      cd - 
    done
```

## Success Criteria
- [ ] Real coverage report showing actual percentages
- [ ] All placeholder `assert!(true)` tests replaced
- [ ] Core 5 components have 80%+ coverage
- [ ] WASM tests running in CI
- [ ] Coverage gates preventing regression
- [ ] Documentation on how to write proper tests

## Risk Mitigation
- **Risk**: Breaking existing functionality while fixing tests
- **Mitigation**: Fix one component at a time, test in isolation

- **Risk**: WASM test setup complexity  
- **Mitigation**: Use proven wasm-bindgen-test patterns

- **Risk**: Performance impact of coverage
- **Mitigation**: Only run coverage on merge requests, not every push

## Dependencies
- Rust 1.70+ for coverage tooling
- Chrome/Firefox for WASM testing
- CI runner with sufficient memory

## Owner
**Primary**: Senior Frontend Engineer with Rust/WASM experience  
**Secondary**: Test Engineer for CI integration  
**Reviewer**: Staff Engineer for architecture validation
