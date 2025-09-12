use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use leptos::prelude::*;
use leptos_shadcn_button::default::{Button, ButtonVariant, ButtonSize};

/// Button Component Performance Benchmarks
/// 
/// TDD Approach: These benchmarks define the performance requirements
/// and will guide the implementation of comprehensive performance testing.

fn benchmark_button_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("button_creation");
    
    // Test different button variants
    let variants = vec![
        ButtonVariant::Default,
        ButtonVariant::Destructive,
        ButtonVariant::Outline,
        ButtonVariant::Secondary,
        ButtonVariant::Ghost,
        ButtonVariant::Link,
    ];
    
    for variant in variants {
        group.bench_with_input(
            BenchmarkId::new("variant", format!("{:?}", variant)),
            &variant,
            |b, variant| {
                b.iter(|| {
                    let _button = Button {
                        variant: Some(*variant),
                        size: Some(ButtonSize::Default),
                        disabled: Signal::derive(|| false),
                        on_click: None,
                        class: MaybeProp::from("benchmark-button"),
                        id: MaybeProp::from("benchmark-button"),
                        style: Signal::derive(|| leptos_style::Style::default()),
                        children: Some(Children::new(|_| view! { "Benchmark Button" })),
                    };
                    black_box(_button);
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_button_rendering(c: &mut Criterion) {
    let mut group = c.benchmark_group("button_rendering");
    
    // Test different button sizes
    let sizes = vec![
        ButtonSize::Sm,
        ButtonSize::Default,
        ButtonSize::Lg,
        ButtonSize::Icon,
    ];
    
    for size in sizes {
        group.bench_with_input(
            BenchmarkId::new("size", format!("{:?}", size)),
            &size,
            |b, size| {
                b.iter(|| {
                    let button = Button {
                        variant: Some(ButtonVariant::Default),
                        size: Some(*size),
                        disabled: Signal::derive(|| false),
                        on_click: None,
                        class: MaybeProp::from("benchmark-button"),
                        id: MaybeProp::from("benchmark-button"),
                        style: Signal::derive(|| leptos_style::Style::default()),
                        children: Some(Children::new(|_| view! { "Benchmark Button" })),
                    };
                    
                    // Simulate rendering by calling into_view
                    let _view = button.into_view();
                    black_box(_view);
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_button_state_changes(c: &mut Criterion) {
    let mut group = c.benchmark_group("button_state_changes");
    
    // Test disabled state changes
    group.bench_function("disabled_toggle", |b| {
        let disabled_signal = RwSignal::new(false);
        let button = Button {
            variant: Some(ButtonVariant::Default),
            size: Some(ButtonSize::Default),
            disabled: disabled_signal.into(),
            on_click: None,
            class: MaybeProp::from("benchmark-button"),
            id: MaybeProp::from("benchmark-button"),
            style: Signal::derive(|| leptos_style::Style::default()),
            children: Some(Children::new(|_| view! { "Benchmark Button" })),
        };
        
        b.iter(|| {
            disabled_signal.set(!disabled_signal.get());
            black_box(button.disabled.get());
        });
    });
    
    // Test class changes
    group.bench_function("class_changes", |b| {
        let class_signal = RwSignal::new("benchmark-button".to_string());
        let button = Button {
            variant: Some(ButtonVariant::Default),
            size: Some(ButtonSize::Default),
            disabled: Signal::derive(|| false),
            on_click: None,
            class: MaybeProp::from(class_signal),
            id: MaybeProp::from("benchmark-button"),
            style: Signal::derive(|| leptos_style::Style::default()),
            children: Some(Children::new(|_| view! { "Benchmark Button" })),
        };
        
        b.iter(|| {
            class_signal.set(format!("benchmark-button-{}", rand::random::<u32>()));
            black_box(button.class.get());
        });
    });
    
    group.finish();
}

fn benchmark_button_click_handling(c: &mut Criterion) {
    let mut group = c.benchmark_group("button_click_handling");
    
    // Test click callback performance
    group.bench_function("click_callback", |b| {
        let click_count = RwSignal::new(0);
        let callback = Callback::new(move |_| {
            click_count.set(click_count.get() + 1);
        });
        
        let button = Button {
            variant: Some(ButtonVariant::Default),
            size: Some(ButtonSize::Default),
            disabled: Signal::derive(|| false),
            on_click: Some(callback),
            class: MaybeProp::from("benchmark-button"),
            id: MaybeProp::from("benchmark-button"),
            style: Signal::derive(|| leptos_style::Style::default()),
            children: Some(Children::new(|_| view! { "Benchmark Button" })),
        };
        
        b.iter(|| {
            if let Some(callback) = &button.on_click {
                callback.call(());
            }
            black_box(click_count.get());
        });
    });
    
    // Test rapid clicks
    group.bench_function("rapid_clicks", |b| {
        let click_count = RwSignal::new(0);
        let callback = Callback::new(move |_| {
            click_count.set(click_count.get() + 1);
        });
        
        let button = Button {
            variant: Some(ButtonVariant::Default),
            size: Some(ButtonSize::Default),
            disabled: Signal::derive(|| false),
            on_click: Some(callback),
            class: MaybeProp::from("benchmark-button"),
            id: MaybeProp::from("benchmark-button"),
            style: Signal::derive(|| leptos_style::Style::default()),
            children: Some(Children::new(|_| view! { "Benchmark Button" })),
        };
        
        b.iter(|| {
            for _ in 0..100 {
                if let Some(callback) = &button.on_click {
                    callback.call(());
                }
            }
            black_box(click_count.get());
        });
    });
    
    group.finish();
}

fn benchmark_button_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("button_memory_usage");
    
    // Test memory usage for multiple buttons
    group.bench_function("multiple_buttons", |b| {
        b.iter(|| {
            let mut buttons = Vec::new();
            for i in 0..1000 {
                let button = Button {
                    variant: Some(ButtonVariant::Default),
                    size: Some(ButtonSize::Default),
                    disabled: Signal::derive(|| false),
                    on_click: None,
                    class: MaybeProp::from(format!("benchmark-button-{}", i)),
                    id: MaybeProp::from(format!("benchmark-button-{}", i)),
                    style: Signal::derive(|| leptos_style::Style::default()),
                    children: Some(Children::new(move |_| view! { format!("Button {}", i) })),
                };
                buttons.push(button);
            }
            black_box(buttons);
        });
    });
    
    // Test memory usage for buttons with complex children
    group.bench_function("complex_children", |b| {
        b.iter(|| {
            let button = Button {
                variant: Some(ButtonVariant::Default),
                size: Some(ButtonSize::Default),
                disabled: Signal::derive(|| false),
                on_click: None,
                class: MaybeProp::from("benchmark-button"),
                id: MaybeProp::from("benchmark-button"),
                style: Signal::derive(|| leptos_style::Style::default()),
                children: Some(Children::new(|_| {
                    view! {
                        <div>
                            <span>"Complex Button Content"</span>
                            <div>
                                <span>"Nested Content"</span>
                                <span>"More Nested Content"</span>
                            </div>
                        </div>
                    }
                })),
            };
            black_box(button);
        });
    });
    
    group.finish();
}

fn benchmark_button_accessibility(c: &mut Criterion) {
    let mut group = c.benchmark_group("button_accessibility");
    
    // Test accessibility attribute generation
    group.bench_function("accessibility_attributes", |b| {
        b.iter(|| {
            let button = Button {
                variant: Some(ButtonVariant::Default),
                size: Some(ButtonSize::Default),
                disabled: Signal::derive(|| false),
                on_click: None,
                class: MaybeProp::from("benchmark-button"),
                id: MaybeProp::from("benchmark-button"),
                style: Signal::derive(|| leptos_style::Style::default()),
                children: Some(Children::new(|_| view! { "Accessible Button" })),
            };
            
            // Simulate accessibility attribute generation
            let _aria_label = "Accessible Button";
            let _role = "button";
            let _tabindex = 0;
            
            black_box((_aria_label, _role, _tabindex));
        });
    });
    
    // Test keyboard navigation performance
    group.bench_function("keyboard_navigation", |b| {
        let buttons = (0..100).map(|i| {
            Button {
                variant: Some(ButtonVariant::Default),
                size: Some(ButtonSize::Default),
                disabled: Signal::derive(|| false),
                on_click: None,
                class: MaybeProp::from(format!("benchmark-button-{}", i)),
                id: MaybeProp::from(format!("benchmark-button-{}", i)),
                style: Signal::derive(|| leptos_style::Style::default()),
                children: Some(Children::new(move |_| view! { format!("Button {}", i) })),
            }
        }).collect::<Vec<_>>();
        
        b.iter(|| {
            // Simulate tab navigation through buttons
            for i in 0..100 {
                let button = &buttons[i % buttons.len()];
                black_box(button.id.get());
            }
        });
    });
    
    group.finish();
}

// Custom benchmark configuration
criterion_group!(
    name = button_benches;
    config = Criterion::default()
        .sample_size(1000)
        .measurement_time(std::time::Duration::from_secs(10))
        .warm_up_time(std::time::Duration::from_secs(2))
        .noise_threshold(0.05);
    targets = 
        benchmark_button_creation,
        benchmark_button_rendering,
        benchmark_button_state_changes,
        benchmark_button_click_handling,
        benchmark_button_memory_usage,
        benchmark_button_accessibility
);

criterion_main!(button_benches);
