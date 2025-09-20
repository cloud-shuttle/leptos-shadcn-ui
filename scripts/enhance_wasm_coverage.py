#!/usr/bin/env python3
"""
Enhance WASM test coverage by adding more functional WASM tests to components.
This script identifies components with low WASM coverage and adds comprehensive WASM tests.
"""

import os
import re
import subprocess
from pathlib import Path

# Components that need more WASM tests (based on current coverage analysis)
COMPONENTS_TO_ENHANCE = [
    "accordion", "alert", "alert-dialog", "aspect-ratio", "avatar", "badge",
    "breadcrumb", "calendar", "card", "carousel", "collapsible", "combobox",
    "command", "context-menu", "date-picker", "dialog", "drawer", "dropdown-menu",
    "error-boundary", "form", "hover-card", "input-otp", "label", "lazy-loading",
    "menubar", "navigation-menu", "pagination", "popover", "progress", "radio-group",
    "resizable", "scroll-area", "select", "separator", "sheet", "skeleton",
    "slider", "switch", "table", "tabs", "textarea", "toast", "toggle", "tooltip"
]

# Enhanced WASM test templates for different component types
WASM_TEST_TEMPLATES = {
    "basic": '''
    #[wasm_bindgen_test]
    fn test_{component_name}_dom_rendering() {{
        mount_to_body(|| {{
            view! {{
                <{main_component} class="test-dom-render">
                    "DOM Test {component_name}"
                </{main_component}>
            }}
        }});
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-dom-render").unwrap();
        assert!(element.is_some(), "{component_name} should render in DOM");
        
        let element = element.unwrap();
        assert!(element.text_content().unwrap().contains("DOM Test"), "Content should be rendered");
    }}

    #[wasm_bindgen_test]
    fn test_{component_name}_class_application() {{
        mount_to_body(|| {{
            view! {{
                <{main_component} class="test-class-application custom-class">
                    "Class Test {component_name}"
                </{main_component}>
            }}
        }});
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-class-application").unwrap().unwrap();
        let class_list = element.class_list();
        
        assert!(class_list.contains("test-class-application"), "Base class should be applied");
        assert!(class_list.contains("custom-class"), "Custom class should be applied");
    }}

    #[wasm_bindgen_test]
    fn test_{component_name}_attribute_handling() {{
        mount_to_body(|| {{
            view! {{
                <{main_component} 
                    class="test-attributes"
                    data-test="test-value"
                    aria-label="Test {component_name}"
                >
                    "Attribute Test {component_name}"
                </{main_component}>
            }}
        }});
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-attributes").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("data-test").unwrap(), "test-value");
        assert_eq!(element.get_attribute("aria-label").unwrap(), "Test {component_name}");
    }}''',
    
    "form": '''
    #[wasm_bindgen_test]
    fn test_{component_name}_form_integration() {{
        mount_to_body(|| {{
            view! {{
                <form class="test-form">
                    <{main_component} name="test-field" class="test-form-field">
                        "Form {component_name}"
                    </{main_component}>
                </form>
            }}
        }});
        
        let document = web_sys::window().unwrap().document().unwrap();
        let form = document.query_selector(".test-form").unwrap();
        let field = document.query_selector(".test-form-field").unwrap();
        
        assert!(form.is_some(), "Form should render");
        assert!(field.is_some(), "{component_name} should render in form");
    }}

    #[wasm_bindgen_test]
    fn test_{component_name}_validation_state() {{
        mount_to_body(|| {{
            view! {{
                <{main_component} 
                    class="test-validation" 
                    data-valid="true"
                    data-error="false"
                >
                    "Valid {component_name}"
                </{main_component}>
            }}
        }});
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-validation").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("data-valid").unwrap(), "true");
        assert_eq!(element.get_attribute("data-error").unwrap(), "false");
    }}''',
    
    "interactive": '''
    #[wasm_bindgen_test]
    fn test_{component_name}_click_handling() {{
        let click_count = RwSignal::new(0);
        
        mount_to_body(move || {{
            view! {{
                <{main_component} 
                    class="test-click"
                    on_click=move || click_count.update(|count| *count += 1)
                >
                    "Clickable {component_name}"
                </{main_component}>
            }}
        }});
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-click").unwrap().unwrap();
        
        // Simulate click
        let click_event = web_sys::MouseEvent::new("click").unwrap();
        element.dispatch_event(&click_event).unwrap();
        
        assert_eq!(click_count.get(), 1, "Click should be handled");
    }}

    #[wasm_bindgen_test]
    fn test_{component_name}_focus_behavior() {{
        mount_to_body(|| {{
            view! {{
                <{main_component} 
                    class="test-focus"
                    tabindex="0"
                >
                    "Focusable {component_name}"
                </{main_component}>
            }}
        }});
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-focus").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("tabindex").unwrap(), "0");
        
        // Test focus
        element.focus().unwrap();
        assert_eq!(document.active_element().unwrap(), element);
    }}''',
    
    "layout": '''
    #[wasm_bindgen_test]
    fn test_{component_name}_responsive_behavior() {{
        mount_to_body(|| {{
            view! {{
                <{main_component} 
                    class="test-responsive" 
                    data-responsive="true"
                    style="width: 100%; max-width: 500px;"
                >
                    "Responsive {component_name}"
                </{main_component}>
            }}
        }});
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-responsive").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("data-responsive").unwrap(), "true");
        assert!(element.get_attribute("style").unwrap().contains("width: 100%"));
        assert!(element.get_attribute("style").unwrap().contains("max-width: 500px"));
    }}

    #[wasm_bindgen_test]
    fn test_{component_name}_layout_integration() {{
        mount_to_body(|| {{
            view! {{
                <div class="test-layout-container">
                    <{main_component} class="test-layout-item">
                        "Layout {component_name}"
                    </{main_component}>
                </div>
            }}
        }});
        
        let document = web_sys::window().unwrap().document().unwrap();
        let container = document.query_selector(".test-layout-container").unwrap();
        let item = document.query_selector(".test-layout-item").unwrap();
        
        assert!(container.is_some(), "Container should render");
        assert!(item.is_some(), "{component_name} should render in layout");
    }}'''
}

def get_component_type(component_name):
    """Determine the component type for appropriate test templates"""
    form_components = ["input", "textarea", "select", "checkbox", "radio-group", "form", "input-otp"]
    interactive_components = ["button", "toggle", "switch", "slider", "progress", "pagination", "tabs", "accordion", "collapsible"]
    layout_components = ["card", "sheet", "dialog", "drawer", "popover", "tooltip", "hover-card", "alert", "badge"]
    
    if component_name in form_components:
        return "form"
    elif component_name in interactive_components:
        return "interactive"
    elif component_name in layout_components:
        return "layout"
    else:
        return "basic"

def get_main_component(component_name):
    """Get the main component name for a given component"""
    component_map = {
        "accordion": "Accordion",
        "alert": "Alert",
        "alert-dialog": "AlertDialog",
        "aspect-ratio": "AspectRatio",
        "avatar": "Avatar",
        "badge": "Badge",
        "breadcrumb": "Breadcrumb",
        "calendar": "Calendar",
        "card": "Card",
        "carousel": "Carousel",
        "checkbox": "Checkbox",
        "collapsible": "Collapsible",
        "combobox": "Combobox",
        "command": "Command",
        "context-menu": "ContextMenu",
        "date-picker": "DatePicker",
        "dialog": "Dialog",
        "drawer": "Drawer",
        "dropdown-menu": "DropdownMenu",
        "error-boundary": "ErrorBoundary",
        "form": "Form",
        "hover-card": "HoverCard",
        "input-otp": "InputOTP",
        "label": "Label",
        "lazy-loading": "LazyLoading",
        "menubar": "Menubar",
        "navigation-menu": "NavigationMenu",
        "pagination": "Pagination",
        "popover": "Popover",
        "progress": "Progress",
        "radio-group": "RadioGroup",
        "resizable": "ResizablePanel",
        "scroll-area": "ScrollArea",
        "select": "Select",
        "separator": "Separator",
        "sheet": "Sheet",
        "skeleton": "Skeleton",
        "slider": "Slider",
        "switch": "Switch",
        "table": "Table",
        "tabs": "Tabs",
        "textarea": "Textarea",
        "toast": "Toast",
        "toggle": "Toggle",
        "tooltip": "Tooltip",
    }
    return component_map.get(component_name, component_name.title())

def count_wasm_tests_in_component(component_name):
    """Count current WASM tests in a component"""
    component_dir = f"packages/leptos/{component_name}/src"
    
    if not os.path.exists(component_dir):
        return 0
    
    wasm_count = 0
    for root, dirs, files in os.walk(component_dir):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                try:
                    with open(file_path, 'r') as f:
                        content = f.read()
                    wasm_count += content.count('#[wasm_bindgen_test]')
                except Exception as e:
                    print(f"Error reading {file_path}: {e}")
    
    return wasm_count

def add_wasm_tests_to_component(component_name):
    """Add enhanced WASM tests to a component"""
    test_path = f"packages/leptos/{component_name}/src/real_tests.rs"
    
    if not os.path.exists(test_path):
        print(f"  ⚠️  No real_tests.rs found for {component_name}")
        return False
    
    try:
        with open(test_path, 'r') as f:
            content = f.read()
        
        # Check if component already has enough WASM tests
        current_wasm_count = content.count('#[wasm_bindgen_test]')
        if current_wasm_count >= 8:  # Already has good WASM coverage
            print(f"  ℹ️  {component_name} already has {current_wasm_count} WASM tests")
            return False
        
        # Get component type and main component name
        component_type = get_component_type(component_name)
        main_component = get_main_component(component_name)
        
        # Get the appropriate test template
        test_template = WASM_TEST_TEMPLATES.get(component_type, WASM_TEST_TEMPLATES["basic"])
        new_tests = test_template.format(
            component_name=component_name,
            main_component=main_component
        )
        
        # Add the new tests before the closing brace
        if '}' in content:
            # Find the last closing brace of the module
            last_brace_index = content.rfind('}')
            if last_brace_index != -1:
                # Insert new tests before the last closing brace
                new_content = content[:last_brace_index] + new_tests + '\n' + content[last_brace_index:]
                
                with open(test_path, 'w') as f:
                    f.write(new_content)
                
                print(f"  ✅ Added enhanced WASM tests to {component_name}")
                return True
        
        return False
    except Exception as e:
        print(f"  ❌ Error enhancing {component_name}: {e}")
        return False

def test_compilation(component_name):
    """Test if the component still compiles after adding WASM tests"""
    try:
        result = subprocess.run(
            ['cargo', 'test', '-p', f'leptos-shadcn-{component_name}', '--lib', 'real_tests', '--no-run'],
            capture_output=True,
            text=True,
            cwd='.'
        )
        return result.returncode == 0
    except Exception as e:
        print(f"  Error testing compilation for {component_name}: {e}")
        return False

def main():
    """Main function to enhance WASM test coverage"""
    print("🌐 Enhancing WASM test coverage across all components...")
    print(f"📦 Processing {len(COMPONENTS_TO_ENHANCE)} components")
    
    enhanced_count = 0
    total_components = len(COMPONENTS_TO_ENHANCE)
    
    for component_name in COMPONENTS_TO_ENHANCE:
        print(f"\n🔨 Enhancing WASM tests for {component_name}...")
        
        # Count current WASM tests
        current_wasm_count = count_wasm_tests_in_component(component_name)
        print(f"  📊 Current WASM tests: {current_wasm_count}")
        
        # Add enhanced WASM tests
        if add_wasm_tests_to_component(component_name):
            # Test compilation
            if test_compilation(component_name):
                enhanced_count += 1
                print(f"  ✅ {component_name} enhanced successfully")
            else:
                print(f"  ❌ {component_name} compilation failed after enhancement")
    
    print(f"\n🎉 WASM Enhancement Summary:")
    print(f"✅ Successfully enhanced: {enhanced_count}/{total_components} components")
    print(f"📊 Enhancement rate: {(enhanced_count/total_components)*100:.1f}%")
    
    return 0

if __name__ == "__main__":
    exit(main())
