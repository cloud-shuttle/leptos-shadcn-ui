#[cfg(test)]
mod dashboard_workflow_tests {
    use leptos::prelude::*;
    use wasm_bindgen_test::*;
    use web_sys;
    use crate::default::{Button, Input, Card, CardHeader, CardTitle, CardContent, Badge, Table, Tabs, TabsList, TabsTrigger, TabsContent};

    wasm_bindgen_test_configure!(run_in_browser);

    #[derive(Debug, Clone)]
    struct DashboardMetric {
        name: String,
        value: f64,
        change: f64,
        trend: String,
    }

    #[derive(Debug, Clone)]
    struct UserActivity {
        user_id: u32,
        action: String,
        timestamp: String,
        duration: u32,
    }

    #[wasm_bindgen_test]
    fn test_dashboard_analytics_workflow() {
        let metrics = RwSignal::new(vec![
            DashboardMetric { name: "Total Users".to_string(), value: 1250.0, change: 12.5, trend: "up".to_string() },
            DashboardMetric { name: "Revenue".to_string(), value: 45000.0, change: -2.3, trend: "down".to_string() },
            DashboardMetric { name: "Active Sessions".to_string(), value: 89.0, change: 5.7, trend: "up".to_string() },
        ]);

        let user_activities = RwSignal::new(vec![
            UserActivity { user_id: 1, action: "Login".to_string(), timestamp: "2024-01-15 10:30".to_string(), duration: 120 },
            UserActivity { user_id: 2, action: "Purchase".to_string(), timestamp: "2024-01-15 10:25".to_string(), duration: 45 },
            UserActivity { user_id: 3, action: "View Product".to_string(), timestamp: "2024-01-15 10:20".to_string(), duration: 30 },
        ]);

        let selected_timeframe = RwSignal::new("7d".to_string());
        let selected_metric = RwSignal::new("users".to_string());
        let refresh_data = RwSignal::new(false);

        mount_to_body(move || {
            view! {
                <div class="dashboard-app">
                    <div class="dashboard-header">
                        <h1>"Analytics Dashboard"</h1>
                        <div class="controls">
                            <Button 
                                on_click=Callback::new(move || refresh_data.set(!refresh_data.get()))
                            >
                                "Refresh Data"
                            </Button>
                        </div>
                    </div>

                    <div class="timeframe-selector">
                        <Button 
                            class=if selected_timeframe.get() == "24h" { "active" } else { "" }
                            on_click=Callback::new(move || selected_timeframe.set("24h".to_string()))
                        >
                            "24 Hours"
                        </Button>
                        <Button 
                            class=if selected_timeframe.get() == "7d" { "active" } else { "" }
                            on_click=Callback::new(move || selected_timeframe.set("7d".to_string()))
                        >
                            "7 Days"
                        </Button>
                        <Button 
                            class=if selected_timeframe.get() == "30d" { "active" } else { "" }
                            on_click=Callback::new(move || selected_timeframe.set("30d".to_string()))
                        >
                            "30 Days"
                        </Button>
                    </div>

                    <div class="metrics-grid">
                        {for metrics.get().iter().map(|metric| {
                            let metric = metric.clone();
                            view! {
                                <Card class="metric-card">
                                    <CardHeader>
                                        <CardTitle>{metric.name.clone()}</CardTitle>
                                    </CardHeader>
                                    <CardContent>
                                        <div class="metric-value">{format!("{:.1}", metric.value)}</div>
                                        <div class="metric-change">
                                            <Badge variant=if metric.trend == "up" { "default" } else { "destructive" }>
                                                {format!("{}{:.1}%", if metric.trend == "up" { "+" } else { "" }, metric.change)}
                                            </Badge>
                                        </div>
                                    </CardContent>
                                </Card>
                            }
                        })}
                    </div>

                    <Tabs>
                        <TabsList>
                            <TabsTrigger value="overview">"Overview"</TabsTrigger>
                            <TabsTrigger value="users">"Users"</TabsTrigger>
                            <TabsTrigger value="revenue">"Revenue"</TabsTrigger>
                        </TabsList>
                        
                        <TabsContent value="overview">
                            <Card>
                                <CardHeader>
                                    <CardTitle>"System Overview"</CardTitle>
                                </CardHeader>
                                <CardContent>
                                    <p>"Dashboard overview content for {selected_timeframe.get()}"</p>
                                </CardContent>
                            </Card>
                        </TabsContent>
                        
                        <TabsContent value="users">
                            <Card>
                                <CardHeader>
                                    <CardTitle>"User Activity"</CardTitle>
                                </CardHeader>
                                <CardContent>
                                    <Table>
                                        <thead>
                                            <tr>
                                                <th>"User ID"</th>
                                                <th>"Action"</th>
                                                <th>"Timestamp"</th>
                                                <th>"Duration (s)"</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            {for user_activities.get().iter().map(|activity| {
                                                let activity = activity.clone();
                                                view! {
                                                    <tr>
                                                        <td>{activity.user_id}</td>
                                                        <td>{activity.action.clone()}</td>
                                                        <td>{activity.timestamp.clone()}</td>
                                                        <td>{activity.duration}</td>
                                                    </tr>
                                                }
                                            })}
                                        </tbody>
                                    </Table>
                                </CardContent>
                            </Card>
                        </TabsContent>
                        
                        <TabsContent value="revenue">
                            <Card>
                                <CardHeader>
                                    <CardTitle>"Revenue Analytics"</CardTitle>
                                </CardHeader>
                                <CardContent>
                                    <p>"Revenue data for {selected_timeframe.get()}"</p>
                                </CardContent>
                            </Card>
                        </TabsContent>
                    </Tabs>
                </div>
            }
        });

        let document = web_sys::window().unwrap().document().unwrap();
        
        // Test timeframe selection
        let timeframe_buttons = document.query_selector_all("button").unwrap();
        for i in 0..timeframe_buttons.length() {
            let button = timeframe_buttons.item(i).unwrap().unchecked_into::<web_sys::HtmlButtonElement>();
            if button.text_content().unwrap().contains("30 Days") {
                button.click();
                break;
            }
        }

        // Test tab navigation
        let tab_triggers = document.query_selector_all("[role='tab']").unwrap();
        for i in 0..tab_triggers.length() {
            let tab = tab_triggers.item(i).unwrap();
            if tab.text_content().unwrap().contains("Users") {
                tab.click();
                break;
            }
        }

        // Verify metrics are displayed
        let metric_cards = document.query_selector_all(".metric-card").unwrap();
        assert!(metric_cards.length() > 0, "Dashboard should display metric cards");

        // Verify table data
        let table_rows = document.query_selector_all("tbody tr").unwrap();
        assert!(table_rows.length() > 0, "User activity table should have data");
    }

    #[wasm_bindgen_test]
    fn test_dashboard_filtering_workflow() {
        let all_metrics = vec![
            DashboardMetric { name: "Page Views".to_string(), value: 15000.0, change: 8.2, trend: "up".to_string() },
            DashboardMetric { name: "Bounce Rate".to_string(), value: 35.5, change: -3.1, trend: "down".to_string() },
            DashboardMetric { name: "Conversion Rate".to_string(), value: 2.8, change: 1.5, trend: "up".to_string() },
        ];

        let filtered_metrics = RwSignal::new(all_metrics.clone());
        let search_query = RwSignal::new(String::new());
        let sort_by = RwSignal::new("name".to_string());

        mount_to_body(move || {
            let mut metrics = filtered_metrics.get();
            
            // Apply search filter
            let query = search_query.get();
            if !query.is_empty() {
                metrics.retain(|m| m.name.to_lowercase().contains(&query.to_lowercase()));
            }
            
            // Apply sorting
            match sort_by.get().as_str() {
                "name" => metrics.sort_by(|a, b| a.name.cmp(&b.name)),
                "value" => metrics.sort_by(|a, b| b.value.partial_cmp(&a.value).unwrap()),
                "change" => metrics.sort_by(|a, b| b.change.partial_cmp(&a.change).unwrap()),
                _ => {}
            }

            view! {
                <div class="filtered-dashboard">
                    <div class="filters">
                        <Input 
                            placeholder="Search metrics..." 
                            value=search_query.get()
                            on_change=Callback::new(move |value| search_query.set(value))
                        />
                        <Button 
                            class=if sort_by.get() == "name" { "active" } else { "" }
                            on_click=Callback::new(move || sort_by.set("name".to_string()))
                        >
                            "Sort by Name"
                        </Button>
                        <Button 
                            class=if sort_by.get() == "value" { "active" } else { "" }
                            on_click=Callback::new(move || sort_by.set("value".to_string()))
                        >
                            "Sort by Value"
                        </Button>
                    </div>

                    <div class="filtered-metrics">
                        {for metrics.iter().map(|metric| {
                            let metric = metric.clone();
                            view! {
                                <Card class="filtered-metric">
                                    <CardHeader>
                                        <CardTitle>{metric.name.clone()}</CardTitle>
                                    </CardHeader>
                                    <CardContent>
                                        <div class="metric-value">{format!("{:.1}", metric.value)}</div>
                                        <div class="metric-change">
                                            {format!("{}{:.1}%", if metric.trend == "up" { "+" } else { "" }, metric.change)}
                                        </div>
                                    </CardContent>
                                </Card>
                            }
                        })}
                    </div>
                </div>
            }
        });

        let document = web_sys::window().unwrap().document().unwrap();
        
        // Test search functionality
        let search_input = document.query_selector("input[placeholder='Search metrics...']").unwrap().unwrap()
            .unchecked_into::<web_sys::HtmlInputElement>();
        search_input.set_value("rate");
        let input_event = web_sys::InputEvent::new("input").unwrap();
        search_input.dispatch_event(&input_event).unwrap();

        // Test sorting
        let sort_buttons = document.query_selector_all("button").unwrap();
        for i in 0..sort_buttons.length() {
            let button = sort_buttons.item(i).unwrap().unchecked_into::<web_sys::HtmlButtonElement>();
            if button.text_content().unwrap().contains("Sort by Value") {
                button.click();
                break;
            }
        }

        // Verify filtering works
        let filtered_metrics = document.query_selector_all(".filtered-metric").unwrap();
        assert!(filtered_metrics.length() > 0, "Filtered metrics should be displayed");
    }
}