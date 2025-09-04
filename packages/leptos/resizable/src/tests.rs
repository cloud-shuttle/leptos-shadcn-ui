#[cfg(test)]
mod tests {
    use leptos::prelude::*;
    use crate::default::{
        ResizablePanelGroup, ResizablePanel, ResizableHandle, ResizeDirection
    };

    #[test]
    fn test_resizable_panel_group_creation() {
        let test_result = std::panic::catch_unwind(|| {
            let _component = view! {
                <ResizablePanelGroup>
                    <ResizablePanel>
                        <div>"Panel 1"</div>
                    </ResizablePanel>
                    <ResizableHandle />
                    <ResizablePanel>
                        <div>"Panel 2"</div>
                    </ResizablePanel>
                </ResizablePanelGroup>
            };

            true
        });

        assert!(test_result.is_ok(), "ResizablePanelGroup creation test failed");
    }

    #[test]
    fn test_resizable_panel_creation() {
        let test_result = std::panic::catch_unwind(|| {
            let _component = view! {
                <ResizablePanel
                    default_size=30.0
                    min_size=10.0
                    max_size=80.0
                >
                    <div>"Test Panel"</div>
                </ResizablePanel>
            };

            true
        });

        assert!(test_result.is_ok(), "ResizablePanel creation test failed");
    }

    #[test]
    fn test_resizable_handle_creation() {
        let test_result = std::panic::catch_unwind(|| {
            let _component = view! {
                <ResizableHandle
                    with_handle=true
                    disabled=false
                />
            };

            true
        });

        assert!(test_result.is_ok(), "ResizableHandle creation test failed");
    }

    #[test]
    fn test_collapsible_panel() {
        let test_result = std::panic::catch_unwind(|| {
            let _component = view! {
                <ResizablePanel
                    default_size=30.0
                    collapsible=true
                    collapsed_size=0.0
                    collapsed=false
                >
                    <div>"Collapsible Panel"</div>
                </ResizablePanel>
            };

            true
        });

        assert!(test_result.is_ok(), "Collapsible panel test failed");
    }

    #[test]
    fn test_horizontal_direction() {
        let test_result = std::panic::catch_unwind(|| {
            let _component = view! {
                <ResizablePanelGroup
                    direction=ResizeDirection::Horizontal
                >
                    <ResizablePanel>
                        <div>"Left Panel"</div>
                    </ResizablePanel>
                    <ResizableHandle />
                    <ResizablePanel>
                        <div>"Right Panel"</div>
                    </ResizablePanel>
                </ResizablePanelGroup>
            };

            true
        });

        assert!(test_result.is_ok(), "Horizontal direction test failed");
    }

    #[test]
    fn test_vertical_direction() {
        let test_result = std::panic::catch_unwind(|| {
            let _component = view! {
                <ResizablePanelGroup
                    direction=ResizeDirection::Vertical
                >
                    <ResizablePanel>
                        <div>"Top Panel"</div>
                    </ResizablePanel>
                    <ResizableHandle />
                    <ResizablePanel>
                        <div>"Bottom Panel"</div>
                    </ResizablePanel>
                </ResizablePanelGroup>
            };

            true
        });

        assert!(test_result.is_ok(), "Vertical direction test failed");
    }
}
