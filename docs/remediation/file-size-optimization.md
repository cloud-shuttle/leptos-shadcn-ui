# 📏 File Size Optimization Plan

**Priority**: 🟡 **HIGH**  
**Timeline**: Weeks 2-3  
**Impact**: Improves maintainability, testability, and LLM comprehension

## 🚨 Files Exceeding 300 Lines

### **Critical Files (500+ lines)**
| File | Lines | Priority | Refactoring Strategy |
|------|-------|----------|---------------------|
| `packages/leptos/input/src/implementation_tests.rs` | 867 | 🔴 Critical | Split into test modules |
| `packages/leptos/form/src/implementation_tests.rs` | 783 | 🔴 Critical | Split into test modules |
| `packages/signal-management/src/signal_management_tests.rs` | 766 | 🔴 Critical | Split into test modules |
| `packages/signal-management/src/simple_tests.rs` | 753 | 🔴 Critical | Split into test modules |
| `packages/leptos/input/src/tdd_tests.rs` | 663 | 🔴 Critical | Split into test modules |
| `packages/leptos/command/src/tdd_tests.rs` | 607 | 🔴 Critical | Split into test modules |
| `packages/signal-management/src/lifecycle_tests.rs` | 648 | 🔴 Critical | Split into test modules |
| `packages/signal-management/src/memory_management_tests.rs` | 554 | 🔴 Critical | Split into test modules |
| `packages/signal-management/src/component_migration.rs` | 541 | 🔴 Critical | Split into modules |
| `packages/leptos/button/src/tdd_tests.rs` | 560 | 🔴 Critical | Split into test modules |
| `packages/signal-management/src/component_migration_tests.rs` | 541 | 🔴 Critical | Split into test modules |

### **High Priority Files (400-500 lines)**
| File | Lines | Priority | Refactoring Strategy |
|------|-------|----------|---------------------|
| `packages/signal-management/src/batched_updates_tests.rs` | 456 | 🟡 High | Split into test modules |
| `packages/leptos/button/src/implementation_tests.rs` | 530 | 🟡 High | Split into test modules |
| `performance-audit/src/benchmarks.rs` | 802 | 🟡 High | Split into benchmark modules |
| `performance-audit/src/memory_safety.rs` | 659 | 🟡 High | Split into modules |
| `performance-audit/src/optimization_roadmap.rs` | 642 | 🟡 High | Split into modules |

### **Medium Priority Files (300-400 lines)**
| File | Lines | Priority | Refactoring Strategy |
|------|-------|----------|---------------------|
| `packages/signal-management/src/memory_management.rs` | 348 | 🟢 Medium | Extract helper modules |
| `packages/signal-management/src/advanced_memory.rs` | 266 | 🟢 Medium | Extract helper modules |
| `packages/leptos/command/src/default.rs` | 298 | 🟢 Medium | Extract helper modules |
| `packages/leptos/command/src/new_york.rs` | 293 | 🟢 Medium | Extract helper modules |

## 🎯 Refactoring Strategy

### **Phase 1: Test File Refactoring (Week 2)**

#### **1.1 TDD Tests Refactoring**
```rust
// Current: packages/leptos/command/src/tdd_tests.rs (607 lines)
// Split into:
├── tdd_tests/
│   ├── mod.rs                    // Module declarations
│   ├── basic_functionality.rs    // Basic component tests (~100 lines)
│   ├── accessibility_tests.rs    // Accessibility tests (~100 lines)
│   ├── performance_tests.rs      // Performance tests (~100 lines)
│   ├── integration_tests.rs      // Integration tests (~100 lines)
│   ├── edge_case_tests.rs        // Edge case tests (~100 lines)
│   └── error_handling_tests.rs   // Error handling tests (~100 lines)
```

#### **1.2 Implementation Tests Refactoring**
```rust
// Current: packages/leptos/input/src/implementation_tests.rs (867 lines)
// Split into:
├── implementation_tests/
│   ├── mod.rs                    // Module declarations
│   ├── prop_handling_tests.rs    // Prop handling tests (~150 lines)
│   ├── signal_management_tests.rs // Signal management tests (~150 lines)
│   ├── event_handling_tests.rs   // Event handling tests (~150 lines)
│   ├── validation_tests.rs       // Validation tests (~150 lines)
│   ├── styling_tests.rs          // Styling tests (~150 lines)
│   └── integration_tests.rs      // Integration tests (~150 lines)
```

#### **1.3 Signal Management Tests Refactoring**
```rust
// Current: packages/signal-management/src/signal_management_tests.rs (766 lines)
// Split into:
├── signal_management_tests/
│   ├── mod.rs                    // Module declarations
│   ├── basic_signal_tests.rs     // Basic signal tests (~150 lines)
│   ├── memory_management_tests.rs // Memory management tests (~150 lines)
│   ├── lifecycle_tests.rs        // Lifecycle tests (~150 lines)
│   ├── performance_tests.rs      // Performance tests (~150 lines)
│   └── integration_tests.rs      // Integration tests (~150 lines)
```

### **Phase 2: Implementation File Refactoring (Week 3)**

#### **2.1 Component Migration Refactoring**
```rust
// Current: packages/signal-management/src/component_migration.rs (541 lines)
// Split into:
├── component_migration/
│   ├── mod.rs                    // Module declarations
│   ├── migration_strategies.rs   // Migration strategies (~150 lines)
│   ├── compatibility_checker.rs  // Compatibility checker (~150 lines)
│   ├── migration_executor.rs     // Migration executor (~150 lines)
│   └── migration_validator.rs    // Migration validator (~150 lines)
```

#### **2.2 Performance Audit Refactoring**
```rust
// Current: performance-audit/src/benchmarks.rs (802 lines)
// Split into:
├── benchmarks/
│   ├── mod.rs                    // Module declarations
│   ├── component_benchmarks.rs   // Component benchmarks (~200 lines)
│   ├── memory_benchmarks.rs      // Memory benchmarks (~200 lines)
│   ├── render_benchmarks.rs      // Render benchmarks (~200 lines)
│   └── integration_benchmarks.rs // Integration benchmarks (~200 lines)
```

## 📋 Implementation Steps

### **Week 2: Test File Refactoring**

#### **Day 1-2: Command Component Tests**
```bash
# Create new directory structure:
mkdir -p packages/leptos/command/src/tdd_tests

# Split tdd_tests.rs into modules:
# - basic_functionality.rs
# - accessibility_tests.rs
# - performance_tests.rs
# - integration_tests.rs
# - edge_case_tests.rs
# - error_handling_tests.rs
```

#### **Day 3-4: Input Component Tests**
```bash
# Create new directory structure:
mkdir -p packages/leptos/input/src/implementation_tests

# Split implementation_tests.rs into modules:
# - prop_handling_tests.rs
# - signal_management_tests.rs
# - event_handling_tests.rs
# - validation_tests.rs
# - styling_tests.rs
# - integration_tests.rs
```

#### **Day 5: Form Component Tests**
```bash
# Create new directory structure:
mkdir -p packages/leptos/form/src/implementation_tests

# Split implementation_tests.rs into modules
```

### **Week 3: Implementation File Refactoring**

#### **Day 6-7: Signal Management Refactoring**
```bash
# Create new directory structure:
mkdir -p packages/signal-management/src/component_migration

# Split component_migration.rs into modules
```

#### **Day 8-9: Performance Audit Refactoring**
```bash
# Create new directory structure:
mkdir -p performance-audit/src/benchmarks

# Split benchmarks.rs into modules
```

#### **Day 10: Integration and Testing**
```bash
# Test all refactored modules
# Verify compilation
# Run all tests
# Update documentation
```

## 🧪 Testing Strategy

### **Refactoring Validation**
```rust
// Each split module should have:
#[cfg(test)]
mod tests {
    use super::*;
    
    // Module-specific tests
    // Integration with other modules
    // Error handling tests
}
```

### **Compilation Testing**
```bash
# Test each refactored module:
cargo check --package leptos-shadcn-command
cargo check --package leptos-shadcn-input
cargo check --package leptos-shadcn-form
cargo check --package leptos-shadcn-signal-management
cargo check --package leptos-shadcn-performance-audit
```

### **Test Execution**
```bash
# Run all tests for refactored modules:
cargo test --package leptos-shadcn-command
cargo test --package leptos-shadcn-input
cargo test --package leptos-shadcn-form
cargo test --package leptos-shadcn-signal-management
cargo test --package leptos-shadcn-performance-audit
```

## 📊 Success Criteria

- ✅ **All files under 300 lines** of code
- ✅ **Logical module separation** by functionality
- ✅ **Maintained test coverage** after refactoring
- ✅ **Clean compilation** for all refactored modules
- ✅ **Improved maintainability** and readability

## 🚨 Risk Mitigation

### **Refactoring Risks**
- **Test Coverage Loss**: Ensure all tests are preserved during refactoring
- **Compilation Errors**: Test compilation after each module split
- **Functionality Regression**: Run comprehensive tests after refactoring

### **Quality Assurance**
- **Code Review**: Review each refactored module
- **Documentation**: Update module documentation
- **Examples**: Ensure examples still work

### **Rollback Strategy**
- **Git Branches**: Create feature branch for refactoring
- **Incremental Commits**: Commit after each successful refactoring
- **Backup**: Keep original files until refactoring is complete

## 📁 Example Refactoring

### **Before: Large Test File**
```rust
// packages/leptos/command/src/tdd_tests.rs (607 lines)
#[cfg(test)]
mod tdd_tests {
    use super::*;
    
    // 50+ test functions mixed together
    // Different test categories in one file
    // Hard to navigate and maintain
}
```

### **After: Modular Test Structure**
```rust
// packages/leptos/command/src/tdd_tests/mod.rs
pub mod basic_functionality;
pub mod accessibility_tests;
pub mod performance_tests;
pub mod integration_tests;
pub mod edge_case_tests;
pub mod error_handling_tests;

// packages/leptos/command/src/tdd_tests/basic_functionality.rs (~100 lines)
#[cfg(test)]
mod tests {
    use super::*;
    
    // 8-10 focused test functions
    // Clear test category
    // Easy to navigate and maintain
}
```

## 📈 Benefits

### **For Developers**
- **Easier Navigation**: Find specific functionality quickly
- **Better Testing**: Focused test modules
- **Improved Maintainability**: Smaller, focused files

### **For LLMs**
- **Better Comprehension**: Smaller context windows
- **Focused Analysis**: Specific functionality per file
- **Improved Code Generation**: More targeted suggestions

### **For CI/CD**
- **Faster Compilation**: Smaller files compile faster
- **Parallel Testing**: Test modules can run in parallel
- **Better Error Reporting**: More specific error locations

---

**Next Steps**: Start with the most critical files (500+ lines) and work systematically through the refactoring plan. Focus on test files first as they have the highest impact on maintainability.
