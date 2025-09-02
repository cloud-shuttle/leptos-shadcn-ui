# Leptos ShadCN UI Components

[![Leptos Version](https://img.shields.io/badge/Leptos-0.8%2B-blue?style=for-the-badge&logo=rust)](https://github.com/leptos-rs/leptos)
[![License](https://img.shields.io/badge/License-MIT-green.svg?style=for-the-badge)](LICENSE)

A comprehensive collection of beautiful, accessible UI components built for [Leptos](https://github.com/leptos-rs/leptos) **v0.8+**, inspired by [shadcn/ui](https://ui.shadcn.com/).

**⚠️ IMPORTANT: This project requires Leptos v0.8+ and is NOT compatible with earlier versions.**

## 🚨 Version Requirements

**Leptos v0.8+ is MANDATORY for this project.**

- ✅ **Supported**: Leptos v0.8.0, v0.8.1, v0.8.2, v0.8.3, v0.8.4, v0.8.5, v0.8.6, v0.8.7, v0.8.8+
- ❌ **NOT Supported**: Leptos v0.7.x, v0.6.x, or any earlier versions
- 🔄 **Future**: Will continue to support the latest Leptos v0.8.x releases

**Why v0.8+?** This project leverages breaking changes and new features introduced in Leptos v0.8, including improved view macros, better type safety, and enhanced performance.

## 🚀 Features

- **Leptos v0.8+ Required**: Built specifically for Leptos v0.8+ and NOT compatible with earlier versions
- **ShadCN UI Design**: Follows the same design principles and styling as shadcn/ui
- **TypeScript Definitions**: Full TypeScript support for better developer experience
- **Accessibility First**: All components follow accessibility best practices
- **Customizable**: Easy to customize with Tailwind CSS classes
- **Lightweight**: Only includes the components you need

## 📦 Available Components

### ✅ Core Components (Fully Implemented - 52 Packages!)
- **Form Components**: Button, Input, Label, Checkbox, Switch, Radio Group, Select, Textarea, Form
- **Layout Components**: Card, Separator, Skeleton, Tabs, Accordion, Collapsible, Aspect Ratio
- **Navigation**: Breadcrumb, Navigation Menu, Pagination, Tabs
- **Feedback**: Alert, Alert Dialog, Progress, Toast, Skeleton
- **Overlay**: Dialog, Popover, Tooltip, Sheet, Drawer, Hover Card
- **Data Display**: Table, Badge, Calendar, Date Picker
- **Input**: Input OTP, Slider, Toggle, Combobox, Command
- **Utilities**: Utils, Registry, Error Boundary, Lazy Loading
- **Advanced**: Context Menu, Dropdown Menu, Menubar, Scroll Area

**All 52 packages are fully tested and working with Leptos v0.8.8!**

## 🛠️ Installation

### 1. Verify Leptos Version

**CRITICAL**: Ensure your project uses Leptos v0.8+:

```toml
[dependencies]
leptos = "0.8.0"  # Must be 0.8.0 or higher
leptos_router = "0.8.0"  # Must be 0.8.0 or higher
```

### 2. Add Components to your `Cargo.toml`

```toml
[dependencies]
shadcn-ui-leptos-button = { path = "path/to/leptos-shadcn-ui/packages/leptos/button" }
shadcn-ui-leptos-input = { path = "path/to/leptos-shadcn-ui/packages/leptos/input" }
shadcn-ui-leptos-card = { path = "path/to/leptos-shadcn-ui/packages/leptos/card" }
shadcn-ui-leptos-alert = { path = "path/to/leptos-shadcn-ui/packages/leptos/alert" }
shadcn-ui-leptos-label = { path = "path/to/leptos-shadcn-ui/packages/leptos/label" }
shadcn-ui-leptos-separator = { path = "path/to/leptos-shadcn-ui/packages/leptos/separator" }
```

### 2. Import and use in your Leptos components

```rust
use leptos::*;
use shadcn_ui_leptos_button::{Button, ButtonVariant, ButtonSize};
use shadcn_ui_leptos_input::Input;
use shadcn_ui_leptos_card::{Card, CardHeader, CardTitle, CardContent};

#[component]
pub fn MyComponent() -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <CardTitle>"Welcome"</CardTitle>
            </CardHeader>
            <CardContent>
                <div class="space-y-4">
                    <Input placeholder="Enter your name" />
                    <Button variant=ButtonVariant::Default>"Submit"</Button>
                </div>
            </CardContent>
        </Card>
    }
}
```

## 🎨 Usage Examples

### Button Component

```rust
// Different variants
<Button variant=ButtonVariant::Default>"Default"</Button>
<Button variant=ButtonVariant::Destructive>"Delete"</Button>
<Button variant=ButtonVariant::Outline>"Outline"</Button>
<Button variant=ButtonVariant::Secondary>"Secondary"</Button>
<Button variant=ButtonVariant::Ghost>"Ghost"</Button>
<Button variant=ButtonVariant::Link>"Link"</Button>

// Different sizes
<Button size=ButtonSize::Sm>"Small"</Button>
<Button size=ButtonSize::Default>"Default"</Button>
<Button size=ButtonSize::Lg>"Large"</Button>
<Button size=ButtonSize::Icon>"🔍"</Button>
```

### Input Component

```rust
<Input
    placeholder="Type something..."
    input_type="email"
    value=Signal::derive(move || input_value.get())
    on_change=Callback::new(move |value| set_input_value.set(value))
/>
```

### Card Component

```rust
<Card>
    <CardHeader>
        <CardTitle>"Card Title"</CardTitle>
        <CardDescription>"Card description goes here"</CardDescription>
    </CardHeader>
    <CardContent>
        <p>"Your content here"</p>
    </CardContent>
</Card>
```

### Alert Component

```rust
<Alert variant=AlertVariant::Default>
    <AlertTitle>"Information"</AlertTitle>
    <AlertDescription>"This is an informational message."</AlertDescription>
</Alert>

<Alert variant=AlertVariant::Destructive>
    <AlertTitle>"Error"</AlertTitle>
    <AlertDescription>"Something went wrong."</AlertDescription>
</Alert>
```

## 🎯 Demo

Check out the live demo in the `examples/leptos` directory. To run it:

```bash
cd examples/leptos
cargo run
```

The demo showcases all available components with interactive examples and usage patterns.

## 🏗️ Project Structure

```
leptos-shadcn-ui/
├── packages/
│   ├── leptos/           # Leptos-specific components
│   │   ├── button/       # Button component
│   │   ├── input/        # Input component
│   │   ├── card/         # Card component
│   │   ├── alert/        # Alert component
│   │   ├── label/        # Label component
│   │   └── separator/    # Separator component
│   ├── registry/         # Component registry
│   ├── shadcn/           # Core shadcn utilities
│   └── test-utils/       # Testing utilities
├── examples/
│   └── leptos/           # Leptos demo application
└── docs/                 # Documentation
```

## 🔧 Development

### Prerequisites

- Rust 1.70+
- **Leptos v0.8+ (REQUIRED - no earlier versions supported)**
- Node.js (for Tailwind CSS)

### Building

```bash
# Build all packages
cargo build --workspace

# Build specific package
cargo build -p shadcn-ui-leptos-button

# Run tests
cargo test --workspace
```

### Adding New Components

1. Create a new package in `packages/leptos/`
2. Follow the existing component structure
3. Add to the workspace in `Cargo.toml`
4. Update the demo application
5. Add TypeScript definitions

## 🎨 Styling

All components use Tailwind CSS for styling. The design system follows shadcn/ui conventions:

- **Colors**: Semantic color tokens (primary, secondary, destructive, etc.)
- **Spacing**: Consistent spacing scale
- **Typography**: Standard font sizes and weights
- **Borders**: Consistent border radius and styles
- **Shadows**: Subtle shadows for depth

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Workflow

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [shadcn/ui](https://ui.shadcn.com/) for the beautiful design system
- [Leptos](https://github.com/leptos-rs/leptos) team for the amazing Rust web framework
- [Tailwind CSS](https://tailwindcss.com/) for the utility-first CSS framework

## 🚨 Troubleshooting

### Common Issues

**"Leptos version not found" or "Incompatible version" errors:**
- Ensure you're using Leptos v0.8.0 or higher
- Check your `Cargo.toml` has `leptos = "0.8.0"` (not `"0.7.0"`)
- Run `cargo update` to ensure you have the latest compatible versions

**Compilation errors with view macros:**
- These often indicate version incompatibility
- Verify both `leptos` and `leptos_router` are v0.8.0+

### Version Check

Add this to your code to verify the Leptos version at runtime:

```rust
use leptos::*;

#[component]
pub fn VersionCheck() -> impl IntoView {
    view! {
        <div>
            <p>"Leptos version: {leptos::VERSION}"</p>
            <p>"Required: 0.8.0+"</p>
        </div>
    }
}
```

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/cloud-shuttle/leptos-shadcn-ui/issues)
- **Discussions**: [GitHub Discussions](https://github.com/cloud-shuttle/leptos-shadcn-ui/discussions)
- **Documentation**: [docs/](docs/)

---

Built with ❤️ by the CloudShuttle team
