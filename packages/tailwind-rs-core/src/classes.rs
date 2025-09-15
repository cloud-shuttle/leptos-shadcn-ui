//! Type-safe class generation and management.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// A type-safe Tailwind class container that provides compile-time validation
/// and runtime optimization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TailwindClasses {
    /// Base classes that are always applied
    pub base: String,
    /// Variant-specific classes
    pub variants: HashMap<String, String>,
    /// Responsive classes
    pub responsive: HashMap<String, String>,
    /// State classes (hover, focus, etc.)
    pub states: HashMap<String, String>,
    /// Custom classes
    pub custom: Vec<String>,
}

impl TailwindClasses {
    /// Create a new TailwindClasses instance with base classes.
    pub fn new(base: impl Into<String>) -> Self {
        Self {
            base: base.into(),
            variants: HashMap::new(),
            responsive: HashMap::new(),
            states: HashMap::new(),
            custom: Vec::new(),
        }
    }

    /// Add a variant class.
    pub fn variant(mut self, name: impl Into<String>, classes: impl Into<String>) -> Self {
        self.variants.insert(name.into(), classes.into());
        self
    }

    /// Add responsive classes.
    pub fn responsive(mut self, breakpoint: impl Into<String>, classes: impl Into<String>) -> Self {
        self.responsive.insert(breakpoint.into(), classes.into());
        self
    }

    /// Add state classes.
    pub fn state(mut self, state: impl Into<String>, classes: impl Into<String>) -> Self {
        self.states.insert(state.into(), classes.into());
        self
    }

    /// Add custom classes.
    pub fn custom(mut self, classes: impl Into<String>) -> Self {
        self.custom.push(classes.into());
        self
    }

    /// Generate the final class string.
    pub fn to_string(&self) -> String {
        let mut classes = vec![self.base.clone()];
        
        // Add variants
        for variant in self.variants.values() {
            classes.push(variant.clone());
        }
        
        // Add responsive classes with proper breakpoint prefixes
        for (breakpoint, responsive) in &self.responsive {
            classes.push(format!("{}:{}", breakpoint, responsive));
        }
        
        // Add state classes
        for state in self.states.values() {
            classes.push(state.clone());
        }
        
        // Add custom classes
        classes.extend(self.custom.clone());
        
        classes.join(" ")
    }

    /// Merge with another TailwindClasses instance.
    pub fn merge(mut self, other: TailwindClasses) -> Self {
        // Merge base classes
        if !other.base.is_empty() {
            self.base = format!("{} {}", self.base, other.base);
        }
        
        // Merge variants
        for (key, value) in other.variants {
            self.variants.insert(key, value);
        }
        
        // Merge responsive
        for (key, value) in other.responsive {
            self.responsive.insert(key, value);
        }
        
        // Merge states
        for (key, value) in other.states {
            self.states.insert(key, value);
        }
        
        // Merge custom
        self.custom.extend(other.custom);
        
        self
    }
}

impl Default for TailwindClasses {
    fn default() -> Self {
        Self {
            base: String::new(),
            variants: HashMap::new(),
            responsive: HashMap::new(),
            states: HashMap::new(),
            custom: Vec::new(),
        }
    }
}

impl From<String> for TailwindClasses {
    fn from(classes: String) -> Self {
        Self::new(classes)
    }
}

impl From<&str> for TailwindClasses {
    fn from(classes: &str) -> Self {
        Self::new(classes)
    }
}

/// A builder for creating TailwindClasses with a fluent API.
#[derive(Debug, Default)]
pub struct ClassBuilder {
    classes: TailwindClasses,
}

impl ClassBuilder {
    /// Create a new ClassBuilder.
    pub fn new() -> Self {
        Self {
            classes: TailwindClasses::default(),
        }
    }

    /// Set base classes.
    pub fn base(mut self, classes: impl Into<String>) -> Self {
        self.classes.base = classes.into();
        self
    }

    /// Add a variant.
    pub fn variant(mut self, name: impl Into<String>, classes: impl Into<String>) -> Self {
        self.classes = self.classes.variant(name, classes);
        self
    }

    /// Add responsive classes.
    pub fn responsive(mut self, breakpoint: impl Into<String>, classes: impl Into<String>) -> Self {
        self.classes = self.classes.responsive(breakpoint, classes);
        self
    }

    /// Add state classes.
    pub fn state(mut self, state: impl Into<String>, classes: impl Into<String>) -> Self {
        self.classes = self.classes.state(state, classes);
        self
    }

    /// Add custom classes.
    pub fn custom(mut self, classes: impl Into<String>) -> Self {
        self.classes = self.classes.custom(classes);
        self
    }

    /// Build the final TailwindClasses.
    pub fn build(self) -> TailwindClasses {
        self.classes
    }
}

/// Utility function to create a ClassBuilder.
pub fn classes() -> ClassBuilder {
    ClassBuilder::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tailwind_classes_creation() {
        let classes = TailwindClasses::new("px-4 py-2")
            .variant("primary", "bg-blue-600 text-white")
            .responsive("sm", "text-sm")
            .state("hover", "hover:bg-blue-700")
            .custom("rounded-md");

        let result = classes.to_string();
        assert!(result.contains("px-4 py-2"));
        assert!(result.contains("bg-blue-600 text-white"));
        assert!(result.contains("sm:text-sm"));
        assert!(result.contains("hover:bg-blue-700"));
        assert!(result.contains("rounded-md"));
    }

    #[test]
    fn test_class_builder() {
        let classes = classes()
            .base("px-4 py-2")
            .variant("primary", "bg-blue-600 text-white")
            .responsive("sm", "text-sm")
            .build();

        let result = classes.to_string();
        assert!(result.contains("px-4 py-2"));
        assert!(result.contains("bg-blue-600 text-white"));
        assert!(result.contains("sm:text-sm"));
    }

    #[test]
    fn test_classes_merge() {
        let classes1 = TailwindClasses::new("px-4 py-2")
            .variant("primary", "bg-blue-600");
        
        let classes2 = TailwindClasses::new("rounded-md")
            .variant("secondary", "bg-gray-200");

        let merged = classes1.merge(classes2);
        let result = merged.to_string();
        
        assert!(result.contains("px-4 py-2 rounded-md"));
        assert!(result.contains("bg-blue-600"));
        assert!(result.contains("bg-gray-200"));
    }
}
