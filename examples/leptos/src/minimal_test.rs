use leptos::*;
use leptos::prelude::*;
use tailwind_rs_core::Color;

#[component]
pub fn MinimalTest() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-white text-gray-900 p-8">
            <h1 class="text-4xl font-bold mb-8">"Minimal Tailwind-RS-Core Test"</h1>
            
            <div class="space-y-4">
                /* Static classes for comparison */
                <div class="p-4 rounded-lg bg-blue-100">
                    <h2 class="text-xl font-semibold mb-2">"Static Blue Background"</h2>
                    <p class="text-gray-700">
                        "Blue100 Background with static classes"
                    </p>
                </div>
                
                /* Testing tailwind-rs-core Color API */
                <div class="p-4 rounded-lg bg-green-100">
                    <h2 class="text-xl font-semibold mb-2">"Green Background with tailwind-rs-core"</h2>
                    <p class="text-gray-700">
                        "Color::Green: " {Color::Green.to_string()}
                    </p>
                </div>
                
                <div class="p-4 rounded-lg bg-red-100">
                    <h2 class="text-xl font-semibold mb-2">"Red Background with tailwind-rs-core"</h2>
                    <p class="text-red-800">
                        "Color::Red: " {Color::Red.to_string()}
                    </p>
                </div>
                
                <div class="p-4 rounded-lg bg-blue-200">
                    <h2 class="text-xl font-semibold mb-2">"Blue Background with tailwind-rs-core"</h2>
                    <p class="text-blue-800">
                        "Color::Blue: " {Color::Blue.to_string()}
                    </p>
                    <p class="text-blue-600">
                        "This demonstrates that tailwind-rs-core v0.4.0 is working!"
                    </p>
                </div>
            </div>
        </div>
    }
}
