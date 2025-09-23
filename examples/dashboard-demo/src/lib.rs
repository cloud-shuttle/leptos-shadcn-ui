use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::Router;

// Import all the ShadCN components we'll need
use leptos_shadcn_button::*;
use leptos_shadcn_card::*;
use leptos_shadcn_input::*;
use leptos_shadcn_badge::*;
use leptos_shadcn_avatar::*;
use leptos_shadcn_dropdown_menu::*;
use leptos_shadcn_separator::*;
use leptos_shadcn_progress::*;
use leptos_shadcn_tabs::*;
use leptos_shadcn_dialog::*;
use leptos_shadcn_sheet::*;
use leptos_shadcn_scroll_area::*;
use leptos_shadcn_collapsible::*;
use leptos_shadcn_command::*;
use leptos_shadcn_popover::*;
use leptos_shadcn_select::*;
use leptos_shadcn_switch::*;
use leptos_shadcn_toast::*;
use leptos_shadcn_tooltip::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html />
        <Title text="Leptos ShadCN Dashboard Demo" />
        <Meta charset="utf-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1" />
        <Link rel="stylesheet" href="https://cdn.tailwindcss.com" />
        
        <Router>
            <DashboardLayout />
        </Router>
    }
}

#[component]
pub fn DashboardLayout() -> impl IntoView {
    let sidebar_open = RwSignal::new(true);
    let theme = RwSignal::new("light".to_string());

    view! {
        <div class="min-h-screen bg-background">
            <div class="flex">
                // Sidebar
                <div class=move || {
                    if sidebar_open.get() {
                        "w-64 bg-card border-r border-border transition-all duration-300"
                    } else {
                        "w-0 bg-card border-r border-border transition-all duration-300 overflow-hidden"
                    }
                }>
                    <Sidebar />
                </div>
                
                // Main content
                <div class="flex-1 flex flex-col">
                    // Header
                    <Header sidebar_open=sidebar_open theme=theme />
                    
                    // Dashboard content
                    <main class="flex-1 p-6">
                        <DashboardContent />
                    </main>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <div class="p-6">
            <div class="flex items-center gap-2 mb-8">
                <div class="w-8 h-8 bg-primary rounded-md flex items-center justify-center">
                    <span class="text-primary-foreground font-bold">L</span>
                </div>
                <div>
                    <h1 class="text-lg font-semibold">Leptos Dashboard</h1>
                    <p class="text-sm text-muted-foreground">Rust/WASM Demo</p>
                </div>
            </div>
            
            <nav class="space-y-2">
                <NavItem icon="🏠" label="Dashboard" active=true />
                <NavItem icon="📊" label="Analytics" active=false />
                <NavItem icon="📁" label="Projects" active=false />
                <NavItem icon="👥" label="Team" active=false />
                <NavItem icon="📄" label="Documents" active=false />
                <NavItem icon="⚙️" label="Settings" active=false />
            </nav>
        </div>
    }
}

#[component]
pub fn NavItem(
    #[prop(into)] icon: String,
    #[prop(into)] label: String,
    #[prop(into)] active: bool,
) -> impl IntoView {
    view! {
        <a 
            href="#" 
            class=move || {
                if active {
                    "flex items-center gap-3 px-3 py-2 rounded-md bg-accent text-accent-foreground"
                } else {
                    "flex items-center gap-3 px-3 py-2 rounded-md hover:bg-accent hover:text-accent-foreground transition-colors"
                }
            }
        >
            <span class="text-lg">{icon}</span>
            <span class="font-medium">{label}</span>
        </a>
    }
}

#[component]
pub fn Header(
    sidebar_open: RwSignal<bool>,
    theme: RwSignal<String>,
) -> impl IntoView {
    view! {
        <header class="border-b border-border bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
            <div class="flex h-16 items-center justify-between px-6">
                <div class="flex items-center gap-4">
                    <Button 
                        variant=ButtonVariant::Ghost
                        size=ButtonSize::Icon
                        on:click=move |_| sidebar_open.set(!sidebar_open.get())
                    >
                        <span class="sr-only">"Toggle sidebar"</span>
                        <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                        </svg>
                    </Button>
                    
                    <div class="flex items-center gap-2">
                        <h2 class="text-xl font-semibold">"Dashboard"</h2>
                        <Badge variant=BadgeVariant::Secondary>"Rust/WASM"</Badge>
                    </div>
                </div>
                
                <div class="flex items-center gap-4">
                    <Button 
                        variant=ButtonVariant::Ghost
                        size=ButtonSize::Icon
                        on:click=move |_| theme.set(if theme.get() == "light" { "dark".to_string() } else { "light".to_string() })
                    >
                        <span class="sr-only">"Toggle theme"</span>
                        <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
                        </svg>
                    </Button>
                    
                    <Button variant=ButtonVariant::Ghost size=ButtonSize::Icon>
                        <Avatar>
                            <AvatarImage src="https://github.com/shadcn.png" />
                            <AvatarFallback>"CN"</AvatarFallback>
                        </Avatar>
                    </Button>
                </div>
            </div>
        </header>
    }
}

#[component]
pub fn DashboardContent() -> impl IntoView {
    view! {
        <div class="space-y-6">
            // Welcome section
            <div class="flex items-center justify-between">
                <div>
                    <h1 class="text-3xl font-bold tracking-tight">"Welcome back!"</h1>
                    <p class="text-muted-foreground">"Here's what's happening with your projects today."</p>
                </div>
                <Button>
                    <svg class="mr-2 h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                    </svg>
                    "New Project"
                </Button>
            </div>
            
            // Stats cards
            <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
                <StatCard 
                    title="Total Revenue" 
                    value="$1,250.00" 
                    change="+12.5%" 
                    change_type="positive"
                    description="Trending up this month"
                />
                <StatCard 
                    title="New Customers" 
                    value="1,234" 
                    change="-20%" 
                    change_type="negative"
                    description="Down 20% this period"
                />
                <StatCard 
                    title="Active Accounts" 
                    value="45,678" 
                    change="+12.5%" 
                    change_type="positive"
                    description="Strong user retention"
                />
                <StatCard 
                    title="Growth Rate" 
                    value="4.5%" 
                    change="+4.5%" 
                    change_type="positive"
                    description="Steady performance increase"
                />
            </div>
            
            // Charts and tables section
            <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-7">
                <div class="col-span-4">
                    <Card>
                        <CardHeader>
                            <CardTitle>"Visitors for the last 6 months"</CardTitle>
                        </CardHeader>
                        <CardContent>
                            <div class="h-[300px] flex items-center justify-center bg-muted/50 rounded-md">
                                <div class="text-center">
                                    <div class="text-4xl mb-2">"📊"</div>
                                    <p class="text-muted-foreground">"Chart would go here"</p>
                                    <p class="text-sm text-muted-foreground">"Built with Rust/WASM"</p>
                                </div>
                            </div>
                        </CardContent>
                    </Card>
                </div>
                
                <div class="col-span-3">
                    <Card>
                        <CardHeader>
                            <CardTitle>"Recent Activity"</CardTitle>
                        </CardHeader>
                        <CardContent>
                            <div class="space-y-4">
                                <ActivityItem 
                                    user="Eddie Lake" 
                                    action="completed" 
                                    item="Cover page" 
                                    time="2 hours ago"
                                />
                                <ActivityItem 
                                    user="Jamik Tashpulatov" 
                                    action="updated" 
                                    item="Technical approach" 
                                    time="4 hours ago"
                                />
                                <ActivityItem 
                                    user="Sarah Wilson" 
                                    action="created" 
                                    item="New project" 
                                    time="6 hours ago"
                                />
                            </div>
                        </CardContent>
                    </Card>
                </div>
            </div>
            
            // Data table
            <Card>
                <CardHeader>
                    <CardTitle>"Project Documents"</CardTitle>
                    <CardDescription>"Manage your project documents and track progress"</CardDescription>
                </CardHeader>
                <CardContent>
                    <div class="rounded-md border">
                        <div class="overflow-x-auto">
                            <table class="w-full">
                                <thead class="bg-muted/50">
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
                                        <td class="px-4 py-3 font-medium">"Cover page"</td>
                                        <td class="px-4 py-3">"Cover page"</td>
                                        <td class="px-4 py-3">
                                            <Badge variant=BadgeVariant::Secondary>"In Process"</Badge>
                                        </td>
                                        <td class="px-4 py-3">"Eddie Lake"</td>
                                        <td class="px-4 py-3">
                                            <Button variant=ButtonVariant::Ghost size=ButtonSize::Sm>
                                                "Open menu"
                                            </Button>
                                        </td>
                                    </tr>
                                    <tr>
                                        <td class="px-4 py-3 font-medium">"Table of contents"</td>
                                        <td class="px-4 py-3">"Table of contents"</td>
                                        <td class="px-4 py-3">
                                            <Badge variant=BadgeVariant::Default>"Done"</Badge>
                                        </td>
                                        <td class="px-4 py-3">"Eddie Lake"</td>
                                        <td class="px-4 py-3">
                                            <Button variant=ButtonVariant::Ghost size=ButtonSize::Sm>
                                                "Open menu"
                                            </Button>
                                        </td>
                                    </tr>
                                    <tr>
                                        <td class="px-4 py-3 font-medium">"Executive summary"</td>
                                        <td class="px-4 py-3">"Narrative"</td>
                                        <td class="px-4 py-3">
                                            <Badge variant=BadgeVariant::Default>"Done"</Badge>
                                        </td>
                                        <td class="px-4 py-3">"Eddie Lake"</td>
                                        <td class="px-4 py-3">
                                            <Button variant=ButtonVariant::Ghost size=ButtonSize::Sm>
                                                "Open menu"
                                            </Button>
                                        </td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>
                    </div>
                </CardContent>
            </Card>
        </div>
    }
}

#[component]
pub fn StatCard(
    #[prop(into)] title: String,
    #[prop(into)] value: String,
    #[prop(into)] change: String,
    #[prop(into)] change_type: String,
    #[prop(into)] description: String,
) -> impl IntoView {
    view! {
        <Card>
            <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle class="text-sm font-medium">{title}</CardTitle>
                <svg class="h-4 w-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1" />
                </svg>
            </CardHeader>
            <CardContent>
                <div class="text-2xl font-bold">{value}</div>
                <p class=move || {
                    if change_type == "positive" {
                        "text-xs text-green-600"
                    } else {
                        "text-xs text-red-600"
                    }
                }>
                    {change}
                </p>
                <p class="text-xs text-muted-foreground">{description}</p>
            </CardContent>
        </Card>
    }
}

#[component]
pub fn ActivityItem(
    #[prop(into)] user: String,
    #[prop(into)] action: String,
    #[prop(into)] item: String,
    #[prop(into)] time: String,
) -> impl IntoView {
    let user_initials = user.chars().take(2).collect::<String>();
    let user_display = user.clone();
    
    view! {
        <div class="flex items-center space-x-4">
            <Avatar class="h-8 w-8">
                <AvatarImage src="https://github.com/shadcn.png" />
                <AvatarFallback>{user_initials}</AvatarFallback>
            </Avatar>
            <div class="flex-1 space-y-1">
                <p class="text-sm font-medium leading-none">
                    {user_display} " " {action} " " {item}
                </p>
                <p class="text-xs text-muted-foreground">{time}</p>
            </div>
        </div>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
