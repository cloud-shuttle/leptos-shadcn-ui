use leptos::prelude::*;
use leptos_meta::*;
use console_error_panic_hook::set_once as set_panic_hook;
use wasm_bindgen::prelude::*;

// Import our ShadCN UI components (commented out for debugging)
// use leptos_shadcn_button::{Button, ButtonVariant};
// use leptos_shadcn_card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
// use leptos_shadcn_input::Input;

#[wasm_bindgen(start)]
pub fn main() {
    set_panic_hook();
    
    // Remove the loading screen immediately
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    if let Some(loading_div) = document.get_element_by_id("loading") {
        loading_div.remove();
    }
    
    // Mount the Leptos app to the body
    mount_to_body(|| {
        view! { <App /> }
    });
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let (selected_color, set_selected_color) = signal("blue".to_string());
    let (total_revenue, set_total_revenue) = signal(125000);
    let (new_customers, set_new_customers) = signal(1247);
    let (active_accounts, _set_active_accounts) = signal(8934);
    let (growth_rate, set_growth_rate) = signal(12.5);
    let (input_value, set_input_value) = signal(String::new());

    let change_color = move |color: &str| {
        set_selected_color.set(color.to_string());
    };

    view! {
        <Title text="Leptos ShadCN Dashboard"/>
        <Meta charset="utf-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1"/>
        
        <div class="min-h-screen bg-slate-50 font-sans">
            <div class="max-w-6xl mx-auto p-8">
                // Main heading - matches test expectation
                <h1 class="text-5xl font-bold text-center mb-4 text-slate-800">
                    "Dashboard"
                </h1>
                
                // Subtitle - matches test expectation
                <p class="text-center text-slate-600 mb-12 text-lg">
                    "Welcome to your Leptos ShadCN dashboard"
                </p>
                
                // Color control buttons using basic HTML
                <div class="flex justify-center gap-4 mb-12">
                    <button 
                        class="bg-blue-600 hover:bg-blue-700 text-white font-bold py-3 px-6 rounded-lg transition-colors"
                        on:click=move |_| change_color("blue")
                    >
                        "Blue"
                    </button>
                    <button 
                        class="bg-green-600 hover:bg-green-700 text-white font-bold py-3 px-6 rounded-lg transition-colors"
                        on:click=move |_| change_color("green")
                    >
                        "Green"
                    </button>
                    <button 
                        class="bg-purple-600 hover:bg-purple-700 text-white font-bold py-3 px-6 rounded-lg transition-colors"
                        on:click=move |_| change_color("purple")
                    >
                        "Purple"
                    </button>
                </div>
                
                // Dashboard metrics using basic HTML
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-12">
                    <div class="bg-white p-6 rounded-lg shadow-md">
                        <h3 class="text-sm font-semibold text-slate-500 uppercase tracking-wide mb-2">"Total Revenue"</h3>
                        <p class="text-3xl font-bold text-slate-900">"$"{total_revenue}</p>
                    </div>
                    
                    <div class="bg-white p-6 rounded-lg shadow-md">
                        <h3 class="text-sm font-semibold text-slate-500 uppercase tracking-wide mb-2">"New Customers"</h3>
                        <p class="text-3xl font-bold text-slate-900">{new_customers}</p>
                    </div>
                    
                    <div class="bg-white p-6 rounded-lg shadow-md">
                        <h3 class="text-sm font-semibold text-slate-500 uppercase tracking-wide mb-2">"Active Accounts"</h3>
                        <p class="text-3xl font-bold text-slate-900">{active_accounts}</p>
                    </div>
                    
                    <div class="bg-white p-6 rounded-lg shadow-md">
                        <h3 class="text-sm font-semibold text-slate-500 uppercase tracking-wide mb-2">"Growth Rate"</h3>
                        <p class="text-3xl font-bold text-slate-900">{growth_rate}"%"</p>
                    </div>
                </div>
                
                // Interactive demo section using basic HTML
                <div class="bg-white p-6 rounded-lg shadow-md">
                    <h2 class="text-2xl font-semibold text-slate-900 mb-2">"Interactive Demo"</h2>
                    <p class="text-slate-600 mb-6">"Selected color theme: " <strong>{selected_color}</strong></p>
                    
                    <div class="space-y-4">
                        // Input component demo
                        <div>
                            <label class="block text-sm font-medium text-slate-700 mb-2">"Enter some text:"</label>
                            <input 
                                class="w-full px-3 py-2 border border-slate-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                placeholder="Type something here..."
                                prop:value=input_value
                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    set_input_value.set(value);
                                }
                            />
                            <p class="mt-2 text-sm text-slate-600">"You typed: " {input_value}</p>
                        </div>
                        
                        // Button interactions
                        <div class="flex gap-4 flex-wrap">
                            <button 
                                class="bg-red-600 hover:bg-red-700 text-white font-bold py-2 px-4 rounded transition-colors"
                                on:click=move |_| set_total_revenue.update(|v| *v += 1000)
                            >
                                "Increase Revenue"
                            </button>
                            <button 
                                class="bg-slate-600 hover:bg-slate-700 text-white font-bold py-2 px-4 rounded transition-colors"
                                on:click=move |_| set_new_customers.update(|v| *v += 1)
                            >
                                "Add Customer"
                            </button>
                            <button 
                                class="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded transition-colors"
                                on:click=move |_| set_growth_rate.update(|v| *v += 0.1)
                            >
                                "Boost Growth"
                            </button>
                        </div>
                    </div>
                </div>
                
                // Technical info
                <div class="text-center mt-12 text-slate-600">
                    <p class="mb-2">"Built with Leptos, ShadCN UI, and WebAssembly"</p>
                    <p class="text-sm">"Technology: WASM + Rust + Tailwind-RS + Real ShadCN Components"</p>
                </div>
            </div>
        </div>
    }
}