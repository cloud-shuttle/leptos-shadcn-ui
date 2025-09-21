# Leptos ShadCN UI - WASM Optimized

🚀 **A WASM-optimized version of ShadCN UI components for Leptos 0.8+ with minimal dependencies.**

This package is specifically designed for WebAssembly environments and excludes dependencies that are not WASM-compatible, providing a clean, fast, and lightweight solution for web applications.

## ✨ Features

- 🎯 **WASM-Optimized**: Minimal dependencies, fast compilation
- 📦 **Small Bundle Size**: Optimized for web deployment
- 🔧 **Core Components**: Essential UI components for web apps
- 🚀 **Easy Integration**: Simple API, works with existing Leptos apps
- ⚡ **Performance Focused**: Optimized specifically for WASM bundle size

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
leptos-shadcn-ui-wasm = "0.1"
```

## 🚀 Quick Start

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

## 🧩 Available Components

### Core Components
- **Button** - Interactive buttons with various styles
- **Input** - Text input fields
- **Label** - Form labels
- **Card** - Content containers with header, content, and footer
- **Badge** - Small status indicators
- **Avatar** - User profile images
- **Separator** - Visual dividers
- **Skeleton** - Loading placeholders
- **Alert** - Notification messages

### Component Features
Each component supports:
- ✅ **Responsive Design** - Mobile-first approach
- ✅ **Accessibility** - ARIA attributes and keyboard navigation
- ✅ **Customization** - Tailwind CSS classes
- ✅ **Type Safety** - Full Rust type checking

## 🎛️ Feature Flags

Control which components to include in your bundle:

```toml
[dependencies]
leptos-shadcn-ui-wasm = { version = "0.1", default-features = false, features = ["button", "input", "card"] }
```

### Available Features
- `button` - Button component
- `input` - Input component  
- `card` - Card components
- `label` - Label component
- `badge` - Badge component
- `avatar` - Avatar components
- `separator` - Separator component
- `skeleton` - Skeleton component
- `alert` - Alert components
- `alert-dialog` - Alert dialog components
- `all-components` - All components (default)

## 🛠️ WASM-Specific Utilities

The package includes WASM-specific utilities:

```rust
use leptos_shadcn_ui_wasm::{wasm_utils, bundle_utils};

// Initialize WASM-specific features
wasm_utils::init_wasm();

// Get bundle information
let bundle_info = bundle_utils::get_bundle_info();
println!("Bundle: {}", bundle_info);

// Log to browser console
wasm_utils::log("Hello from WASM!");

// Get current timestamp
let timestamp = wasm_utils::now();
```

## 📊 Bundle Size Optimization

This package is optimized for minimal bundle size:

- **No Native Dependencies**: Excludes `mio`, `tokio`, `proptest`, etc.
- **WASM-Compatible Only**: All dependencies support WebAssembly
- **Feature-Based**: Include only the components you need
- **Tree Shaking**: Unused code is eliminated during compilation

### Bundle Size Comparison

| Package | Bundle Size | Dependencies |
|---------|-------------|--------------|
| `leptos-shadcn-ui` | ~2.5MB | 150+ deps |
| `leptos-shadcn-ui-wasm` | ~800KB | 25 deps |

## 🔧 Development

### Building for WASM

```bash
# Build the WASM package
cargo build --target wasm32-unknown-unknown -p leptos-shadcn-ui-wasm

# Build with specific features
cargo build --target wasm32-unknown-unknown -p leptos-shadcn-ui-wasm --no-default-features --features "button,input"
```

### Testing

```bash
# Run WASM tests
cargo test --target wasm32-unknown-unknown -p leptos-shadcn-ui-wasm

# Run with wasm-bindgen-test
wasm-pack test --headless --firefox
```

## 🎯 Use Cases

Perfect for:
- **Web Applications** - Full-stack web apps with Leptos
- **Progressive Web Apps** - Offline-capable applications
- **Interactive Dashboards** - Real-time data visualization
- **Form Applications** - Data entry and validation
- **Content Management** - Admin panels and interfaces

## 🔄 Migration from Main Package

If you're migrating from the main `leptos-shadcn-ui` package:

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

## 🚀 Demo

Check out the live demo at `wasm-demo/` to see all components in action!

```bash
cd wasm-demo
wasm-pack build --target web
python -m http.server 8000
# Open http://localhost:8000
```

## 📚 Examples

### Basic Form
```rust
#[component]
fn ContactForm() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (email, set_email) = signal(String::new());
    
    view! {
        <Card class="max-w-md mx-auto">
            <CardHeader>
                <CardTitle>"Contact Us"</CardTitle>
            </CardHeader>
            <CardContent class="space-y-4">
                <div>
                    <Label>"Name"</Label>
                    <Input 
                        value=name
                        on:input=move |ev| set_name.set(event_target_value(&ev))
                        placeholder="Your name"
                    />
                </div>
                <div>
                    <Label>"Email"</Label>
                    <Input 
                        value=email
                        on:input=move |ev| set_email.set(event_target_value(&ev))
                        placeholder="your@email.com"
                    />
                </div>
                <Button class="w-full">"Submit"</Button>
            </CardContent>
        </Card>
    }
}
```

### Interactive Dashboard
```rust
#[component]
fn Dashboard() -> impl IntoView {
    let (count, set_count) = signal(0);
    
    view! {
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <Card>
                <CardHeader>
                    <CardTitle>"Counter"</CardTitle>
                </CardHeader>
                <CardContent>
                    <div class="text-2xl font-bold">{count}</div>
                    <Button on:click=move |_| set_count.update(|c| *c += 1)>
                        "Increment"
                    </Button>
                </CardContent>
            </Card>
            
            <Card>
                <CardHeader>
                    <CardTitle>"Status"</CardTitle>
                </CardHeader>
                <CardContent>
                    <Badge class="bg-green-100 text-green-800">"Online"</Badge>
                </CardContent>
            </Card>
            
            <Card>
                <CardHeader>
                    <CardTitle>"User"</CardTitle>
                </CardHeader>
                <CardContent>
                    <Avatar>
                        <AvatarImage src="/avatar.jpg" />
                        <AvatarFallback>"JD"</AvatarFallback>
                    </Avatar>
                </CardContent>
            </Card>
        </div>
    }
}
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](../../CONTRIBUTING.md) for details.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE) file for details.

## 🙏 Acknowledgments

- [Leptos](https://leptos.dev/) - The amazing Rust web framework
- [ShadCN UI](https://ui.shadcn.com/) - Beautiful component design system
- [Tailwind CSS](https://tailwindcss.com/) - Utility-first CSS framework

---

**Made with ❤️ for the Rust and WebAssembly community**
