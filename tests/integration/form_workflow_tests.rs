#[cfg(test)]
mod form_workflow_tests {
    use leptos::prelude::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    use leptos_shadcn_button::default::{Button, ButtonVariant};
    use leptos_shadcn_input::default::Input;
    use leptos_shadcn_card::default::{Card, CardHeader, CardTitle, CardContent};

    #[wasm_bindgen_test]
    fn test_form_workflow_integration() {
        let form_data = RwSignal::new(String::new());
        let is_submitted = RwSignal::new(false);
        
        mount_to_body(move || {
            view! {
                <div class="form-workflow-test">
                    <Card>
                        <CardHeader>
                            <CardTitle>"Form Workflow Test"</CardTitle>
                        </CardHeader>
                        <CardContent>
                            <Input 
                                value=form_data.get()
                                on_change=move |value| form_data.set(value)
                                placeholder="Enter your data"
                            />
                            
                            <Button 
                                on_click=move || {
                                    if !form_data.get().is_empty() {
                                        is_submitted.set(true);
                                    }
                                }
                            >
                                "Submit Form"
                            </Button>
                            
                            <div class="submission-status">
                                {if is_submitted.get() {
                                    "Form submitted successfully!"
                                } else {
                                    "Form not submitted"
                                }}
                            </div>
                        </CardContent>
                    </Card>
                </div>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        
        // Test form submission workflow
        let input = document.query_selector("input").unwrap().unwrap();
        let html_input = input.unchecked_into::<web_sys::HtmlInputElement>();
        html_input.set_value("test data");
        
        let button = document.query_selector("button").unwrap().unwrap();
        let click_event = web_sys::MouseEvent::new("click").unwrap();
        button.dispatch_event(&click_event).unwrap();
        
        // Verify state management
        let status = document.query_selector(".submission-status").unwrap().unwrap();
        assert!(status.text_content().unwrap().contains("submitted successfully"));
    }

    #[wasm_bindgen_test]
    fn test_form_workflow_accessibility() {
        mount_to_body(|| {
            view! {
                <div class="form-accessibility-test" role="main">
                    <h1 id="form-heading">"Form Accessibility Test"</h1>
                    
                    <Button 
                        aria-label="Submit form"
                        aria-describedby="button-description"
                    >
                        "Submit"
                    </Button>
                    
                    <Input 
                        aria-label="Email address"
                        aria-required="true"
                        type="email"
                    />
                    
                    <div id="button-description">
                        "This button submits the form"
                    </div>
                </div>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        
        // Test accessibility attributes
        let main = document.query_selector("[role='main']").unwrap();
        assert!(main.is_some(), "Main role should be present");
        
        let button = document.query_selector("button").unwrap().unwrap();
        assert_eq!(button.get_attribute("aria-label").unwrap(), "Submit form");
        assert_eq!(button.get_attribute("aria-describedby").unwrap(), "button-description");
        
        let input = document.query_selector("input").unwrap().unwrap();
        assert_eq!(input.get_attribute("aria-label").unwrap(), "Email address");
        assert_eq!(input.get_attribute("aria-required").unwrap(), "true");
    }
}
