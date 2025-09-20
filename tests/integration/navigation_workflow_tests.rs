#[cfg(test)]
mod navigation_workflow_tests {
    use leptos::prelude::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    use leptos_shadcn_button::default::{Button, ButtonVariant};
    use leptos_shadcn_card::default::{Card, CardHeader, CardTitle, CardContent};

    #[wasm_bindgen_test]
    fn test_navigation_workflow_integration() {
        let current_page = RwSignal::new("home".to_string());
        let navigation_history = RwSignal::new(vec!["home".to_string()]);
        
        mount_to_body(move || {
            view! {
                <div class="navigation-workflow-test">
                    <Card>
                        <CardHeader>
                            <CardTitle>"Navigation Workflow Test"</CardTitle>
                        </CardHeader>
                        <CardContent>
                            <nav class="navigation-menu" role="navigation">
                                <Button 
                                    variant=if current_page.get() == "home" { ButtonVariant::Default } else { ButtonVariant::Secondary }
                                    on_click=move || {
                                        current_page.set("home".to_string());
                                        navigation_history.update(|history| history.push("home".to_string()));
                                    }
                                >
                                    "Home"
                                </Button>
                                
                                <Button 
                                    variant=if current_page.get() == "about" { ButtonVariant::Default } else { ButtonVariant::Secondary }
                                    on_click=move || {
                                        current_page.set("about".to_string());
                                        navigation_history.update(|history| history.push("about".to_string()));
                                    }
                                >
                                    "About"
                                </Button>
                                
                                <Button 
                                    variant=if current_page.get() == "contact" { ButtonVariant::Default } else { ButtonVariant::Secondary }
                                    on_click=move || {
                                        current_page.set("contact".to_string());
                                        navigation_history.update(|history| history.push("contact".to_string()));
                                    }
                                >
                                    "Contact"
                                </Button>
                            </nav>
                            
                            <div class="page-content">
                                <h2>{format!("Current Page: {}", current_page.get())}</h2>
                                <p>{format!("Navigation History: {:?}", navigation_history.get())}</p>
                            </div>
                        </CardContent>
                    </Card>
                </div>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        
        // Test navigation
        let buttons = document.query_selector_all("button");
        assert!(buttons.length() >= 3, "Should have at least 3 navigation buttons");
        
        // Click on About button
        let about_button = buttons.get(1).unwrap();
        let click_event = web_sys::MouseEvent::new("click").unwrap();
        about_button.dispatch_event(&click_event).unwrap();
        
        // Verify navigation state
        let page_content = document.query_selector(".page-content").unwrap().unwrap();
        assert!(page_content.text_content().unwrap().contains("Current Page: about"));
        assert!(page_content.text_content().unwrap().contains("Navigation History"));
    }

    #[wasm_bindgen_test]
    fn test_navigation_workflow_accessibility() {
        mount_to_body(|| {
            view! {
                <div class="navigation-accessibility-test">
                    <nav class="main-navigation" role="navigation" aria-label="Main navigation">
                        <Button 
                            aria-current="page"
                            aria-label="Go to home page"
                        >
                            "Home"
                        </Button>
                        
                        <Button 
                            aria-label="Go to about page"
                        >
                            "About"
                        </Button>
                        
                        <Button 
                            aria-label="Go to contact page"
                        >
                            "Contact"
                        </Button>
                    </nav>
                </div>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        
        // Test accessibility attributes
        let nav = document.query_selector("nav").unwrap().unwrap();
        assert_eq!(nav.get_attribute("role").unwrap(), "navigation");
        assert_eq!(nav.get_attribute("aria-label").unwrap(), "Main navigation");
        
        let buttons = document.query_selector_all("button");
        let first_button = buttons.get(0).unwrap();
        assert_eq!(first_button.get_attribute("aria-current").unwrap(), "page");
        assert_eq!(first_button.get_attribute("aria-label").unwrap(), "Go to home page");
    }
}
