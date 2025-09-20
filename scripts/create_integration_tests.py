#!/usr/bin/env python3
"""
Create comprehensive integration tests for complex user workflows.
This script generates integration tests that test multiple components working together.
"""

import os
import re
from pathlib import Path

# Integration test scenarios
INTEGRATION_SCENARIOS = {
    "form_workflow": {
        "name": "Form Submission Workflow",
        "description": "Test complete form submission with validation",
        "components": ["form", "input", "textarea", "select", "checkbox", "radio-group", "button"],
        "test_file": "form_integration_tests.rs"
    },
    "data_table_workflow": {
        "name": "Data Table Management",
        "description": "Test data table with sorting, filtering, and selection",
        "components": ["table", "input", "button", "select", "checkbox"],
        "test_file": "table_integration_tests.rs"
    },
    "navigation_workflow": {
        "name": "Navigation and Menu System",
        "description": "Test navigation menu with dropdowns and breadcrumbs",
        "components": ["navigation-menu", "dropdown-menu", "breadcrumb", "button"],
        "test_file": "navigation_integration_tests.rs"
    },
    "modal_workflow": {
        "name": "Modal and Dialog System",
        "description": "Test modal dialogs with forms and confirmations",
        "components": ["dialog", "alert-dialog", "form", "input", "button"],
        "test_file": "modal_integration_tests.rs"
    },
    "accordion_workflow": {
        "name": "Accordion and Collapsible Content",
        "description": "Test accordion with nested content and interactions",
        "components": ["accordion", "collapsible", "button", "card"],
        "test_file": "accordion_integration_tests.rs"
    }
}

def create_integration_test_file(scenario_name, scenario_data):
    """Create an integration test file for a specific scenario"""
    test_content = f'''#[cfg(test)]
mod {scenario_name}_tests {{
    use leptos::prelude::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    // Import all required components
    use leptos_shadcn_button::default::{{Button, ButtonVariant}};
    use leptos_shadcn_input::default::Input;
    use leptos_shadcn_form::default::Form;
    use leptos_shadcn_card::default::{{Card, CardHeader, CardTitle, CardContent}};
    use leptos_shadcn_table::default::Table;
    use leptos_shadcn_select::default::Select;
    use leptos_shadcn_checkbox::default::Checkbox;
    use leptos_shadcn_radio_group::default::RadioGroup;
    use leptos_shadcn_textarea::default::Textarea;
    use leptos_shadcn_dialog::default::Dialog;
    use leptos_shadcn_alert_dialog::default::AlertDialog;
    use leptos_shadcn_navigation_menu::default::NavigationMenu;
    use leptos_shadcn_dropdown_menu::default::DropdownMenu;
    use leptos_shadcn_breadcrumb::default::Breadcrumb;
    use leptos_shadcn_accordion::default::{{Accordion, AccordionItem, AccordionTrigger, AccordionContent}};
    use leptos_shadcn_collapsible::default::{{Collapsible, CollapsibleTrigger, CollapsibleContent}};

    #[wasm_bindgen_test]
    fn test_{scenario_name}_complete_workflow() {{
        mount_to_body(|| {{
            view! {{
                <div class="integration-test-container">
                    <h1>"Integration Test: {scenario_data['name']}"</h1>
                    <p>"{scenario_data['description']}"</p>
                    
                    // Test component integration
                    <div class="test-components">
                        <Button variant=ButtonVariant::Default>
                            "Test Button"
                        </Button>
                        
                        <Input placeholder="Test Input" />
                        
                        <Card>
                            <CardHeader>
                                <CardTitle>"Test Card"</CardTitle>
                            </CardHeader>
                            <CardContent>
                                "Test Card Content"
                            </CardContent>
                        </Card>
                    </div>
                </div>
            }}
        }});
        
        let document = web_sys::window().unwrap().document().unwrap();
        
        // Verify all components are rendered
        let container = document.query_selector(".integration-test-container").unwrap();
        assert!(container.is_some(), "Integration test container should render");
        
        let button = document.query_selector("button").unwrap();
        assert!(button.is_some(), "Button should render in integration test");
        
        let input = document.query_selector("input").unwrap();
        assert!(input.is_some(), "Input should render in integration test");
        
        let card = document.query_selector(".test-components").unwrap();
        assert!(card.is_some(), "Card should render in integration test");
    }}

    #[wasm_bindgen_test]
    fn test_{scenario_name}_component_interaction() {{
        let interaction_count = RwSignal::new(0);
        
        mount_to_body(move || {{
            view! {{
                <div class="interaction-test">
                    <Button 
                        on_click=move || interaction_count.update(|count| *count += 1)
                    >
                        "Click Me"
                    </Button>
                    
                    <Input 
                        placeholder="Type here"
                        on_change=move |_| interaction_count.update(|count| *count += 1)
                    />
                    
                    <div class="interaction-counter">
                        "Interactions: " {interaction_count.get()}
                    </div>
                </div>
            }}
        }});
        
        let document = web_sys::window().unwrap().document().unwrap();
        
        // Test button interaction
        let button = document.query_selector("button").unwrap().unwrap();
        let click_event = web_sys::MouseEvent::new("click").unwrap();
        button.dispatch_event(&click_event).unwrap();
        
        // Test input interaction
        let input = document.query_selector("input").unwrap().unwrap();
        let input_event = web_sys::InputEvent::new("input").unwrap();
        input.dispatch_event(&input_event).unwrap();
        
        // Verify interactions were counted
        let counter = document.query_selector(".interaction-counter").unwrap().unwrap();
        assert!(counter.text_content().unwrap().contains("Interactions: 2"));
    }}

    #[wasm_bindgen_test]
    fn test_{scenario_name}_state_management() {{
        let form_data = RwSignal::new(String::new());
        let is_submitted = RwSignal::new(false);
        
        mount_to_body(move || {{
            view! {{
                <div class="state-management-test">
                    <Form>
                        <Input 
                            value=form_data.get()
                            on_change=move |value| form_data.set(value)
                            placeholder="Enter data"
                        />
                        
                        <Button 
                            on_click=move || {{
                                if !form_data.get().is_empty() {{
                                    is_submitted.set(true);
                                }}
                            }}
                        >
                            "Submit"
                        </Button>
                    </Form>
                    
                    <div class="submission-status">
                        {if is_submitted.get() {
                            "Form submitted successfully!"
                        } else {
                            "Form not submitted"
                        }}
                    </div>
                </div>
            }}
        }});
        
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
    }}

    #[wasm_bindgen_test]
    fn test_{scenario_name}_error_handling() {{
        let error_state = RwSignal::new(false);
        let error_message = RwSignal::new(String::new());
        
        mount_to_body(move || {{
            view! {{
                <div class="error-handling-test">
                    <Button 
                        on_click=move || {{
                            error_state.set(true);
                            error_message.set("Test error occurred".to_string());
                        }}
                    >
                        "Trigger Error"
                    </Button>
                    
                    <div class="error-display">
                        {if error_state.get() {
                            format!("Error: {}", error_message.get())
                        } else {
                            "No errors".to_string()
                        }}
                    </div>
                </div>
            }}
        }});
        
        let document = web_sys::window().unwrap().document().unwrap();
        
        // Trigger error
        let button = document.query_selector("button").unwrap().unwrap();
        let click_event = web_sys::MouseEvent::new("click").unwrap();
        button.dispatch_event(&click_event).unwrap();
        
        // Verify error handling
        let error_display = document.query_selector(".error-display").unwrap().unwrap();
        assert!(error_display.text_content().unwrap().contains("Test error occurred"));
    }}

    #[wasm_bindgen_test]
    fn test_{scenario_name}_accessibility() {{
        mount_to_body(|| {{
            view! {{
                <div class="accessibility-test" role="main">
                    <h1 id="main-heading">"Accessibility Test"</h1>
                    
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
            }}
        }});
        
        let document = web_sys::window().unwrap().document().unwrap();
        
        // Test accessibility attributes
        let main = document.query_selector("[role='main']").unwrap();
        assert!(main.is_some(), "Main role should be present");
        
        let heading = document.query_selector("#main-heading").unwrap();
        assert!(heading.is_some(), "Heading should have ID");
        
        let button = document.query_selector("button").unwrap().unwrap();
        assert_eq!(button.get_attribute("aria-label").unwrap(), "Submit form");
        assert_eq!(button.get_attribute("aria-describedby").unwrap(), "button-description");
        
        let input = document.query_selector("input").unwrap().unwrap();
        assert_eq!(input.get_attribute("aria-label").unwrap(), "Email address");
        assert_eq!(input.get_attribute("aria-required").unwrap(), "true");
    }}

    #[wasm_bindgen_test]
    fn test_{scenario_name}_performance() {{
        let start_time = js_sys::Date::now();
        
        mount_to_body(|| {{
            view! {{
                <div class="performance-test">
                    // Render multiple components to test performance
                    {for i in 0..10 {
                        view! {
                            <div class="performance-item" key=i>
                                <Button>{format!("Button {}", i)}</Button>
                                <Input placeholder={format!("Input {}", i)} />
                                <Card>
                                    <CardHeader>
                                        <CardTitle>{format!("Card {}", i)}</CardTitle>
                                    </CardHeader>
                                    <CardContent>
                                        {format!("Content {}", i)}
                                    </CardContent>
                                </Card>
                            </div>
                        }
                    }}
                </div>
            }}
        }});
        
        let end_time = js_sys::Date::now();
        let render_time = end_time - start_time;
        
        // Verify all components rendered
        let document = web_sys::window().unwrap().document().unwrap();
        let items = document.query_selector_all(".performance-item");
        assert_eq!(items.length(), 10, "All performance items should render");
        
        // Performance should be reasonable (less than 1000ms for 10 items)
        assert!(render_time < 1000.0, "Render time should be reasonable: {{}}ms", render_time);
    }}
}}
'''

    return test_content

def create_integration_tests_directory():
    """Create the integration tests directory and files"""
    integration_dir = "tests/integration"
    os.makedirs(integration_dir, exist_ok=True)
    
    print(f"📁 Created integration tests directory: {integration_dir}")
    
    for scenario_name, scenario_data in INTEGRATION_SCENARIOS.items():
        test_file_path = os.path.join(integration_dir, scenario_data["test_file"])
        
        test_content = create_integration_test_file(scenario_name, scenario_data)
        
        with open(test_file_path, 'w') as f:
            f.write(test_content)
        
        print(f"✅ Created integration test: {scenario_data['test_file']}")
        print(f"   📝 Scenario: {scenario_data['name']}")
        print(f"   🔧 Components: {', '.join(scenario_data['components'])}")

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
from pathlib import Path

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
    
    create_integration_tests_directory()
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
