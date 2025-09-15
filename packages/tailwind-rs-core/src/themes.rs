//! Theme system for Tailwind CSS.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::colors::Color;

/// A theme variant that defines the visual style of a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Variant {
    /// Default variant
    Default,
    /// Primary variant
    Primary,
    /// Secondary variant
    Secondary,
    /// Success variant
    Success,
    /// Warning variant
    Warning,
    /// Error variant
    Error,
    /// Info variant
    Info,
    /// Outline variant
    Outline,
    /// Ghost variant
    Ghost,
    /// Link variant
    Link,
    /// Destructive variant
    Destructive,
}

impl Variant {
    /// Get the variant name as a string.
    pub fn name(&self) -> &'static str {
        match self {
            Variant::Default => "default",
            Variant::Primary => "primary",
            Variant::Secondary => "secondary",
            Variant::Success => "success",
            Variant::Warning => "warning",
            Variant::Error => "error",
            Variant::Info => "info",
            Variant::Outline => "outline",
            Variant::Ghost => "ghost",
            Variant::Link => "link",
            Variant::Destructive => "destructive",
        }
    }
}

/// A size variant that defines the dimensions of a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Size {
    /// Extra small size
    Xs,
    /// Small size
    Sm,
    /// Medium size (default)
    Md,
    /// Large size
    Lg,
    /// Extra large size
    Xl,
}

impl Size {
    /// Get the size name as a string.
    pub fn name(&self) -> &'static str {
        match self {
            Size::Xs => "xs",
            Size::Sm => "sm",
            Size::Md => "md",
            Size::Lg => "lg",
            Size::Xl => "xl",
        }
    }
}

/// A theme that defines the visual appearance of components.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Theme {
    /// The primary color
    pub primary: Color,
    /// The secondary color
    pub secondary: Color,
    /// The success color
    pub success: Color,
    /// The warning color
    pub warning: Color,
    /// The error color
    pub error: Color,
    /// The info color
    pub info: Color,
    /// Custom variant definitions
    pub variants: HashMap<String, String>,
}

impl Theme {
    /// Create a new theme with default colors.
    pub fn new() -> Self {
        Self {
            primary: Color::Blue,
            secondary: Color::Gray,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Sky,
            variants: HashMap::new(),
        }
    }

    /// Create a new theme with custom primary color.
    pub fn with_primary(mut self, color: Color) -> Self {
        self.primary = color;
        self
    }

    /// Create a new theme with custom secondary color.
    pub fn with_secondary(mut self, color: Color) -> Self {
        self.secondary = color;
        self
    }

    /// Create a new theme with custom success color.
    pub fn with_success(mut self, color: Color) -> Self {
        self.success = color;
        self
    }

    /// Create a new theme with custom warning color.
    pub fn with_warning(mut self, color: Color) -> Self {
        self.warning = color;
        self
    }

    /// Create a new theme with custom error color.
    pub fn with_error(mut self, color: Color) -> Self {
        self.error = color;
        self
    }

    /// Create a new theme with custom info color.
    pub fn with_info(mut self, color: Color) -> Self {
        self.info = color;
        self
    }

    /// Add a custom variant to the theme.
    pub fn variant(mut self, name: impl Into<String>, classes: impl Into<String>) -> Self {
        self.variants.insert(name.into(), classes.into());
        self
    }

    /// Get classes for a specific variant.
    pub fn get_variant_classes(&self, variant: &Variant) -> String {
        match variant {
            Variant::Default => "bg-gray-100 text-gray-900 hover:bg-gray-200".to_string(),
            Variant::Primary => format!("{} {} hover:{}", 
                self.primary.background(600), 
                self.primary.text(900), 
                self.primary.hover(700)
            ),
            Variant::Secondary => format!("{} {} hover:{}", 
                self.secondary.background(200), 
                self.secondary.text(900), 
                self.secondary.hover(300)
            ),
            Variant::Success => format!("{} {} hover:{}", 
                self.success.background(600), 
                self.success.text(900), 
                self.success.hover(700)
            ),
            Variant::Warning => format!("{} {} hover:{}", 
                self.warning.background(600), 
                self.warning.text(900), 
                self.warning.hover(700)
            ),
            Variant::Error => format!("{} {} hover:{}", 
                self.error.background(600), 
                self.error.text(900), 
                self.error.hover(700)
            ),
            Variant::Info => format!("{} {} hover:{}", 
                self.info.background(600), 
                self.info.text(900), 
                self.info.hover(700)
            ),
            Variant::Outline => format!("{} {} hover:{}", 
                self.primary.outline(600), 
                self.primary.text(600), 
                self.primary.background(50)
            ),
            Variant::Ghost => format!("{} hover:{}", 
                self.primary.ghost(100), 
                self.primary.background(100)
            ),
            Variant::Link => format!("{} hover:{}", 
                self.primary.link(600), 
                self.primary.text(700)
            ),
            Variant::Destructive => format!("{} {} hover:{}", 
                self.error.background(600), 
                self.error.text(900), 
                self.error.hover(700)
            ),
        }
    }

    /// Get classes for a specific size.
    pub fn get_size_classes(&self, size: &Size) -> String {
        match size {
            Size::Xs => "px-2 py-1 text-xs".to_string(),
            Size::Sm => "px-3 py-1.5 text-sm".to_string(),
            Size::Md => "px-4 py-2 text-base".to_string(),
            Size::Lg => "px-6 py-3 text-lg".to_string(),
            Size::Xl => "px-8 py-4 text-xl".to_string(),
        }
    }

    /// Get classes for a specific variant and size combination.
    pub fn get_classes(&self, variant: &Variant, size: &Size) -> String {
        format!("{} {}", 
            self.get_variant_classes(variant), 
            self.get_size_classes(size)
        )
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::new()
    }
}

/// Predefined themes for common use cases.
pub mod themes {
    use super::*;

    /// Default theme with blue primary color.
    pub fn default() -> Theme {
        Theme::new()
    }

    /// Dark theme with darker colors.
    pub fn dark() -> Theme {
        Theme::new()
            .with_primary(Color::Blue)
            .with_secondary(Color::Gray)
            .with_success(Color::Green)
            .with_warning(Color::Yellow)
            .with_error(Color::Red)
            .with_info(Color::Sky)
    }

    /// Light theme with lighter colors.
    pub fn light() -> Theme {
        Theme::new()
            .with_primary(Color::Blue)
            .with_secondary(Color::Gray)
            .with_success(Color::Green)
            .with_warning(Color::Yellow)
            .with_error(Color::Red)
            .with_info(Color::Sky)
    }

    /// High contrast theme for accessibility.
    pub fn high_contrast() -> Theme {
        Theme::new()
            .with_primary(Color::Blue)
            .with_secondary(Color::Gray)
            .with_success(Color::Green)
            .with_warning(Color::Yellow)
            .with_error(Color::Red)
            .with_info(Color::Sky)
    }

    /// Monochrome theme with grayscale colors.
    pub fn monochrome() -> Theme {
        Theme::new()
            .with_primary(Color::Gray)
            .with_secondary(Color::Gray)
            .with_success(Color::Gray)
            .with_warning(Color::Gray)
            .with_error(Color::Gray)
            .with_info(Color::Gray)
    }
}

/// A theme manager that handles theme switching and customization.
#[derive(Debug, Clone)]
pub struct ThemeManager {
    /// Current theme
    pub current_theme: Theme,
    /// Available themes
    pub themes: HashMap<String, Theme>,
}

impl ThemeManager {
    /// Create a new theme manager with the default theme.
    pub fn new() -> Self {
        let mut themes = HashMap::new();
        themes.insert("default".to_string(), themes::default());
        themes.insert("dark".to_string(), themes::dark());
        themes.insert("light".to_string(), themes::light());
        themes.insert("high-contrast".to_string(), themes::high_contrast());
        themes.insert("monochrome".to_string(), themes::monochrome());

        Self {
            current_theme: themes::default(),
            themes,
        }
    }

    /// Switch to a different theme.
    pub fn switch_theme(&mut self, theme_name: &str) -> Result<(), String> {
        if let Some(theme) = self.themes.get(theme_name) {
            self.current_theme = theme.clone();
            Ok(())
        } else {
            Err(format!("Theme '{}' not found", theme_name))
        }
    }

    /// Add a custom theme.
    pub fn add_theme(&mut self, name: impl Into<String>, theme: Theme) {
        self.themes.insert(name.into(), theme);
    }

    /// Get the current theme.
    pub fn current_theme(&self) -> &Theme {
        &self.current_theme
    }

    /// Get all available theme names.
    pub fn available_themes(&self) -> Vec<String> {
        self.themes.keys().cloned().collect()
    }

    /// Get classes for a variant and size using the current theme.
    pub fn get_classes(&self, variant: &Variant, size: &Size) -> String {
        self.current_theme.get_classes(variant, size)
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variant_name() {
        assert_eq!(Variant::Primary.name(), "primary");
        assert_eq!(Variant::Secondary.name(), "secondary");
        assert_eq!(Variant::Success.name(), "success");
    }

    #[test]
    fn test_size_name() {
        assert_eq!(Size::Xs.name(), "xs");
        assert_eq!(Size::Sm.name(), "sm");
        assert_eq!(Size::Md.name(), "md");
        assert_eq!(Size::Lg.name(), "lg");
        assert_eq!(Size::Xl.name(), "xl");
    }

    #[test]
    fn test_theme_creation() {
        let theme = Theme::new()
            .with_primary(Color::Blue)
            .with_secondary(Color::Gray);

        assert_eq!(theme.primary, Color::Blue);
        assert_eq!(theme.secondary, Color::Gray);
    }

    #[test]
    fn test_theme_variant_classes() {
        let theme = Theme::new();
        let primary_classes = theme.get_variant_classes(&Variant::Primary);
        
        assert!(primary_classes.contains("bg-blue-600"));
        assert!(primary_classes.contains("text-blue-900"));
        assert!(primary_classes.contains("hover:bg-blue-700"));
    }

    #[test]
    fn test_theme_size_classes() {
        let theme = Theme::new();
        let md_classes = theme.get_size_classes(&Size::Md);
        
        assert_eq!(md_classes, "px-4 py-2 text-base");
    }

    #[test]
    fn test_theme_combined_classes() {
        let theme = Theme::new();
        let classes = theme.get_classes(&Variant::Primary, &Size::Md);
        
        assert!(classes.contains("bg-blue-600"));
        assert!(classes.contains("px-4 py-2"));
    }

    #[test]
    fn test_theme_manager() {
        let mut manager = ThemeManager::new();
        
        assert!(manager.available_themes().contains(&"default".to_string()));
        assert!(manager.available_themes().contains(&"dark".to_string()));
        
        let result = manager.switch_theme("dark");
        assert!(result.is_ok());
        
        let result = manager.switch_theme("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_predefined_themes() {
        let default_theme = themes::default();
        let dark_theme = themes::dark();
        let light_theme = themes::light();
        
        assert_eq!(default_theme.primary, Color::Blue);
        assert_eq!(dark_theme.primary, Color::Blue);
        assert_eq!(light_theme.primary, Color::Blue);
    }
}
