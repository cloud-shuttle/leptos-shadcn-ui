//! Simple integration example showing tailwind-rs-core usage.

use tailwind_rs_core::*;

fn main() {
    println!("🎨 Tailwind-RS-Core Integration Example");
    println!("=====================================");

    // 1. Basic class generation
    println!("\n1. Basic Class Generation:");
    let basic_classes = TailwindClasses::new("px-4 py-2")
        .variant("primary", "bg-blue-600 text-white")
        .responsive("sm", "text-sm")
        .state("hover", "hover:bg-blue-700");
    
    println!("   Classes: {}", basic_classes.to_string());

    // 2. Color system usage
    println!("\n2. Color System:");
    let color = Color::Blue;
    println!("   Background: {}", color.background(600));
    println!("   Text: {}", color.text(900));
    println!("   Hover: {}", color.hover(700));
    println!("   Primary: {}", color.primary());

    // 3. Responsive design
    println!("\n3. Responsive Design:");
    let responsive = Responsive::new()
        .sm("text-sm")
        .md("text-base")
        .lg("text-lg")
        .xl("text-xl");
    
    println!("   Responsive classes: {}", responsive.to_string());

    // 4. Theme system
    println!("\n4. Theme System:");
    let theme = Theme::new()
        .with_primary(Color::Blue)
        .with_secondary(Color::Gray);
    
    let primary_classes = theme.get_classes(&Variant::Primary, &Size::Md);
    let secondary_classes = theme.get_classes(&Variant::Secondary, &Size::Md);
    
    println!("   Primary classes: {}", primary_classes);
    println!("   Secondary classes: {}", secondary_classes);

    // 5. Class validation
    println!("\n5. Class Validation:");
    let validator = ClassValidator::new();
    let valid_class = validator.validate_class("bg-blue-600");
    let invalid_class = validator.validate_class("invalid-class");
    
    println!("   'bg-blue-600' is: {:?}", valid_class);
    println!("   'invalid-class' is: {:?}", invalid_class);

    // 6. Class optimization
    println!("\n6. Class Optimization:");
    let classes = "bg-blue-600 text-white bg-blue-600 invalid-class px-4";
    let optimized = optimize_classes(classes);
    println!("   Original: {}", classes);
    println!("   Optimized: {}", optimized);

    // 7. Predefined patterns
    println!("\n7. Predefined Patterns:");
    let text_sizing = patterns::text_sizing();
    let spacing = patterns::spacing();
    let grid = patterns::grid();
    
    println!("   Text sizing: {}", text_sizing.to_string());
    println!("   Spacing: {}", spacing.to_string());
    println!("   Grid: {}", grid.to_string());

    println!("\n✅ All examples completed successfully!");
    println!("\n🚀 Ready for Leptos integration!");
}
