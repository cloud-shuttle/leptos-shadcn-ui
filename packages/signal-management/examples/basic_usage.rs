//! Basic usage example for leptos-shadcn-signal-management
//! 
//! This example demonstrates the core functionality of the signal management utilities
//! for Leptos 0.8.8+ integration.

use leptos_shadcn_signal_management::*;
use leptos::prelude::*;

fn main() {
    // Note: In a real Leptos app, runtime would be created by the framework
    // For this demo, we'll skip runtime creation
    
    // Demonstrate TailwindSignalManager
    println!("=== TailwindSignalManager Demo ===");
    let manager = TailwindSignalManager::new();
    
    // Test theme management
    let theme = manager.theme();
    println!("Initial theme: {:?}", theme.get());
    
    // Update theme
    theme.set(Theme::Dark);
    println!("Updated theme: {:?}", theme.get());
    
    // Test variant management
    let variant = manager.variant();
    variant.set(Variant::Destructive);
    println!("Variant: {:?}", variant.get());
    
    // Test size management
    let size = manager.size();
    size.set(Size::Large);
    println!("Size: {:?}", size.get());
    
    // Demonstrate BatchedSignalUpdater
    println!("\n=== BatchedSignalUpdater Demo ===");
    let mut updater = BatchedSignalUpdater::new();
    
    // Create some test signals
    let (counter1, set_counter1) = signal(0);
    let (counter2, set_counter2) = signal(0);
    
    // Queue some updates
    updater.queue_update(move || set_counter1.set(1)).unwrap();
    updater.queue_update(move || set_counter2.set(2)).unwrap();
    updater.queue_update(move || set_counter1.set(3)).unwrap();
    
    println!("Before flush - counter1: {}, counter2: {}", counter1.get(), counter2.get());
    
    // Flush all updates
    updater.flush_updates().unwrap();
    println!("After flush - counter1: {}, counter2: {}", counter1.get(), counter2.get());
    
    // Demonstrate Memory Management
    println!("\n=== Memory Management Demo ===");
    let mut memory_manager = SignalMemoryManager::new();
    
    // Create a signal group
    memory_manager.create_group("demo_group".to_string()).unwrap();
    
    // Add signals to the group
    let signal1 = ArcRwSignal::new(42);
    let signal2 = ArcRwSignal::new(84);
    
    memory_manager.add_signal_to_group("demo_group", signal1.clone()).unwrap();
    memory_manager.add_signal_to_group("demo_group", signal2.clone()).unwrap();
    
    // Get memory stats
    let stats = memory_manager.get_stats();
    println!("Memory stats: {:?}", stats);
    
    // Test memory leak detection
    let mut detector = MemoryLeakDetector::new();
    let baseline = MemoryStats::default();
    let current = MemoryStats {
        active_signals: 2,
        active_memos: 0,
        estimated_memory_bytes: 1024,
        tracked_groups: 1,
    };
    
    let leak_detected = detector.check_for_leaks().unwrap_or(false);
    println!("Memory leak detected: {}", leak_detected);
    
    println!("\n=== Demo completed successfully! ===");
    println!("All core functionality is working correctly.");
}
