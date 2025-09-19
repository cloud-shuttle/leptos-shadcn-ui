#[cfg(test)]
mod theme_tests {
    use crate::*;
    use std::collections::HashMap;

    #[test]
    fn test_theme_enum_variants() {
        // Test Theme enum variants
        let default_theme = Theme::Default;
        let dark_theme = Theme::Dark;
        let light_theme = Theme::Light;
        
        // Test custom theme
        let mut custom_props = HashMap::new();
        custom_props.insert("primary".to_string(), "#3b82f6".to_string());
        custom_props.insert("secondary".to_string(), "#64748b".to_string());
        let custom_theme = Theme::Custom(custom_props);
        
        // Test theme equality
        assert_eq!(default_theme, Theme::Default);
        assert_eq!(dark_theme, Theme::Dark);
        assert_eq!(light_theme, Theme::Light);
        
        // Test custom theme properties
        if let Theme::Custom(props) = custom_theme {
            assert_eq!(props.get("primary"), Some(&"#3b82f6".to_string()));
            assert_eq!(props.get("secondary"), Some(&"#64748b".to_string()));
        }
    }

    #[test]
    fn test_theme_default_implementation() {
        // Test Theme default implementation
        let default_theme = Theme::default();
        assert_eq!(default_theme, Theme::Default);
    }

    #[test]
    fn test_variant_enum_variants() {
        // Test Variant enum variants
        let primary_variant = Variant::Primary;
        let secondary_variant = Variant::Secondary;
        let destructive_variant = Variant::Destructive;
        let outline_variant = Variant::Outline;
        let ghost_variant = Variant::Ghost;
        let link_variant = Variant::Link;
        
        // Test variant equality
        assert_eq!(primary_variant, Variant::Primary);
        assert_eq!(secondary_variant, Variant::Secondary);
        assert_eq!(destructive_variant, Variant::Destructive);
        assert_eq!(outline_variant, Variant::Outline);
        assert_eq!(ghost_variant, Variant::Ghost);
        assert_eq!(link_variant, Variant::Link);
    }

    #[test]
    fn test_variant_default_implementation() {
        // Test Variant default implementation
        let default_variant = Variant::default();
        assert_eq!(default_variant, Variant::Primary);
    }

    #[test]
    fn test_size_enum_variants() {
        // Test Size enum variants
        let small_size = Size::Small;
        let medium_size = Size::Medium;
        let large_size = Size::Large;
        
        // Test size equality
        assert_eq!(small_size, Size::Small);
        assert_eq!(medium_size, Size::Medium);
        assert_eq!(large_size, Size::Large);
    }

    #[test]
    fn test_size_default_implementation() {
        // Test Size default implementation
        let default_size = Size::default();
        assert_eq!(default_size, Size::Medium);
    }

    #[test]
    fn test_responsive_config_creation() {
        // Test ResponsiveConfig creation
        let responsive_config = ResponsiveConfig {
            sm: Some("sm:text-sm".to_string()),
            md: Some("md:text-base".to_string()),
            lg: Some("lg:text-lg".to_string()),
            xl: Some("xl:text-xl".to_string()),
        };
        
        // Test responsive config properties
        assert_eq!(responsive_config.sm, Some("sm:text-sm".to_string()));
        assert_eq!(responsive_config.md, Some("md:text-base".to_string()));
        assert_eq!(responsive_config.lg, Some("lg:text-lg".to_string()));
        assert_eq!(responsive_config.xl, Some("xl:text-xl".to_string()));
    }

    #[test]
    fn test_responsive_config_default_implementation() {
        // Test ResponsiveConfig default implementation
        let default_config = ResponsiveConfig::default();
        assert_eq!(default_config.sm, None);
        assert_eq!(default_config.md, None);
        assert_eq!(default_config.lg, None);
        assert_eq!(default_config.xl, None);
    }

    #[test]
    fn test_theme_custom_properties() {
        // Test custom theme properties
        let mut custom_props = HashMap::new();
        custom_props.insert("primary".to_string(), "#3b82f6".to_string());
        custom_props.insert("secondary".to_string(), "#64748b".to_string());
        custom_props.insert("accent".to_string(), "#f59e0b".to_string());
        
        let custom_theme = Theme::Custom(custom_props.clone());
        
        // Test custom theme access
        if let Theme::Custom(props) = custom_theme {
            assert_eq!(props.len(), 3);
            assert_eq!(props.get("primary"), Some(&"#3b82f6".to_string()));
            assert_eq!(props.get("secondary"), Some(&"#64748b".to_string()));
            assert_eq!(props.get("accent"), Some(&"#f59e0b".to_string()));
        }
    }

    #[test]
    fn test_variant_all_variants() {
        // Test all variant types
        let variants = vec![
            Variant::Primary,
            Variant::Secondary,
            Variant::Destructive,
            Variant::Outline,
            Variant::Ghost,
            Variant::Link,
        ];
        
        // Test that all variants are unique
        for (i, variant1) in variants.iter().enumerate() {
            for (j, variant2) in variants.iter().enumerate() {
                if i == j {
                    assert_eq!(variant1, variant2);
                } else {
                    assert_ne!(variant1, variant2);
                }
            }
        }
    }

    #[test]
    fn test_size_all_sizes() {
        // Test all size types
        let sizes = vec![
            Size::Small,
            Size::Medium,
            Size::Large,
        ];
        
        // Test that all sizes are unique
        for (i, size1) in sizes.iter().enumerate() {
            for (j, size2) in sizes.iter().enumerate() {
                if i == j {
                    assert_eq!(size1, size2);
                } else {
                    assert_ne!(size1, size2);
                }
            }
        }
    }

    #[test]
    fn test_responsive_config_partial_config() {
        // Test partial responsive config
        let partial_config = ResponsiveConfig {
            sm: Some("sm:text-sm".to_string()),
            md: None,
            lg: Some("lg:text-lg".to_string()),
            xl: None,
        };
        
        // Test partial config properties
        assert_eq!(partial_config.sm, Some("sm:text-sm".to_string()));
        assert_eq!(partial_config.md, None);
        assert_eq!(partial_config.lg, Some("lg:text-lg".to_string()));
        assert_eq!(partial_config.xl, None);
    }

    #[test]
    fn test_theme_clone_behavior() {
        // Test theme cloning behavior
        let mut custom_props = HashMap::new();
        custom_props.insert("primary".to_string(), "#3b82f6".to_string());
        let original_theme = Theme::Custom(custom_props);
        
        // Test cloning
        let cloned_theme = original_theme.clone();
        assert_eq!(original_theme, cloned_theme);
        
        // Test that cloned theme has same properties
        if let (Theme::Custom(original_props), Theme::Custom(cloned_props)) = (&original_theme, &cloned_theme) {
            assert_eq!(original_props, cloned_props);
        }
    }

    #[test]
    fn test_variant_clone_behavior() {
        // Test variant cloning behavior
        let original_variant = Variant::Primary;
        let cloned_variant = original_variant.clone();
        
        assert_eq!(original_variant, cloned_variant);
    }

    #[test]
    fn test_size_clone_behavior() {
        // Test size cloning behavior
        let original_size = Size::Large;
        let cloned_size = original_size.clone();
        
        assert_eq!(original_size, cloned_size);
    }

    #[test]
    fn test_responsive_config_clone_behavior() {
        // Test responsive config cloning behavior
        let original_config = ResponsiveConfig {
            sm: Some("sm:text-sm".to_string()),
            md: Some("md:text-base".to_string()),
            lg: Some("lg:text-lg".to_string()),
            xl: Some("xl:text-xl".to_string()),
        };
        
        let cloned_config = original_config.clone();
        assert_eq!(original_config, cloned_config);
    }

    #[test]
    fn test_theme_debug_formatting() {
        // Test theme debug formatting
        let theme = Theme::Dark;
        let debug_str = format!("{:?}", theme);
        assert!(debug_str.contains("Dark"));
        
        let custom_theme = Theme::Custom(HashMap::new());
        let custom_debug_str = format!("{:?}", custom_theme);
        assert!(custom_debug_str.contains("Custom"));
    }

    #[test]
    fn test_variant_debug_formatting() {
        // Test variant debug formatting
        let variant = Variant::Primary;
        let debug_str = format!("{:?}", variant);
        assert!(debug_str.contains("Primary"));
    }

    #[test]
    fn test_size_debug_formatting() {
        // Test size debug formatting
        let size = Size::Medium;
        let debug_str = format!("{:?}", size);
        assert!(debug_str.contains("Medium"));
    }

    #[test]
    fn test_responsive_config_debug_formatting() {
        // Test responsive config debug formatting
        let config = ResponsiveConfig::default();
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("ResponsiveConfig"));
    }
}
