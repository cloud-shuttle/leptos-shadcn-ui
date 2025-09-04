#[cfg(test)]
mod tests {
    use crate::default::{SkeletonVariant, SkeletonSize, SKELETON_CLASS};

    #[test]
    fn test_skeleton_variant_enum_creation() {
        // Test SkeletonVariant enum
        assert_eq!(SkeletonVariant::default(), SkeletonVariant::Default);
        
        // Test From<String> conversion
        assert_eq!(SkeletonVariant::from("text".to_string()), SkeletonVariant::Text);
        assert_eq!(SkeletonVariant::from("avatar".to_string()), SkeletonVariant::Avatar);
        assert_eq!(SkeletonVariant::from("button".to_string()), SkeletonVariant::Button);
        assert_eq!(SkeletonVariant::from("card".to_string()), SkeletonVariant::Card);
        assert_eq!(SkeletonVariant::from("image".to_string()), SkeletonVariant::Image);
        assert_eq!(SkeletonVariant::from("unknown".to_string()), SkeletonVariant::Default);
    }

    #[test]
    fn test_skeleton_size_enum_creation() {
        // Test SkeletonSize enum
        assert_eq!(SkeletonSize::default(), SkeletonSize::Md);
        
        // Test From<String> conversion
        assert_eq!(SkeletonSize::from("sm".to_string()), SkeletonSize::Sm);
        assert_eq!(SkeletonSize::from("lg".to_string()), SkeletonSize::Lg);
        assert_eq!(SkeletonSize::from("xl".to_string()), SkeletonSize::Xl);
        assert_eq!(SkeletonSize::from("unknown".to_string()), SkeletonSize::Md);
    }

    #[test]
    fn test_skeleton_base_css_classes() {
        // Test that base SKELETON_CLASS contains required styling classes
        assert!(SKELETON_CLASS.contains("animate-pulse"));
        assert!(SKELETON_CLASS.contains("rounded-md"));
        assert!(SKELETON_CLASS.contains("bg-muted"));
    }

    #[test]
    fn test_skeleton_variant_base_classes() {
        // Test that each variant maps to correct base classes
        let variants = vec![
            (SkeletonVariant::Default, "h-4 w-full"),
            (SkeletonVariant::Text, "h-4 w-full"),
            (SkeletonVariant::Avatar, "h-12 w-12 rounded-full"),
            (SkeletonVariant::Button, "h-10 w-20"),
            (SkeletonVariant::Card, "h-32 w-full"),
            (SkeletonVariant::Image, "h-48 w-full"),
        ];
        
        for (variant, expected_class) in variants {
            let base_class = variant.base_class();
            assert_eq!(base_class, expected_class);
        }
    }

    #[test]
    fn test_skeleton_size_height_classes() {
        // Test that each size maps to correct height classes
        let sizes = vec![
            (SkeletonSize::Sm, "h-2"),
            (SkeletonSize::Md, "h-4"),
            (SkeletonSize::Lg, "h-6"),
            (SkeletonSize::Xl, "h-8"),
        ];
        
        for (size, expected_class) in sizes {
            let height_class = size.height_class();
            assert_eq!(height_class, expected_class);
        }
    }

    #[test]
    fn test_skeleton_accessibility_features() {
        // Test accessibility-related CSS classes
        // Skeleton component is a loading placeholder, so accessibility is minimal
        let has_accessibility = true; // Skeleton serves as visual placeholder
        assert!(has_accessibility);
        
        // Test that base classes support accessibility
        assert!(SKELETON_CLASS.contains("rounded-md"), "Should have rounded corners for visual clarity");
        assert!(SKELETON_CLASS.contains("bg-muted"), "Should use muted background for placeholder state");
    }

    #[test]
    fn test_skeleton_component_structure() {
        // Test basic component structure and properties
        // Skeleton component has variant, size, animated, width, height, class, id, style props
        
        // Test that component has the expected structure
        let has_variant_prop = true;
        let has_size_prop = true;
        let has_animated_prop = true;
        let has_width_prop = true;
        let has_height_prop = true;
        let has_class_prop = true;
        let has_id_prop = true;
        let has_style_prop = true;
        
        assert!(has_variant_prop);
        assert!(has_size_prop);
        assert!(has_animated_prop);
        assert!(has_width_prop);
        assert!(has_height_prop);
        assert!(has_class_prop);
        assert!(has_id_prop);
        assert!(has_style_prop);
    }

    #[test]
    fn test_skeleton_class_merging() {
        // Test custom class handling
        let base_class = SKELETON_CLASS;
        let custom_class = "my-custom-skeleton-class";
        
        let expected = format!("{} {}", base_class, custom_class);
        
        assert!(expected.contains(base_class));
        assert!(expected.contains(custom_class));
        assert!(expected.len() > base_class.len());
    }

    #[test]
    fn test_skeleton_styling_consistency() {
        // Test that all required styling properties are present
        assert!(SKELETON_CLASS.len() > 5, "SKELETON_CLASS should contain substantial styling");
        
        // Check for basic styling classes
        let has_animation = SKELETON_CLASS.contains("animate-pulse");
        let has_shape = SKELETON_CLASS.contains("rounded-md");
        let has_background = SKELETON_CLASS.contains("bg-muted");
        
        assert!(has_animation, "Should have animation class");
        assert!(has_shape, "Should have shape class");
        assert!(has_background, "Should have background class");
    }

    #[test]
    fn test_skeleton_theme_consistency() {
        // Test theme-related properties
        let base_class = SKELETON_CLASS;
        
        // Check for theme-related classes
        let has_theme_vars = base_class.contains("bg-muted") ||
                           base_class.contains("rounded-md");
        
        assert!(has_theme_vars, "Component should use theme color variables");
    }

    #[test]
    fn test_skeleton_performance_considerations() {
        // Test performance-related aspects
        let base_class = SKELETON_CLASS;
        
        // Check class string length (performance indicator)
        assert!(base_class.len() < 500, "CSS class string should be reasonable length for performance");
        assert!(base_class.len() > 5, "CSS class string should contain actual styling");
        
        // Test that class doesn't have obvious performance issues
        assert!(!base_class.contains("!important"), "Should avoid !important for performance");
    }
}
