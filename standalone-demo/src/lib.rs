use leptos::prelude::*;
use leptos_meta::*;
use console_error_panic_hook::set_once as set_panic_hook;
use wasm_bindgen::prelude::*;

// Import our ShadCN UI components
use leptos_shadcn_button::{Button, ButtonVariant};
use leptos_shadcn_card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use leptos_shadcn_input::Input;

#[wasm_bindgen]
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

    // Dashboard state
    let (sidebar_open, set_sidebar_open) = signal(false);
    let (total_revenue, set_total_revenue) = signal(125000);
    let (new_customers, set_new_customers) = signal(1234);
    let (active_accounts, _set_active_accounts) = signal(45678);
    let (growth_rate, set_growth_rate) = signal(4.5);
    let (search_query, set_search_query) = signal(String::new());
    let (selected_rows, set_selected_rows) = signal(0);

    // Sample data for the table
    let documents = vec![
        ("Cover page", "Cover page", "In Process", "Target", "Limit", "Eddie Lake"),
        ("Table of contents", "Table of contents", "Done", "Target", "Limit", "Eddie Lake"),
        ("Executive summary", "Narrative", "Done", "Target", "Limit", "Eddie Lake"),
        ("Technical approach", "Narrative", "Done", "Target", "Limit", "Jamik Tashpulatov"),
        ("Design", "Narrative", "In Process", "Target", "Limit", "Jamik Tashpulatov"),
        ("Capabilities", "Narrative", "In Process", "Target", "Limit", "Jamik Tashpulatov"),
        ("Integration with existing systems", "Narrative", "In Process", "Target", "Limit", "Jamik Tashpulatov"),
        ("Innovation and Advantages", "Narrative", "Done", "Target", "Limit", "Reviewer"),
        ("Overview of EMR's Innovative Solutions", "Technical content", "Done", "Target", "Limit", "Reviewer"),
        ("Advanced Algorithms and Machine Learning", "Narrative", "Done", "Target", "Limit", "Reviewer"),
    ];

    view! {
        <Title text="Leptos ShadCN Dashboard"/>
        <Meta charset="utf-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1"/>
        
        <div class="min-h-screen bg-background font-sans">
            // Sidebar
            <div class="hidden md:flex md:w-64 md:flex-col">
                <div class="flex flex-col flex-grow pt-5 overflow-y-auto bg-white border-r border-gray-200">
                    <div class="flex items-center flex-shrink-0 px-4">
                        <h1 class="text-xl font-bold text-gray-900">"Acme Inc."</h1>
                        </div>
                    <div class="mt-5 flex-grow flex flex-col">
                        <nav class="flex-1 px-2 pb-4 space-y-1">
                            <a href="#" class="bg-gray-100 text-gray-900 group flex items-center px-2 py-2 text-sm font-medium rounded-md">
                                <span class="mr-3">"📊"</span>
                                "Dashboard"
                            </a>
                            <a href="#" class="text-gray-600 hover:bg-gray-50 hover:text-gray-900 group flex items-center px-2 py-2 text-sm font-medium rounded-md">
                                <span class="mr-3">"🔄"</span>
                                "Lifecycle"
                            </a>
                            <a href="#" class="text-gray-600 hover:bg-gray-50 hover:text-gray-900 group flex items-center px-2 py-2 text-sm font-medium rounded-md">
                                <span class="mr-3">"📈"</span>
                                "Analytics"
                            </a>
                            <a href="#" class="text-gray-600 hover:bg-gray-50 hover:text-gray-900 group flex items-center px-2 py-2 text-sm font-medium rounded-md">
                                <span class="mr-3">"📁"</span>
                                "Projects"
                            </a>
                            <a href="#" class="text-gray-600 hover:bg-gray-50 hover:text-gray-900 group flex items-center px-2 py-2 text-sm font-medium rounded-md">
                                <span class="mr-3">"👥"</span>
                                "Team"
                            </a>
                        </nav>
                    </div>
                </div>
                        </div>

            // Main content
            <div class="md:pl-64 flex flex-col flex-1">
                // Top navigation
                <div class="sticky top-0 z-10 md:hidden pl-1 pt-1 sm:pl-3 sm:pt-3 bg-gray-100">
                    <Button 
                        variant=ButtonVariant::Outline
                        class="bg-white"
                        on_click=move || set_sidebar_open.update(|v| *v = !*v)
                    >
                        "☰"
                    </Button>
                </div>

                <main class="flex-1">
                    <div class="py-6">
                        <div class="max-w-7xl mx-auto px-4 sm:px-6 md:px-8">
                            // Page header
                            <div class="md:flex md:items-center md:justify-between">
                                <div class="flex-1 min-w-0">
                                    <h2 class="text-2xl font-bold leading-7 text-gray-900 sm:text-3xl sm:truncate">
                                        "Dashboard"
                                    </h2>
                                </div>
                                <div class="mt-4 flex md:mt-0 md:ml-4">
                                    <Button variant=ButtonVariant::Outline class="mr-3">
                                        "View"
                                    </Button>
                                    <Button variant=ButtonVariant::Default>
                                        "Quick Create"
                                    </Button>
                                </div>
                            </div>

                            // Key metrics cards
                            <div class="mt-8 grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-4">
                                <Card class="overflow-hidden">
                                    <CardContent class="p-6">
                                        <div class="flex items-center">
                                            <div class="flex-shrink-0">
                                                <div class="w-8 h-8 bg-green-500 rounded-md flex items-center justify-center">
                                                    <span class="text-white text-sm font-bold">"$"</span>
                                                </div>
                                            </div>
                                            <div class="ml-5 w-0 flex-1">
                                                <dl>
                                                    <dt class="text-sm font-medium text-gray-500 truncate">"Total Revenue"</dt>
                                                    <dd class="flex items-baseline">
                                                        <div class="text-2xl font-semibold text-gray-900">"$"{total_revenue}</div>
                                                        <div class="ml-2 flex items-baseline text-sm font-semibold text-green-600">
                                                            <span>"+12.5%"</span>
                                                        </div>
                                                    </dd>
                                                </dl>
                                            </div>
                                        </div>
                                        <div class="mt-4">
                                            <div class="text-sm text-gray-500">
                                                "Trending up this month"
                                            </div>
                                        </div>
                                    </CardContent>
                                </Card>

                                <Card class="overflow-hidden">
                                    <CardContent class="p-6">
                                        <div class="flex items-center">
                                            <div class="flex-shrink-0">
                                                <div class="w-8 h-8 bg-blue-500 rounded-md flex items-center justify-center">
                                                    <span class="text-white text-sm font-bold">"👥"</span>
                                                </div>
                                            </div>
                                            <div class="ml-5 w-0 flex-1">
                                                <dl>
                                                    <dt class="text-sm font-medium text-gray-500 truncate">"New Customers"</dt>
                                                    <dd class="flex items-baseline">
                                                        <div class="text-2xl font-semibold text-gray-900">{new_customers}</div>
                                                        <div class="ml-2 flex items-baseline text-sm font-semibold text-red-600">
                                                            <span>"-20%"</span>
                                                        </div>
                                                    </dd>
                                                </dl>
                                            </div>
                                        </div>
                                        <div class="mt-4">
                                            <div class="text-sm text-gray-500">
                                                "Down 20% this period"
                                            </div>
                                        </div>
                                    </CardContent>
                                </Card>

                                <Card class="overflow-hidden">
                                    <CardContent class="p-6">
                                        <div class="flex items-center">
                                            <div class="flex-shrink-0">
                                                <div class="w-8 h-8 bg-purple-500 rounded-md flex items-center justify-center">
                                                    <span class="text-white text-sm font-bold">"📊"</span>
                                                </div>
                                            </div>
                                            <div class="ml-5 w-0 flex-1">
                                                <dl>
                                                    <dt class="text-sm font-medium text-gray-500 truncate">"Active Accounts"</dt>
                                                    <dd class="flex items-baseline">
                                                        <div class="text-2xl font-semibold text-gray-900">{active_accounts}</div>
                                                        <div class="ml-2 flex items-baseline text-sm font-semibold text-green-600">
                                                            <span>"+12.5%"</span>
                                                        </div>
                                                    </dd>
                                                </dl>
                                            </div>
                                        </div>
                                        <div class="mt-4">
                                            <div class="text-sm text-gray-500">
                                                "Strong user retention"
                                            </div>
                                        </div>
                                    </CardContent>
                                </Card>

                                <Card class="overflow-hidden">
                                    <CardContent class="p-6">
                                        <div class="flex items-center">
                                            <div class="flex-shrink-0">
                                                <div class="w-8 h-8 bg-orange-500 rounded-md flex items-center justify-center">
                                                    <span class="text-white text-sm font-bold">"📈"</span>
                                                </div>
                                            </div>
                                            <div class="ml-5 w-0 flex-1">
                                                <dl>
                                                    <dt class="text-sm font-medium text-gray-500 truncate">"Growth Rate"</dt>
                                                    <dd class="flex items-baseline">
                                                        <div class="text-2xl font-semibold text-gray-900">{growth_rate}"%"</div>
                                                        <div class="ml-2 flex items-baseline text-sm font-semibold text-green-600">
                                                            <span>"+4.5%"</span>
                                                        </div>
                                                    </dd>
                                                </dl>
                                            </div>
                                        </div>
                                        <div class="mt-4">
                                            <div class="text-sm text-gray-500">
                                                "Steady performance increase"
                                            </div>
                                        </div>
                                    </CardContent>
                                </Card>
                            </div>

                            // Documents section
                            <div class="mt-8">
                                <div class="sm:flex sm:items-center">
                                    <div class="sm:flex-auto">
                                        <h1 class="text-xl font-semibold text-gray-900">"Documents"</h1>
                                        <p class="mt-2 text-sm text-gray-700">
                                            "A list of all documents including their status, target, limit, and reviewer."
                                        </p>
                                    </div>
                                    <div class="mt-4 sm:mt-0 sm:ml-16 sm:flex-none">
                                        <Button variant=ButtonVariant::Default>
                                            "Add Document"
                                        </Button>
                                    </div>
                                </div>

                                // Search and filters
                                <div class="mt-6 flex flex-col sm:flex-row gap-4">
                                    <div class="flex-1">
                                        <Input 
                                            placeholder="Search documents..."
                                            value=search_query
                                            on_change=Callback::new(move |value| set_search_query.set(value))
                                        />
                                    </div>
                                    <div class="flex gap-2">
                                        <Button variant=ButtonVariant::Outline>
                                            "Past Performance"
                                        </Button>
                                        <Button variant=ButtonVariant::Outline>
                                            "Key Personnel"
                                        </Button>
                                        <Button variant=ButtonVariant::Outline>
                                            "Focus Documents"
                                        </Button>
                                    </div>
                                </div>

                                // Data table
                                <Card class="mt-6">
                                    <CardContent class="p-0">
                                        <div class="overflow-x-auto">
                                            <table class="min-w-full divide-y divide-gray-300">
                                                <thead class="bg-gray-50">
                                                    <tr>
                                                        <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                                            <input type="checkbox" class="rounded border-gray-300" />
                                                        </th>
                                                        <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                                            "Header"
                                                        </th>
                                                        <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                                            "Section Type"
                                                        </th>
                                                        <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                                            "Status"
                                                        </th>
                                                        <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                                            "Target"
                                                        </th>
                                                        <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                                            "Limit"
                                                        </th>
                                                        <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                                            "Reviewer"
                                                        </th>
                                                        <th scope="col" class="relative px-6 py-3">
                                                            <span class="sr-only">"Actions"</span>
                                                        </th>
                                                    </tr>
                                                </thead>
                                                <tbody class="bg-white divide-y divide-gray-200">
                                                    {documents.into_iter().enumerate().map(|(index, (header, section_type, status, target, limit, reviewer))| {
                                                        view! {
                                                            <tr class="hover:bg-gray-50">
                                                                <td class="px-6 py-4 whitespace-nowrap">
                                                                    <input type="checkbox" class="rounded border-gray-300" />
                                                                </td>
                                                                <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                                                                    {header}
                                                                </td>
                                                                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                                    {section_type}
                                                                </td>
                                                                <td class="px-6 py-4 whitespace-nowrap">
                                                                    <span class="inline-flex px-2 py-1 text-xs font-semibold rounded-full bg-green-100 text-green-800">
                                                                        {status}
                                                                    </span>
                                                                </td>
                                                                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                                    {target}
                                                                </td>
                                                                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                                    {limit}
                                                                </td>
                                                                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                                    {reviewer}
                                                                </td>
                                                                <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                                                                    <Button variant=ButtonVariant::Outline class="text-indigo-600 hover:text-indigo-900">
                                                                        "Open menu"
                                                                    </Button>
                                                                </td>
                                                            </tr>
                                                        }
                                                    }).collect_view()}
                                                </tbody>
                                            </table>
                                        </div>
                                    </CardContent>
                                </Card>

                                // Pagination
                                <div class="mt-6 flex items-center justify-between">
                                    <div class="text-sm text-gray-700">
                                        "Showing " <span class="font-medium">"1"</span> " to " <span class="font-medium">"10"</span> " of " <span class="font-medium">"68"</span> " results"
                                    </div>
                                    <div class="flex items-center space-x-2">
                                        <Button variant=ButtonVariant::Outline class="px-3 py-1 text-sm">
                                            "Previous"
                                        </Button>
                                        <Button variant=ButtonVariant::Default class="px-3 py-1 text-sm">
                                            "1"
                                        </Button>
                                        <Button variant=ButtonVariant::Outline class="px-3 py-1 text-sm">
                                            "2"
                                        </Button>
                                        <Button variant=ButtonVariant::Outline class="px-3 py-1 text-sm">
                                            "3"
                                        </Button>
                                        <span class="px-3 py-1 text-sm text-gray-500">"..."</span>
                                        <Button variant=ButtonVariant::Outline class="px-3 py-1 text-sm">
                                            "7"
                                        </Button>
                                        <Button variant=ButtonVariant::Outline class="px-3 py-1 text-sm">
                                            "Next"
                                        </Button>
                        </div>
                        </div>
                        </div>
                        </div>
                    </div>
                </main>
            </div>
        </div>
    }
}