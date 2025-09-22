# 🔧 Code Refactoring Plan
**Priority 2: File Size & Code Organization**

## 📏 File Size Analysis

### **Files Exceeding 300-Line Limit**

| File | Lines | Issue | Refactoring Strategy |
|------|-------|-------|---------------------|
| `input/tdd_tests/basic_rendering_tests.rs` | 224 | Test file too large | Split into 3 test modules |
| `input/tdd_tests/validation_tests.rs` | 180+ | Test file too large | Split by test category |
| `button/default.rs` | 143 | Component implementation | Extract prop builders |
| `contract-testing/src/lib.rs` | 160 | Framework code | Split into sub-modules |
| `input/tdd_tests/accessibility_tests.rs` | 150+ | Test file too large | Split by accessibility feature |
| `input/tdd_tests/performance_tests.rs` | 140+ | Test file too large | Split by performance metric |

### **Target File Structure**
- **Maximum 300 lines** per file
- **Optimal 100-200 lines** for maintainability
- **Test files maximum 100 lines** for clarity
- **Component files maximum 200 lines** for readability

---

## 🏗️ Refactoring Strategy

### **Phase 1: Test File Refactoring (Week 1)**

#### **Input Component Test Refactoring**

**Current Structure:**
```
packages/leptos/input/src/tdd_tests/
├── basic_rendering_tests.rs (224 lines) ❌
├── validation_tests.rs (180+ lines) ❌
├── accessibility_tests.rs (150+ lines) ❌
├── performance_tests.rs (140+ lines) ❌
└── integration_tests.rs (120+ lines) ❌
```

**Target Structure:**
```
packages/leptos/input/src/tests/
├── basic_rendering/
│   ├── mod.rs (20 lines)
│   ├── simple_rendering.rs (80 lines)
│   ├── prop_handling.rs (90 lines)
│   └── state_management.rs (85 lines)
├── validation/
│   ├── mod.rs (20 lines)
│   ├── input_validation.rs (95 lines)
│   ├── error_handling.rs (90 lines)
│   └── validation_messages.rs (85 lines)
├── accessibility/
│   ├── mod.rs (20 lines)
│   ├── aria_attributes.rs (90 lines)
│   ├── keyboard_navigation.rs (95 lines)
│   └── screen_reader.rs (85 lines)
├── performance/
│   ├── mod.rs (20 lines)
│   ├── render_performance.rs (90 lines)
│   ├── memory_usage.rs (85 lines)
│   └── bundle_size.rs (80 lines)
└── integration/
    ├── mod.rs (20 lines)
    ├── form_integration.rs (95 lines)
    ├── state_integration.rs (90 lines)
    └── component_interaction.rs (85 lines)
```

#### **Refactoring Implementation**

**1. Basic Rendering Tests Split**
```rust
// packages/leptos/input/src/tests/basic_rendering/simple_rendering.rs
#[cfg(test)]
mod simple_rendering {
    use crate::default::Input;
    use leptos::prelude::*;

    #[test]
    fn test_input_renders_without_errors() {
        let input = Input::new(InputProps::default());
        let rendered = input.render();
        assert!(rendered.contains("input"));
    }

    #[test]
    fn test_input_with_placeholder() {
        let props = InputProps {
            placeholder: "Enter text".to_string(),
            ..Default::default()
        };
        let input = Input::new(props);
        let rendered = input.render();
        assert!(rendered.contains("Enter text"));
    }

    // ... more tests (max 80 lines)
}
```

**2. Validation Tests Split**
```rust
// packages/leptos/input/src/tests/validation/input_validation.rs
#[cfg(test)]
mod input_validation {
    use crate::default::Input;
    use crate::validation::ValidationRule;

    #[test]
    fn test_required_validation() {
        let rule = ValidationRule::Required;
        let props = InputProps {
            validation_rules: vec![rule],
            ..Default::default()
        };
        let input = Input::new(props);
        // ... validation tests (max 95 lines)
    }

    // ... more validation tests
}
```

### **Phase 2: Component Implementation Refactoring (Week 2)**

#### **Button Component Refactoring**

**Current Structure:**
```
packages/leptos/button/src/
├── default.rs (143 lines) ❌
├── new_york.rs (120+ lines) ❌
└── signal_managed.rs (100+ lines) ✅
```

**Target Structure:**
```
packages/leptos/button/src/
├── default/
│   ├── mod.rs (20 lines)
│   ├── component.rs (80 lines)
│   ├── props.rs (60 lines)
│   ├── variants.rs (70 lines)
│   └── styles.rs (50 lines)
├── new_york/
│   ├── mod.rs (20 lines)
│   ├── component.rs (80 lines)
│   ├── props.rs (60 lines)
│   └── styles.rs (50 lines)
└── signal_managed/
    ├── mod.rs (20 lines)
    ├── component.rs (80 lines)
    ├── state.rs (60 lines)
    └── handlers.rs (50 lines)
```

#### **Component Refactoring Implementation**

**1. Extract Props Module**
```rust
// packages/leptos/button/src/default/props.rs
use leptos::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ButtonProps {
    pub variant: MaybeProp<ButtonVariant>,
    pub size: MaybeProp<ButtonSize>,
    pub disabled: MaybeProp<bool>,
    pub class: MaybeProp<String>,
    pub children: Children,
    pub on_click: Option<Callback<MouseEvent>>,
}

impl Default for ButtonProps {
    fn default() -> Self {
        Self {
            variant: MaybeProp::Static(ButtonVariant::Default),
            size: MaybeProp::Static(ButtonSize::Default),
            disabled: MaybeProp::Static(false),
            class: MaybeProp::Static(String::new()),
            children: Children::new(),
            on_click: None,
        }
    }
}
```

**2. Extract Variants Module**
```rust
// packages/leptos/button/src/default/variants.rs
use leptos::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ButtonVariant {
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

impl ButtonVariant {
    pub fn css_class(&self) -> &'static str {
        match self {
            ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
            ButtonVariant::Destructive => "bg-destructive text-destructive-foreground hover:bg-destructive/90",
            ButtonVariant::Outline => "border border-input bg-background hover:bg-accent hover:text-accent-foreground",
            ButtonVariant::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
            ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
            ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
        }
    }
}
```

**3. Extract Styles Module**
```rust
// packages/leptos/button/src/default/styles.rs
use leptos::prelude::*;

pub const BUTTON_BASE_CLASS: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";

pub fn get_button_class(variant: &ButtonVariant, size: &ButtonSize, custom_class: &str) -> String {
    let mut classes = vec![BUTTON_BASE_CLASS.to_string()];
    
    classes.push(variant.css_class().to_string());
    classes.push(size.css_class().to_string());
    
    if !custom_class.is_empty() {
        classes.push(custom_class.to_string());
    }
    
    classes.join(" ")
}
```

### **Phase 3: Framework Code Refactoring (Week 3)**

#### **Contract Testing Framework Refactoring**

**Current Structure:**
```
packages/contract-testing/src/
├── lib.rs (160 lines) ❌
├── dependency_contracts.rs (100+ lines) ❌
├── dependency_fixer.rs (120+ lines) ❌
└── wasm_performance.rs (80+ lines) ✅
```

**Target Structure:**
```
packages/contract-testing/src/
├── lib.rs (50 lines)
├── core/
│   ├── mod.rs (20 lines)
│   ├── contract_trait.rs (80 lines)
│   ├── tester.rs (90 lines)
│   └── error_types.rs (60 lines)
├── validation/
│   ├── mod.rs (20 lines)
│   ├── props_validator.rs (85 lines)
│   ├── accessibility_validator.rs (90 lines)
│   └── performance_validator.rs (85 lines)
├── testing/
│   ├── mod.rs (20 lines)
│   ├── test_runner.rs (90 lines)
│   ├── compatibility_matrix.rs (85 lines)
│   └── reporting.rs (80 lines)
└── utils/
    ├── mod.rs (20 lines)
    ├── dependency_contracts.rs (80 lines)
    ├── dependency_fixer.rs (90 lines)
    └── wasm_performance.rs (80 lines)
```

---

## 🛠️ Implementation Plan

### **Week 1: Test File Refactoring**
- [ ] Refactor Input component test files
- [ ] Create test module structure
- [ ] Implement test utilities
- [ ] Update test documentation

### **Week 2: Component Refactoring**
- [ ] Refactor Button component
- [ ] Refactor Input component
- [ ] Refactor Card component
- [ ] Create component templates

### **Week 3: Framework Refactoring**
- [ ] Refactor contract testing framework
- [ ] Split large framework files
- [ ] Create framework utilities
- [ ] Update framework documentation

### **Week 4: Validation & Documentation**
- [ ] Validate all files under 300 lines
- [ ] Update documentation
- [ ] Create refactoring guidelines
- [ ] Train team on new structure

---

## 📈 Success Metrics

### **File Size Targets**
- **All files under 300 lines** (100% compliance)
- **Test files under 100 lines** (100% compliance)
- **Component files under 200 lines** (100% compliance)
- **Framework files under 150 lines** (100% compliance)

### **Code Quality Metrics**
- **Improved readability** (measured by team feedback)
- **Better maintainability** (measured by bug fix time)
- **Easier testing** (measured by test writing time)
- **LLM-friendly structure** (measured by AI assistance effectiveness)

### **Organization Metrics**
- **Logical file grouping** by functionality
- **Consistent naming conventions** across all files
- **Clear module boundaries** with minimal coupling
- **Comprehensive documentation** for each module

---

## 🔧 Refactoring Tools & Guidelines

### **File Size Monitoring**
```bash
# Check file sizes
find . -name "*.rs" -exec wc -l {} + | sort -nr | head -20

# Validate file size limits
find . -name "*.rs" -exec sh -c 'lines=$(wc -l < "$1"); if [ $lines -gt 300 ]; then echo "$1: $lines lines"; fi' _ {} \;
```

### **Refactoring Guidelines**
1. **Single Responsibility**: Each file should have one clear purpose
2. **Logical Grouping**: Related functionality should be in the same module
3. **Clear Interfaces**: Module boundaries should be well-defined
4. **Consistent Patterns**: Use the same structure across similar files

### **Code Templates**
- **Test file template** (max 100 lines)
- **Component file template** (max 200 lines)
- **Framework file template** (max 150 lines)
- **Utility file template** (max 100 lines)

---

## 🚀 Next Steps

1. **Start with Input component tests** (highest impact)
2. **Create refactoring templates** for consistency
3. **Set up file size monitoring** in CI/CD
4. **Document refactoring patterns** for team
5. **Train team on new structure** and guidelines

This refactoring plan will improve code maintainability, readability, and LLM compatibility while ensuring all files meet the 300-line limit requirement.

---

*Plan created: September 20, 2025*  
*Target completion: October 18, 2025*
