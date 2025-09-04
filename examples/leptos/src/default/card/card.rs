use leptos::prelude::*;


use crate::default::components::ui::{
    button::Button,
    card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle},
};

struct Notification {
    id: usize,
    title: &'static str,
    description: &'static str,
}

fn notifications() -> Vec<Notification> {
    vec![
        Notification {
            id: 0,
            title: "Your call has been confirmed.",
            description: "1 hour ago",
        },
        Notification {
            id: 1,
            title: "You have a new message!",
            description: "1 hour ago",
        },
        Notification {
            id: 2,
            title: "Your subscription is expiring soon!",
            description: "2 hours ago",
        },
    ]
}

#[component]
pub fn CardDemo() -> impl IntoView {
    view! {
        <Card class="w-[380px]">
            <CardHeader>
                <CardTitle>{"Notifications"}</CardTitle>
                <CardDescription>{"You have 3 unread messages."}</CardDescription>
            </CardHeader>
            <CardContent class="grid gap-4">
                <div class=" flex items-center space-x-4 rounded-md border p-4">
                    <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M6 8a6 6 0 0 1 12 0c0 7 3 9 3 9H3s3-2 3-9"/>
                        <path d="M10.3 21a1.94 1.94 0 0 0 3.4 0"/>
                    </svg>
                    <div class="flex-1 space-y-1">
                        <p class="text-sm font-medium leading-none">
                            {"Push Notifications"}
                        </p>
                        <p class="text-sm text-muted-foreground">
                            {"Send notifications to device."}
                        </p>
                    </div>
                </div>
                <div>
                    <For
                        each=move || notifications()
                        key=|notification| notification.id
                        children=move |notification: Notification| {
                    view! {
                        <div
                            class="mb-4 grid grid-cols-[25px_1fr] items-start pb-4 last:mb-0 last:pb-0"
                        >
                            <span class="flex h-2 w-2 translate-y-1 rounded-full bg-sky-500" />
                            <div class="space-y-1">
                                <p class="text-sm font-medium leading-none">
                                    {notification.title}
                                </p>
                                <p class="text-sm text-muted-foreground">
                                    {notification.description}
                                </p>
                            </div>
                        </div>
                    }
                }
                    />
                </div>
            </CardContent>
            <CardFooter>
                <Button class="w-full">
                    <svg class="mr-2 h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M20 6 9 17l-5-5"/>
                    </svg>
                    {" Mark all as read"}
                </Button>
            </CardFooter>
        </Card>
    }
}
