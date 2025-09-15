//! Responsive design utilities for Tailwind CSS.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Breakpoint definitions for responsive design.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Breakpoint {
    /// Small screens (640px and up)
    Sm,
    /// Medium screens (768px and up)
    Md,
    /// Large screens (1024px and up)
    Lg,
    /// Extra large screens (1280px and up)
    Xl,
    /// 2X large screens (1536px and up)
    Xl2,
}

impl Breakpoint {
    /// Get the breakpoint prefix for Tailwind classes.
    pub fn prefix(&self) -> &'static str {
        match self {
            Breakpoint::Sm => "sm",
            Breakpoint::Md => "md",
            Breakpoint::Lg => "lg",
            Breakpoint::Xl => "xl",
            Breakpoint::Xl2 => "2xl",
        }
    }

    /// Get the minimum width in pixels for this breakpoint.
    pub fn min_width(&self) -> u32 {
        match self {
            Breakpoint::Sm => 640,
            Breakpoint::Md => 768,
            Breakpoint::Lg => 1024,
            Breakpoint::Xl => 1280,
            Breakpoint::Xl2 => 1536,
        }
    }
}

/// A responsive design system that provides type-safe responsive classes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Responsive {
    /// Classes for different breakpoints
    pub breakpoints: HashMap<Breakpoint, String>,
}

impl Responsive {
    /// Create a new Responsive instance.
    pub fn new() -> Self {
        Self {
            breakpoints: HashMap::new(),
        }
    }

    /// Add classes for a specific breakpoint.
    pub fn breakpoint(mut self, breakpoint: Breakpoint, classes: impl Into<String>) -> Self {
        self.breakpoints.insert(breakpoint, classes.into());
        self
    }

    /// Add classes for small screens.
    pub fn sm(self, classes: impl Into<String>) -> Self {
        self.breakpoint(Breakpoint::Sm, classes)
    }

    /// Add classes for medium screens.
    pub fn md(self, classes: impl Into<String>) -> Self {
        self.breakpoint(Breakpoint::Md, classes)
    }

    /// Add classes for large screens.
    pub fn lg(self, classes: impl Into<String>) -> Self {
        self.breakpoint(Breakpoint::Lg, classes)
    }

    /// Add classes for extra large screens.
    pub fn xl(self, classes: impl Into<String>) -> Self {
        self.breakpoint(Breakpoint::Xl, classes)
    }

    /// Add classes for 2X large screens.
    pub fn xl2(self, classes: impl Into<String>) -> Self {
        self.breakpoint(Breakpoint::Xl2, classes)
    }

    /// Generate the final responsive class string.
    pub fn to_string(&self) -> String {
        let mut classes = Vec::new();
        
        // Sort breakpoints by min-width to ensure proper order
        let mut sorted_breakpoints: Vec<_> = self.breakpoints.iter().collect();
        sorted_breakpoints.sort_by_key(|(bp, _)| bp.min_width());
        
        for (breakpoint, class) in sorted_breakpoints {
            classes.push(format!("{}:{}", breakpoint.prefix(), class));
        }
        
        classes.join(" ")
    }

    /// Merge with another Responsive instance.
    pub fn merge(mut self, other: Responsive) -> Self {
        for (breakpoint, classes) in other.breakpoints {
            self.breakpoints.insert(breakpoint, classes);
        }
        self
    }
}

impl Default for Responsive {
    fn default() -> Self {
        Self::new()
    }
}

/// A builder for creating responsive designs with a fluent API.
#[derive(Debug, Default)]
pub struct ResponsiveBuilder {
    responsive: Responsive,
}

impl ResponsiveBuilder {
    /// Create a new ResponsiveBuilder.
    pub fn new() -> Self {
        Self {
            responsive: Responsive::new(),
        }
    }

    /// Add classes for small screens.
    pub fn sm(mut self, classes: impl Into<String>) -> Self {
        self.responsive = self.responsive.sm(classes);
        self
    }

    /// Add classes for medium screens.
    pub fn md(mut self, classes: impl Into<String>) -> Self {
        self.responsive = self.responsive.md(classes);
        self
    }

    /// Add classes for large screens.
    pub fn lg(mut self, classes: impl Into<String>) -> Self {
        self.responsive = self.responsive.lg(classes);
        self
    }

    /// Add classes for extra large screens.
    pub fn xl(mut self, classes: impl Into<String>) -> Self {
        self.responsive = self.responsive.xl(classes);
        self
    }

    /// Add classes for 2X large screens.
    pub fn xl2(mut self, classes: impl Into<String>) -> Self {
        self.responsive = self.responsive.xl2(classes);
        self
    }

    /// Build the final Responsive instance.
    pub fn build(self) -> Responsive {
        self.responsive
    }
}

/// Utility function to create a ResponsiveBuilder.
pub fn responsive() -> ResponsiveBuilder {
    ResponsiveBuilder::new()
}

/// Predefined responsive patterns for common use cases.
pub mod patterns {
    use super::*;

    /// Mobile-first text sizing pattern.
    pub fn text_sizing() -> Responsive {
        Responsive::new()
            .sm("text-sm")
            .md("text-base")
            .lg("text-lg")
            .xl("text-xl")
    }

    /// Mobile-first spacing pattern.
    pub fn spacing() -> Responsive {
        Responsive::new()
            .sm("p-2")
            .md("p-4")
            .lg("p-6")
            .xl("p-8")
    }

    /// Mobile-first grid pattern.
    pub fn grid() -> Responsive {
        Responsive::new()
            .sm("grid-cols-1")
            .md("grid-cols-2")
            .lg("grid-cols-3")
            .xl("grid-cols-4")
    }

    /// Mobile-first flex pattern.
    pub fn flex() -> Responsive {
        Responsive::new()
            .sm("flex-col")
            .md("flex-row")
    }

    /// Mobile-first visibility pattern.
    pub fn visibility() -> Responsive {
        Responsive::new()
            .sm("hidden")
            .md("block")
    }
}

/// Utility functions for responsive design.
pub mod utils {
    use super::*;

    /// Create a responsive instance from a string.
    pub fn from_string(input: &str) -> Responsive {
        let mut responsive = Responsive::new();
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        for part in parts {
            if let Some((prefix, class)) = part.split_once(':') {
                let breakpoint = match prefix {
                    "sm" => Breakpoint::Sm,
                    "md" => Breakpoint::Md,
                    "lg" => Breakpoint::Lg,
                    "xl" => Breakpoint::Xl,
                    "2xl" => Breakpoint::Xl2,
                    _ => continue,
                };
                responsive = responsive.breakpoint(breakpoint, class);
            }
        }
        
        responsive
    }

    /// Get all available breakpoints.
    pub fn all_breakpoints() -> Vec<Breakpoint> {
        vec![
            Breakpoint::Sm,
            Breakpoint::Md,
            Breakpoint::Lg,
            Breakpoint::Xl,
            Breakpoint::Xl2,
        ]
    }

    /// Check if a breakpoint is active based on screen width.
    pub fn is_breakpoint_active(breakpoint: &Breakpoint, screen_width: u32) -> bool {
        screen_width >= breakpoint.min_width()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breakpoint_prefix() {
        assert_eq!(Breakpoint::Sm.prefix(), "sm");
        assert_eq!(Breakpoint::Md.prefix(), "md");
        assert_eq!(Breakpoint::Lg.prefix(), "lg");
        assert_eq!(Breakpoint::Xl.prefix(), "xl");
        assert_eq!(Breakpoint::Xl2.prefix(), "2xl");
    }

    #[test]
    fn test_breakpoint_min_width() {
        assert_eq!(Breakpoint::Sm.min_width(), 640);
        assert_eq!(Breakpoint::Md.min_width(), 768);
        assert_eq!(Breakpoint::Lg.min_width(), 1024);
        assert_eq!(Breakpoint::Xl.min_width(), 1280);
        assert_eq!(Breakpoint::Xl2.min_width(), 1536);
    }

    #[test]
    fn test_responsive_creation() {
        let responsive = Responsive::new()
            .sm("text-sm")
            .md("text-base")
            .lg("text-lg");

        let result = responsive.to_string();
        assert!(result.contains("sm:text-sm"));
        assert!(result.contains("md:text-base"));
        assert!(result.contains("lg:text-lg"));
    }

    #[test]
    fn test_responsive_builder() {
        let responsive = responsive()
            .sm("p-2")
            .md("p-4")
            .lg("p-6")
            .build();

        let result = responsive.to_string();
        assert!(result.contains("sm:p-2"));
        assert!(result.contains("md:p-4"));
        assert!(result.contains("lg:p-6"));
    }

    #[test]
    fn test_responsive_merge() {
        let responsive1 = Responsive::new()
            .sm("text-sm")
            .md("text-base");
        
        let responsive2 = Responsive::new()
            .lg("text-lg")
            .xl("text-xl");

        let merged = responsive1.merge(responsive2);
        let result = merged.to_string();
        
        assert!(result.contains("sm:text-sm"));
        assert!(result.contains("md:text-base"));
        assert!(result.contains("lg:text-lg"));
        assert!(result.contains("xl:text-xl"));
    }

    #[test]
    fn test_patterns() {
        let text_sizing = patterns::text_sizing();
        let result = text_sizing.to_string();
        
        assert!(result.contains("sm:text-sm"));
        assert!(result.contains("md:text-base"));
        assert!(result.contains("lg:text-lg"));
        assert!(result.contains("xl:text-xl"));
    }

    #[test]
    fn test_utils_from_string() {
        let responsive = utils::from_string("sm:text-sm md:text-base lg:text-lg");
        let result = responsive.to_string();
        
        assert!(result.contains("sm:text-sm"));
        assert!(result.contains("md:text-base"));
        assert!(result.contains("lg:text-lg"));
    }

    #[test]
    fn test_utils_is_breakpoint_active() {
        assert!(utils::is_breakpoint_active(&Breakpoint::Sm, 640));
        assert!(utils::is_breakpoint_active(&Breakpoint::Sm, 800));
        assert!(!utils::is_breakpoint_active(&Breakpoint::Sm, 500));
        
        assert!(utils::is_breakpoint_active(&Breakpoint::Md, 768));
        assert!(utils::is_breakpoint_active(&Breakpoint::Md, 1000));
        assert!(!utils::is_breakpoint_active(&Breakpoint::Md, 700));
    }
}
