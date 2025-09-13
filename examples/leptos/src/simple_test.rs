use leptos::*;
use leptos::prelude::*;

#[component]
pub fn SimpleTest() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gradient-to-br from-blue-50 to-purple-50 p-8">
            <div class="max-w-4xl mx-auto">
                <h1 class="text-4xl font-bold text-center mb-8 text-blue-800">
                    "🚀 Simple WASM Test"
                </h1>
                <div class="bg-white rounded-lg shadow-lg p-6">
                    <h2 class="text-2xl font-semibold mb-4 text-gray-800">
                        "This is a simple test component"
                    </h2>
                    <p class="text-gray-600 mb-4">
                        "If you can see this, WASM rendering is working!"
                    </p>
                    <div class="bg-blue-100 p-4 rounded-lg">
                        <p class="text-blue-800 font-medium">
                            "✅ WASM is rendering correctly"
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}

