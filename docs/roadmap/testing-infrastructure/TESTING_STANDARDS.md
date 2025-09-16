# 🧪 **leptos-shadcn-ui Testing Standards**
**Enterprise-Grade Testing Guidelines for v1.0**

---

## 🎯 **Testing Philosophy**

**"Test-First, Quality-Always, Performance-Minded"**

Every component, feature, and change must be **thoroughly tested** before implementation, maintain **industry-leading quality standards**, and consider **performance implications** from day one.

---

## 📋 **Testing Pyramid Structure**

### **Level 1: Unit Tests (70% of total tests)**
**Purpose**: Validate individual component functionality in isolation

**Requirements**:
- ✅ **Coverage**: 98%+ line coverage per component
- ✅ **Speed**: <100ms per test, <5s total suite
- ✅ **Isolation**: No external dependencies
- ✅ **Deterministic**: Consistent results across environments

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_button_renders_with_default_props() {
        // Arrange
        let props = ButtonProps::default();
        
        // Act
        let component = Button::render(props);
        
        // Assert
        assert!(component.is_ok());
        assert_contains_class(component, "btn-default");
    }

    #[test]
    fn test_button_handles_click_events() {
        // Property-based testing for event handling
        let clicked = Arc::new(Mutex::new(false));
        let clicked_clone = clicked.clone();
        
        let props = ButtonProps {
            onclick: Some(Box::new(move || {
                *clicked_clone.lock().unwrap() = true;
            })),
            ..Default::default()
        };
        
        let component = Button::render(props);
        simulate_click(component);
        
        assert!(*clicked.lock().unwrap());
    }
}
```

### **Level 2: Integration Tests (25% of total tests)**
**Purpose**: Validate component interactions and system behavior

**Requirements**:
- ✅ **Component Compatibility**: Cross-component testing
- ✅ **Event Propagation**: Event handling between components
- ✅ **State Management**: Shared state validation
- ✅ **Theme Consistency**: Visual consistency testing

```rust
#[cfg(test)]
mod integration_tests {
    use leptos::*;
    use leptos_shadcn_form::*;
    use leptos_shadcn_button::*;
    use leptos_shadcn_input::*;

    #[test]
    fn test_form_submission_workflow() {
        // Test complete form workflow
        let form_data = Arc::new(Mutex::new(FormData::default()));
        let form_submitted = Arc::new(Mutex::new(false));
        
        let form_component = Form::render(FormProps {
            onsubmit: Some(create_form_handler(form_data.clone(), form_submitted.clone())),
            children: vec![
                Input::render(InputProps { name: "email".to_string(), ..Default::default() }),
                Button::render(ButtonProps { r#type: "submit".to_string(), ..Default::default() }),
            ],
            ..Default::default()
        });
        
        // Simulate user interaction
        fill_input(form_component, "email", "test@example.com");
        click_submit_button(form_component);
        
        // Validate integration
        assert!(*form_submitted.lock().unwrap());
        assert_eq!(form_data.lock().unwrap().email, "test@example.com");
    }
}
```

### **Level 3: E2E Tests (5% of total tests)**
**Purpose**: Validate complete user workflows and real-world scenarios

**Requirements**:
- ✅ **User Journeys**: Complete workflow validation
- ✅ **Cross-Browser**: Chrome, Firefox, Safari compatibility
- ✅ **Performance**: Real-world performance validation
- ✅ **Accessibility**: Screen reader and keyboard navigation

```typescript
// tests/e2e/form-workflow.spec.ts
import { test, expect } from '@playwright/test';

test.describe('Form Workflow Integration', () => {
  test('complete user registration flow', async ({ page }) => {
    await page.goto('/examples/registration');
    
    // Fill form fields
    await page.fill('[data-testid="email-input"]', 'user@example.com');
    await page.fill('[data-testid="password-input"]', 'SecurePass123!');
    await page.check('[data-testid="terms-checkbox"]');
    
    // Submit form
    await page.click('[data-testid="submit-button"]');
    
    // Validate success
    await expect(page.locator('[data-testid="success-message"]')).toBeVisible();
    
    // Performance validation
    const metrics = await page.evaluate(() => performance.getEntriesByType('measure'));
    expect(metrics.find(m => m.name === 'form-render-time')?.duration).toBeLessThan(16);
  });

  test('accessibility compliance', async ({ page }) => {
    await page.goto('/examples/all-components');
    
    // Keyboard navigation test
    await page.keyboard.press('Tab');
    await expect(page.locator(':focus')).toBeVisible();
    
    // Screen reader test
    const ariaLabels = await page.$$eval('[aria-label]', els => els.map(el => el.getAttribute('aria-label')));
    expect(ariaLabels.length).toBeGreaterThan(0);
  });
});
```

---

## ⚡ **Performance Testing Standards**

### **Performance Benchmarks**
All components must meet these performance thresholds:

```rust
// Performance testing with Criterion
use criterion::{criterion_group, criterion_main, Criterion};

fn component_performance_benchmarks(c: &mut Criterion) {
    // Render performance benchmark
    c.bench_function("button_render", |b| {
        b.iter(|| {
            Button::render(ButtonProps::default())
        })
    });
    
    // Memory usage benchmark
    c.bench_function("button_memory_usage", |b| {
        b.iter_with_setup(
            || ButtonProps::default(),
            |props| {
                let component = Button::render(props);
                std::mem::drop(component); // Ensure cleanup
            }
        )
    });
    
    // Event handling performance
    c.bench_function("button_event_handling", |b| {
        let click_count = Arc::new(Mutex::new(0));
        let props = ButtonProps {
            onclick: Some(create_click_handler(click_count.clone())),
            ..Default::default()
        };
        
        b.iter(|| {
            let component = Button::render(props.clone());
            simulate_click(component);
        })
    });
}

criterion_group!(benches, component_performance_benchmarks);
criterion_main!(benches);
```

### **Performance Thresholds**
| Metric | Threshold | Measurement Method |
|--------|-----------|-------------------|
| **Render Time** | <16ms | Criterion benchmarks |
| **Memory Usage** | <1MB per component | Memory profiling |
| **Bundle Size** | <10KB per component | Webpack bundle analyzer |
| **First Paint** | <1.5s | Lighthouse/E2E tests |
| **Event Response** | <4ms | Performance API |

---

## 🛡️ **Quality Gates**

### **Pre-Commit Gates**
```bash
#!/bin/bash
# .git/hooks/pre-commit

echo "🧪 Running pre-commit quality gates..."

# Format check
cargo fmt -- --check || exit 1

# Lint check
cargo clippy --all-targets --all-features -- -D warnings || exit 1

# Quick unit tests
cargo test --lib --all-features --quiet || exit 1

# Security audit
cargo audit || exit 1

echo "✅ Pre-commit gates passed!"
```

### **CI/CD Quality Gates**
```yaml
# Quality gate thresholds
quality_gates:
  unit_test_coverage: 98%
  integration_test_coverage: 95%
  e2e_test_coverage: 90%
  performance_regression: 0%
  security_vulnerabilities: 0
  accessibility_score: 95%
  bundle_size_increase: 5%
```

### **Release Gates**
- [ ] All tests passing (unit, integration, E2E)
- [ ] Performance benchmarks within thresholds
- [ ] Security audit clean
- [ ] Accessibility compliance verified
- [ ] Documentation updated
- [ ] Migration guide provided (if breaking changes)

---

## 📊 **Test Coverage Requirements**

### **Component Coverage Matrix**
| Component Type | Unit Tests | Integration Tests | E2E Tests | Performance Tests |
|----------------|------------|-------------------|-----------|-------------------|
| **Form Components** | 98% | 95% | 90% | Required |
| **Layout Components** | 98% | 90% | 80% | Required |
| **Navigation Components** | 98% | 95% | 95% | Required |
| **Overlay Components** | 98% | 90% | 85% | Recommended |
| **Data Display** | 98% | 85% | 75% | Recommended |
| **Interactive Components** | 98% | 95% | 90% | Required |
| **Utility Components** | 98% | 80% | 60% | Optional |

### **Test Categories**
Each component must include tests for:

**✅ Functional Testing**
- [ ] Default rendering
- [ ] Props validation
- [ ] Event handling
- [ ] State management
- [ ] Error conditions

**✅ Visual Testing**
- [ ] CSS class application
- [ ] Theme variations
- [ ] Responsive behavior
- [ ] Animation states

**✅ Accessibility Testing**
- [ ] ARIA attributes
- [ ] Keyboard navigation
- [ ] Screen reader compatibility
- [ ] Focus management

**✅ Performance Testing**
- [ ] Render time benchmarks
- [ ] Memory usage validation
- [ ] Event handling performance
- [ ] Bundle size optimization

---

## 🔧 **Testing Tools & Framework**

### **Core Testing Stack**
```toml
[dev-dependencies]
# Unit testing
leptos = { version = "0.8", features = ["testing"] }
wasm-bindgen-test = "0.3"

# Property-based testing
proptest = "1.0"
quickcheck = "1.0"

# Performance testing
criterion = "0.5"

# Mocking and stubbing
mockall = "0.12"

# Test utilities
rstest = "0.18"
serial_test = "3.0"

# E2E testing (Node.js)
@playwright/test = "^1.40.0"
@axe-core/playwright = "^4.8.0"
```

### **Custom Test Utilities**
```rust
// packages/test-utils/src/lib.rs
pub mod component_testing {
    use leptos::*;
    
    pub fn create_test_context() -> TestContext {
        // Setup test environment with proper context
    }
    
    pub fn simulate_user_interaction(component: Component, interaction: UserInteraction) {
        // Simulate real user interactions for testing
    }
    
    pub fn assert_accessibility_compliance(component: Component) -> Result<(), AccessibilityError> {
        // Validate WCAG compliance
    }
    
    pub fn measure_performance<F>(test_fn: F) -> PerformanceMetrics 
    where 
        F: FnOnce() -> ComponentResult 
    {
        // Measure component performance
    }
}
```

---

## 📈 **Continuous Quality Monitoring**

### **Quality Metrics Dashboard**
Track these key metrics continuously:

```yaml
quality_metrics:
  testing:
    unit_test_coverage: 98%
    integration_test_coverage: 95%
    e2e_test_coverage: 90%
    test_execution_time: <5min
    
  performance:
    avg_render_time: <10ms
    p99_render_time: <16ms
    memory_usage: <1MB
    bundle_size: <500KB
    
  quality:
    code_duplication: <3%
    complexity_score: <10
    technical_debt_ratio: <5%
    security_vulnerabilities: 0
    
  reliability:
    test_flakiness: <1%
    build_success_rate: >99%
    deployment_success_rate: >99%
```

### **Quality Alerts**
Set up automated alerts for:
- Test coverage drops below 98%
- Performance regression >5%
- New security vulnerabilities
- Build failures
- Accessibility compliance issues

---

## 🚀 **Testing Best Practices**

### **Test Writing Guidelines**

**✅ DO:**
- Write tests before implementing features (TDD)
- Use descriptive test names that explain the scenario
- Follow the AAA pattern (Arrange, Act, Assert)
- Test both happy paths and edge cases
- Use property-based testing for complex logic
- Mock external dependencies
- Validate both behavior and performance

**❌ DON'T:**
- Test implementation details
- Write flaky or non-deterministic tests
- Skip error condition testing
- Ignore performance implications
- Use real external services in tests
- Write overly complex test setups

### **Test Organization**
```
packages/leptos/button/src/
├── lib.rs
├── tests.rs           # Unit tests
├── integration_tests/ # Integration tests
│   ├── form_integration.rs
│   └── theme_compatibility.rs
└── benches/          # Performance benchmarks
    └── button_benchmarks.rs
```

### **Test Naming Convention**
```rust
#[test]
fn test_<component>_<scenario>_<expected_outcome>() {
    // test_button_with_disabled_prop_prevents_click_events()
    // test_input_with_invalid_value_shows_error_message()
    // test_dialog_on_escape_key_closes_modal()
}
```

---

**This testing standard ensures leptos-shadcn-ui maintains enterprise-grade quality while providing an exceptional developer experience. Every test adds value, every benchmark drives optimization, and every quality gate prevents regressions.**

---

*Last Updated: December 2024*  
*Status: 🚀 Active Implementation*  
*Next Review: Q1 2025*