#!/usr/bin/env python3
"""
Create simple integration tests for complex user workflows.
"""

import os

def create_simple_integration_tests():
    """Create simple integration test files"""
    integration_dir = "tests/integration"
    os.makedirs(integration_dir, exist_ok=True)
    
    print(f"📁 Created integration tests directory: {integration_dir}")
    
    # Create form workflow integration test
    form_test_content = '''#[cfg(test)]
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
'''
    
    form_test_path = os.path.join(integration_dir, "form_workflow_tests.rs")
    with open(form_test_path, 'w') as f:
        f.write(form_test_content)
    print(f"✅ Created form workflow integration test: {form_test_path}")
    
    # Create table workflow integration test
    table_test_content = '''#[cfg(test)]
mod table_workflow_tests {
    use leptos::prelude::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    use leptos_shadcn_table::default::Table;
    use leptos_shadcn_button::default::{Button, ButtonVariant};
    use leptos_shadcn_input::default::Input;
    use leptos_shadcn_card::default::{Card, CardHeader, CardTitle, CardContent};

    #[derive(Debug, Clone, PartialEq)]
    struct TestData {
        id: usize,
        name: String,
        email: String,
        department: String,
    }

    impl TestData {
        fn new(id: usize) -> Self {
            Self {
                id,
                name: format!("User {}", id),
                email: format!("user{}@example.com", id),
                department: match id % 3 {
                    0 => "Engineering".to_string(),
                    1 => "Marketing".to_string(),
                    _ => "Sales".to_string(),
                },
            }
        }
    }

    #[wasm_bindgen_test]
    fn test_table_workflow_integration() {
        let selected_items = RwSignal::new(Vec::<usize>::new());
        let filter_text = RwSignal::new(String::new());
        
        mount_to_body(move || {
            let data = (0..10).map(|i| TestData::new(i)).collect::<Vec<_>>();
            let filtered_data = data.into_iter()
                .filter(|item| item.name.contains(&filter_text.get()))
                .collect::<Vec<_>>();
            
            view! {
                <div class="table-workflow-test">
                    <Card>
                        <CardHeader>
                            <CardTitle>"Table Workflow Test"</CardTitle>
                        </CardHeader>
                        <CardContent>
                            <Input 
                                value=filter_text.get()
                                on_change=move |value| filter_text.set(value)
                                placeholder="Filter by name"
                            />
                            
                            <Table>
                                <thead>
                                    <tr>
                                        <th>"ID"</th>
                                        <th>"Name"</th>
                                        <th>"Email"</th>
                                        <th>"Department"</th>
                                        <th>"Actions"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {filtered_data.into_iter().map(|item| {
                                        let is_selected = selected_items.get().contains(&item.id);
                                        view! {
                                            <tr key=item.id class=if is_selected { "selected" } else { "" }>
                                                <td>{item.id}</td>
                                                <td>{item.name}</td>
                                                <td>{item.email}</td>
                                                <td>{item.department}</td>
                                                <td>
                                                    <Button 
                                                        variant=if is_selected { ButtonVariant::Secondary } else { ButtonVariant::Default }
                                                        on_click=move || {
                                                            let mut items = selected_items.get();
                                                            if items.contains(&item.id) {
                                                                items.retain(|&x| x != item.id);
                                                            } else {
                                                                items.push(item.id);
                                                            }
                                                            selected_items.set(items);
                                                        }
                                                    >
                                                        {if is_selected { "Deselect" } else { "Select" }}
                                                    </Button>
                                                </td>
                                            </tr>
                                        }
                                    }).collect::<Vec<_>>()}
                                </tbody>
                            </Table>
                            
                            <div class="selection-status">
                                "Selected items: " {selected_items.get().len()}
                            </div>
                        </CardContent>
                    </Card>
                </div>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        
        // Test filtering
        let input = document.query_selector("input").unwrap().unwrap();
        let html_input = input.unchecked_into::<web_sys::HtmlInputElement>();
        html_input.set_value("User 1");
        
        // Test selection
        let buttons = document.query_selector_all("button");
        if buttons.length() > 0 {
            let first_button = buttons.get(0).unwrap();
            let click_event = web_sys::MouseEvent::new("click").unwrap();
            first_button.dispatch_event(&click_event).unwrap();
        }
        
        // Verify table functionality
        let table = document.query_selector("table").unwrap();
        assert!(table.is_some(), "Table should render");
        
        let rows = document.query_selector_all("tbody tr");
        assert!(rows.length() > 0, "Table should have rows");
    }

    #[wasm_bindgen_test]
    fn test_table_workflow_performance() {
        let start_time = js_sys::Date::now();
        
        mount_to_body(|| {
            let data = (0..100).map(|i| TestData::new(i)).collect::<Vec<_>>();
            
            view! {
                <div class="table-performance-test">
                    <Table>
                        <thead>
                            <tr>
                                <th>"ID"</th>
                                <th>"Name"</th>
                                <th>"Email"</th>
                                <th>"Department"</th>
                            </tr>
                        </thead>
                        <tbody>
                            {data.into_iter().map(|item| {
                                view! {
                                    <tr key=item.id>
                                        <td>{item.id}</td>
                                        <td>{item.name}</td>
                                        <td>{item.email}</td>
                                        <td>{item.department}</td>
                                    </tr>
                                }
                            }).collect::<Vec<_>>()}
                        </tbody>
                    </Table>
                </div>
            }
        });
        
        let end_time = js_sys::Date::now();
        let render_time = end_time - start_time;
        
        // Verify all rows rendered
        let document = web_sys::window().unwrap().document().unwrap();
        let rows = document.query_selector_all("tbody tr");
        assert_eq!(rows.length(), 100, "All 100 rows should render");
        
        // Performance should be reasonable (less than 500ms for 100 rows)
        assert!(render_time < 500.0, "Render time should be less than 500ms, got {}ms", render_time);
        
        println!("✅ Rendered 100 table rows in {:.2}ms", render_time);
    }
}
'''
    
    table_test_path = os.path.join(integration_dir, "table_workflow_tests.rs")
    with open(table_test_path, 'w') as f:
        f.write(table_test_content)
    print(f"✅ Created table workflow integration test: {table_test_path}")
    
    # Create navigation workflow integration test
    nav_test_content = '''#[cfg(test)]
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
'''
    
    nav_test_path = os.path.join(integration_dir, "navigation_workflow_tests.rs")
    with open(nav_test_path, 'w') as f:
        f.write(nav_test_content)
    print(f"✅ Created navigation workflow integration test: {nav_test_path}")

def create_integration_test_runner():
    """Create a test runner script for integration tests"""
    runner_content = '''#!/usr/bin/env python3
"""
Integration Test Runner
Runs all integration tests and provides comprehensive reporting.
"""

import subprocess
import sys
import os

def run_integration_tests():
    """Run all integration tests"""
    print("🧪 Running Integration Tests...")
    print("=" * 50)
    
    integration_dir = "tests/integration"
    
    if not os.path.exists(integration_dir):
        print("❌ Integration tests directory not found")
        return False
    
    test_files = [f for f in os.listdir(integration_dir) if f.endswith('.rs')]
    
    if not test_files:
        print("❌ No integration test files found")
        return False
    
    print(f"📁 Found {len(test_files)} integration test files:")
    for test_file in test_files:
        print(f"   - {test_file}")
    
    print("\\n🚀 Running integration tests...")
    
    try:
        # Run integration tests
        result = subprocess.run(
            ['cargo', 'test', '--test', 'integration'],
            capture_output=True,
            text=True,
            cwd='.'
        )
        
        if result.returncode == 0:
            print("✅ All integration tests passed!")
            print("\\n📊 Test Results:")
            print(result.stdout)
            return True
        else:
            print("❌ Some integration tests failed!")
            print("\\n📊 Test Results:")
            print(result.stdout)
            print("\\n❌ Errors:")
            print(result.stderr)
            return False
            
    except Exception as e:
        print(f"❌ Error running integration tests: {e}")
        return False

def main():
    """Main function"""
    success = run_integration_tests()
    sys.exit(0 if success else 1)

if __name__ == "__main__":
    main()
'''
    
    runner_path = "scripts/run_integration_tests.py"
    with open(runner_path, 'w') as f:
        f.write(runner_content)
    
    os.chmod(runner_path, 0o755)
    print(f"✅ Created integration test runner: {runner_path}")

def main():
    """Main function to create integration tests"""
    print("🔗 Creating Integration Tests for Complex User Workflows...")
    print("=" * 60)
    
    create_simple_integration_tests()
    create_integration_test_runner()
    
    print("\\n🎉 Integration Tests Created Successfully!")
    print("=" * 60)
    print("📁 Integration tests directory: tests/integration/")
    print("🚀 Test runner: scripts/run_integration_tests.py")
    print("\\n💡 Next steps:")
    print("   1. Run: python3 scripts/run_integration_tests.py")
    print("   2. Review test results and adjust as needed")
    print("   3. Add more complex scenarios as needed")

if __name__ == "__main__":
    main()
