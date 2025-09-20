# File Size Remediation Plan

## Issue Summary
**Severity**: 🟡 HIGH  
**Effort**: 20-30 hours  
**Priority**: P1 (Blocks testing and LLM comprehension)

## Problem Description
Multiple files exceed 300-line limit, impacting:
- Test granularity and isolation
- LLM context understanding
- Code review efficiency  
- Maintainability and debugging

**Files Exceeding Limit**:
- `select/src/implementation_tests.rs` - 891 lines
- `button/src/tests.rs` - 844 lines
- `switch/src/implementation_tests.rs` - 760 lines
- `table/src/data_table.rs` - 689 lines
- Plus 15 more files 500+ lines

## Root Cause Analysis
1. **Monolithic Test Files**: All tests crammed into single files
2. **God Objects**: Complex components not properly decomposed
3. **Copy-Paste Inflation**: Repeated test patterns instead of helpers
4. **Missing Abstractions**: No shared test utilities

## Remediation Strategy

### Phase 1: Test File Decomposition (12-16 hours)

**Break down by test category**:
```
button/src/tests.rs (844 lines) →
├── tests/
│   ├── rendering_tests.rs      (~150 lines)
│   ├── interaction_tests.rs    (~150 lines) 
│   ├── accessibility_tests.rs  (~150 lines)
│   ├── variant_tests.rs        (~150 lines)
│   ├── edge_case_tests.rs      (~150 lines)
│   └── integration_tests.rs    (~100 lines)
└── test_utils.rs               (~50 lines)
```

**Example Decomposition**:
```rust
// button/src/tests/rendering_tests.rs
use super::super::*;
use crate::test_utils::*;

#[cfg(test)]
mod rendering {
    use super::*;
    
    #[wasm_bindgen_test]
    fn renders_default_button() {
        let result = render_component(|| {
            view! { <Button>"Test"</Button> }
        });
        
        assert_button_has_class(&result, "bg-primary");
        assert_button_text(&result, "Test");
    }
    
    // More focused rendering tests...
}
```

### Phase 2: Component Decomposition (8-12 hours)

**Break down large components**:
```
table/src/data_table.rs (689 lines) →
├── components/
│   ├── table_header.rs         (~100 lines)
│   ├── table_row.rs           (~100 lines)
│   ├── table_cell.rs          (~80 lines)
│   ├── table_pagination.rs    (~120 lines)
│   └── table_sorting.rs       (~100 lines)
├── hooks/
│   ├── use_table_state.rs     (~80 lines)
│   └── use_sorting.rs         (~60 lines)
└── lib.rs                     (~60 lines - exports only)
```

### Phase 3: Shared Test Utilities (4-6 hours)

**Create common test infrastructure**:
```rust
// packages/test-utils/src/component_testing.rs
pub fn render_component<F, V>(component: F) -> ComponentTestResult 
where
    F: Fn() -> V + 'static,
    V: IntoView,
{
    // Standard component mounting and testing setup
}

pub fn assert_button_has_class(result: &ComponentTestResult, class: &str) {
    // Reusable assertion logic
}

pub fn assert_accessibility_compliance(result: &ComponentTestResult) {
    // Shared a11y testing
}
```

## Implementation Plan

### Week 1: Critical Test Files
**Day 1-2**: button/src/tests.rs → 6 focused test files
**Day 3-4**: select/src/implementation_tests.rs → category-based split
**Day 5**: switch/src/implementation_tests.rs → interaction focus

### Week 2: Component Architecture  
**Day 1-2**: table/src/data_table.rs → component decomposition
**Day 3-4**: Remaining large implementation files
**Day 5**: Shared utilities and cleanup

### File Size Rules Going Forward
```rust
// Add to rustfmt.toml or clippy config
max_lines = 300

// Add to CI checks
- name: Check File Sizes
  run: |
    large_files=$(find packages/leptos -name "*.rs" -type f -exec wc -l {} + | awk '$1 > 300 {print $2 " has " $1 " lines"}')
    if [ -n "$large_files" ]; then
      echo "Files exceeding 300 line limit:"
      echo "$large_files"
      exit 1
    fi
```

## Specific File Remediation

### select/implementation_tests.rs (891 lines)
**Split into**:
- `select_rendering_tests.rs` (150 lines)
- `select_option_tests.rs` (150 lines)  
- `select_keyboard_tests.rs` (150 lines)
- `select_accessibility_tests.rs` (150 lines)
- `select_performance_tests.rs` (100 lines)
- `select_integration_tests.rs` (150 lines)

### button/tests.rs (844 lines)
**Split into**:
- `button_variants_tests.rs` (200 lines)
- `button_interactions_tests.rs` (200 lines)
- `button_accessibility_tests.rs` (200 lines)
- `button_edge_cases_tests.rs` (200 lines)

### table/data_table.rs (689 lines)
**Architecture refactor**:
- Extract sorting logic → `table_sorting.rs`
- Extract pagination → `table_pagination.rs`  
- Extract row rendering → `table_row_renderer.rs`
- Core table logic → max 200 lines

## Success Criteria
- [ ] No files exceed 300 lines
- [ ] Test files split by logical categories
- [ ] Shared test utilities reduce duplication
- [ ] CI enforces line limits going forward
- [ ] Component architecture follows single responsibility
- [ ] Documentation updated for new structure

## Benefits
1. **Better Test Isolation**: Easier to run specific test categories
2. **Improved LLM Context**: Each file fits in model context windows
3. **Faster Code Reviews**: Reviewers can focus on specific areas
4. **Better Test Parallelization**: Categories can run independently
5. **Easier Debugging**: Smaller surface area per file

## Risk Mitigation
- **Risk**: Breaking existing imports during refactor
- **Mitigation**: Use `pub use` re-exports to maintain compatibility

- **Risk**: Test discovery issues after split
- **Mitigation**: Update Cargo.toml test configurations

- **Risk**: Increased compilation time from more files
- **Mitigation**: Profile build times, optimize if needed

## Dependencies
- Working knowledge of Rust module system
- Test infrastructure already in place
- CI pipeline for enforcement

## Owner  
**Primary**: Senior Rust Engineer familiar with component architecture  
**Secondary**: Test Engineer for test splitting validation
**Reviewer**: Staff Engineer for architectural approval
