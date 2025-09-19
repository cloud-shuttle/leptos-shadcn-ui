# 📏 File Size Quick Reference

**Target**: All files under 300 lines  
**Priority**: High-impact files first  
**Timeline**: 2-3 weeks

## 🚨 Critical Files (500+ lines) - Fix First

| File | Lines | Split Into | Priority |
|------|-------|------------|----------|
| `packages/leptos/input/src/implementation_tests.rs` | 867 | 6 modules | 🔴 Critical |
| `packages/leptos/form/src/implementation_tests.rs` | 783 | 6 modules | 🔴 Critical |
| `packages/signal-management/src/signal_management_tests.rs` | 766 | 6 modules | 🔴 Critical |
| `packages/signal-management/src/simple_tests.rs` | 753 | 6 modules | 🔴 Critical |
| `packages/leptos/input/src/tdd_tests.rs` | 663 | 6 modules | 🔴 Critical |
| `packages/leptos/command/src/tdd_tests.rs` | 607 | 6 modules | 🔴 Critical |
| `packages/signal-management/src/lifecycle_tests.rs` | 648 | 6 modules | 🔴 Critical |
| `packages/signal-management/src/memory_management_tests.rs` | 554 | 6 modules | 🔴 Critical |
| `packages/signal-management/src/component_migration.rs` | 541 | 4 modules | 🔴 Critical |
| `packages/leptos/button/src/tdd_tests.rs` | 560 | 6 modules | 🔴 Critical |

## 🟡 High Priority Files (400-500 lines)

| File | Lines | Split Into | Priority |
|------|-------|------------|----------|
| `packages/signal-management/src/batched_updates_tests.rs` | 456 | 4 modules | 🟡 High |
| `packages/leptos/button/src/implementation_tests.rs` | 530 | 4 modules | 🟡 High |
| `performance-audit/src/benchmarks.rs` | 802 | 4 modules | 🟡 High |
| `performance-audit/src/memory_safety.rs` | 659 | 4 modules | 🟡 High |
| `performance-audit/src/optimization_roadmap.rs` | 642 | 4 modules | 🟡 High |

## 🟢 Medium Priority Files (300-400 lines)

| File | Lines | Split Into | Priority |
|------|-------|------------|----------|
| `packages/signal-management/src/memory_management.rs` | 348 | 3 modules | 🟢 Medium |
| `packages/signal-management/src/advanced_memory.rs` | 266 | 2 modules | 🟢 Medium |
| `packages/leptos/command/src/default.rs` | 298 | 2 modules | 🟢 Medium |
| `packages/leptos/command/src/new_york.rs` | 293 | 2 modules | 🟢 Medium |

## 🎯 Refactoring Patterns

### **Test Files Pattern**
```
original_tests.rs (600+ lines)
├── mod.rs                    // Module declarations
├── basic_functionality.rs    // Basic tests (~100 lines)
├── accessibility_tests.rs    // Accessibility tests (~100 lines)
├── performance_tests.rs      // Performance tests (~100 lines)
├── integration_tests.rs      // Integration tests (~100 lines)
├── edge_case_tests.rs        // Edge case tests (~100 lines)
└── error_handling_tests.rs   // Error handling tests (~100 lines)
```

### **Implementation Files Pattern**
```
original_implementation.rs (500+ lines)
├── mod.rs                    // Module declarations
├── core_functionality.rs     // Core functionality (~150 lines)
├── helper_functions.rs       // Helper functions (~150 lines)
├── integration_layer.rs      // Integration layer (~150 lines)
└── error_handling.rs         // Error handling (~150 lines)
```

### **Performance Files Pattern**
```
original_performance.rs (600+ lines)
├── mod.rs                    // Module declarations
├── component_benchmarks.rs   // Component benchmarks (~200 lines)
├── memory_benchmarks.rs      // Memory benchmarks (~200 lines)
├── render_benchmarks.rs      // Render benchmarks (~200 lines)
└── integration_benchmarks.rs // Integration benchmarks (~200 lines)
```

## 📋 Quick Implementation Steps

### **1. Create Directory Structure**
```bash
mkdir -p path/to/component/src/module_name
```

### **2. Create Module Files**
```bash
touch path/to/component/src/module_name/mod.rs
touch path/to/component/src/module_name/part1.rs
touch path/to/component/src/module_name/part2.rs
touch path/to/component/src/module_name/part3.rs
```

### **3. Update Module Declaration**
```rust
// mod.rs
pub mod part1;
pub mod part2;
pub mod part3;
```

### **4. Update Parent Module**
```rust
// lib.rs or parent module
mod module_name;
```

### **5. Test and Validate**
```bash
cargo check --package package-name
cargo test --package package-name
```

## 🧪 Testing Checklist

- [ ] All modules compile successfully
- [ ] All tests pass after refactoring
- [ ] No test coverage is lost
- [ ] Module boundaries are logical
- [ ] Documentation is updated
- [ ] Examples still work

## 📊 Success Metrics

- ✅ **All files under 300 lines**
- ✅ **Logical module separation**
- ✅ **Maintained test coverage**
- ✅ **Clean compilation**
- ✅ **Improved maintainability**

## 🚨 Common Pitfalls

1. **Don't split too aggressively** - Keep related functionality together
2. **Don't lose test coverage** - Ensure all tests are preserved
3. **Don't break module boundaries** - Maintain clear separation of concerns
4. **Don't forget documentation** - Update module documentation
5. **Don't skip testing** - Test after each refactoring step

## 📁 Related Documents

- [File Size Optimization Plan](./file-size-optimization.md) - Detailed refactoring strategy
- [Input Tests Refactoring](./component-fixes/input-tests-refactoring.md) - Example implementation
- [Build System Remediation](./build-system-remediation.md) - Fix compilation issues first

---

**Remember**: Start with the largest files (500+ lines) and work systematically. Test after each refactoring step to ensure nothing breaks.
