use leptos::prelude::*;
use leptos_meta::*;
use console_error_panic_hook::set_once as set_panic_hook;
use wasm_bindgen::prelude::*;
use web_sys;

// Import all the refactored components
use leptos_shadcn_button::{Button, ButtonVariant, ButtonSize};
use leptos_shadcn_card::*;
use leptos_shadcn_input::*;

#[wasm_bindgen(start)]
pub fn main() {
    set_panic_hook();
    mount_to_body(|| view! { <App /> })
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let (is_dark, set_is_dark) = signal(false);
    let (sidebar_open, set_sidebar_open) = signal(true);
    let (count, set_count) = signal(0);
    let (input_value, set_input_value) = signal(String::new());
    let (revenue, set_revenue) = signal(1250.00);
    let (customers, set_customers) = signal(1234);
    let (accounts, set_accounts) = signal(45678);
    let (growth_rate, set_growth_rate) = signal(4.5);
    let (menu_open, set_menu_open) = signal(false);

    let toggle_theme = move |_| {
        set_is_dark.update(|dark| *dark = !*dark);
    };

    let toggle_sidebar = move |_| {
        set_sidebar_open.update(|open| *open = !*open);
    };

    let increment = move |_| {
        set_count.update(|c| *c += 1);
    };

    let decrement = move |_| {
        set_count.update(|c| *c -= 1);
    };

    let reset = move |_| {
        set_count.set(0);
    };

    let update_revenue = move |_| {
        set_revenue.update(|r| *r += 100.0);
    };

    let update_customers = move |_| {
        set_customers.update(|c| *c += 50);
    };

    let update_accounts = move |_| {
        set_accounts.update(|a| *a += 100);
    };

    let update_growth = move |_| {
        set_growth_rate.update(|g| *g += 0.1);
    };

    let toggle_menu = move |_| {
        set_menu_open.update(|open| *open = !*open);
    };

    view! {
        <Title text="Leptos Dashboard - ShadCN UI Demo"/>
        <Meta charset="utf-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1"/>
        <Meta name="description" content="Professional dashboard built with Leptos and ShadCN UI components"/>
        
        <div class=Signal::derive(move || {
            if is_dark.get() {
                "min-h-screen bg-background text-foreground dark".to_string()
            } else {
                "min-h-screen bg-background text-foreground".to_string()
            }
        })>
            <div class="flex h-screen">
                // Sidebar
                {move || if sidebar_open.get() {
                    view! {
                        <div class="w-64 bg-card border-r border-border flex flex-col">
                            <div class="p-6 border-b border-border">
                                <div class="flex items-center gap-2">
                                    <div class="w-8 h-8 bg-primary rounded-md flex items-center justify-center">
                                        <span class="text-primary-foreground font-bold">"L"</span>
                                    </div>
                                    <span class="font-semibold">"Leptos Dashboard"</span>
                    </div>
                </div>

                            <nav class="flex-1 p-4 space-y-2">
                                <a href="#" class="flex items-center gap-3 px-3 py-2 rounded-md bg-primary text-primary-foreground">
                                    <span>"🏠"</span>
                                    "Dashboard"
                                </a>
                                <a href="#" class="flex items-center gap-3 px-3 py-2 rounded-md hover:bg-accent hover:text-accent-foreground">
                                    <span>"📊"</span>
                                    "Analytics"
                                </a>
                                <a href="#" class="flex items-center gap-3 px-3 py-2 rounded-md hover:bg-accent hover:text-accent-foreground">
                                    <span>"📁"</span>
                                    "Projects"
                                </a>
                                <a href="#" class="flex items-center gap-3 px-3 py-2 rounded-md hover:bg-accent hover:text-accent-foreground">
                                    <span>"👥"</span>
                                    "Team"
                                </a>
                                <a href="#" class="flex items-center gap-3 px-3 py-2 rounded-md hover:bg-accent hover:text-accent-foreground">
                                    <span>"📄"</span>
                                    "Documents"
                                </a>
                                <a href="#" class="flex items-center gap-3 px-3 py-2 rounded-md hover:bg-accent hover:text-accent-foreground">
                                    <span>"⚙️"</span>
                                    "Settings"
                                </a>
                            </nav>
                            
                            <div class="p-4 border-t border-border">
                                <div class="flex items-center gap-3">
                                    <div class="w-8 h-8 bg-muted rounded-full flex items-center justify-center">
                                        <span class="text-sm font-medium">"U"</span>
                                    </div>
                                    <div class="flex-1">
                                        <p class="text-sm font-medium">"shadcn"</p>
                                        <p class="text-xs text-muted-foreground">"shadcn@example.com"</p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }.into_any()
                        } else {
                    view! { <div></div> }.into_any()
                }}

                // Main Content
                <div class="flex-1 flex flex-col">
                    // Header
                    <header class="bg-card border-b border-border px-6 py-4">
                        <div class="flex items-center justify-between">
                            <div class="flex items-center gap-4">
                                <Button variant=ButtonVariant::Ghost on:click=toggle_sidebar class="p-2 hover:bg-accent transition-colors">
                                    <span class="text-lg">{move || if sidebar_open.get() { "☰" } else { "☰" }}</span>
                                </Button>
                                <h1 class="text-2xl font-semibold">"Dashboard"</h1>
                            </div>
                            
                            <div class="flex items-center gap-4">
                                <Button variant=ButtonVariant::Ghost on:click=toggle_theme class="flex items-center gap-2">
                                    {move || if is_dark.get() { "🌞" } else { "🌙" }}
                                    <span class="text-sm">{move || if is_dark.get() { "Light" } else { "Dark" }}</span>
                                </Button>
                                <div class="flex items-center gap-2">
                                    <span class="text-sm text-muted-foreground">"CN"</span>
                                </div>
                            </div>
                        </div>
                    </header>

                    // Dashboard Content
                    <main class="flex-1 p-6 bg-background">
                        <div class="space-y-6">
                            // Welcome Section
                            <div class="mb-8">
                                <h2 class="text-4xl font-bold mb-3 tracking-tight">"Welcome back!"</h2>
                                <p class="text-lg text-muted-foreground">"Here's what's happening with your projects today."</p>
                            </div>

                            // Metrics Cards
                            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                                <Card class="p-8 hover:shadow-xl transition-all duration-300 cursor-pointer border-2 hover:border-primary/20" on:click=update_revenue>
                                    <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-4">
                                        <CardTitle class="text-sm font-semibold text-muted-foreground uppercase tracking-wide">"Total Revenue"</CardTitle>
                                        <span class="text-3xl">"💰"</span>
                        </CardHeader>
                                    <CardContent class="space-y-3">
                                        <div class="text-4xl font-bold tracking-tight">"$" {move || format!("{:.2}", revenue.get())}</div>
                                        <div class="flex items-center gap-2">
                                            <span class="text-sm font-medium text-green-600 bg-green-50 px-2 py-1 rounded-full">"+12.5%"</span>
                                            <span class="text-sm text-muted-foreground">"from last month"</span>
                                        </div>
                                        <p class="text-xs text-blue-600 font-medium">"Click to increase!"</p>
                        </CardContent>
                    </Card>

                                <Card class="p-8 hover:shadow-xl transition-all duration-300 cursor-pointer border-2 hover:border-primary/20" on:click=update_customers>
                                    <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-4">
                                        <CardTitle class="text-sm font-semibold text-muted-foreground uppercase tracking-wide">"New Customers"</CardTitle>
                                        <span class="text-3xl">"👥"</span>
                        </CardHeader>
                                    <CardContent class="space-y-3">
                                        <div class="text-4xl font-bold tracking-tight">{customers}</div>
                                        <div class="flex items-center gap-2">
                                            <span class="text-sm font-medium text-red-600 bg-red-50 px-2 py-1 rounded-full">"-20%"</span>
                                            <span class="text-sm text-muted-foreground">"from last period"</span>
                                    </div>
                                        <p class="text-xs text-blue-600 font-medium">"Click to add customers!"</p>
                        </CardContent>
                    </Card>

                                <Card class="p-8 hover:shadow-xl transition-all duration-300 cursor-pointer border-2 hover:border-primary/20" on:click=update_accounts>
                                    <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-4">
                                        <CardTitle class="text-sm font-semibold text-muted-foreground uppercase tracking-wide">"Active Accounts"</CardTitle>
                                        <span class="text-3xl">"📈"</span>
                        </CardHeader>
                                    <CardContent class="space-y-3">
                                        <div class="text-4xl font-bold tracking-tight">{accounts}</div>
                                        <div class="flex items-center gap-2">
                                            <span class="text-sm font-medium text-green-600 bg-green-50 px-2 py-1 rounded-full">"+12.5%"</span>
                                            <span class="text-sm text-muted-foreground">"from last month"</span>
                                        </div>
                                        <p class="text-xs text-blue-600 font-medium">"Click to add accounts!"</p>
                        </CardContent>
                    </Card>

                                <Card class="p-8 hover:shadow-xl transition-all duration-300 cursor-pointer border-2 hover:border-primary/20" on:click=update_growth>
                                    <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-4">
                                        <CardTitle class="text-sm font-semibold text-muted-foreground uppercase tracking-wide">"Growth Rate"</CardTitle>
                                        <span class="text-3xl">"📊"</span>
                                    </CardHeader>
                                    <CardContent class="space-y-3">
                                        <div class="text-4xl font-bold tracking-tight">{move || format!("{:.1}%", growth_rate.get())}</div>
                                        <div class="flex items-center gap-2">
                                            <span class="text-sm font-medium text-green-600 bg-green-50 px-2 py-1 rounded-full">"+4.5%"</span>
                                            <span class="text-sm text-muted-foreground">"from last month"</span>
                                        </div>
                                        <p class="text-xs text-blue-600 font-medium">"Click to boost growth!"</p>
                                    </CardContent>
                                </Card>
                            </div>

                            // Interactive Dashboard Section
                            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                    // Interactive Counter
                    <Card class="p-6">
                        <CardHeader>
                            <CardTitle>"🔢 Interactive Counter"</CardTitle>
                            <CardDescription>"Demonstrating reactive state management"</CardDescription>
                        </CardHeader>
                        <CardContent>
                            <div class="text-center space-y-4">
                                <div class="text-4xl font-bold text-primary">{count}</div>
                                <div class="flex gap-2 justify-center">
                                    <Button on:click=increment>"+"</Button>
                                    <Button on:click=decrement>"-"</Button>
                                    <Button variant=ButtonVariant::Outline on:click=reset>"Reset"</Button>
                                </div>
                            </div>
                        </CardContent>
                    </Card>

                    // Input Component
                    <Card class="p-6">
                        <CardHeader>
                            <CardTitle>"📝 Input Component"</CardTitle>
                            <CardDescription>"Demonstrating form input with reactive state"</CardDescription>
                        </CardHeader>
                        <CardContent>
                            <div class="space-y-4">
                                <Input 
                                    placeholder="Type something..."
                                    prop:value=input_value
                                    on:input=move |ev| {
                                        let value = event_target_value(&ev);
                                        set_input_value.set(value);
                                    }
                                />
                                <p class="text-sm text-muted-foreground">
                                    "Current value: " {input_value}
                                </p>
                            </div>
                        </CardContent>
                    </Card>

                                // Tailwind-RS-WASM Demo
                                <Card class="p-6">
                                    <CardHeader>
                                        <CardTitle>"🎨 Tailwind-RS-WASM Demo"</CardTitle>
                                        <CardDescription>"Dynamic styling with WasmClassBuilder"</CardDescription>
                                    </CardHeader>
                                    <CardContent>
                                        <div class="space-y-4">
                                            // Using ShadCN components
                                            <Button class="mb-4">"ShadCN Button"</Button>
                                            
                                            // Using tailwind-rs-wasm for dynamic styling
                                            <div class="bg-blue-600 text-white px-4 py-2 rounded">
                                                "Dynamic styling with Tailwind CSS"
                                            </div>
                                            
                                            // Combining both approaches
                                            <Card class="border rounded-lg p-4 hover:shadow-md transition-shadow cursor-pointer" on:click=move |_| {
                                                web_sys::console::log_1(&"Best of both worlds clicked!".into());
                                            }>
                                                <CardContent>
                                                    <Button class="w-full">"Best of both worlds!"</Button>
                                                </CardContent>
                                            </Card>
                                        </div>
                                    </CardContent>
                                </Card>
                            </div>

                            // Recent Activity Section
                            <Card class="p-6">
                                <CardHeader>
                                    <CardTitle>"Recent Activity"</CardTitle>
                                    <CardDescription>"Live updates and user interactions"</CardDescription>
                                </CardHeader>
                                <CardContent>
                                    <div class="space-y-4">
                                        <div class="flex items-center gap-3">
                                            <div class="w-8 h-8 bg-blue-100 rounded-full flex items-center justify-center">
                                                <span class="text-blue-600 font-bold text-sm">"Ed"</span>
                                            </div>
                                            <div class="flex-1">
                                                <p class="text-sm font-medium">"Eddie Lake completed Cover page"</p>
                                                <p class="text-xs text-muted-foreground">"2 hours ago"</p>
                                            </div>
                                        </div>
                                        <div class="flex items-center gap-3">
                                            <div class="w-8 h-8 bg-green-100 rounded-full flex items-center justify-center">
                                                <span class="text-green-600 font-bold text-sm">"Ja"</span>
                                            </div>
                                            <div class="flex-1">
                                                <p class="text-sm font-medium">"Jamik Tashpulatov updated Technical approach"</p>
                                                <p class="text-xs text-muted-foreground">"4 hours ago"</p>
                                            </div>
                                        </div>
                                        <div class="flex items-center gap-3">
                                            <div class="w-8 h-8 bg-purple-100 rounded-full flex items-center justify-center">
                                                <span class="text-purple-600 font-bold text-sm">"Sa"</span>
                                            </div>
                                            <div class="flex-1">
                                                <p class="text-sm font-medium">"Sarah Wilson created New project"</p>
                                                <p class="text-xs text-muted-foreground">"6 hours ago"</p>
                                            </div>
                                        </div>
                                    </div>
                                </CardContent>
                            </Card>

                            // Data Table Section
                    <Card class="p-6">
                        <CardHeader>
                                    <CardTitle>"Project Documents"</CardTitle>
                                    <CardDescription>"Manage your project documents and track progress"</CardDescription>
                        </CardHeader>
                        <CardContent>
                                    <div class="rounded-md border">
                                        <div class="overflow-x-auto">
                                            <table class="w-full">
                                                <thead class="bg-muted">
                                                    <tr>
                                                        <th class="px-4 py-3 text-left text-sm font-medium">"Document"</th>
                                                        <th class="px-4 py-3 text-left text-sm font-medium">"Type"</th>
                                                        <th class="px-4 py-3 text-left text-sm font-medium">"Status"</th>
                                                        <th class="px-4 py-3 text-left text-sm font-medium">"Assignee"</th>
                                                        <th class="px-4 py-3 text-left text-sm font-medium">"Actions"</th>
                                                    </tr>
                                                </thead>
                                                <tbody class="divide-y divide-border">
                                                    <tr>
                                                        <td class="px-4 py-3 text-sm">"Cover page"</td>
                                                        <td class="px-4 py-3 text-sm">"Cover page"</td>
                                                        <td class="px-4 py-3 text-sm">
                                                            <span class="inline-flex items-center px-2 py-1 rounded-full text-xs bg-yellow-100 text-yellow-800">
                                                                "In Process"
                                                            </span>
                                                        </td>
                                                        <td class="px-4 py-3 text-sm">"Eddie Lake"</td>
                                                        <td class="px-4 py-3 text-sm">
                                                            <Button variant=ButtonVariant::Ghost size=ButtonSize::Sm on:click=toggle_menu>"Open menu"</Button>
                                                        </td>
                                                    </tr>
                                                    <tr>
                                                        <td class="px-4 py-3 text-sm">"Table of contents"</td>
                                                        <td class="px-4 py-3 text-sm">"Table of contents"</td>
                                                        <td class="px-4 py-3 text-sm">
                                                            <span class="inline-flex items-center px-2 py-1 rounded-full text-xs bg-green-100 text-green-800">
                                                                "Done"
                                                            </span>
                                                        </td>
                                                        <td class="px-4 py-3 text-sm">"Eddie Lake"</td>
                                                        <td class="px-4 py-3 text-sm">
                                                            <Button variant=ButtonVariant::Ghost size=ButtonSize::Sm on:click=toggle_menu>"Open menu"</Button>
                                                        </td>
                                                    </tr>
                                                    <tr>
                                                        <td class="px-4 py-3 text-sm">"Executive summary"</td>
                                                        <td class="px-4 py-3 text-sm">"Narrative"</td>
                                                        <td class="px-4 py-3 text-sm">
                                                            <span class="inline-flex items-center px-2 py-1 rounded-full text-xs bg-green-100 text-green-800">
                                                                "Done"
                                                            </span>
                                                        </td>
                                                        <td class="px-4 py-3 text-sm">"Eddie Lake"</td>
                                                        <td class="px-4 py-3 text-sm">
                                                            <Button variant=ButtonVariant::Ghost size=ButtonSize::Sm on:click=toggle_menu>"Open menu"</Button>
                                                        </td>
                                                    </tr>
                                                </tbody>
                                            </table>
                                </div>
                            </div>
                        </CardContent>
                    </Card>
                </div>
                    </main>
            </div>
        </div>

            // Simple Dropdown Menu
            {move || if menu_open.get() {
            view! {
                    <div class="fixed inset-0 z-50" on:click=move |_| set_menu_open.set(false)>
                        <div class="absolute top-16 right-4 bg-card border border-border rounded-md shadow-lg p-2 min-w-48" on:click=|e| e.stop_propagation()>
                            <div class="space-y-1">
                                <button class="w-full text-left px-3 py-2 text-sm hover:bg-accent rounded" on:click=move |_| {
                                    set_menu_open.set(false);
                                    web_sys::console::log_1(&"Edit clicked".into());
                                }>
                                    "✏️ Edit"
                                </button>
                                <button class="w-full text-left px-3 py-2 text-sm hover:bg-accent rounded" on:click=move |_| {
                                    set_menu_open.set(false);
                                    web_sys::console::log_1(&"Copy clicked".into());
                                }>
                                    "📋 Copy"
                                </button>
                                <button class="w-full text-left px-3 py-2 text-sm hover:bg-accent rounded" on:click=move |_| {
                                    set_menu_open.set(false);
                                    web_sys::console::log_1(&"Delete clicked".into());
                                }>
                                    "🗑️ Delete"
                                </button>
                        </div>
                    </div>
                </div>
            }.into_any()
        } else {
            view! { <div></div> }.into_any()
        }}
        </div>
    }
}