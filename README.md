# 🚀 **leptos-shadcn-ui**

**Production-ready ShadCN UI components for Leptos v0.8+ applications**

[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://rust-lang.org)
[![Leptos](https://img.shields.io/badge/leptos-0.8+-green.svg)](https://leptos.dev)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-300%2B%20passing-brightgreen.svg)](docs/tdd/completion/TDD_COMPLETION_SUMMARY.md)
[![E2E Tests](https://img.shields.io/badge/e2e%20tests-129%20passing-brightgreen.svg)](tests/e2e)

## 🏆 **Project Status: 100% TDD Implementation Complete!**

**All 46 components are thoroughly tested and production-ready!**

- ✅ **Unit Tests**: 300+ comprehensive tests (100% coverage)
- ✅ **E2E Tests**: 129 Playwright tests covering all workflows  
- ✅ **Quality Standards**: Industry-best practices implemented
- ✅ **Documentation**: Comprehensive guides and examples
- ✅ **Performance Audit**: Complete TDD performance monitoring system

---

## 🎯 **What This Is**

**leptos-shadcn-ui** is a comprehensive component library that brings the beautiful, accessible, and customizable ShadCN UI components to the Leptos ecosystem. Built with Rust and WebAssembly, it provides:

- **46 Production-Ready Components** - All thoroughly tested and validated
- **100% Test Coverage** - Comprehensive unit and integration testing
- **Accessibility First** - WCAG compliance built into every component
- **Performance Optimized** - No memory leaks or performance bottlenecks
- **Cross-Platform** - Works consistently across all major browsers and devices
- **Performance Monitoring** - Built-in performance audit and optimization tools

## 🚀 **Quick Start**

### **Installation**

Add components to your `Cargo.toml`:

```toml
[dependencies]
leptos-shadcn-button = "0.2.0"
leptos-shadcn-input = "0.2.0"
leptos-shadcn-card = "0.2.0"
leptos-shadcn-checkbox = "0.2.0"
# ... add more components as needed
```

### **Basic Usage**

```rust
use leptos::*;
use leptos_shadcn_button::Button;
use leptos_shadcn_input::Input;

#[component]
pub fn MyForm() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <Input placeholder="Enter your name" />
            <Button>"Submit"</Button>
        </div>
    }
}
```

### **Performance Monitoring**

Monitor and optimize your component performance with the built-in audit system:

```bash
# Install the performance audit tool
cargo install leptos-shadcn-performance-audit

# Run comprehensive performance audit
performance-audit audit

# Analyze bundle sizes
performance-audit bundle --components-path packages/leptos

# Monitor real-time performance
performance-audit monitor --duration 30s
```

### **Run Examples**

```bash
# Clone the repository
git clone https://github.com/cloud-shuttle/leptos-shadcn-ui.git
cd leptos-shadcn-ui

# Run the example app
cd examples/leptos
trunk serve
```

---

## 🧪 **Testing & Quality**

### **Run Unit Tests**

```bash
# Test individual components
cargo test --package leptos-shadcn-button --lib
cargo test --package leptos-shadcn-input --lib

# Test all components
cargo test --workspace
```

### **Run E2E Tests**

```bash
# Install Playwright
make install-playwright

# Run all E2E tests
make test-e2e

# Run specific test categories
make test-e2e-specific FILE=tests/e2e/accessibility.spec.ts
```

### **Quality Metrics**

- **Components**: 46/46 (100% tested)
- **Unit Tests**: 300+ tests passing
- **E2E Tests**: 129 tests passing
- **Test Coverage**: 100% for all components
- **Quality Standards**: Production-ready

---

## 📚 **Documentation**

### **📁 Organized Documentation Structure**

- **[📚 Documentation Index](docs/README.md)** - Complete documentation overview
- **[🧪 TDD Implementation](docs/tdd/)** - Complete Test-Driven Development docs
- **[🏗️ Architecture](docs/architecture/)** - System design and migration guides
- **[🔧 Development](docs/development/)** - Tools and component generation
- **[📦 Releases](docs/releases/)** - Release notes and changelog
- **[🎨 Components](docs/components/)** - Usage examples and guides

### **Key Documentation**

- **[TDD Completion Summary](docs/tdd/completion/TDD_COMPLETION_SUMMARY.md)** - Our achievement story
- **[Testing Guide](docs/testing/TESTING_GUIDE.md)** - How to test components
- **[Component Examples](docs/components/example-usage.md)** - Usage patterns
- **[Release Notes](docs/releases/RELEASE_NOTES.md)** - What's new

---

## 🎨 **Available Components**

### **Form Components**
- **Button** - Variants, sizes, and accessibility
- **Input** - Text, email, password with validation
- **Checkbox** - State management and accessibility
- **Label** - Form associations and styling
- **Form** - Complete form handling
- **Textarea** - Multi-line input
- **Select** - Dropdown selection
- **Switch** - Toggle controls
- **Radio Group** - Radio button groups
- **Input OTP** - One-time password input

### **Layout Components**
- **Card** - Content containers
- **Separator** - Visual dividers
- **Accordion** - Collapsible content
- **Collapsible** - Content hiding/showing
- **Tabs** - Tabbed interfaces
- **Table** - Data presentation
- **Aspect Ratio** - Responsive containers
- **Scroll Area** - Scrollable content

### **Navigation Components**
- **Navigation Menu** - Main navigation
- **Menubar** - Application menus
- **Context Menu** - Right-click menus
- **Breadcrumb** - Navigation paths
- **Pagination** - Page navigation

### **Overlay Components**
- **Dialog** - Modal dialogs
- **Popover** - Floating content
- **Sheet** - Side panels
- **Drawer** - Drawer panels
- **Tooltip** - Hover information
- **Hover Card** - Rich hover content
- **Alert** - Notifications
- **Alert Dialog** - Confirmation dialogs
- **Toast** - Temporary messages

### **Data Display**
- **Calendar** - Date display
- **Date Picker** - Date selection
- **Progress** - Loading indicators
- **Skeleton** - Loading placeholders
- **Badge** - Status indicators
- **Avatar** - User representation

### **Interactive Components**
- **Slider** - Range input
- **Carousel** - Image rotation
- **Combobox** - Search and select
- **Command** - Command palette
- **Dropdown Menu** - Expandable menus

### **Utility Components**
- **Error Boundary** - Error handling
- **Lazy Loading** - Performance optimization

---

## 🏗️ **Architecture**

### **Built for Leptos v0.8+**
- **Modern Reactivity** - Uses latest Leptos signals and effects
- **Type Safety** - Comprehensive Rust type system
- **Performance** - WebAssembly compilation for speed
- **Accessibility** - WCAG compliance built-in

### **Design System**
- **ShadCN UI** - Beautiful, accessible design patterns
- **Tailwind CSS** - Utility-first styling
- **Theme Support** - Light/dark mode and customization
- **Responsive** - Mobile-first design approach

---

## 🤝 **Contributing**

### **Development Workflow**
1. **Fork** the repository
2. **Create** a feature branch
3. **Implement** your changes with tests
4. **Run** the test suite
5. **Submit** a pull request

### **Testing Requirements**
- All new components must have comprehensive unit tests
- E2E tests must pass for affected workflows
- Accessibility standards must be maintained
- Performance benchmarks must be met

### **Quality Standards**
- **100% Test Coverage** - Every component must be tested
- **Accessibility First** - WCAG compliance required
- **Performance** - No memory leaks or bottlenecks
- **Documentation** - Clear examples and guides

---

## 📊 **Performance & Quality**

### **Test Results**
- **Unit Tests**: ✅ All 300+ tests passing
- **E2E Tests**: ✅ All 129 tests passing
- **Accessibility**: ✅ WCAG 2.1 AA compliant
- **Performance**: ✅ No memory leaks detected
- **Cross-Browser**: ✅ Chrome, Firefox, Safari, Mobile

### **Bundle Optimization**
- **Code Splitting** - Components load on demand
- **Tree Shaking** - Unused code eliminated
- **WASM Optimization** - Optimized WebAssembly output
- **CSS Optimization** - Minimal, efficient styles

---

## 🚀 **Getting Help**

### **Resources**
- **[GitHub Issues](https://github.com/cloud-shuttle/leptos-shadcn-ui/issues)** - Bug reports and feature requests
- **[Discussions](https://github.com/cloud-shuttle/leptos-shadcn-ui/discussions)** - Community support
- **[Documentation](https://shadcn-ui.rustforweb.org/)** - Component API reference

### **Common Issues**
- Check the [testing guide](docs/testing/TESTING_GUIDE.md) for common problems
- Review the [defects register](docs/quality/defects-register.md) for known issues
- Consult the [architecture documentation](docs/architecture/) for system design questions

---

## 🏆 **Achievements**

This project represents a **major achievement** in component library development:

- **Industry-Leading Quality**: 100% test coverage with comprehensive validation
- **Production Ready**: All components tested and validated for real-world use
- **Accessibility First**: WCAG compliance built into every component
- **Performance Optimized**: No memory leaks or performance bottlenecks
- **Cross-Platform**: Works consistently across all major browsers and devices

**We've achieved what many enterprise teams strive for but rarely accomplish - comprehensive testing coverage at both the unit and integration levels!** 🚀

---

## 📄 **License**

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

**Last Updated**: December 2024  
**Status**: ✅ **Production Ready**  
**Version**: 0.2.0

**Built with ❤️ by the CloudShuttle team**
