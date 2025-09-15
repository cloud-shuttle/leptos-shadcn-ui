//! Type-safe color system for Tailwind CSS.

use serde::{Deserialize, Serialize};

/// A type-safe color system that provides compile-time validation
/// and consistent color usage across components.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Color {
    /// Slate color palette
    Slate,
    /// Gray color palette
    Gray,
    /// Zinc color palette
    Zinc,
    /// Neutral color palette
    Neutral,
    /// Stone color palette
    Stone,
    /// Red color palette
    Red,
    /// Orange color palette
    Orange,
    /// Amber color palette
    Amber,
    /// Yellow color palette
    Yellow,
    /// Lime color palette
    Lime,
    /// Green color palette
    Green,
    /// Emerald color palette
    Emerald,
    /// Teal color palette
    Teal,
    /// Cyan color palette
    Cyan,
    /// Sky color palette
    Sky,
    /// Blue color palette
    Blue,
    /// Indigo color palette
    Indigo,
    /// Violet color palette
    Violet,
    /// Purple color palette
    Purple,
    /// Fuchsia color palette
    Fuchsia,
    /// Pink color palette
    Pink,
    /// Rose color palette
    Rose,
}

impl Color {
    /// Get the color name as a string.
    pub fn name(&self) -> &'static str {
        match self {
            Color::Slate => "slate",
            Color::Gray => "gray",
            Color::Zinc => "zinc",
            Color::Neutral => "neutral",
            Color::Stone => "stone",
            Color::Red => "red",
            Color::Orange => "orange",
            Color::Amber => "amber",
            Color::Yellow => "yellow",
            Color::Lime => "lime",
            Color::Green => "green",
            Color::Emerald => "emerald",
            Color::Teal => "teal",
            Color::Cyan => "cyan",
            Color::Sky => "sky",
            Color::Blue => "blue",
            Color::Indigo => "indigo",
            Color::Violet => "violet",
            Color::Purple => "purple",
            Color::Fuchsia => "fuchsia",
            Color::Pink => "pink",
            Color::Rose => "rose",
        }
    }

    /// Create a color from a string name.
    pub fn from_name(name: &str) -> Option<Color> {
        match name.to_lowercase().as_str() {
            "slate" => Some(Color::Slate),
            "gray" => Some(Color::Gray),
            "zinc" => Some(Color::Zinc),
            "neutral" => Some(Color::Neutral),
            "stone" => Some(Color::Stone),
            "red" => Some(Color::Red),
            "orange" => Some(Color::Orange),
            "amber" => Some(Color::Amber),
            "yellow" => Some(Color::Yellow),
            "lime" => Some(Color::Lime),
            "green" => Some(Color::Green),
            "emerald" => Some(Color::Emerald),
            "teal" => Some(Color::Teal),
            "cyan" => Some(Color::Cyan),
            "sky" => Some(Color::Sky),
            "blue" => Some(Color::Blue),
            "indigo" => Some(Color::Indigo),
            "violet" => Some(Color::Violet),
            "purple" => Some(Color::Purple),
            "fuchsia" => Some(Color::Fuchsia),
            "pink" => Some(Color::Pink),
            "rose" => Some(Color::Rose),
            _ => None,
        }
    }

    /// Generate a background color class.
    pub fn background(&self, shade: u16) -> String {
        format!("bg-{}-{}", self.name(), shade)
    }

    /// Generate a text color class.
    pub fn text(&self, shade: u16) -> String {
        format!("text-{}-{}", self.name(), shade)
    }

    /// Generate a border color class.
    pub fn border(&self, shade: u16) -> String {
        format!("border-{}-{}", self.name(), shade)
    }

    /// Generate a hover background color class.
    pub fn hover(&self, shade: u16) -> String {
        format!("hover:bg-{}-{}", self.name(), shade)
    }

    /// Generate a focus ring color class.
    pub fn focus_ring(&self, shade: u16) -> String {
        format!("focus:ring-{}-{}", self.name(), shade)
    }

    /// Generate a shadow color class.
    pub fn shadow(&self, shade: u16) -> String {
        format!("shadow-{}-{}", self.name(), shade)
    }

    /// Get the primary color variant (typically 600).
    pub fn primary(&self) -> String {
        self.background(600)
    }

    /// Get the primary text color variant (typically white or 900).
    pub fn primary_text(&self) -> String {
        self.text(900)
    }

    /// Get the secondary color variant (typically 100 or 200).
    pub fn secondary(&self) -> String {
        self.background(100)
    }

    /// Get the secondary text color variant (typically 600 or 700).
    pub fn secondary_text(&self) -> String {
        self.text(600)
    }

    /// Get the muted color variant (typically 50 or 100).
    pub fn muted(&self) -> String {
        self.background(50)
    }

    /// Get the muted text color variant (typically 500 or 600).
    pub fn muted_text(&self) -> String {
        self.text(500)
    }

    /// Get the accent color variant (typically 500 or 600).
    pub fn accent(&self) -> String {
        self.background(500)
    }

    /// Get the accent text color variant (typically white or 900).
    pub fn accent_text(&self) -> String {
        self.text(900)
    }

    /// Get the destructive color variant (typically red-600).
    pub fn destructive(&self) -> String {
        "bg-red-600".to_string()
    }

    /// Get the destructive text color variant (typically white).
    pub fn destructive_text(&self) -> String {
        "text-white".to_string()
    }

    /// Get the outline color variant (typically border-2 with the color).
    pub fn outline(&self, shade: u16) -> String {
        format!("border-2 border-{}-{}", self.name(), shade)
    }

    /// Get the ghost color variant (typically transparent with hover).
    pub fn ghost(&self, shade: u16) -> String {
        format!("bg-transparent hover:bg-{}-{}", self.name(), shade)
    }

    /// Get the link color variant (typically text color with underline).
    pub fn link(&self, shade: u16) -> String {
        format!("text-{}-{} underline", self.name(), shade)
    }
}

/// Predefined color palettes for common use cases.
pub mod palettes {
    use super::Color;

    /// Primary color palette (Blue).
    pub const PRIMARY: Color = Color::Blue;
    
    /// Secondary color palette (Gray).
    pub const SECONDARY: Color = Color::Gray;
    
    /// Success color palette (Green).
    pub const SUCCESS: Color = Color::Green;
    
    /// Warning color palette (Yellow).
    pub const WARNING: Color = Color::Yellow;
    
    /// Error color palette (Red).
    pub const ERROR: Color = Color::Red;
    
    /// Info color palette (Sky).
    pub const INFO: Color = Color::Sky;
}

/// Utility functions for common color operations.
pub mod utils {
    use super::Color;

    /// Create a color from a string name.
    pub fn from_name(name: &str) -> Option<Color> {
        match name.to_lowercase().as_str() {
            "slate" => Some(Color::Slate),
            "gray" => Some(Color::Gray),
            "zinc" => Some(Color::Zinc),
            "neutral" => Some(Color::Neutral),
            "stone" => Some(Color::Stone),
            "red" => Some(Color::Red),
            "orange" => Some(Color::Orange),
            "amber" => Some(Color::Amber),
            "yellow" => Some(Color::Yellow),
            "lime" => Some(Color::Lime),
            "green" => Some(Color::Green),
            "emerald" => Some(Color::Emerald),
            "teal" => Some(Color::Teal),
            "cyan" => Some(Color::Cyan),
            "sky" => Some(Color::Sky),
            "blue" => Some(Color::Blue),
            "indigo" => Some(Color::Indigo),
            "violet" => Some(Color::Violet),
            "purple" => Some(Color::Purple),
            "fuchsia" => Some(Color::Fuchsia),
            "pink" => Some(Color::Pink),
            "rose" => Some(Color::Rose),
            _ => None,
        }
    }

    /// Get all available colors.
    pub fn all_colors() -> Vec<Color> {
        vec![
            Color::Slate, Color::Gray, Color::Zinc, Color::Neutral, Color::Stone,
            Color::Red, Color::Orange, Color::Amber, Color::Yellow, Color::Lime,
            Color::Green, Color::Emerald, Color::Teal, Color::Cyan, Color::Sky,
            Color::Blue, Color::Indigo, Color::Violet, Color::Purple, Color::Fuchsia,
            Color::Pink, Color::Rose,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_background() {
        let color = Color::Blue;
        assert_eq!(color.background(600), "bg-blue-600");
        assert_eq!(color.background(500), "bg-blue-500");
    }

    #[test]
    fn test_color_text() {
        let color = Color::Red;
        assert_eq!(color.text(600), "text-red-600");
        assert_eq!(color.text(500), "text-red-500");
    }

    #[test]
    fn test_color_hover() {
        let color = Color::Green;
        assert_eq!(color.hover(700), "hover:bg-green-700");
    }

    #[test]
    fn test_color_primary() {
        let color = Color::Blue;
        assert_eq!(color.primary(), "bg-blue-600");
        assert_eq!(color.primary_text(), "text-blue-900");
    }

    #[test]
    fn test_color_secondary() {
        let color = Color::Gray;
        assert_eq!(color.secondary(), "bg-gray-100");
        assert_eq!(color.secondary_text(), "text-gray-600");
    }

    #[test]
    fn test_color_destructive() {
        let color = Color::Red;
        assert_eq!(color.destructive(), "bg-red-600");
        assert_eq!(color.destructive_text(), "text-white");
    }

    #[test]
    fn test_color_outline() {
        let color = Color::Blue;
        assert_eq!(color.outline(600), "border-2 border-blue-600");
    }

    #[test]
    fn test_color_ghost() {
        let color = Color::Blue;
        assert_eq!(color.ghost(100), "bg-transparent hover:bg-blue-100");
    }

    #[test]
    fn test_color_link() {
        let color = Color::Blue;
        assert_eq!(color.link(600), "text-blue-600 underline");
    }

    #[test]
    fn test_palettes() {
        assert_eq!(palettes::PRIMARY, Color::Blue);
        assert_eq!(palettes::SECONDARY, Color::Gray);
        assert_eq!(palettes::SUCCESS, Color::Green);
        assert_eq!(palettes::WARNING, Color::Yellow);
        assert_eq!(palettes::ERROR, Color::Red);
        assert_eq!(palettes::INFO, Color::Sky);
    }

    #[test]
    fn test_utils_from_name() {
        assert_eq!(utils::from_name("blue"), Some(Color::Blue));
        assert_eq!(utils::from_name("red"), Some(Color::Red));
        assert_eq!(utils::from_name("invalid"), None);
    }

    #[test]
    fn test_utils_all_colors() {
        let colors = utils::all_colors();
        assert!(colors.contains(&Color::Blue));
        assert!(colors.contains(&Color::Red));
        assert!(colors.contains(&Color::Green));
        assert_eq!(colors.len(), 22);
    }
}
