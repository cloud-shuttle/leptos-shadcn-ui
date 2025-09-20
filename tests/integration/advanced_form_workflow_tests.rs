#[cfg(test)]
mod advanced_form_workflow_tests {
    use leptos::prelude::*;
    use wasm_bindgen_test::*;
    use web_sys;
    use crate::default::{Button, Input, Card, CardHeader, CardTitle, CardContent, Badge, Checkbox, RadioGroup, Select, Textarea, Progress};

    wasm_bindgen_test_configure!(run_in_browser);

    #[derive(Debug, Clone, PartialEq)]
    enum FormStep {
        PersonalInfo,
        Preferences,
        Review,
        Confirmation,
    }

    #[derive(Debug, Clone)]
    struct PersonalInfo {
        first_name: String,
        last_name: String,
        email: String,
        phone: String,
    }

    #[derive(Debug, Clone)]
    struct Preferences {
        newsletter: bool,
        notifications: bool,
        theme: String,
        language: String,
    }

    #[wasm_bindgen_test]
    fn test_multi_step_form_workflow() {
        let current_step = RwSignal::new(FormStep::PersonalInfo);
        let personal_info = RwSignal::new(PersonalInfo {
            first_name: String::new(),
            last_name: String::new(),
            email: String::new(),
            phone: String::new(),
        });
        let preferences = RwSignal::new(Preferences {
            newsletter: false,
            notifications: true,
            theme: "light".to_string(),
            language: "en".to_string(),
        });
        let form_errors = RwSignal::new(Vec::<String>::new());
        let is_submitting = RwSignal::new(false);

        let steps = vec![
            ("Personal Info", FormStep::PersonalInfo),
            ("Preferences", FormStep::Preferences),
            ("Review", FormStep::Review),
            ("Confirmation", FormStep::Confirmation),
        ];

        mount_to_body(move || {
            let step_index = steps.iter().position(|(_, step)| *step == current_step.get()).unwrap_or(0);
            let progress = (step_index as f64 / (steps.len() - 1) as f64) * 100.0;

            view! {
                <div class="multi-step-form">
                    <div class="form-header">
                        <h1>"User Registration"</h1>
                        <Progress value=progress />
                        <div class="step-indicator">
                            {for steps.iter().enumerate().map(|(index, (name, step))| {
                                let step = step.clone();
                                let current_step = current_step.clone();
                                let is_active = *step == current_step.get();
                                let is_completed = index < step_index;
                                
                                view! {
                                    <div class="step" class:active=is_active class:completed=is_completed>
                                        <span class="step-number">{index + 1}</span>
                                        <span class="step-name">{name}</span>
                                    </div>
                                }
                            })}
                        </div>
                    </div>

                    <div class="form-content">
                        {match current_step.get() {
                            FormStep::PersonalInfo => view! {
                                <Card>
                                    <CardHeader>
                                        <CardTitle>"Personal Information"</CardTitle>
                                    </CardHeader>
                                    <CardContent>
                                        <div class="form-group">
                                            <Input 
                                                placeholder="First Name" 
                                                value=personal_info.get().first_name.clone()
                                                on_change=Callback::new(move |value| {
                                                    let mut info = personal_info.get();
                                                    info.first_name = value;
                                                    personal_info.set(info);
                                                })
                                            />
                                            <Input 
                                                placeholder="Last Name" 
                                                value=personal_info.get().last_name.clone()
                                                on_change=Callback::new(move |value| {
                                                    let mut info = personal_info.get();
                                                    info.last_name = value;
                                                    personal_info.set(info);
                                                })
                                            />
                                        </div>
                                        <div class="form-group">
                                            <Input 
                                                placeholder="Email" 
                                                value=personal_info.get().email.clone()
                                                on_change=Callback::new(move |value| {
                                                    let mut info = personal_info.get();
                                                    info.email = value;
                                                    personal_info.set(info);
                                                })
                                            />
                                            <Input 
                                                placeholder="Phone" 
                                                value=personal_info.get().phone.clone()
                                                on_change=Callback::new(move |value| {
                                                    let mut info = personal_info.get();
                                                    info.phone = value;
                                                    personal_info.set(info);
                                                })
                                            />
                                        </div>
                                    </CardContent>
                                </Card>
                            },
                            FormStep::Preferences => view! {
                                <Card>
                                    <CardHeader>
                                        <CardTitle>"Preferences"</CardTitle>
                                    </CardHeader>
                                    <CardContent>
                                        <div class="form-group">
                                            <Checkbox 
                                                checked=preferences.get().newsletter
                                                on_change=Callback::new(move |checked| {
                                                    let mut prefs = preferences.get();
                                                    prefs.newsletter = checked;
                                                    preferences.set(prefs);
                                                })
                                            />
                                            <label>"Subscribe to newsletter"</label>
                                        </div>
                                        <div class="form-group">
                                            <Checkbox 
                                                checked=preferences.get().notifications
                                                on_change=Callback::new(move |checked| {
                                                    let mut prefs = preferences.get();
                                                    prefs.notifications = checked;
                                                    preferences.set(prefs);
                                                })
                                            />
                                            <label>"Enable notifications"</label>
                                        </div>
                                        <div class="form-group">
                                            <label>"Theme:"</label>
                                            <RadioGroup 
                                                value=preferences.get().theme.clone()
                                                on_change=Callback::new(move |value| {
                                                    let mut prefs = preferences.get();
                                                    prefs.theme = value;
                                                    preferences.set(prefs);
                                                })
                                            >
                                                <div class="radio-option">
                                                    <input type="radio" value="light" />
                                                    <label>"Light"</label>
                                                </div>
                                                <div class="radio-option">
                                                    <input type="radio" value="dark" />
                                                    <label>"Dark"</label>
                                                </div>
                                            </RadioGroup>
                                        </div>
                                    </CardContent>
                                </Card>
                            },
                            FormStep::Review => view! {
                                <Card>
                                    <CardHeader>
                                        <CardTitle>"Review Information"</CardTitle>
                                    </CardHeader>
                                    <CardContent>
                                        <div class="review-section">
                                            <h3>"Personal Information"</h3>
                                            <p>{format!("Name: {} {}", personal_info.get().first_name, personal_info.get().last_name)}</p>
                                            <p>{format!("Email: {}", personal_info.get().email)}</p>
                                            <p>{format!("Phone: {}", personal_info.get().phone)}</p>
                                        </div>
                                        <div class="review-section">
                                            <h3>"Preferences"</h3>
                                            <p>{format!("Newsletter: {}", if preferences.get().newsletter { "Yes" } else { "No" })}</p>
                                            <p>{format!("Notifications: {}", if preferences.get().notifications { "Yes" } else { "No" })}</p>
                                            <p>{format!("Theme: {}", preferences.get().theme)}</p>
                                        </div>
                                    </CardContent>
                                </Card>
                            },
                            FormStep::Confirmation => view! {
                                <Card>
                                    <CardHeader>
                                        <CardTitle>"Registration Complete!"</CardTitle>
                                    </CardHeader>
                                    <CardContent>
                                        <div class="confirmation-message">
                                            <h3>"Welcome, {}!"</h3>
                                            <p>"Your account has been created successfully."</p>
                                            <p>"You will receive a confirmation email shortly."</p>
                                        </div>
                                    </CardContent>
                                </Card>
                            },
                        }}
                    </div>

                    <div class="form-navigation">
                        {if matches!(current_step.get(), FormStep::PersonalInfo) {
                            view! { <div></div> }
                        } else {
                            view! {
                                <Button 
                                    on_click=Callback::new(move || {
                                        current_step.set(match current_step.get() {
                                            FormStep::Preferences => FormStep::PersonalInfo,
                                            FormStep::Review => FormStep::Preferences,
                                            FormStep::Confirmation => FormStep::Review,
                                            _ => FormStep::PersonalInfo,
                                        });
                                    })
                                >
                                    "Previous"
                                </Button>
                            }
                        }}
                        
                        {if matches!(current_step.get(), FormStep::Confirmation) {
                            view! { <div></div> }
                        } else {
                            view! {
                                <Button 
                                    on_click=Callback::new(move || {
                                        current_step.set(match current_step.get() {
                                            FormStep::PersonalInfo => FormStep::Preferences,
                                            FormStep::Preferences => FormStep::Review,
                                            FormStep::Review => FormStep::Confirmation,
                                            _ => FormStep::Confirmation,
                                        });
                                    })
                                >
                                    "Next"
                                </Button>
                            }
                        }}
                    </div>

                    {if !form_errors.get().is_empty() {
                        view! {
                            <div class="form-errors">
                                {for form_errors.get().iter().map(|error| {
                                    view! {
                                        <div class="error-message">{error.clone()}</div>
                                    }
                                })}
                            </div>
                        }
                    } else {
                        view! { <div></div> }
                    }}
                </div>
            }
        });

        let document = web_sys::window().unwrap().document().unwrap();
        
        // Test form progression
        let next_button = document.query_selector("button").unwrap().unwrap()
            .unchecked_into::<web_sys::HtmlButtonElement>();
        if next_button.text_content().unwrap().contains("Next") {
            next_button.click();
        }

        // Verify step progression
        let step_indicators = document.query_selector_all(".step").unwrap();
        assert!(step_indicators.length() > 0, "Step indicators should be displayed");

        // Test form completion
        let next_buttons = document.query_selector_all("button").unwrap();
        for i in 0..next_buttons.length() {
            let button = next_buttons.item(i).unwrap().unchecked_into::<web_sys::HtmlButtonElement>();
            if button.text_content().unwrap().contains("Next") {
                button.click();
                break;
            }
        }

        // Verify final confirmation
        let confirmation = document.query_selector(".confirmation-message").unwrap();
        assert!(confirmation.is_some(), "Confirmation message should appear after form completion");
    }
}