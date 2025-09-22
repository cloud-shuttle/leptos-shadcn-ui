use leptos::*;
use leptos_shadcn_ui_wasm::prelude::*;
use tailwind_rs_wasm::*;

fn main() {
    mount_to_body(|| {
        view! {
            <div class="p-4">
                <h1 class="text-2xl font-bold mb-4">"Testing tailwind-rs-wasm Compatibility"</h1>
                
                // Using our WASM package components
                <Button class="mb-4">"ShadCN Button"</Button>
                
                // Using tailwind-rs-wasm for dynamic classes
                <div class={WasmClassBuilder::new()
                    .bg_blue_600()
                    .text_white()
                    .px_4()
                    .py_2()
                    .rounded()
                    .build()}>
                    "Dynamic classes with tailwind-rs-wasm"
                </div>
                
                // Combining both approaches
                <Card class="mt-4">
                    <CardHeader>
                        <CardTitle class={WasmClassBuilder::new()
                            .text_green_600()
                            .font_bold()
                            .build()}>
                            "Combined Approach"
                        </CardTitle>
                    </CardHeader>
                    <CardContent>
                        <p class={WasmClassBuilder::new()
                            .bg_gray_100()
                            .text_gray_800()
                            .p_2()
                            .rounded()
                            .build()}>
                            "ShadCN components + tailwind-rs-wasm classes"
                        </p>
                    </CardContent>
                </Card>
            </div>
        }
    });
}
