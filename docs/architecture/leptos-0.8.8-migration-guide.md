# Leptos 0.8.8 Signal Migration Guide

## Overview

This guide provides step-by-step instructions for migrating existing Leptos components to use the new 0.8.8 signal patterns with our signal management utilities.

## Migration Strategy

### Phase 1: Assessment
1. **Identify Components**: List all components that need migration
2. **Analyze Current Usage**: Understand current signal patterns
3. **Plan Migration Order**: Prioritize components by complexity and usage

### Phase 2: Core Migration
1. **Update Signal Types**: Replace old signals with `ArcRwSignal` and `ArcMemo`
2. **Implement Lifecycle Management**: Add signal tracking
3. **Add Memory Management**: Implement memory monitoring

### Phase 3: Optimization
1. **Performance Tuning**: Optimize signal usage patterns
2. **Memory Optimization**: Implement cleanup strategies
3. **Testing**: Comprehensive testing of migrated components

## Step-by-Step Migration Process

### 1. Before Migration

#### Current Component Structure
```rust
// OLD: Traditional Leptos component
#[component]
fn Button(
    #[prop(optional)] variant: Option<ButtonVariant>,
    #[prop(optional)] size: Option<ButtonSize>,
    children: Children,
) -> impl IntoView {
    let (is_loading, set_loading) = signal(false);
    let (is_disabled, set_disabled) = signal(false);
    
    let button_class = move || {
        format!("btn btn-{} btn-{}", 
            variant.unwrap_or_default(),
            size.unwrap_or_default()
        )
    };
    
    view! {
        <button
            class=button_class
            disabled=move || is_disabled.get()
            on:click=move |_| {
                set_loading.set(true);
                // Handle click
                set_loading.set(false);
            }
        >
            {children()}
        </button>
    }
}
```

### 2. After Migration

#### New Component Structure with Signal Management
```rust
use leptos_shadcn_signal_management::*;

#[component]
fn Button(
    #[prop(optional)] variant: Option<ButtonVariant>,
    #[prop(optional)] size: Option<ButtonSize>,
    children: Children,
) -> impl IntoView {
    // Create persistent signals using ArcRwSignal
    let button_state = ArcRwSignal::new(ButtonState {
        variant: variant.unwrap_or_default(),
        size: size.unwrap_or_default(),
        loading: false,
        disabled: false,
    });
    
    // Create computed signal using ArcMemo
    let button_class = ArcMemo::new(move |_| {
        let state = button_state.get();
        format!("btn btn-{} btn-{}", state.variant, state.size)
    });
    
    // Create theme manager for lifecycle management
    let theme_manager = TailwindSignalManager::new();
    theme_manager.track_signal(button_state.clone());
    theme_manager.track_memo(button_class.clone());
    
    // Create memory manager for monitoring
    let memory_manager = SignalMemoryManager::new();
    
    // Create event handler with proper signal management
    let handle_click = {
        let button_state = button_state.clone();
        move |_| {
            if !button_state.get().disabled && !button_state.get().loading {
                button_state.update(|state| {
                    state.loading = true;
                });
                
                // Simulate async operation
                button_state.update(|state| {
                    state.loading = false;
                });
            }
        }
    };
    
    view! {
        <button
            class=move || button_class.get()
            disabled=move || button_state.get().disabled
            on:click=handle_click
        >
            {children()}
        </button>
    }
}

#[derive(Debug, Clone, PartialEq)]
struct ButtonState {
    variant: ButtonVariant,
    size: ButtonSize,
    loading: bool,
    disabled: bool,
}
```

## Component-Specific Migration Examples

### 1. Button Component Migration

#### Before
```rust
let (is_loading, set_loading) = signal(false);
let (is_disabled, set_disabled) = signal(false);
```

#### After
```rust
let button_state = ArcRwSignal::new(ButtonState {
    loading: false,
    disabled: false,
    // ... other state
});
```

### 2. Input Component Migration

#### Before
```rust
let (value, set_value) = signal(String::new());
let (error, set_error) = signal(None::<String>);
```

#### After
```rust
let input_state = ArcRwSignal::new(InputState {
    value: String::new(),
    error: None,
    focused: false,
    // ... other state
});

// Create validation state using ArcMemo
let validation_state = ArcMemo::new(move |_| {
    let state = input_state.get();
    InputValidationState {
        is_valid: state.error.is_none() && !state.value.is_empty(),
        has_error: state.error.is_some(),
        error_message: state.error.clone(),
    }
});
```

### 3. Form Component Migration

#### Before
```rust
let (is_submitting, set_submitting) = signal(false);
let (errors, set_errors) = signal(Vec::new());
```

#### After
```rust
let form_state = ArcRwSignal::new(FormState {
    is_submitting: false,
    errors: Vec::new(),
    fields: HashMap::new(),
    // ... other state
});

// Create form validation using ArcMemo
let form_validation = ArcMemo::new(move |_| {
    let state = form_state.get();
    FormValidationState {
        can_submit: state.is_valid && !state.is_submitting,
        has_errors: !state.errors.is_empty(),
        error_count: state.errors.len(),
    }
});
```

## Migration Checklist

### ✅ Pre-Migration
- [ ] Identify all components to migrate
- [ ] Understand current signal usage patterns
- [ ] Plan migration order and timeline
- [ ] Set up testing environment

### ✅ Core Migration
- [ ] Replace `signal()` with `ArcRwSignal::new()`
- [ ] Replace computed values with `ArcMemo::new()`
- [ ] Add signal lifecycle management
- [ ] Implement memory management
- [ ] Update event handlers

### ✅ Post-Migration
- [ ] Run comprehensive tests
- [ ] Performance benchmarking
- [ ] Memory usage monitoring
- [ ] Documentation updates

## Common Migration Patterns

### 1. State Consolidation
```rust
// OLD: Multiple separate signals
let (loading, set_loading) = signal(false);
let (disabled, set_disabled) = signal(false);
let (variant, set_variant) = signal(ButtonVariant::Default);

// NEW: Consolidated state
let button_state = ArcRwSignal::new(ButtonState {
    loading: false,
    disabled: false,
    variant: ButtonVariant::Default,
});
```

### 2. Computed Values
```rust
// OLD: Function-based computed values
let button_class = move || {
    format!("btn btn-{}", variant.get())
};

// NEW: ArcMemo-based computed values
let button_class = ArcMemo::new(move |_| {
    let state = button_state.get();
    format!("btn btn-{}", state.variant)
});
```

### 3. Event Handlers
```rust
// OLD: Direct signal updates
let handle_click = move |_| {
    set_loading.set(true);
    // ... async operation
    set_loading.set(false);
};

// NEW: State-based updates
let handle_click = {
    let button_state = button_state.clone();
    move |_| {
        button_state.update(|state| {
            state.loading = true;
        });
        // ... async operation
        button_state.update(|state| {
            state.loading = false;
        });
    }
};
```

## Performance Optimization

### 1. Signal Lifecycle Management
```rust
let manager = TailwindSignalManager::new();
manager.track_signal(button_state.clone());
manager.track_memo(button_class.clone());
manager.apply_lifecycle_optimization();
```

### 2. Memory Management
```rust
let memory_manager = SignalMemoryManager::new();

// Monitor memory pressure
if let Some(pressure) = memory_manager.detect_memory_pressure() {
    if pressure > MemoryPressureLevel::High {
        memory_manager.perform_automatic_cleanup();
    }
}
```

### 3. Batched Updates
```rust
let updater = BatchedSignalUpdater::new();
updater.auto_tune_batch_size();
```

## Testing Migration

### 1. Unit Tests
```rust
#[test]
fn test_button_component_migration() {
    let button_component = create_migrated_button_component();
    assert!(button_component.is_some());
}
```

### 2. Integration Tests
```rust
#[test]
fn test_button_integration() {
    let button_state = ArcRwSignal::new(ButtonState::default());
    let button_class = ArcMemo::new(move |_| {
        format!("btn btn-{}", button_state.get().variant)
    });
    
    assert_eq!(button_class.get(), "btn btn-default");
}
```

### 3. Performance Tests
```rust
#[test]
fn test_button_performance() {
    let start = std::time::Instant::now();
    
    for _ in 0..1000 {
        let _button = create_migrated_button_component();
    }
    
    let duration = start.elapsed();
    assert!(duration.as_millis() < 100); // Should complete in < 100ms
}
```

## Troubleshooting

### Common Issues

#### 1. Signal Ownership
```rust
// ❌ WRONG: Moving signal into closure
let button_class = ArcMemo::new(move |_| {
    button_state.get() // button_state moved here
});

// ✅ CORRECT: Clone signal before moving
let button_state_for_class = button_state.clone();
let button_class = ArcMemo::new(move |_| {
    button_state_for_class.get()
});
```

#### 2. Memory Leaks
```rust
// ❌ WRONG: Not tracking signals
let signal = ArcRwSignal::new(42);
// signal is not tracked, may cause memory leaks

// ✅ CORRECT: Track signals for lifecycle management
let manager = TailwindSignalManager::new();
manager.track_signal(signal);
```

#### 3. Performance Issues
```rust
// ❌ WRONG: Creating signals in render loop
view! {
    {move || {
        let signal = ArcRwSignal::new(42); // Created every render
        signal.get()
    }}
}

// ✅ CORRECT: Create signals outside render loop
let signal = ArcRwSignal::new(42);
view! {
    {move || signal.get()}
}
```

## Migration Tools

### 1. Component Migrator
```rust
let migrator = ComponentMigrator::new();
migrator.mark_migrated("button");
migrator.mark_migrated("input");

let status = migrator.status().get();
println!("Migration progress: {:.1}%", migrator.progress_percentage());
```

### 2. Migration Validation
```rust
let status = validate_all_component_migrations();
assert!(status.all_migrated);
assert_eq!(status.migrated_count, 46);
assert_eq!(status.failed_count, 0);
```

## Best Practices

### 1. Signal Design
- Use `ArcRwSignal` for persistent state
- Use `ArcMemo` for computed values
- Consolidate related state into single signals
- Track all signals for lifecycle management

### 2. Memory Management
- Monitor memory pressure regularly
- Implement automatic cleanup strategies
- Use signal deduplication when possible
- Enable adaptive memory management

### 3. Performance
- Use batched updates for multiple changes
- Auto-tune batch sizes for optimal performance
- Apply lifecycle optimizations
- Monitor performance metrics

### 4. Testing
- Test all migration scenarios
- Benchmark performance before and after
- Monitor memory usage
- Validate migration completeness

## Conclusion

This migration guide provides a comprehensive approach to migrating Leptos components to the new 0.8.8 signal patterns. Follow the step-by-step process, use the provided examples, and leverage the migration tools to ensure a smooth transition.

For additional support, refer to the API documentation and test examples in the codebase.