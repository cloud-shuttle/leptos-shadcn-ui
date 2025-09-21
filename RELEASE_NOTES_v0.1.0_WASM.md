# 🚀 Release Notes: leptos-shadcn-ui-wasm v0.1.0

**Release Date**: September 21, 2024  
**Package**: [leptos-shadcn-ui-wasm](https://crates.io/crates/leptos-shadcn-ui-wasm)  
**Version**: 0.1.0

## 🎉 **MAJOR RELEASE: WASM Compatibility Achieved!**

We're excited to announce the first release of `leptos-shadcn-ui-wasm`, a WASM-optimized version of ShadCN UI components for Leptos 0.8+ with minimal dependencies. This release completely resolves the WASM compatibility issues that previously prevented the use of leptos-shadcn-ui in WebAssembly environments.

## ✨ **What's New**

### 🎯 **Core Features**
- **WASM-Optimized Package**: New `leptos-shadcn-ui-wasm` package specifically designed for WebAssembly
- **10 Core Components**: Button, Input, Card, Label, Badge, Avatar, Separator, Skeleton, Alert, AlertDialog
- **Minimal Dependencies**: Only 25 WASM-compatible dependencies (70% reduction from main package)
- **Feature Flags**: Granular control over included components for optimal bundle size
- **WASM Utilities**: Built-in utilities for WASM-specific functionality

### 🔧 **Technical Improvements**
- **Fixed WASM Compatibility**: Resolved `mio`, `uuid`, and `proptest` dependency issues
- **Conditional Compilation**: Native vs WASM target support in test-utils and contract-testing
- **Bundle Optimization**: Gzipped bundle size ~813KB for full demo
- **Performance Focused**: Optimized specifically for WebAssembly environments

### 📦 **Bundle Size Comparison**

| Package | Bundle Size | Dependencies | Components |
|---------|-------------|--------------|------------|
| **leptos-shadcn-ui** | ~2.5MB | 150+ deps | All (46) |
| **leptos-shadcn-ui-wasm** | ~813KB (gzipped) | 25 deps | 10 core |
| **JavaScript ShadCN UI** | ~150KB | React + Tailwind | Copy-paste |

*Note: WASM bundle is larger than JavaScript due to framework overhead, but provides type safety and performance benefits.*

## 🚀 **Quick Start**

### Installation
```toml
[dependencies]
leptos-shadcn-ui-wasm = "0.1"
```

### Basic Usage
```rust
use leptos::prelude::*;
use leptos_shadcn_ui_wasm::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="p-4">
            <Button>"Click me"</Button>
            <Input placeholder="Enter text..." />
            <Card>
                <CardHeader>
                    <CardTitle>"Hello WASM!"</CardTitle>
                </CardHeader>
                <CardContent>
                    <p>"This is a WASM-optimized component."</p>
                </CardContent>
            </Card>
        </div>
    }
}
```

### Feature Flags
```toml
# Minimal bundle with just button and input
leptos-shadcn-ui-wasm = { version = "0.1", default-features = false, features = ["button", "input"] }
```

## 🧩 **Available Components**

| Component | Feature Flag | Description |
|-----------|--------------|-------------|
| **Button** | `button` | Interactive buttons with various styles |
| **Input** | `input` | Text input fields |
| **Label** | `label` | Form labels |
| **Card** | `card` | Content containers with header, content, footer |
| **Badge** | `badge` | Small status indicators |
| **Avatar** | `avatar` | User profile images |
| **Separator** | `separator` | Visual dividers |
| **Skeleton** | `skeleton` | Loading placeholders |
| **Alert** | `alert` | Notification messages |
| **AlertDialog** | `alert-dialog` | Modal alert dialogs |

## 🛠️ **WASM-Specific Utilities**

```rust
use leptos_shadcn_ui_wasm::{wasm_utils, bundle_utils};

// Initialize WASM-specific features
wasm_utils::init_wasm();

// Get bundle information
let bundle_info = bundle_utils::get_bundle_info();
println!("Bundle: {}", bundle_info);

// Log to browser console
wasm_utils::log("Hello from WASM!");
```

## 🔄 **Migration Guide**

### From Main Package
1. **Update Dependencies**:
   ```toml
   # Before
   leptos-shadcn-ui = "0.9"
   
   # After  
   leptos-shadcn-ui-wasm = "0.1"
   ```

2. **Update Imports**:
   ```rust
   // Before
   use leptos_shadcn_ui::prelude::*;
   
   // After
   use leptos_shadcn_ui_wasm::prelude::*;
   ```

3. **Enable Features**:
   ```toml
   leptos-shadcn-ui-wasm = { version = "0.1", features = ["button", "input", "card"] }
   ```

## 📊 **Performance Metrics**

- **Bundle Size**: 813KB (gzipped) for full demo
- **Dependencies**: 25 WASM-compatible dependencies
- **Compilation Time**: ~44 seconds for full demo
- **Components**: 10 core components included
- **WASM Compatibility**: 100% compatible with `wasm32-unknown-unknown`

## 🧪 **Testing Results**

✅ **All Tests Passed**:
- WASM compilation for all packages
- Feature flags work correctly
- Demo builds and runs successfully
- Published package verified on crates.io
- Backward compatibility maintained

## 🎯 **Use Cases**

Perfect for:
- **Web Applications**: Full-stack web apps with Leptos
- **Progressive Web Apps**: Offline-capable applications
- **Interactive Dashboards**: Real-time data visualization
- **Form Applications**: Data entry and validation
- **Content Management**: Admin panels and interfaces

## 🔮 **What's Next**

- **More Components**: Additional ShadCN components in future releases
- **Bundle Optimization**: Further size reductions with `wasm-opt`
- **Performance Improvements**: Runtime optimizations
- **Documentation**: More examples and tutorials

## 🙏 **Acknowledgments**

- [Leptos](https://leptos.dev/) - The amazing Rust web framework
- [ShadCN UI](https://ui.shadcn.com/) - Beautiful component design system
- [Tailwind CSS](https://tailwindcss.com/) - Utility-first CSS framework
- Rust and WebAssembly community for excellent tooling

## 📚 **Resources**

- **Package**: [crates.io/crates/leptos-shadcn-ui-wasm](https://crates.io/crates/leptos-shadcn-ui-wasm)
- **Documentation**: [docs.rs/leptos-shadcn-ui-wasm](https://docs.rs/leptos-shadcn-ui-wasm)
- **Repository**: [github.com/cloud-shuttle/leptos-shadcn-ui](https://github.com/cloud-shuttle/leptos-shadcn-ui)
- **Demo**: Check out the `wasm-demo/` directory for a live example

## 🐛 **Bug Reports & Feedback**

Found a bug or have feedback? Please open an issue on [GitHub](https://github.com/cloud-shuttle/leptos-shadcn-ui/issues).

---

**Made with ❤️ for the Rust and WebAssembly community**

*This release represents a major milestone in making Leptos ShadCN UI components accessible to WebAssembly applications. We're excited to see what you build with it!*
