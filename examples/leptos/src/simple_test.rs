use leptos::*;
use tailwind_rs_core::Color;

#[component]
pub fn SimpleTest() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-white text-gray-900 p-8">
            <h1 class="text-4xl font-bold mb-8">"Tailwind-RS-Core v0.3.0 Test"</h1>
            
            <div class="space-y-4">
                <div class="p-4 bg-blue-100 rounded-lg">
                    <h2 class="text-xl font-semibold mb-2">"Color Test"</h2>
                    <p class="text-gray-700">
                        "Testing Color::Blue: " {Color::Blue.to_string()}
                    </p>
                </div>
                
                <div class="p-4 bg-green-100 rounded-lg">
                    <h2 class="text-xl font-semibold mb-2">"Basic Styling"</h2>
                    <p class="text-gray-700">
                        "This should have a green background and proper text styling."
                    </p>
                </div>
                
                <div class="p-4 bg-red-100 rounded-lg">
                    <h2 class="text-xl font-semibold mb-2">"Component Integration"</h2>
                    <p class="text-gray-700">
                        "If you can see this, the basic integration is working!"
                    </p>
                </div>
            </div>
        </div>
    }
}