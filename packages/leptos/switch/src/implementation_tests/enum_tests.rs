#[cfg(test)]
mod enum_tests {
    use crate::default::{
        SwitchVariant, SwitchSize, SwitchContextValue
    };
    use leptos::prelude::*;

    // ===== ENUM TESTS =====
    // These tests focus on enum definitions and conversions

    #[test]
    fn test_switch_variant_enum() {
        // Test SwitchVariant enum variants
        let default_variant = SwitchVariant::Default;
        let destructive_variant = SwitchVariant::Destructive;
        let outline_variant = SwitchVariant::Outline;
        let secondary_variant = SwitchVariant::Secondary;
        let ghost_variant = SwitchVariant::Ghost;
        let link_variant = SwitchVariant::Link;
        
        // Test enum variants exist
        assert_eq!(format!("{:?}", default_variant), "Default");
        assert_eq!(format!("{:?}", destructive_variant), "Destructive");
        assert_eq!(format!("{:?}", outline_variant), "Outline");
        assert_eq!(format!("{:?}", secondary_variant), "Secondary");
        assert_eq!(format!("{:?}", ghost_variant), "Ghost");
        assert_eq!(format!("{:?}", link_variant), "Link");
    }

    #[test]
    fn test_switch_variant_default() {
        // Test SwitchVariant default value
        let default_variant = SwitchVariant::default();
        assert_eq!(default_variant, SwitchVariant::Default);
    }

    #[test]
    fn test_switch_variant_from_string() {
        // Test SwitchVariant from string conversion
        let default_from_str = SwitchVariant::from("default");
        let destructive_from_str = SwitchVariant::from("destructive");
        let outline_from_str = SwitchVariant::from("outline");
        
        assert_eq!(default_from_str, SwitchVariant::Default);
        assert_eq!(destructive_from_str, SwitchVariant::Destructive);
        assert_eq!(outline_from_str, SwitchVariant::Outline);
    }

    #[test]
    fn test_switch_size_enum() {
        // Test SwitchSize enum variants
        let small_size = SwitchSize::Small;
        let default_size = SwitchSize::Default;
        let large_size = SwitchSize::Large;
        
        // Test enum variants exist
        assert_eq!(format!("{:?}", small_size), "Small");
        assert_eq!(format!("{:?}", default_size), "Default");
        assert_eq!(format!("{:?}", large_size), "Large");
    }

    #[test]
    fn test_switch_size_default() {
        // Test SwitchSize default value
        let default_size = SwitchSize::default();
        assert_eq!(default_size, SwitchSize::Default);
    }

    #[test]
    fn test_switch_size_from_string() {
        // Test SwitchSize from string conversion
        let small_from_str = SwitchSize::from("small");
        let default_from_str = SwitchSize::from("default");
        let large_from_str = SwitchSize::from("large");
        
        assert_eq!(small_from_str, SwitchSize::Small);
        assert_eq!(default_from_str, SwitchSize::Default);
        assert_eq!(large_from_str, SwitchSize::Large);
    }

    #[test]
    fn test_switch_context_value() {
        // Test SwitchContextValue structure
        let context_value = SwitchContextValue {
            checked: RwSignal::new(false),
            disabled: RwSignal::new(false),
            on_checked_change: None,
        };
        
        // Test context value creation
        assert!(!context_value.checked.get());
        assert!(!context_value.disabled.get());
        assert!(context_value.on_checked_change.is_none());
    }

    #[test]
    fn test_switch_variant_equality() {
        // Test SwitchVariant equality
        let variant1 = SwitchVariant::Default;
        let variant2 = SwitchVariant::Default;
        let variant3 = SwitchVariant::Destructive;
        
        assert_eq!(variant1, variant2);
        assert_ne!(variant1, variant3);
    }

    #[test]
    fn test_switch_size_equality() {
        // Test SwitchSize equality
        let size1 = SwitchSize::Default;
        let size2 = SwitchSize::Default;
        let size3 = SwitchSize::Large;
        
        assert_eq!(size1, size2);
        assert_ne!(size1, size3);
    }

    #[test]
    fn test_switch_variant_clone() {
        // Test SwitchVariant cloning
        let original = SwitchVariant::Destructive;
        let cloned = original.clone();
        
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_switch_size_clone() {
        // Test SwitchSize cloning
        let original = SwitchSize::Large;
        let cloned = original.clone();
        
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_switch_variant_debug() {
        // Test SwitchVariant debug formatting
        let variant = SwitchVariant::Outline;
        let debug_str = format!("{:?}", variant);
        
        assert!(debug_str.contains("Outline"));
    }

    #[test]
    fn test_switch_size_debug() {
        // Test SwitchSize debug formatting
        let size = SwitchSize::Small;
        let debug_str = format!("{:?}", size);
        
        assert!(debug_str.contains("Small"));
    }

    #[test]
    fn test_switch_variant_from_str() {
        // Test SwitchVariant from string parsing
        let variant = SwitchVariant::from("secondary");
        assert_eq!(variant, SwitchVariant::Secondary);
        
        let variant = SwitchVariant::from("ghost");
        assert_eq!(variant, SwitchVariant::Ghost);
        
        let variant = SwitchVariant::from("link");
        assert_eq!(variant, SwitchVariant::Link);
    }

    #[test]
    fn test_switch_size_from_str() {
        // Test SwitchSize from string parsing
        let size = SwitchSize::from("small");
        assert_eq!(size, SwitchSize::Small);
        
        let size = SwitchSize::from("default");
        assert_eq!(size, SwitchSize::Default);
        
        let size = SwitchSize::from("large");
        assert_eq!(size, SwitchSize::Large);
    }

    #[test]
    fn test_switch_variant_combinations() {
        // Test SwitchVariant combinations
        let variants = vec![
            SwitchVariant::Default,
            SwitchVariant::Destructive,
            SwitchVariant::Outline,
            SwitchVariant::Secondary,
            SwitchVariant::Ghost,
            SwitchVariant::Link,
        ];
        
        // Test all variants are unique
        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    #[test]
    fn test_switch_size_combinations() {
        // Test SwitchSize combinations
        let sizes = vec![
            SwitchSize::Small,
            SwitchSize::Default,
            SwitchSize::Large,
        ];
        
        // Test all sizes are unique
        for i in 0..sizes.len() {
            for j in (i + 1)..sizes.len() {
                assert_ne!(sizes[i], sizes[j]);
            }
        }
    }
}
