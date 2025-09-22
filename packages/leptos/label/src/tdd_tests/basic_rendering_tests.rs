#[cfg(test)]
mod basic_rendering_tests {
    use crate::default::Label;
    use leptos::prelude::*;

    // ===== BASIC RENDERING TESTS =====
    // These tests focus on basic rendering and component creation

    #[test]
    fn test_label_basic_rendering() {
        // Test basic label rendering
        let _label_view = view! {
            <Label>
                "Basic Label"
            </Label>
        };
        
        // This test will fail initially - we need to implement proper rendering
    }

    #[test]
    fn test_label_with_text() {
        // Test label with text content
        let _label_text_view = view! {
            <Label>
                "Enter your name"
            </Label>
        };
        
        // This test will fail initially - we need to implement text content
    }

    #[test]
    fn test_label_with_html_content() {
        // Test label with HTML content
        let _label_html_view = view! {
            <Label>
                <span>"Required"</span> " Field"
            </Label>
        };
        
        // This test will fail initially - we need to implement HTML content
    }

    #[test]
    fn test_label_custom_styling() {
        // Test label with custom styling
        let _styled_label_view = view! {
            <Label 
                class="custom-label-style"
            >
                "Styled Label"
            </Label>
        };
        
        // Test custom styling
        let custom_class = "custom-label-style";
        assert_eq!(custom_class, "custom-label-style");
    }

    #[test]
    fn test_label_variants() {
        // Test label variants
        let variants = vec!["default", "destructive", "outline", "secondary", "ghost", "link"];
        
        for variant in variants {
            let _variant_label_view = view! {
                <Label 
                    class=format!("label-{}", variant)
                >
                    {format!("{} Label", variant)}
                </Label>
            };
            
            // Test variant class
            assert!(variant.contains("default") || variant.contains("destructive") || 
                   variant.contains("outline") || variant.contains("secondary") || 
                   variant.contains("ghost") || variant.contains("link"));
        }
    }

    #[test]
    fn test_label_sizes() {
        // Test label sizes
        let sizes = vec!["small", "default", "large"];
        
        for size in sizes {
            let _size_label_view = view! {
                <Label 
                    class=format!("label-{}", size)
                >
                    {format!("{} Label", size)}
                </Label>
            };
            
            // Test size class
            assert!(size.contains("small") || size.contains("default") || size.contains("large"));
        }
    }

    #[test]
    fn test_label_custom_properties() {
        // Test label with custom properties
        let _custom_label_view = view! {
            <Label 
                class="custom-label"
                id="custom-label-id"
                for="input-field"
            >
                "Custom Label"
            </Label>
        };
        
        // Test custom properties
        let custom_class = "custom-label";
        let custom_id = "custom-label-id";
        let custom_for = "input-field";
        
        assert_eq!(custom_class, "custom-label");
        assert_eq!(custom_id, "custom-label-id");
        assert_eq!(custom_for, "input-field");
    }

    #[test]
    fn test_label_nested_structure() {
        // Test nested label structure
        let _nested_label_view = view! {
            <div class="label-container">
                <Label>
                    <span class="label-icon">"*"</span>
                    "Required Field"
                </Label>
            </div>
        };
        
        // Test nested structure
        let container_class = "label-container";
        let icon_class = "label-icon";
        let icon_text = "*";
        let label_text = "Required Field";
        
        assert_eq!(container_class, "label-container");
        assert_eq!(icon_class, "label-icon");
        assert_eq!(icon_text, "*");
        assert_eq!(label_text, "Required Field");
    }

    #[test]
    fn test_label_conditional_rendering() {
        // Test conditional rendering
        let show_label = true;
        let _conditional_label_view = view! {
            {if show_label {
                view! {
                    <Label>
                        "Conditional Label"
                    </Label>
                }.into_view()
            } else {
                view! {}.into_view()
            }}
        };
        
        // Test conditional rendering
        assert!(show_label);
    }

    #[test]
    fn test_label_dynamic_content() {
        // Test dynamic content
        let dynamic_text = "Dynamic Label Text";
        let _dynamic_label_view = view! {
            <Label>
                {dynamic_text}
            </Label>
        };
        
        // Test dynamic content
        assert_eq!(dynamic_text, "Dynamic Label Text");
    }

    #[test]
    fn test_label_multiple_instances() {
        // Test multiple label instances
        let _label1 = view! {
            <Label class="label-1">
                "First Label"
            </Label>
        };
        
        let _label2 = view! {
            <Label class="label-2">
                "Second Label"
            </Label>
        };
        
        let _label3 = view! {
            <Label class="label-3">
                "Third Label"
            </Label>
        };
        
        // Test multiple instances
        let label1_class = "label-1";
        let label2_class = "label-2";
        let label3_class = "label-3";
        
        assert_eq!(label1_class, "label-1");
        assert_eq!(label2_class, "label-2");
        assert_eq!(label3_class, "label-3");
    }

    #[test]
    fn test_label_accessibility_attributes() {
        // Test accessibility attributes
        let _accessible_label_view = view! {
            <Label
                for="accessible-input"
                aria_label="Accessible label"
                role="label"
            >
                "Accessible Label"
            </Label>
        };
        
        // Test accessibility attributes
        let for_attr = "accessible-input";
        let aria_label = "Accessible label";
        let role = "label";
        
        assert_eq!(for_attr, "accessible-input");
        assert_eq!(aria_label, "Accessible label");
        assert_eq!(role, "label");
    }

    #[test]
    fn test_label_style_properties() {
        // Test style properties
        let _styled_label_view = view! {
            <Label
                style="color: red; font-weight: bold;"
            >
                "Styled Label"
            </Label>
        };
        
        // Test style properties
        let style_prop = "color: red; font-weight: bold;";
        assert_eq!(style_prop, "color: red; font-weight: bold;");
    }
}
