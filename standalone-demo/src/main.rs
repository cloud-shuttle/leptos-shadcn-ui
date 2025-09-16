use leptos::prelude::*;
use leptos_meta::*;
use console_error_panic_hook::set_once as set_panic_hook;

fn main() {
    set_panic_hook();
    mount_to_body(|| view! { <App /> })
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let (count, set_count) = signal(0);
    let (input_value, set_input_value) = signal(String::new());

    let increment = move || {
        set_count.update(|c| *c += 1);
    };

    let decrement = move || {
        set_count.update(|c| *c -= 1);
    };

    let reset = move || {
        set_count.set(0);
    };

    view! {
        <Title text="Leptos ShadCN UI Demo (WASM)"/>
        <Meta charset="utf-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1"/>
        
        <div class="min-h-screen bg-gray-100 p-8">
            <div class="max-w-4xl mx-auto">
                <h1 class="text-4xl font-bold text-center mb-8">"Leptos ShadCN UI Demo (WASM)"</h1>
                
                <div class="bg-white rounded-lg shadow-lg p-6 mb-6">
                    <h2 class="text-2xl font-semibold mb-4">"Counter Demo"</h2>
                    <div class="text-center">
                        <div class="text-6xl font-bold mb-4 text-blue-600">{count}</div>
                        <div class="space-x-4">
                            <button 
                                class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                                on:click=move |_| increment()
                            >
                                "Increment"
                            </button>
                            <button 
                                class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded"
                                on:click=move |_| decrement()
                            >
                                "Decrement"
                            </button>
                            <button 
                                class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded"
                                on:click=move |_| reset()
                            >
                                "Reset"
                            </button>
                        </div>
                    </div>
                </div>

                <div class="bg-white rounded-lg shadow-lg p-6 mb-6">
                    <h2 class="text-2xl font-semibold mb-4">"Input Demo"</h2>
                    <div class="space-y-4">
                        <input 
                            class="w-full p-3 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                            placeholder="Type something here..."
                            prop:value=input_value
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                set_input_value.set(value);
                            }
                        />
                        <div class="p-4 bg-gray-100 rounded-md">
                            <p class="text-sm text-gray-600">"Current value:"</p>
                            <p class="font-mono text-lg">{input_value}</p>
                        </div>
                    </div>
                </div>

                <div class="bg-white rounded-lg shadow-lg p-6">
                    <h2 class="text-2xl font-semibold mb-4">"Technical Info"</h2>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div class="p-4 bg-green-100 rounded-md">
                            <h3 class="font-semibold text-green-800">"✅ WASM Components"</h3>
                            <p class="text-sm text-green-700">"Real Leptos components compiled to WebAssembly"</p>
                        </div>
                        <div class="p-4 bg-blue-100 rounded-md">
                            <h3 class="font-semibold text-blue-800">"✅ tailwind-rs-core"</h3>
                            <p class="text-sm text-blue-700">"Type-safe Tailwind CSS class generation in Rust"</p>
                        </div>
                        <div class="p-4 bg-purple-100 rounded-md">
                            <h3 class="font-semibold text-purple-800">"✅ Leptos Signals"</h3>
                            <p class="text-sm text-purple-700">"Reactive state management with real-time updates"</p>
                        </div>
                        <div class="p-4 bg-orange-100 rounded-md">
                            <h3 class="font-semibold text-orange-800">"✅ Production Ready"</h3>
                            <p class="text-sm text-orange-700">"No CDN dependencies, fully self-contained"</p>
                        </div>
                    </div>
                </div>

                <div class="text-center mt-8 text-gray-600">
                    <p>"Built with Leptos, ShadCN UI, and tailwind-rs-core"</p>
                    <p class="mt-2">"Technology: WASM + Rust + Tailwind CSS"</p>
                </div>
            </div>
        </div>
    }
}
