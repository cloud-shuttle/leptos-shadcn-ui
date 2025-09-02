use radix_yew_icons::{BellIcon, CheckIcon};
use yew::prelude::*;

use crate::new_york::components::ui::{
    button::Button,
    card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle},
    switch::Switch,
};

struct Notification {
    title: &'static str,
    description: &'static str,
}

fn notifications() -> Vec<Notification> {
    vec![
        Notification {
            title: "Your call has been confirmed.",
            description: "1 hour ago",
        },
        Notification {
            title: "You have a new message!",
            description: "1 hour ago",
        },
        Notification {
            title: "Your subscription is expiring soon!",
            description: "2 hours ago",
        },
    ]
}

#[function_component]
pub fn CardDemo() -> Html {
    html! {
        <Card class="w-[380px]">
            <CardHeader>
                <CardTitle>{"Notifications"}</CardTitle>
                <CardDescription>{"You have 3 unread messages."}</CardDescription>
            </CardHeader>
            <CardContent class="grid gap-4">
                <div class=" flex items-center space-x-4 rounded-md border p-4">
                    <BellIcon />
                    <div class="flex-1 space-y-1">
                        <p class="text-sm font-medium leading-none">
                            {"Push Notifications"}
                        </p>
                        <p class="text-sm text-muted-foreground">
                            {"Send notifications to device."}
                        </p>
                    </div>
                    <Switch />
                </div>
                <div>
                    {notifications().iter().enumerate().map(|(index, notification)| html! {
                        <div
                            key={index}
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
                    }).collect::<Html>()}
                </div>
            </CardContent>
            <CardFooter>
                <Button class="w-full">
                    <CheckIcon />{" Mark all as read"}
                </Button>
            </CardFooter>
        </Card>
    }
}
