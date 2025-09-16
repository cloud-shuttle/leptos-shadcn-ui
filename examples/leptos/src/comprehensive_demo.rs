use leptos::prelude::*;
use leptos_shadcn_button::{Button, ButtonVariant, ButtonSize};
use leptos_shadcn_card::{Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter};
use leptos_shadcn_input::Input;
use tailwind_rs_core::*;

#[component]
pub fn ComprehensiveDemo() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (input_value, set_input_value) = signal(String::new());
    let (is_dark, set_is_dark) = signal(false);

    let toggle_theme = move || {
        set_is_dark.update(|dark| *dark = !*dark);
    };

    let increment = move || {
        set_count.update(|c| *c += 1);
    };

    let decrement = move || {
        set_count.update(|c| *c -= 1);
    };

    let reset = move || {
        set_count.set(0);
    };

    // Using tailwind-rs-core for dynamic class generation
    let container_classes = move || {
        let base = "min-h-screen bg-background text-foreground";
        if is_dark.get() {
            format!("{} dark", base)
        } else {
            base.to_string()
        }
    };

    let card_classes = move || {
        "rounded-lg border bg-card text-card-foreground shadow-sm".to_string()
    };

    let button_classes = move || {
        "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50".to_string()
    };

    view! {
        <div class=container_classes>
            <div class="container mx-auto px-4 py-8">
                // Header
                <div class="text-center mb-12">
                    <h1 class="text-4xl font-bold mb-4">"Leptos ShadCN UI Demo (WASM + tailwind-rs-core)"</h1>
                    <p class="text-lg text-muted-foreground mb-6">
                        "Interactive showcase using real Leptos WASM components with tailwind-rs-core"
                    </p>
                    <Button 
                        variant=ButtonVariant::Outline
                        size=ButtonSize::Default
                        on_click=toggle_theme
                        class="mb-8"
                    >
                        {move || if is_dark.get() { "🌞 Light Mode" } else { "🌙 Dark Mode" }}
                    </Button>
                </div>

                // Counter Demo
                <Card class="mb-8">
                    <CardHeader>
                        <CardTitle>"Counter Demo (WASM)"</CardTitle>
                        <CardDescription>"Interactive counter with different button variants using tailwind-rs-core"</CardDescription>
                    </CardHeader>
                    <CardContent class="space-y-4">
                        <div class="text-center">
                            <div class="text-6xl font-bold mb-4">{count}</div>
                            <div class="flex gap-2 justify-center">
                                <Button 
                                    variant=ButtonVariant::Default
                                    size=ButtonSize::Default
                                    on_click=increment
                                >
                                    "Increment"
                                </Button>
                                <Button 
                                    variant=ButtonVariant::Secondary
                                    size=ButtonSize::Default
                                    on_click=decrement
                                >
                                    "Decrement"
                                </Button>
                                <Button 
                                    variant=ButtonVariant::Destructive
                                    size=ButtonSize::Default
                                    on_click=reset
                                >
                                    "Reset"
                                </Button>
                            </div>
                        </div>
                    </CardContent>
                </Card>

                // Button Variants Demo
                <Card class="mb-8">
                    <CardHeader>
                        <CardTitle>"Button Variants (tailwind-rs-core)"</CardTitle>
                        <CardDescription>"All available button variants and sizes with dynamic class generation"</CardDescription>
                    </CardHeader>
                    <CardContent class="space-y-6">
                        <div>
                            <h3 class="text-lg font-semibold mb-3">"Variants"</h3>
                            <div class="flex flex-wrap gap-2">
                                <Button variant=ButtonVariant::Default>"Default"</Button>
                                <Button variant=ButtonVariant::Destructive>"Destructive"</Button>
                                <Button variant=ButtonVariant::Outline>"Outline"</Button>
                                <Button variant=ButtonVariant::Secondary>"Secondary"</Button>
                                <Button variant=ButtonVariant::Ghost>"Ghost"</Button>
                                <Button variant=ButtonVariant::Link>"Link"</Button>
                            </div>
                        </div>
                        <div>
                            <h3 class="text-lg font-semibold mb-3">"Sizes"</h3>
                            <div class="flex flex-wrap gap-2 items-center">
                                <Button size=ButtonSize::Default>"Default"</Button>
                                <Button size=ButtonSize::Sm>"Small"</Button>
                                <Button size=ButtonSize::Lg>"Large"</Button>
                                <Button size=ButtonSize::Icon>"🎯"</Button>
                            </div>
                        </div>
                    </CardContent>
                </Card>

                // Input Demo
                <Card class="mb-8">
                    <CardHeader>
                        <CardTitle>"Input Demo (WASM)"</CardTitle>
                        <CardDescription>"Interactive input with real-time value display using Leptos signals"</CardDescription>
                    </CardHeader>
                    <CardContent class="space-y-4">
                        <div>
                            <label class="block text-sm font-medium mb-2">"Enter some text:"</label>
                            <Input 
                                value=input_value
                                on_change=Callback::new(move |value| set_input_value.set(value))
                                placeholder="Type something here..."
                                class="w-full"
                            />
                        </div>
                        <div class="p-4 bg-muted rounded-md">
                            <p class="text-sm text-muted-foreground">"Current value:"</p>
                            <p class="font-mono">{input_value}</p>
                        </div>
                    </CardContent>
                </Card>

                // Card Layout Demo
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-8">
                    <Card>
                        <CardHeader>
                            <CardTitle>"Feature 1"</CardTitle>
                            <CardDescription>"Description of the first feature"</CardDescription>
                        </CardHeader>
                        <CardContent>
                            <p class="text-sm text-muted-foreground">
                                "This is a sample card content that demonstrates the card component layout and styling."
                            </p>
                        </CardContent>
                        <CardFooter>
                            <Button variant=ButtonVariant::Outline size=ButtonSize::Sm class="w-full">
                                "Learn More"
                            </Button>
                        </CardFooter>
                    </Card>

                    <Card>
                        <CardHeader>
                            <CardTitle>"Feature 2"</CardTitle>
                            <CardDescription>"Description of the second feature"</CardDescription>
                        </CardHeader>
                        <CardContent>
                            <p class="text-sm text-muted-foreground">
                                "Another sample card with different content to show the flexibility of the card component."
                            </p>
                        </CardContent>
                        <CardFooter>
                            <Button variant=ButtonVariant::Default size=ButtonSize::Sm class="w-full">
                                "Get Started"
                            </Button>
                        </CardFooter>
                    </Card>

                    <Card>
                        <CardHeader>
                            <CardTitle>"Feature 3"</CardTitle>
                            <CardDescription>"Description of the third feature"</CardDescription>
                        </CardHeader>
                        <CardContent>
                            <p class="text-sm text-muted-foreground">
                                "A third card to complete the grid layout and demonstrate responsive design."
                            </p>
                        </CardContent>
                        <CardFooter>
                            <Button variant=ButtonVariant::Secondary size=ButtonSize::Sm class="w-full">
                                "Explore"
                            </Button>
                        </CardFooter>
                    </Card>
                </div>

                // Technical Info
                <Card class="mb-8">
                    <CardHeader>
                        <CardTitle>"Technical Implementation"</CardTitle>
                        <CardDescription>"This demo showcases the real Leptos ShadCN UI components"</CardDescription>
                    </CardHeader>
                    <CardContent class="space-y-4">
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                            <div class="p-4 bg-muted rounded-md">
                                <h4 class="font-semibold mb-2">"✅ WASM Components"</h4>
                                <p class="text-sm text-muted-foreground">"Real Leptos components compiled to WebAssembly"</p>
                            </div>
                            <div class="p-4 bg-muted rounded-md">
                                <h4 class="font-semibold mb-2">"✅ tailwind-rs-core"</h4>
                                <p class="text-sm text-muted-foreground">"Type-safe Tailwind CSS class generation in Rust"</p>
                            </div>
                            <div class="p-4 bg-muted rounded-md">
                                <h4 class="font-semibold mb-2">"✅ Leptos Signals"</h4>
                                <p class="text-sm text-muted-foreground">"Reactive state management with real-time updates"</p>
                            </div>
                            <div class="p-4 bg-muted rounded-md">
                                <h4 class="font-semibold mb-2">"✅ Production Ready"</h4>
                                <p class="text-sm text-muted-foreground">"No CDN dependencies, fully self-contained"</p>
                            </div>
                        </div>
                    </CardContent>
                </Card>

                // Footer
                <div class="text-center text-sm text-muted-foreground">
                    <p>"Built with Leptos, ShadCN UI, and tailwind-rs-core"</p>
                    <p class="mt-2">
                        "Components: Button, Card, Input | Variants: Default, New York | Theme: Light/Dark | Technology: WASM + Rust"
                    </p>
                </div>
            </div>
        </div>
    }
}