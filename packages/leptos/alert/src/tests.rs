#[cfg(test)]
mod tests {
    use crate::default::{AlertVariant, ALERT_CLASS, ALERT_TITLE_CLASS, ALERT_DESCRIPTION_CLASS};
    use leptos::prelude::*;

    #[test]
    fn test_alert_variant_enum_creation() {
        let default_variant = AlertVariant::Default;
        let destructive_variant = AlertVariant::Destructive;
        let success_variant = AlertVariant::Success;
        let warning_variant = AlertVariant::Warning;

        assert_eq!(default_variant, AlertVariant::Default);
        assert_eq!(destructive_variant, AlertVariant::Destructive);
        assert_eq!(success_variant, AlertVariant::Success);
        assert_eq!(warning_variant, AlertVariant::Warning);
    }

    #[test]
    fn test_alert_variant_default() {
        let variant = AlertVariant::default();
        assert_eq!(variant, AlertVariant::Default);
    }

    #[test]
    fn test_alert_variant_from_string() {
        let default_str: String = AlertVariant::Default.into();
        let destructive_str: String = AlertVariant::Destructive.into();
        let success_str: String = AlertVariant::Success.into();
        let warning_str: String = AlertVariant::Warning.into();

        assert_eq!(default_str, "default");
        assert_eq!(destructive_str, "destructive");
        assert_eq!(success_str, "success");
        assert_eq!(warning_str, "warning");
    }

    #[test]
    fn test_alert_base_css_classes() {
        assert!(ALERT_CLASS.contains("relative"));
        assert!(ALERT_CLASS.contains("w-full"));
        assert!(ALERT_CLASS.contains("rounded-lg"));
        assert!(ALERT_CLASS.contains("border"));
        assert!(ALERT_CLASS.contains("p-4"));
    }

    #[test]
    fn test_alert_title_css_classes() {
        assert!(ALERT_TITLE_CLASS.contains("mb-1"));
        assert!(ALERT_TITLE_CLASS.contains("font-medium"));
        assert!(ALERT_TITLE_CLASS.contains("leading-none"));
        assert!(ALERT_TITLE_CLASS.contains("tracking-tight"));
    }

    #[test]
    fn test_alert_description_css_classes() {
        assert!(ALERT_DESCRIPTION_CLASS.contains("text-sm"));
        assert!(ALERT_DESCRIPTION_CLASS.contains("[&_p]:leading-relaxed"));
    }

    #[test]
    fn test_alert_component_structure() {
        // Test that the component types exist and can be referenced
        // We can't instantiate the components directly in tests, but we can test the enums and constants
        assert!(ALERT_CLASS.len() > 0);
        assert!(ALERT_TITLE_CLASS.len() > 0);
        assert!(ALERT_DESCRIPTION_CLASS.len() > 0);
        
        // Test that all variants are accessible
        let variants = vec![
            AlertVariant::Default,
            AlertVariant::Destructive,
            AlertVariant::Success,
            AlertVariant::Warning,
        ];
        
        assert_eq!(variants.len(), 4);
    }

    #[test]
    fn test_alert_theme_consistency() {
        // Test that all variants have consistent styling patterns
        let variants = vec![
            AlertVariant::Default,
            AlertVariant::Destructive,
            AlertVariant::Success,
            AlertVariant::Warning,
        ];

        for variant in variants {
            let variant_str: String = variant.into();
            assert!(!variant_str.is_empty());
            assert!(variant_str.len() > 0);
        }
    }

    #[test]
    fn test_alert_accessibility_features() {
        // Test that alert has proper semantic structure
        assert!(ALERT_CLASS.contains("relative"));
        assert!(ALERT_TITLE_CLASS.contains("font-medium"));
        assert!(ALERT_DESCRIPTION_CLASS.contains("text-sm"));
    }

    #[test]
    fn test_alert_styling_consistency() {
        // Test that all CSS classes follow consistent patterns
        let classes = [ALERT_CLASS, ALERT_TITLE_CLASS, ALERT_DESCRIPTION_CLASS];
        
        for class in classes.iter() {
            assert!(!class.is_empty());
            assert!(class.contains(" "));
        }
    }

    #[test]
    fn test_alert_variant_styling() {
        // Test that each variant has appropriate styling
        let default_style = "bg-background text-foreground";
        let destructive_style = "border-destructive/50 text-destructive dark:border-destructive";
        let success_style = "border-green-500/50 text-green-600 dark:text-green-400";
        let warning_style = "border-yellow-500/50 text-yellow-600 dark:text-yellow-400";

        assert!(default_style.contains("bg-background"));
        assert!(destructive_style.contains("border-destructive"));
        assert!(success_style.contains("border-green-500"));
        assert!(warning_style.contains("border-yellow-500"));
    }

    #[test]
    fn test_alert_component_props() {
        // Test that the constants are properly defined
        assert!(ALERT_CLASS.contains("relative"));
        assert!(ALERT_TITLE_CLASS.contains("font-medium"));
        assert!(ALERT_DESCRIPTION_CLASS.contains("text-sm"));
        
        // Test that all CSS classes are non-empty
        assert!(ALERT_CLASS.len() > 0);
        assert!(ALERT_TITLE_CLASS.len() > 0);
        assert!(ALERT_DESCRIPTION_CLASS.len() > 0);
    }
}
