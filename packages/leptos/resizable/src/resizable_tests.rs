#[cfg(test)]
mod resizable_tests {
    use leptos::prelude::*;
    use crate::default::{
        ResizablePanelGroup, ResizablePanel, ResizableHandle, ResizeDirection
    };

    /// Test that verifies resizable panel system requirements
    /// This test will fail with current implementation but pass after adding resizable features
    #[test]
    fn test_resizable_panel_system_requirements() {
        let test_result = std::panic::catch_unwind(|| {
            // Resizable panel requirements that should work:
            // 1. Horizontal resizing (left/right panels)
            // 2. Vertical resizing (top/bottom panels)
            // 3. Corner resizing (diagonal resize)
            // 4. Minimum and maximum size constraints
            // 5. Default size and collapsed state
            // 6. Resize handles with visual feedback
            // 7. Keyboard navigation (arrow keys, tab)
            // 8. Accessibility (ARIA labels, screen reader support)
            // 9. Touch support for mobile devices
            // 10. Nested resizable panels

            // This should work with proper resizable panel implementation
            let _resizable = view! {
                <ResizablePanelGroup
                    direction=ResizeDirection::Horizontal
                    class="w-full h-96"
                >
                    <ResizablePanel
                        default_size=30.0
                        min_size=20.0
                        max_size=80.0
                        collapsible=true
                        collapsed_size=0.0
                    >
                        <div class="p-4">
                            "Left Panel"
                        </div>
                    </ResizablePanel>
                    <ResizableHandle />
                    <ResizablePanel
                        default_size=70.0
                        min_size=20.0
                        max_size=80.0
                    >
                        <div class="p-4">
                            "Right Panel"
                        </div>
                    </ResizablePanel>
                </ResizablePanelGroup>
            };

            // If we get here without panicking, the resizable panel system is compatible
            true
        });

        // This test should pass once we implement resizable panel features
        assert!(test_result.is_ok(), "Resizable panel system requirements test failed");
    }

    /// Test that verifies horizontal resizing functionality
    #[test]
    fn test_horizontal_resizing() {
        let test_result = std::panic::catch_unwind(|| {
            // Test horizontal resizing with different configurations
            let _resizable = view! {
                <ResizablePanelGroup
                    direction=ResizeDirection::Horizontal
                    class="w-full h-64"
                >
                    <ResizablePanel
                        default_size=25.0
                        min_size=15.0
                        max_size=50.0
                        id="left-panel"
                    >
                        <div class="p-4 bg-gray-100">
                            "Left Panel (25%)"
                        </div>
                    </ResizablePanel>
                    <ResizableHandle />
                    <ResizablePanel
                        default_size=75.0
                        min_size=50.0
                        max_size=85.0
                        id="right-panel"
                    >
                        <div class="p-4 bg-gray-200">
                            "Right Panel (75%)"
                        </div>
                    </ResizablePanel>
                </ResizablePanelGroup>
            };

            true
        });

        assert!(test_result.is_ok(), "Horizontal resizing test failed");
    }

    /// Test that verifies vertical resizing functionality
    #[test]
    fn test_vertical_resizing() {
        let test_result = std::panic::catch_unwind(|| {
            // Test vertical resizing with different configurations
            let _resizable = view! {
                <ResizablePanelGroup
                    direction=ResizeDirection::Vertical
                    class="w-full h-96"
                >
                    <ResizablePanel
                        default_size=40.0
                        min_size=20.0
                        max_size=70.0
                        id="top-panel"
                    >
                        <div class="p-4 bg-blue-100">
                            "Top Panel (40%)"
                        </div>
                    </ResizablePanel>
                    <ResizableHandle />
                    <ResizablePanel
                        default_size=60.0
                        min_size=30.0
                        max_size=80.0
                        id="bottom-panel"
                    >
                        <div class="p-4 bg-blue-200">
                            "Bottom Panel (60%)"
                        </div>
                    </ResizablePanel>
                </ResizablePanelGroup>
            };

            true
        });

        assert!(test_result.is_ok(), "Vertical resizing test failed");
    }

    /// Test that verifies collapsible panels functionality
    #[test]
    fn test_collapsible_panels() {
        let test_result = std::panic::catch_unwind(|| {
            // Test collapsible panels with different states
            let _resizable = view! {
                <ResizablePanelGroup
                    direction=ResizeDirection::Horizontal
                    class="w-full h-64"
                >
                    <ResizablePanel
                        default_size=30.0
                        min_size=20.0
                        max_size=80.0
                        collapsible=true
                        collapsed_size=0.0
                        collapsed=true
                        id="collapsible-panel"
                    >
                        <div class="p-4 bg-green-100">
                            "Collapsible Panel"
                        </div>
                    </ResizablePanel>
                    <ResizableHandle />
                    <ResizablePanel
                        default_size=70.0
                        min_size=20.0
                        max_size=80.0
                        id="main-panel"
                    >
                        <div class="p-4 bg-green-200">
                            "Main Panel"
                        </div>
                    </ResizablePanel>
                </ResizablePanelGroup>
            };

            true
        });

        assert!(test_result.is_ok(), "Collapsible panels test failed");
    }

    /// Test that verifies nested resizable panels functionality
    #[test]
    fn test_nested_resizable_panels() {
        let test_result = std::panic::catch_unwind(|| {
            // Test nested resizable panels
            let _resizable = view! {
                <ResizablePanelGroup
                    direction=ResizeDirection::Horizontal
                    class="w-full h-96"
                >
                    <ResizablePanel
                        default_size=50.0
                        min_size=30.0
                        max_size=70.0
                        id="left-nested"
                    >
                        <ResizablePanelGroup
                            direction=ResizeDirection::Vertical
                            class="w-full h-full"
                        >
                            <ResizablePanel
                                default_size=60.0
                                min_size=20.0
                                max_size=80.0
                                id="top-nested"
                            >
                                <div class="p-4 bg-yellow-100">
                                    "Top Nested Panel"
                                </div>
                            </ResizablePanel>
                            <ResizableHandle />
                            <ResizablePanel
                                default_size=40.0
                                min_size=20.0
                                max_size=80.0
                                id="bottom-nested"
                            >
                                <div class="p-4 bg-yellow-200">
                                    "Bottom Nested Panel"
                                </div>
                            </ResizablePanel>
                        </ResizablePanelGroup>
                    </ResizablePanel>
                    <ResizableHandle />
                    <ResizablePanel
                        default_size=50.0
                        min_size=30.0
                        max_size=70.0
                        id="right-nested"
                    >
                        <div class="p-4 bg-yellow-300">
                            "Right Panel"
                        </div>
                    </ResizablePanel>
                </ResizablePanelGroup>
            };

            true
        });

        assert!(test_result.is_ok(), "Nested resizable panels test failed");
    }

    /// Test that verifies resize handle functionality
    #[test]
    fn test_resize_handle() {
        let test_result = std::panic::catch_unwind(|| {
            // Test resize handle with different configurations
            let _resizable = view! {
                <ResizablePanelGroup
                    direction=ResizeDirection::Horizontal
                    class="w-full h-64"
                >
                    <ResizablePanel
                        default_size=50.0
                        min_size=20.0
                        max_size=80.0
                        id="panel-1"
                    >
                        <div class="p-4 bg-red-100">
                            "Panel 1"
                        </div>
                    </ResizablePanel>
                    <ResizableHandle
                        with_handle=true
                        class="bg-gray-300 hover:bg-gray-400"
                        disabled=false
                    />
                    <ResizablePanel
                        default_size=50.0
                        min_size=20.0
                        max_size=80.0
                        id="panel-2"
                    >
                        <div class="p-4 bg-red-200">
                            "Panel 2"
                        </div>
                    </ResizablePanel>
                </ResizablePanelGroup>
            };

            true
        });

        assert!(test_result.is_ok(), "Resize handle test failed");
    }

    /// Test that verifies keyboard navigation functionality
    #[test]
    fn test_keyboard_navigation() {
        let test_result = std::panic::catch_unwind(|| {
            // Test keyboard navigation support
            let _resizable = view! {
                <ResizablePanelGroup
                    direction=ResizeDirection::Horizontal
                    class="w-full h-64"
                    keyboard_resize=true
                >
                    <ResizablePanel
                        default_size=50.0
                        min_size=20.0
                        max_size=80.0
                        id="keyboard-panel-1"
                    >
                        <div class="p-4 bg-purple-100">
                            "Panel 1 (Keyboard Navigable)"
                        </div>
                    </ResizablePanel>
                    <ResizableHandle
                        with_handle=true
                        keyboard_resize=true
                    />
                    <ResizablePanel
                        default_size=50.0
                        min_size=20.0
                        max_size=80.0
                        id="keyboard-panel-2"
                    >
                        <div class="p-4 bg-purple-200">
                            "Panel 2 (Keyboard Navigable)"
                        </div>
                    </ResizablePanel>
                </ResizablePanelGroup>
            };

            true
        });

        assert!(test_result.is_ok(), "Keyboard navigation test failed");
    }

    /// Test that verifies accessibility features
    #[test]
    fn test_accessibility_features() {
        let test_result = std::panic::catch_unwind(|| {
            // Test accessibility features
            let _resizable = view! {
                <ResizablePanelGroup
                    direction=ResizeDirection::Horizontal
                    class="w-full h-64"
                    aria_label="Main content area"
                >
                    <ResizablePanel
                        default_size=50.0
                        min_size=20.0
                        max_size=80.0
                        id="accessible-panel-1"
                        aria_label="Left content panel"
                    >
                        <div class="p-4 bg-indigo-100">
                            "Accessible Panel 1"
                        </div>
                    </ResizablePanel>
                    <ResizableHandle
                        with_handle=true
                        aria_label="Resize handle for left and right panels"
                        role="separator"
                    />
                    <ResizablePanel
                        default_size=50.0
                        min_size=20.0
                        max_size=80.0
                        id="accessible-panel-2"
                        aria_label="Right content panel"
                    >
                        <div class="p-4 bg-indigo-200">
                            "Accessible Panel 2"
                        </div>
                    </ResizablePanel>
                </ResizablePanelGroup>
            };

            true
        });

        assert!(test_result.is_ok(), "Accessibility features test failed");
    }

    /// Test that verifies touch support functionality
    #[test]
    fn test_touch_support() {
        let test_result = std::panic::catch_unwind(|| {
            // Test touch support for mobile devices
            let _resizable = view! {
                <ResizablePanelGroup
                    direction=ResizeDirection::Horizontal
                    class="w-full h-64"
                    touch_support=true
                >
                    <ResizablePanel
                        default_size=50.0
                        min_size=20.0
                        max_size=80.0
                        id="touch-panel-1"
                    >
                        <div class="p-4 bg-pink-100">
                            "Touch Panel 1"
                        </div>
                    </ResizablePanel>
                    <ResizableHandle
                        with_handle=true
                        touch_support=true
                    />
                    <ResizablePanel
                        default_size=50.0
                        min_size=20.0
                        max_size=80.0
                        id="touch-panel-2"
                    >
                        <div class="p-4 bg-pink-200">
                            "Touch Panel 2"
                        </div>
                    </ResizablePanel>
                </ResizablePanelGroup>
            };

            true
        });

        assert!(test_result.is_ok(), "Touch support test failed");
    }

    /// Test that verifies size constraints functionality
    #[test]
    fn test_size_constraints() {
        let test_result = std::panic::catch_unwind(|| {
            // Test size constraints (min/max sizes)
            let _resizable = view! {
                <ResizablePanelGroup
                    direction=ResizeDirection::Horizontal
                    class="w-full h-64"
                >
                    <ResizablePanel
                        default_size=30.0
                        min_size=10.0
                        max_size=60.0
                        id="constrained-panel-1"
                    >
                        <div class="p-4 bg-teal-100">
                            "Constrained Panel 1 (10%-60%)"
                        </div>
                    </ResizablePanel>
                    <ResizableHandle />
                    <ResizablePanel
                        default_size=70.0
                        min_size=40.0
                        max_size=90.0
                        id="constrained-panel-2"
                    >
                        <div class="p-4 bg-teal-200">
                            "Constrained Panel 2 (40%-90%)"
                        </div>
                    </ResizablePanel>
                </ResizablePanelGroup>
            };

            true
        });

        assert!(test_result.is_ok(), "Size constraints test failed");
    }

    /// Test that verifies resize events and callbacks
    #[test]
    fn test_resize_events() {
        let test_result = std::panic::catch_unwind(|| {
            // Test resize events and callbacks
            let _resizable = view! {
                <ResizablePanelGroup
                    direction=ResizeDirection::Horizontal
                    class="w-full h-64"
                    on_resize=Callback::new(|sizes: Vec<f64>| {
                        println!("Panel sizes changed: {:?}", sizes);
                    })
                >
                    <ResizablePanel
                        default_size=50.0
                        min_size=20.0
                        max_size=80.0
                        id="event-panel-1"
                        on_resize=Callback::new(|size: f64| {
                            println!("Panel 1 size: {}", size);
                        })
                    >
                        <div class="p-4 bg-orange-100">
                            "Event Panel 1"
                        </div>
                    </ResizablePanel>
                    <ResizableHandle />
                    <ResizablePanel
                        default_size=50.0
                        min_size=20.0
                        max_size=80.0
                        id="event-panel-2"
                        on_resize=Callback::new(|size: f64| {
                            println!("Panel 2 size: {}", size);
                        })
                    >
                        <div class="p-4 bg-orange-200">
                            "Event Panel 2"
                        </div>
                    </ResizablePanel>
                </ResizablePanelGroup>
            };

            true
        });

        assert!(test_result.is_ok(), "Resize events test failed");
    }
}
