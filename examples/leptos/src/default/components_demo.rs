use leptos::*;
use leptos::prelude::*;

// Import only the core components that are known to work
use shadcn_ui_leptos_button::{Button, ButtonVariant, ButtonSize};
use shadcn_ui_leptos_input::Input;
use shadcn_ui_leptos_card::{Card, CardHeader, CardTitle, CardDescription, CardContent};
use shadcn_ui_leptos_alert::{Alert, AlertTitle, AlertDescription, AlertVariant};
use shadcn_ui_leptos_label::Label;
use shadcn_ui_leptos_separator::Separator;

#[component]
pub fn ComponentsDemo() -> impl IntoView {
    let (input_value, set_input_value) = signal("".to_string());

    view! {
        <div class="min-h-screen bg-background text-foreground p-6">
            <div class="max-w-7xl mx-auto">
                <div class="text-center mb-8">
                    <h1 class="text-4xl font-bold mb-4">"Leptos ShadCN UI Components"</h1>
                    <p class="text-xl text-muted-foreground">
                        "A comprehensive collection of beautiful, accessible components built for Leptos v0.8+"
                    </p>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    // Basic Components
                    <Card>
                        <CardHeader>
                            <CardTitle>"Button Variants"</CardTitle>
                            <CardDescription>"All available button styles and sizes"</CardDescription>
                        </CardHeader>
                        <CardContent class="space-y-4">
                            <div class="flex flex-wrap gap-2">
                                <Button variant=ButtonVariant::Default>"Default"</Button>
                                <Button variant=ButtonVariant::Destructive>"Destructive"</Button>
                                <Button variant=ButtonVariant::Outline>"Outline"</Button>
                                <Button variant=ButtonVariant::Secondary>"Secondary"</Button>
                                <Button variant=ButtonVariant::Ghost>"Ghost"</Button>
                                <Button variant=ButtonVariant::Link>"Link"</Button>
                            </div>
                            <div class="flex flex-wrap gap-2">
                                <Button size=ButtonSize::Sm>"Small"</Button>
                                <Button size=ButtonSize::Default>"Default"</Button>
                                <Button size=ButtonSize::Lg>"Large"</Button>
                                <Button size=ButtonSize::Icon>"🔍"</Button>
                            </div>
                        </CardContent>
                    </Card>

                    <Card>
                        <CardHeader>
                            <CardTitle>"Input Components"</CardTitle>
                            <CardDescription>"Form inputs with various types"</CardDescription>
                        </CardHeader>
                        <CardContent class="space-y-4">
                            <div class="space-y-2">
                                <Label>"Basic Input"</Label>
                                <Input
                                    id="basic-input"
                                    placeholder="Type something..."
                                    on_change=Callback::new(move |value| set_input_value.set(value))
                                />
                                <div class="text-sm text-muted-foreground">
                                    "Current value: " {move || input_value.get()}
                                </div>
                            </div>
                            <div class="space-y-2">
                                <Label>"Email Input"</Label>
                                <Input
                                    id="email-input"
                                    input_type="email"
                                    placeholder="Enter your email"
                                />
                            </div>
                        </CardContent>
                    </Card>

                    <Card>
                        <CardHeader>
                            <CardTitle>"Alerts"</CardTitle>
                            <CardDescription>"Different alert types for various messages"</CardDescription>
                        </CardHeader>
                        <CardContent class="space-y-4">
                            <Alert variant=AlertVariant::Default>
                                <AlertTitle>"Default Alert"</AlertTitle>
                                <AlertDescription>"This is a default alert message."</AlertDescription>
                            </Alert>
                            <Alert variant=AlertVariant::Destructive>
                                <AlertTitle>"Destructive Alert"</AlertTitle>
                                <AlertDescription>"This is a destructive alert message."</AlertDescription>
                            </Alert>
                            <Alert variant=AlertVariant::Success>
                                <AlertTitle>"Success Alert"</AlertTitle>
                                <AlertDescription>"This is a success alert message."</AlertDescription>
                            </Alert>
                            <Alert variant=AlertVariant::Warning>
                                <AlertTitle>"Warning Alert"</AlertTitle>
                                <AlertDescription>"This is a warning alert message."</AlertDescription>
                            </Alert>
                        </CardContent>
                    </Card>

                    <Card>
                        <CardHeader>
                            <CardTitle>"Layout Components"</CardTitle>
                            <CardDescription>"Cards, separators, and other layout elements"</CardDescription>
                        </CardHeader>
                        <CardContent class="space-y-4">
                            <div class="space-y-2">
                                <p>"This card demonstrates the layout capabilities."</p>
                                <Separator />
                                <p>"Separators help organize content visually."</p>
                            </div>
                            <div class="flex justify-end">
                                <Button>"Action"</Button>
                            </div>
                        </CardContent>
                    </Card>

                    <Card>
                        <CardHeader>
                            <CardTitle>"Interactive Elements"</CardTitle>
                            <CardDescription>"Buttons and form controls"</CardDescription>
                        </CardHeader>
                        <CardContent class="space-y-4">
                            <div class="flex flex-wrap gap-2">
                                <Button variant=ButtonVariant::Default on:click=move |_| log::info!("Button clicked!")>
                                    "Click Me"
                                </Button>
                                <Button variant=ButtonVariant::Outline disabled=true>
                                    "Disabled"
                                </Button>
                            </div>
                            <div class="text-sm text-muted-foreground">
                                "Try clicking the buttons to see them in action."
                            </div>
                        </CardContent>
                    </Card>

                    <Card>
                        <CardHeader>
                            <CardTitle>"Component Status"</CardTitle>
                            <CardDescription>"Current implementation status"</CardDescription>
                        </CardHeader>
                        <CardContent class="space-y-2">
                            <div class="flex items-center justify-between">
                                <span>"Button Component"</span>
                                <span class="text-green-600">"✅ Complete"</span>
                            </div>
                            <div class="flex items-center justify-between">
                                <span>"Input Component"</span>
                                <span class="text-green-600">"✅ Complete"</span>
                            </div>
                            <div class="flex items-center justify-between">
                                <span>"Card Component"</span>
                                <span class="text-green-600">"✅ Complete"</span>
                            </div>
                            <div class="flex items-center justify-between">
                                <span>"Alert Component"</span>
                                <span class="text-green-600">"✅ Complete"</span>
                            </div>
                            <div class="flex items-center justify-between">
                                <span>"Label Component"</span>
                                <span class="text-green-600">"✅ Complete"</span>
                            </div>
                            <Separator />
                            <div class="text-sm text-muted-foreground">
                                "More components coming soon..."
                            </div>
                        </CardContent>
                    </Card>
                </div>

                <div class="mt-12 text-center">
                    <Card>
                        <CardHeader>
                            <CardTitle>"Getting Started"</CardTitle>
                            <CardDescription>"How to use these components in your project"</CardDescription>
                        </CardHeader>
                        <CardContent class="space-y-4">
                            <div class="text-left space-y-2">
                                <p class="font-medium">"1. Add to your Cargo.toml:"</p>
                                <pre class="bg-muted p-3 rounded text-sm overflow-x-auto">
                                    <code>
                                        "[dependencies]\nshadcn-ui-leptos-button = { path = \"path/to/button\" }\nshadcn-ui-leptos-input = { path = \"path/to/input\" }\nshadcn-ui-leptos-card = { path = \"path/to/card\" }"
                                    </code>
                                </pre>
                                
                                <p class="font-medium">"2. Import and use:"</p>
                                <pre class="bg-muted p-3 rounded text-sm overflow-x-auto">
                                    <code>
                                        "use shadcn_ui_leptos_button::Button;\nuse shadcn_ui_leptos_input::Input;\n\nview! {\n    <Button>\"Click me\"</Button>\n    <Input placeholder=\"Type here...\" />\n}"
                                    </code>
                                </pre>
                            </div>
                        </CardContent>
                    </Card>
                </div>
            </div>
        </div>
    }
}
