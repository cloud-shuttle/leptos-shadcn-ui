use leptos::prelude::*;
use leptos_shadcn_ui::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! {
        <div class="min-h-screen bg-background p-8">
            <div class="max-w-4xl mx-auto space-y-8">
                <header class="text-center">
                    <h1 class="text-4xl font-bold text-foreground mb-2">
                        "🧪 Leptos v0.8 Compatibility Test"
                    </h1>
                    <p class="text-muted-foreground">
                        "Comprehensive verification of all leptos-shadcn-ui components with Leptos v0.8"
                    </p>
                </header>

                <div class="grid gap-8">
                    <SignalReactivityTest />
                    <EventHandlingTest />
                    <AttributeBindingTest />
                </div>
            </div>
        </div>
    })
}

#[component]
fn SignalReactivityTest() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (is_visible, set_is_visible) = signal(true);
    let (text, set_text) = signal("Hello, Leptos v0.8!".to_string());
    
    view! {
        <Card class="p-6">
            <CardHeader>
                <CardTitle>"🔄 Signal Reactivity Test"</CardTitle>
                <CardDescription>
                    "Testing signal updates and reactivity with Leptos v0.8"
                </CardDescription>
            </CardHeader>
            <CardContent class="space-y-4">
                <div class="flex gap-4 items-center">
                    <Button on_click=Callback::new(move |_| set_count.update(|c| *c += 1))>
                        "Count: " {move || count.get()}
                    </Button>
                    <Button 
                        variant=ButtonVariant::Secondary
                        on_click=Callback::new(move |_| set_count.set(0))
                    >
                        "Reset"
                    </Button>
                </div>
                
                <div class="flex gap-4 items-center">
                    <Button 
                        variant=ButtonVariant::Outline
                        on_click=Callback::new(move |_| set_is_visible.update(|v| *v = !*v))
                    >
                        "Toggle Visibility"
                    </Button>
                    <div 
                        class="p-2 bg-muted rounded"
                        style:display=move || if is_visible.get() { "block" } else { "none" }
                    >
                        "This should toggle visibility"
                    </div>
                </div>
                
                <div class="space-y-2">
                    <Input 
                        value=text
                        on_change=Callback::new(move |value| set_text.set(value))
                        placeholder="Type something..."
                    />
                    <p class="text-sm text-muted-foreground">
                        "Text: " {move || text.get()}
                    </p>
                </div>
            </CardContent>
        </Card>
    }
}

#[component]
fn EventHandlingTest() -> impl IntoView {
    let (button_clicks, set_button_clicks) = signal(0);
    let (input_value, set_input_value) = signal(String::new());
    let (checkbox_checked, set_checkbox_checked) = signal(false);
    
    view! {
        <Card class="p-6">
            <CardHeader>
                <CardTitle>"🎯 Event Handling Test"</CardTitle>
                <CardDescription>
                    "Testing event handlers with Leptos v0.8"
                </CardDescription>
            </CardHeader>
            <CardContent class="space-y-4">
                <div class="space-y-2">
                    <Button 
                        variant=ButtonVariant::Destructive
                        on_click=Callback::new(move |_| set_button_clicks.update(|c| *c += 1))
                    >
                        "Button clicked: " {move || button_clicks.get()} " times"
                    </Button>
                </div>
                
                <div class="space-y-2">
                    <Label>"Input Event Test"</Label>
                    <Input 
                        value=input_value
                        on_change=Callback::new(move |value| set_input_value.set(value))
                        placeholder="Type to test input events..."
                    />
                    <p class="text-sm text-muted-foreground">
                        "Input value: " {move || input_value.get()}
                    </p>
                </div>
                
                <div class="flex items-center space-x-2">
                    <Checkbox 
                        checked=checkbox_checked
                        on_change=Callback::new(move |checked| set_checkbox_checked.set(checked))
                    />
                    <Label>"Checkbox: " {move || if checkbox_checked.get() { "Checked" } else { "Unchecked" }}</Label>
                </div>
            </CardContent>
        </Card>
    }
}

#[component]
fn AttributeBindingTest() -> impl IntoView {
    let (button_variant, set_button_variant) = signal(ButtonVariant::Default);
    let (input_disabled, set_input_disabled) = signal(false);
    let (custom_class, set_custom_class) = signal("bg-blue-500".to_string());
    
    view! {
        <Card class="p-6">
            <CardHeader>
                <CardTitle>"🎨 Attribute Binding Test"</CardTitle>
                <CardDescription>
                    "Testing attribute binding and updates with Leptos v0.8"
                </CardDescription>
            </CardHeader>
            <CardContent class="space-y-4">
                <div class="space-y-2">
                    <Label>"Button Variant Test"</Label>
                    <Button 
                        variant=button_variant
                        on_click=Callback::new(move |_| {
                            let current = button_variant.get();
                            let next = match current {
                                ButtonVariant::Default => ButtonVariant::Destructive,
                                ButtonVariant::Destructive => ButtonVariant::Outline,
                                ButtonVariant::Outline => ButtonVariant::Secondary,
                                ButtonVariant::Secondary => ButtonVariant::Ghost,
                                ButtonVariant::Ghost => ButtonVariant::Link,
                                ButtonVariant::Link => ButtonVariant::Default,
                            };
                            set_button_variant.set(next);
                        })
                    >
                        "Current: " {move || format!("{:?}", button_variant.get())}
                    </Button>
                </div>
                
                <div class="space-y-2">
                    <Label>"Input Disabled State Test"</Label>
                    <div class="flex gap-2 items-center">
                        <Input 
                            disabled=input_disabled
                            placeholder="Disabled state test"
                        />
                        <Button 
                            variant=ButtonVariant::Outline
                            on_click=Callback::new(move |_| set_input_disabled.update(|d| *d = !*d))
                        >
                            {move || if input_disabled.get() { "Enable" } else { "Disable" }}
                        </Button>
                    </div>
                </div>
                
                <div class="space-y-2">
                    <Label>"Dynamic Class Test"</Label>
                    <div class="flex gap-2 items-center">
                        <div 
                            class=move || format!("p-4 rounded {}", custom_class.get())
                        >
                            "Dynamic background color"
                        </div>
                        <Button 
                            variant=ButtonVariant::Secondary
                            on_click=Callback::new(move |_| {
                                let current = custom_class.get();
                                let next = match current.as_str() {
                                    "bg-blue-500" => "bg-red-500",
                                    "bg-red-500" => "bg-green-500",
                                    "bg-green-500" => "bg-yellow-500",
                                    "bg-yellow-500" => "bg-purple-500",
                                    "bg-purple-500" => "bg-blue-500",
                                    _ => "bg-blue-500",
                                };
                                set_custom_class.set(next.to_string());
                            })
                        >
                            "Change Color"
                        </Button>
                    </div>
                </div>
            </CardContent>
        </Card>
    }
}