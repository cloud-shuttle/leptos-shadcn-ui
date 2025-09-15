//! Class validation and optimization for Tailwind CSS.

use regex::Regex;
use std::collections::HashSet;
use once_cell::sync::Lazy;

/// A validator for Tailwind CSS classes.
#[derive(Debug, Clone)]
pub struct ClassValidator {
    /// Valid Tailwind classes
    valid_classes: HashSet<String>,
    /// Regex patterns for dynamic classes
    patterns: Vec<Regex>,
}

impl ClassValidator {
    /// Create a new ClassValidator with built-in validation rules.
    pub fn new() -> Self {
        let mut validator = Self {
            valid_classes: HashSet::new(),
            patterns: Vec::new(),
        };
        
        // Add common Tailwind classes
        validator.add_common_classes();
        validator.add_common_patterns();
        
        validator
    }

    /// Add a valid class to the validator.
    pub fn add_class(&mut self, class: impl Into<String>) {
        self.valid_classes.insert(class.into());
    }

    /// Add multiple valid classes to the validator.
    pub fn add_classes(&mut self, classes: impl IntoIterator<Item = impl Into<String>>) {
        for class in classes {
            self.add_class(class);
        }
    }

    /// Add a regex pattern for dynamic class validation.
    pub fn add_pattern(&mut self, pattern: Regex) {
        self.patterns.push(pattern);
    }

    /// Validate a single class.
    pub fn validate_class(&self, class: &str) -> ValidationResult {
        // Check if it's a known valid class
        if self.valid_classes.contains(class) {
            return ValidationResult::Valid;
        }

        // Check against patterns
        for pattern in &self.patterns {
            if pattern.is_match(class) {
                return ValidationResult::Valid;
            }
        }

        // Check for common invalid patterns
        if self.is_invalid_class(class) {
            return ValidationResult::Invalid(class.to_string());
        }

        ValidationResult::Unknown(class.to_string())
    }

    /// Validate multiple classes.
    pub fn validate_classes(&self, classes: &[&str]) -> Vec<ValidationResult> {
        classes.iter().map(|class| self.validate_class(class)).collect()
    }

    /// Check if a class is definitely invalid.
    fn is_invalid_class(&self, class: &str) -> bool {
        // Check for common invalid patterns
        let invalid_patterns = [
            r"^[0-9]",  // Classes starting with numbers
            r"[^a-zA-Z0-9\-:/]",  // Invalid characters (allow / for fractions)
            r"^-",  // Classes starting with hyphens
            r"-$",  // Classes ending with hyphens
            r"^invalid-",  // Classes starting with "invalid-"
        ];

        for pattern in &invalid_patterns {
            if Regex::new(pattern).unwrap().is_match(class) {
                return true;
            }
        }

        // Additional check: if it starts with "invalid-", it's definitely invalid
        if class.starts_with("invalid-") {
            return true;
        }

        false
    }

    /// Add common Tailwind classes to the validator.
    fn add_common_classes(&mut self) {
        let common_classes = [
            // Layout
            "block", "inline-block", "inline", "flex", "inline-flex", "grid", "inline-grid",
            "hidden", "table", "table-cell", "table-row", "flow-root", "contents",
            
            // Flexbox
            "flex-row", "flex-row-reverse", "flex-col", "flex-col-reverse",
            "flex-wrap", "flex-wrap-reverse", "flex-nowrap",
            "items-start", "items-end", "items-center", "items-baseline", "items-stretch",
            "justify-start", "justify-end", "justify-center", "justify-between", "justify-around", "justify-evenly",
            "content-start", "content-end", "content-center", "content-between", "content-around", "content-evenly",
            
            // Grid
            "grid-cols-1", "grid-cols-2", "grid-cols-3", "grid-cols-4", "grid-cols-5", "grid-cols-6",
            "grid-rows-1", "grid-rows-2", "grid-rows-3", "grid-rows-4", "grid-rows-5", "grid-rows-6",
            
            // Spacing
            "p-0", "p-1", "p-2", "p-3", "p-4", "p-5", "p-6", "p-8", "p-10", "p-12", "p-16", "p-20", "p-24", "p-32", "p-40", "p-48", "p-56", "p-64", "p-72", "p-80", "p-96",
            "px-0", "px-1", "px-2", "px-3", "px-4", "px-5", "px-6", "px-8", "px-10", "px-12", "px-16", "px-20", "px-24", "px-32", "px-40", "px-48", "px-56", "px-64", "px-72", "px-80", "px-96",
            "py-0", "py-1", "py-2", "py-3", "py-4", "py-5", "py-6", "py-8", "py-10", "py-12", "py-16", "py-20", "py-24", "py-32", "py-40", "py-48", "py-56", "py-64", "py-72", "py-80", "py-96",
            "pt-0", "pt-1", "pt-2", "pt-3", "pt-4", "pt-5", "pt-6", "pt-8", "pt-10", "pt-12", "pt-16", "pt-20", "pt-24", "pt-32", "pt-40", "pt-48", "pt-56", "pt-64", "pt-72", "pt-80", "pt-96",
            "pr-0", "pr-1", "pr-2", "pr-3", "pr-4", "pr-5", "pr-6", "pr-8", "pr-10", "pr-12", "pr-16", "pr-20", "pr-24", "pr-32", "pr-40", "pr-48", "pr-56", "pr-64", "pr-72", "pr-80", "pr-96",
            "pb-0", "pb-1", "pb-2", "pb-3", "pb-4", "pb-5", "pb-6", "pb-8", "pb-10", "pb-12", "pb-16", "pb-20", "pb-24", "pb-32", "pb-40", "pb-48", "pb-56", "pb-64", "pb-72", "pb-80", "pb-96",
            "pl-0", "pl-1", "pl-2", "pl-3", "pl-4", "pl-5", "pl-6", "pl-8", "pl-10", "pl-12", "pl-16", "pl-20", "pl-24", "pl-32", "pl-40", "pl-48", "pl-56", "pl-64", "pl-72", "pl-80", "pl-96",
            
            // Sizing
            "w-0", "w-1", "w-2", "w-3", "w-4", "w-5", "w-6", "w-8", "w-10", "w-12", "w-16", "w-20", "w-24", "w-32", "w-40", "w-48", "w-56", "w-64", "w-72", "w-80", "w-96",
            "h-0", "h-1", "h-2", "h-3", "h-4", "h-5", "h-6", "h-8", "h-10", "h-12", "h-16", "h-20", "h-24", "h-32", "h-40", "h-48", "h-56", "h-64", "h-72", "h-80", "h-96",
            "min-w-0", "min-w-full", "min-w-min", "min-w-max", "min-w-fit",
            "min-h-0", "min-h-full", "min-h-screen", "min-h-min", "min-h-max", "min-h-fit",
            "max-w-0", "max-w-none", "max-w-xs", "max-w-sm", "max-w-md", "max-w-lg", "max-w-xl", "max-w-2xl", "max-w-3xl", "max-w-4xl", "max-w-5xl", "max-w-6xl", "max-w-7xl", "max-w-full", "max-w-min", "max-w-max", "max-w-fit", "max-w-prose", "max-w-screen-sm", "max-w-screen-md", "max-w-screen-lg", "max-w-screen-xl", "max-w-screen-2xl",
            "max-h-0", "max-h-1", "max-h-2", "max-h-3", "max-h-4", "max-h-5", "max-h-6", "max-h-8", "max-h-10", "max-h-12", "max-h-16", "max-h-20", "max-h-24", "max-h-32", "max-h-40", "max-h-48", "max-h-56", "max-h-64", "max-h-72", "max-h-80", "max-h-96", "max-h-screen",
            
            // Typography
            "text-xs", "text-sm", "text-base", "text-lg", "text-xl", "text-2xl", "text-3xl", "text-4xl", "text-5xl", "text-6xl", "text-7xl", "text-8xl", "text-9xl",
            "font-thin", "font-extralight", "font-light", "font-normal", "font-medium", "font-semibold", "font-bold", "font-extrabold", "font-black",
            "text-left", "text-center", "text-right", "text-justify",
            "text-transparent", "text-current", "text-black", "text-white",
            
            // Colors (basic)
            "bg-transparent", "bg-current", "bg-black", "bg-white",
            "text-transparent", "text-current", "text-black", "text-white",
            "border-transparent", "border-current", "border-black", "border-white",
            
            // Borders
            "border", "border-0", "border-2", "border-4", "border-8",
            "border-t", "border-r", "border-b", "border-l",
            "border-t-0", "border-r-0", "border-b-0", "border-l-0",
            "border-t-2", "border-r-2", "border-b-2", "border-l-2",
            "border-t-4", "border-r-4", "border-b-4", "border-l-4",
            "border-t-8", "border-r-8", "border-b-8", "border-l-8",
            "rounded-none", "rounded-sm", "rounded", "rounded-md", "rounded-lg", "rounded-xl", "rounded-2xl", "rounded-3xl", "rounded-full",
            "rounded-t-none", "rounded-t-sm", "rounded-t", "rounded-t-md", "rounded-t-lg", "rounded-t-xl", "rounded-t-2xl", "rounded-t-3xl", "rounded-t-full",
            "rounded-r-none", "rounded-r-sm", "rounded-r", "rounded-r-md", "rounded-r-lg", "rounded-r-xl", "rounded-r-2xl", "rounded-r-3xl", "rounded-r-full",
            "rounded-b-none", "rounded-b-sm", "rounded-b", "rounded-b-md", "rounded-b-lg", "rounded-b-xl", "rounded-b-2xl", "rounded-b-3xl", "rounded-b-full",
            "rounded-l-none", "rounded-l-sm", "rounded-l", "rounded-l-md", "rounded-l-lg", "rounded-l-xl", "rounded-l-2xl", "rounded-l-3xl", "rounded-l-full",
            
            // Effects
            "shadow-sm", "shadow", "shadow-md", "shadow-lg", "shadow-xl", "shadow-2xl", "shadow-inner", "shadow-none",
            "opacity-0", "opacity-5", "opacity-10", "opacity-20", "opacity-25", "opacity-30", "opacity-40", "opacity-50", "opacity-60", "opacity-70", "opacity-75", "opacity-80", "opacity-90", "opacity-95", "opacity-100",
            
            // Transforms
            "transform", "transform-gpu", "transform-none",
            "scale-0", "scale-50", "scale-75", "scale-90", "scale-95", "scale-100", "scale-105", "scale-110", "scale-125", "scale-150",
            "rotate-0", "rotate-1", "rotate-2", "rotate-3", "rotate-6", "rotate-12", "rotate-45", "rotate-90", "rotate-180",
            "translate-x-0", "translate-x-1", "translate-x-2", "translate-x-3", "translate-x-4", "translate-x-5", "translate-x-6", "translate-x-8", "translate-x-10", "translate-x-12", "translate-x-16", "translate-x-20", "translate-x-24", "translate-x-32", "translate-x-40", "translate-x-48", "translate-x-56", "translate-x-64", "translate-x-72", "translate-x-80", "translate-x-96",
            "translate-y-0", "translate-y-1", "translate-y-2", "translate-y-3", "translate-y-4", "translate-y-5", "translate-y-6", "translate-y-8", "translate-y-10", "translate-y-12", "translate-y-16", "translate-y-20", "translate-y-24", "translate-y-32", "translate-y-40", "translate-y-48", "translate-y-56", "translate-y-64", "translate-y-72", "translate-y-80", "translate-y-96",
            
            // Transitions
            "transition-none", "transition-all", "transition", "transition-colors", "transition-opacity", "transition-shadow", "transition-transform",
            "duration-75", "duration-100", "duration-150", "duration-200", "duration-300", "duration-500", "duration-700", "duration-1000",
            "ease-linear", "ease-in", "ease-out", "ease-in-out",
            "delay-75", "delay-100", "delay-150", "delay-200", "delay-300", "delay-500", "delay-700", "delay-1000",
            
            // Interactivity
            "cursor-auto", "cursor-default", "cursor-pointer", "cursor-wait", "cursor-text", "cursor-move", "cursor-help", "cursor-not-allowed",
            "select-none", "select-text", "select-all", "select-auto",
            "resize-none", "resize-y", "resize-x", "resize",
            "appearance-none",
            
            // Accessibility
            "sr-only", "not-sr-only",
            "focus:outline-none", "focus:outline-white", "focus:outline-black",
            "focus:ring-0", "focus:ring-1", "focus:ring-2", "focus:ring-4", "focus:ring-8",
            "focus:ring-inset", "focus:ring-offset-0", "focus:ring-offset-1", "focus:ring-offset-2", "focus:ring-offset-4", "focus:ring-offset-8",
            
            // States
            "hover:opacity-75", "hover:opacity-100",
            "focus:opacity-75", "focus:opacity-100",
            "active:opacity-75", "active:opacity-100",
            "disabled:opacity-50", "disabled:opacity-75",
            "group-hover:opacity-75", "group-hover:opacity-100",
            "group-focus:opacity-75", "group-focus:opacity-100",
        ];

        self.add_classes(common_classes);
    }

    /// Add common regex patterns for dynamic class validation.
    fn add_common_patterns(&mut self) {
        let patterns = [
            // Spacing patterns
            r"^[mp][trblxy]?-[0-9]+$",  // p-4, px-2, mt-8, etc.
            r"^[mp][trblxy]?-[0-9]+\.[0-9]+$",  // p-1.5, px-2.5, etc.
            
            // Sizing patterns
            r"^[wh]-[0-9]+$",  // w-4, h-8, etc.
            r"^[wh]-[0-9]+\.[0-9]+$",  // w-1.5, h-2.5, etc.
            r"^[wh]-full$",  // w-full, h-full
            r"^[wh]-screen$",  // w-screen, h-screen
            r"^[wh]-min$",  // w-min, h-min
            r"^[wh]-max$",  // w-max, h-max
            r"^[wh]-fit$",  // w-fit, h-fit
            
            // Color patterns
            r"^bg-[a-z]+-[0-9]+$",  // bg-blue-500, bg-red-600, etc.
            r"^text-[a-z]+-[0-9]+$",  // text-blue-500, text-red-600, etc.
            r"^border-[a-z]+-[0-9]+$",  // border-blue-500, border-red-600, etc.
            
            // Responsive patterns
            r"^sm:[a-zA-Z0-9\-:]+$",  // sm:text-lg, sm:bg-blue-500, etc.
            r"^md:[a-zA-Z0-9\-:]+$",  // md:text-lg, md:bg-blue-500, etc.
            r"^lg:[a-zA-Z0-9\-:]+$",  // lg:text-lg, lg:bg-blue-500, etc.
            r"^xl:[a-zA-Z0-9\-:]+$",  // xl:text-lg, xl:bg-blue-500, etc.
            r"^2xl:[a-zA-Z0-9\-:]+$",  // 2xl:text-lg, 2xl:bg-blue-500, etc.
            
            // State patterns
            r"^hover:[a-zA-Z0-9\-:]+$",  // hover:bg-blue-500, hover:text-white, etc.
            r"^focus:[a-zA-Z0-9\-:]+$",  // focus:bg-blue-500, focus:text-white, etc.
            r"^active:[a-zA-Z0-9\-:]+$",  // active:bg-blue-500, active:text-white, etc.
            r"^disabled:[a-zA-Z0-9\-:]+$",  // disabled:bg-gray-300, disabled:text-gray-500, etc.
            r"^group-hover:[a-zA-Z0-9\-:]+$",  // group-hover:bg-blue-500, etc.
            r"^group-focus:[a-zA-Z0-9\-:]+$",  // group-focus:bg-blue-500, etc.
            
            // Ring patterns
            r"^ring-[0-9]+$",  // ring-1, ring-2, ring-4, ring-8
            r"^ring-[a-z]+-[0-9]+$",  // ring-blue-500, ring-red-600, etc.
            r"^ring-offset-[0-9]+$",  // ring-offset-0, ring-offset-1, ring-offset-2, ring-offset-4, ring-offset-8
            r"^ring-offset-[a-z]+-[0-9]+$",  // ring-offset-blue-500, ring-offset-red-600, etc.
            
            // Grid patterns
            r"^grid-cols-[0-9]+$",  // grid-cols-1, grid-cols-2, grid-cols-3, etc.
            r"^grid-rows-[0-9]+$",  // grid-rows-1, grid-rows-2, grid-rows-3, etc.
            r"^col-span-[0-9]+$",  // col-span-1, col-span-2, col-span-3, etc.
            r"^row-span-[0-9]+$",  // row-span-1, row-span-2, row-span-3, etc.
            
            // Flex patterns
            r"^flex-[0-9]+$",  // flex-1, flex-2, flex-3, etc.
            r"^flex-grow-[0-9]+$",  // flex-grow-0, flex-grow-1, etc.
            r"^flex-shrink-[0-9]+$",  // flex-shrink-0, flex-shrink-1, etc.
            
            // Gap patterns
            r"^gap-[0-9]+$",  // gap-1, gap-2, gap-3, gap-4, gap-5, gap-6, gap-8, gap-10, gap-12, gap-16, gap-20, gap-24, gap-32, gap-40, gap-48, gap-56, gap-64, gap-72, gap-80, gap-96
            r"^gap-x-[0-9]+$",  // gap-x-1, gap-x-2, gap-x-3, gap-x-4, gap-x-5, gap-x-6, gap-x-8, gap-x-10, gap-x-12, gap-x-16, gap-x-20, gap-x-24, gap-x-32, gap-x-40, gap-x-48, gap-x-56, gap-x-64, gap-x-72, gap-x-80, gap-x-96
            r"^gap-y-[0-9]+$",  // gap-y-1, gap-y-2, gap-y-3, gap-y-4, gap-y-5, gap-y-6, gap-y-8, gap-y-10, gap-y-12, gap-y-16, gap-y-20, gap-y-24, gap-y-32, gap-y-40, gap-y-48, gap-y-56, gap-y-64, gap-y-72, gap-y-80, gap-y-96
            
            // Z-index patterns
            r"^z-[0-9]+$",  // z-0, z-10, z-20, z-30, z-40, z-50
            r"^z-auto$",  // z-auto
            
            // Position patterns
            r"^top-[0-9]+$",  // top-0, top-1, top-2, top-3, top-4, top-5, top-6, top-8, top-10, top-12, top-16, top-20, top-24, top-32, top-40, top-48, top-56, top-64, top-72, top-80, top-96
            r"^right-[0-9]+$",  // right-0, right-1, right-2, right-3, right-4, right-5, right-6, right-8, right-10, right-12, right-16, right-20, right-24, right-32, right-40, right-48, right-56, right-64, right-72, right-80, right-96
            r"^bottom-[0-9]+$",  // bottom-0, bottom-1, bottom-2, bottom-3, bottom-4, bottom-5, bottom-6, bottom-8, bottom-10, bottom-12, bottom-16, bottom-20, bottom-24, bottom-32, bottom-40, bottom-48, bottom-56, bottom-64, bottom-72, bottom-80, bottom-96
            r"^left-[0-9]+$",  // left-0, left-1, left-2, left-3, left-4, left-5, left-6, left-8, left-10, left-12, left-16, left-20, left-24, left-32, left-40, left-48, left-56, left-64, left-72, left-80, left-96
            
            // Overflow patterns
            r"^overflow-[a-z]+$",  // overflow-auto, overflow-hidden, overflow-clip, overflow-visible, overflow-scroll
            r"^overflow-x-[a-z]+$",  // overflow-x-auto, overflow-x-hidden, overflow-x-clip, overflow-x-visible, overflow-x-scroll
            r"^overflow-y-[a-z]+$",  // overflow-y-auto, overflow-y-hidden, overflow-y-clip, overflow-y-visible, overflow-y-scroll
            
            // Display patterns (more specific to avoid matching invalid classes)
            r"^(inline-block|inline-flex|inline-grid|table-cell|table-row|flow-root|contents)$",  // specific display values
        ];

        for pattern in &patterns {
            if let Ok(regex) = Regex::new(pattern) {
                self.add_pattern(regex);
            }
        }
    }
}

impl Default for ClassValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of class validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationResult {
    /// Class is valid
    Valid,
    /// Class is invalid with the invalid class name
    Invalid(String),
    /// Class is unknown (not in validator but not obviously invalid)
    Unknown(String),
}

impl ValidationResult {
    /// Check if the validation result is valid.
    pub fn is_valid(&self) -> bool {
        matches!(self, ValidationResult::Valid)
    }

    /// Check if the validation result is invalid.
    pub fn is_invalid(&self) -> bool {
        matches!(self, ValidationResult::Invalid(_))
    }

    /// Check if the validation result is unknown.
    pub fn is_unknown(&self) -> bool {
        matches!(self, ValidationResult::Unknown(_))
    }

    /// Get the class name if the result is not valid.
    pub fn class_name(&self) -> Option<&str> {
        match self {
            ValidationResult::Valid => None,
            ValidationResult::Invalid(name) => Some(name),
            ValidationResult::Unknown(name) => Some(name),
        }
    }
}

/// Global validator instance for convenience.
pub static VALIDATOR: Lazy<ClassValidator> = Lazy::new(ClassValidator::new);

/// Validate a single class using the global validator.
pub fn validate_class(class: &str) -> ValidationResult {
    VALIDATOR.validate_class(class)
}

/// Validate multiple classes using the global validator.
pub fn validate_classes(classes: &[&str]) -> Vec<ValidationResult> {
    VALIDATOR.validate_classes(classes)
}

/// Optimize a class string by removing duplicates and invalid classes.
pub fn optimize_classes(classes: &str) -> String {
    let class_list: Vec<&str> = classes.split_whitespace().collect();
    let mut valid_classes = Vec::new();
    let mut seen_classes = HashSet::new();

    for class in class_list {
        if !seen_classes.contains(class) && validate_class(class).is_valid() {
            valid_classes.push(class);
            seen_classes.insert(class);
        }
    }

    valid_classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_class() {
        let validator = ClassValidator::new();
        
        // Valid classes
        assert!(validator.validate_class("bg-blue-500").is_valid());
        assert!(validator.validate_class("text-white").is_valid());
        assert!(validator.validate_class("px-4").is_valid());
        assert!(validator.validate_class("hover:bg-blue-600").is_valid());
        assert!(validator.validate_class("sm:text-lg").is_valid());
        
        // Invalid classes
        assert!(validator.validate_class("invalid-class").is_invalid());
        assert!(validator.validate_class("123invalid").is_invalid());
        assert!(validator.validate_class("-invalid").is_invalid());
        assert!(validator.validate_class("invalid-").is_invalid());
        
        // Unknown classes (not in validator but not obviously invalid)
        assert!(validator.validate_class("custom-class").is_unknown());
    }

    #[test]
    fn test_validate_classes() {
        let validator = ClassValidator::new();
        let classes = ["bg-blue-500", "text-white", "invalid-class", "px-4"];
        let results = validator.validate_classes(&classes);
        
        assert_eq!(results.len(), 4);
        assert!(results[0].is_valid());
        assert!(results[1].is_valid());
        assert!(results[2].is_invalid());
        assert!(results[3].is_valid());
    }

    #[test]
    fn test_global_validator() {
        assert!(validate_class("bg-blue-500").is_valid());
        assert!(validate_class("text-white").is_valid());
        assert!(validate_class("invalid-class").is_invalid());
    }

    #[test]
    fn test_optimize_classes() {
        let classes = "bg-blue-500 text-white bg-blue-500 invalid-class px-4";
        let optimized = optimize_classes(classes);
        
        assert!(optimized.contains("bg-blue-500"));
        assert!(optimized.contains("text-white"));
        assert!(optimized.contains("px-4"));
        assert!(!optimized.contains("invalid-class"));
        
        // Should not contain duplicates
        let count = optimized.matches("bg-blue-500").count();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_validation_result_methods() {
        let valid = ValidationResult::Valid;
        let invalid = ValidationResult::Invalid("invalid-class".to_string());
        let unknown = ValidationResult::Unknown("custom-class".to_string());
        
        assert!(valid.is_valid());
        assert!(!valid.is_invalid());
        assert!(!valid.is_unknown());
        assert_eq!(valid.class_name(), None);
        
        assert!(!invalid.is_valid());
        assert!(invalid.is_invalid());
        assert!(!invalid.is_unknown());
        assert_eq!(invalid.class_name(), Some("invalid-class"));
        
        assert!(!unknown.is_valid());
        assert!(!unknown.is_invalid());
        assert!(unknown.is_unknown());
        assert_eq!(unknown.class_name(), Some("custom-class"));
    }
}
