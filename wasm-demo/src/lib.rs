//! # WASM Demo for Leptos ShadCN UI
//!
//! This demo showcases the new `leptos-shadcn-ui-wasm` package with minimal dependencies
//! and optimized bundle size for WebAssembly environments.

use leptos::prelude::*;
use leptos_shadcn_ui_wasm::prelude::*;
use leptos_shadcn_ui_wasm::{wasm_utils, bundle_utils};
use wasm_bindgen::prelude::*;

/// Initialize the WASM demo
#[wasm_bindgen(start)]
pub fn main() {
    // Initialize WASM-specific features
    wasm_utils::init_wasm();
    
    // Log bundle information
    let bundle_info = bundle_utils::get_bundle_info();
    wasm_utils::log(&format!("🚀 {}", bundle_info));
    
    // Mount the demo app
    mount_to_body(|| view! { <DemoApp /> })
}

/// Main demo application component
#[component]
fn DemoApp() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (input_value, set_input_value) = signal(String::new());
    let (show_alert, set_show_alert) = signal(false);

    view! {
        <div class="min-h-screen bg-gradient-to-br from-slate-50 to-slate-100 p-8">
            <div class="max-w-4xl mx-auto space-y-8">
                // Header
                <div class="text-center space-y-4">
                    <h1 class="text-4xl font-bold text-slate-900">
                        "🚀 Leptos ShadCN UI WASM Demo"
                    </h1>
                    <p class="text-lg text-slate-600">
                        "Minimal dependencies, optimized for WebAssembly"
                    </p>
                    <Badge class="text-sm">
                        {move || format!("Bundle: {}", bundle_utils::get_bundle_info().version)}
                    </Badge>
                </div>

                // Interactive Components Section
                <Card class="p-6">
                    <CardHeader>
                        <CardTitle>"Interactive Components"</CardTitle>
                        <CardDescription>
                            "Test the core WASM-optimized components"
                        </CardDescription>
                    </CardHeader>
                    <CardContent class="space-y-6">
                        // Button Demo
                        <div class="space-y-2">
                            <Label>"Button Component"</Label>
                            <div class="flex gap-2">
                                <Button 
                                    on:click=move |_| set_count.update(|c| *c += 1)
                                    class="bg-blue-600 hover:bg-blue-700"
                                >
                                    "Count: " {count}
                                </Button>
                                <Button 
                                    on:click=move |_| set_count.set(0)
                                    class="border border-slate-300 bg-white hover:bg-slate-50"
                                >
                                    "Reset"
                                </Button>
                                <Button 
                                    on:click=move |_| set_show_alert.set(true)
                                    class="bg-red-600 hover:bg-red-700"
                                >
                                    "Show Alert"
                                </Button>
                            </div>
                        </div>

                        <Separator />

                        // Input Demo
                        <div class="space-y-2">
                            <Label>"Input Component"</Label>
                            <Input
                                placeholder="Type something..."
                                value=input_value
                                on:input=move |ev| set_input_value.set(event_target_value(&ev))
                                class="max-w-md"
                            />
                            <p class="text-sm text-slate-600">
                                "You typed: " <strong>{input_value}</strong>
                            </p>
                        </div>

                        <Separator />

                        // Badge Demo
                        <div class="space-y-2">
                            <Label>"Badge Components"</Label>
                            <div class="flex gap-2 flex-wrap">
                                <Badge>"Default"</Badge>
                                <Badge class="bg-slate-100 text-slate-800">"Secondary"</Badge>
                                <Badge class="bg-red-100 text-red-800">"Destructive"</Badge>
                                <Badge class="border border-slate-300 bg-white">"Outline"</Badge>
                            </div>
                        </div>
                    </CardContent>
                </Card>

                // Layout Components Section
                <Card class="p-6">
                    <CardHeader>
                        <CardTitle>"Layout Components"</CardTitle>
                        <CardDescription>
                            "Cards, separators, and other layout elements"
                        </CardDescription>
                    </CardHeader>
                    <CardContent class="space-y-4">
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                            <Card>
                                <CardHeader>
                                    <CardTitle class="text-lg">"Feature 1"</CardTitle>
                                    <CardDescription>"WASM Optimized"</CardDescription>
                                </CardHeader>
                                <CardContent>
                                    <p class="text-sm text-slate-600">
                                        "Minimal dependencies for fast loading"
                                    </p>
                                </CardContent>
                            </Card>
                            
                            <Card>
                                <CardHeader>
                                    <CardTitle class="text-lg">"Feature 2"</CardTitle>
                                    <CardDescription>"Bundle Size Optimized"</CardDescription>
                                </CardHeader>
                                <CardContent>
                                    <p class="text-sm text-slate-600">
                                        "Smaller bundle size for better performance"
                                    </p>
                                </CardContent>
                            </Card>
                        </div>
                    </CardContent>
                </Card>

                // Performance Section
                <Card class="p-6">
                    <CardHeader>
                        <CardTitle>"Performance Metrics"</CardTitle>
                        <CardDescription>
                            "WASM-specific performance information"
                        </CardDescription>
                    </CardHeader>
                    <CardContent class="space-y-4">
                        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                            <div class="text-center p-4 bg-slate-50 rounded-lg">
                                <div class="text-2xl font-bold text-green-600">
                                    {move || format!("{:.1}ms", wasm_utils::now())}
                                </div>
                                <div class="text-sm text-slate-600">"Load Time"</div>
                            </div>
                            
                            <div class="text-center p-4 bg-slate-50 rounded-lg">
                                <div class="text-2xl font-bold text-blue-600">
                                    {move || bundle_utils::get_enabled_features().len()}
                                </div>
                                <div class="text-sm text-slate-600">"Components"</div>
                            </div>
                            
                            <div class="text-center p-4 bg-slate-50 rounded-lg">
                                <div class="text-2xl font-bold text-purple-600">
                                    {count}
                                </div>
                                <div class="text-sm text-slate-600">"Interactions"</div>
                            </div>
                        </div>
                    </CardContent>
                </Card>

                // Simple Alert
                <Show when=move || show_alert.get()>
                    <Alert class="max-w-md mx-auto">
                        <AlertTitle>"WASM Demo Alert"</AlertTitle>
                        <AlertDescription>
                            "This alert is rendered using the WASM-optimized components!"
                        </AlertDescription>
                        <div class="mt-4">
                            <Button 
                                on:click=move |_| set_show_alert.set(false)
                                class="bg-blue-600 hover:bg-blue-700"
                            >
                                "Close"
                            </Button>
                        </div>
                    </Alert>
                </Show>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_demo_app_renders() {
        // Test that the demo app can be rendered
        let app = view! { <DemoApp /> };
        assert!(app.into_view().is_some());
    }
    
    #[wasm_bindgen_test]
    fn test_bundle_info() {
        let info = bundle_utils::get_bundle_info();
        assert!(!info.version.is_empty());
        assert!(!info.features.is_empty());
    }
}
