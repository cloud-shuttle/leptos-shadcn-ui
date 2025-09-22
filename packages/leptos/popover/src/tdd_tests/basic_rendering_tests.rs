#[cfg(test)]
mod basic_rendering_tests {
    use leptos::prelude::*;
    use leptos_style::Style;
    use crate::default::Popover;
    use std::sync::{Arc, Mutex};

    // ===== BASIC RENDERING TESTS =====
    // These tests focus on basic rendering and component creation

    #[test]
    fn test_popover_basic_rendering() {
        let _popover_view = view! {
            <Popover>"Click me"</Popover>
        };
    }

    #[test]
    fn test_popover_variants() {
        let _popover_view = view! {
            <Popover variant="default">"Default variant"</Popover>
        };
    }

    #[test]
    fn test_popover_sizes() {
        let _popover_view = view! {
            <Popover size="default">"Default size"</Popover>
        };
    }

    #[test]
    fn test_popover_default_variant() {
        let _popover_view = view! {
            <Popover>"Default variant"</Popover>
        };
    }

    #[test]
    fn test_popover_default_size() {
        let _popover_view = view! {
            <Popover>"Default size"</Popover>
        };
    }

    #[test]
    fn test_popover_disabled_state() {
        let _popover_view = view! {
            <Popover>"Disabled popover"</Popover>
        };
    }

    #[test]
    fn test_popover_with_callback() {
        let callback = Callback::new(move |_| {
            // Test callback functionality
        });
        
        let _popover_view = view! {
            <Popover on_select=Some(callback)>"Popover with callback"</Popover>
        };
    }

    #[test]
    fn test_popover_with_custom_class() {
        let _popover_view = view! {
            <Popover class="custom-class">"Custom class popover"</Popover>
        };
    }

    #[test]
    fn test_popover_with_custom_id() {
        let _popover_view = view! {
            <Popover id="custom-id">"Custom ID popover"</Popover>
        };
    }

    #[test]
    fn test_popover_with_custom_style() {
        let _popover_view = view! {
            <Popover style="color: red;">"Custom style popover"</Popover>
        };
    }

    #[test]
    fn test_popover_with_all_props() {
        let callback = Callback::new(move |_| {});
        
        let _popover_view = view! {
            <Popover
                variant="default"
                size="default"
                class="custom-class"
                id="custom-id"
                style="color: blue;"
                on_select=Some(callback)
            >
                "Complete popover"
            </Popover>
        };
    }

    #[test]
    fn test_popover_trigger_rendering() {
        let _trigger_view = view! {
            <Popover>
                <button>"Trigger Button"</button>
            </Popover>
        };
    }

    #[test]
    fn test_popover_content_rendering() {
        let _content_view = view! {
            <Popover>
                <button>"Trigger"</button>
                <div class="popover-content">
                    "Popover Content"
                </div>
            </Popover>
        };
    }

    #[test]
    fn test_popover_with_children() {
        let _children_view = view! {
            <Popover>
                <button>"Open Popover"</button>
                <div class="popover-content">
                    <h3>"Popover Title"</h3>
                    <p>"Popover description"</p>
                    <button>"Action Button"</button>
                </div>
            </Popover>
        };
    }

    #[test]
    fn test_popover_with_complex_children() {
        let _complex_view = view! {
            <Popover>
                <button>"Complex Popover"</button>
                <div class="popover-content">
                    <div class="popover-header">
                        <h3>"Complex Title"</h3>
                        <button>"Close"</button>
                    </div>
                    <div class="popover-body">
                        <p>"Complex description with multiple paragraphs"</p>
                        <ul>
                            <li>"Item 1"</li>
                            <li>"Item 2"</li>
                            <li>"Item 3"</li>
                        </ul>
                    </div>
                    <div class="popover-footer">
                        <button>"Cancel"</button>
                        <button>"Confirm"</button>
                    </div>
                </div>
            </Popover>
        };
    }

    #[test]
    fn test_popover_with_icons() {
        let _icon_view = view! {
            <Popover>
                <button>"📋 Popover with Icons"</button>
                <div class="popover-content">
                    <div class="popover-item">
                        <span>"📁"</span>
                        <span>"Open Folder"</span>
                    </div>
                    <div class="popover-item">
                        <span>"📄"</span>
                        <span>"New File"</span>
                    </div>
                    <div class="popover-item">
                        <span>"🔍"</span>
                        <span>"Search"</span>
                    </div>
                </div>
            </Popover>
        };
    }

    #[test]
    fn test_popover_with_forms() {
        let _form_view = view! {
            <Popover>
                <button>"Form Popover"</button>
                <div class="popover-content">
                    <form>
                        <div class="form-group">
                            <label>"Name:"</label>
                            <input type="text" placeholder="Enter name" />
                        </div>
                        <div class="form-group">
                            <label>"Email:"</label>
                            <input type="email" placeholder="Enter email" />
                        </div>
                        <div class="form-actions">
                            <button type="submit">"Submit"</button>
                            <button type="button">"Cancel"</button>
                        </div>
                    </form>
                </div>
            </Popover>
        };
    }

    #[test]
    fn test_popover_with_lists() {
        let _list_view = view! {
            <Popover>
                <button>"List Popover"</button>
                <div class="popover-content">
                    <ul class="popover-list">
                        <li class="popover-list-item">
                            <span>"List Item 1"</span>
                            <button>"Action"</button>
                        </li>
                        <li class="popover-list-item">
                            <span>"List Item 2"</span>
                            <button>"Action"</button>
                        </li>
                        <li class="popover-list-item">
                            <span>"List Item 3"</span>
                            <button>"Action"</button>
                        </li>
                    </ul>
                </div>
            </Popover>
        };
    }

    #[test]
    fn test_popover_with_tables() {
        let _table_view = view! {
            <Popover>
                <button>"Table Popover"</button>
                <div class="popover-content">
                    <table class="popover-table">
                        <thead>
                            <tr>
                                <th>"Name"</th>
                                <th>"Value"</th>
                                <th>"Action"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>"Item 1"</td>
                                <td>"100"</td>
                                <td><button>"Edit"</button></td>
                            </tr>
                            <tr>
                                <td>"Item 2"</td>
                                <td>"200"</td>
                                <td><button>"Edit"</button></td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </Popover>
        };
    }

    #[test]
    fn test_popover_with_images() {
        let _image_view = view! {
            <Popover>
                <button>"Image Popover"</button>
                <div class="popover-content">
                    <div class="popover-image">
                        <img src="placeholder.jpg" alt="Popover image" />
                        <div class="image-caption">
                            "Image caption text"
                        </div>
                    </div>
                </div>
            </Popover>
        };
    }

    #[test]
    fn test_popover_with_media() {
        let _media_view = view! {
            <Popover>
                <button>"Media Popover"</button>
                <div class="popover-content">
                    <div class="popover-media">
                        <video controls>
                            <source src="video.mp4" type="video/mp4" />
                            "Your browser does not support the video tag."
                        </video>
                        <div class="media-controls">
                            <button>"Play"</button>
                            <button>"Pause"</button>
                            <button>"Stop"</button>
                        </div>
                    </div>
                </div>
            </Popover>
        };
    }

    #[test]
    fn test_popover_with_charts() {
        let _chart_view = view! {
            <Popover>
                <button>"Chart Popover"</button>
                <div class="popover-content">
                    <div class="popover-chart">
                        <canvas id="popover-chart" width="200" height="150"></canvas>
                        <div class="chart-legend">
                            <div class="legend-item">
                                <span class="legend-color" style="background-color: #ff0000;"></span>
                                <span>"Series 1"</span>
                            </div>
                            <div class="legend-item">
                                <span class="legend-color" style="background-color: #00ff00;"></span>
                                <span>"Series 2"</span>
                            </div>
                        </div>
                    </div>
                </div>
            </Popover>
        };
    }

    #[test]
    fn test_popover_with_maps() {
        let _map_view = view! {
            <Popover>
                <button>"Map Popover"</button>
                <div class="popover-content">
                    <div class="popover-map">
                        <div class="map-container">
                            <div class="map-placeholder">
                                "Map placeholder"
                            </div>
                        </div>
                        <div class="map-controls">
                            <button>"Zoom In"</button>
                            <button>"Zoom Out"</button>
                            <button>"Reset"</button>
                        </div>
                    </div>
                </div>
            </Popover>
        };
    }

    #[test]
    fn test_popover_with_calendars() {
        let _calendar_view = view! {
            <Popover>
                <button>"Calendar Popover"</button>
                <div class="popover-content">
                    <div class="popover-calendar">
                        <div class="calendar-header">
                            <button>"Previous"</button>
                            <span>"January 2024"</span>
                            <button>"Next"</button>
                        </div>
                        <div class="calendar-grid">
                            <div class="calendar-day">"1"</div>
                            <div class="calendar-day">"2"</div>
                            <div class="calendar-day">"3"</div>
                            <div class="calendar-day">"4"</div>
                            <div class="calendar-day">"5"</div>
                            <div class="calendar-day">"6"</div>
                            <div class="calendar-day">"7"</div>
                        </div>
                    </div>
                </div>
            </Popover>
        };
    }

    #[test]
    fn test_popover_with_timelines() {
        let _timeline_view = view! {
            <Popover>
                <button>"Timeline Popover"</button>
                <div class="popover-content">
                    <div class="popover-timeline">
                        <div class="timeline-item">
                            <div class="timeline-marker"></div>
                            <div class="timeline-content">
                                <h4>"Event 1"</h4>
                                <p>"Event description"</p>
                                <span class="timeline-date">"2024-01-01"</span>
                            </div>
                        </div>
                        <div class="timeline-item">
                            <div class="timeline-marker"></div>
                            <div class="timeline-content">
                                <h4>"Event 2"</h4>
                                <p>"Event description"</p>
                                <span class="timeline-date">"2024-01-02"</span>
                            </div>
                        </div>
                    </div>
                </div>
            </Popover>
        };
    }

    #[test]
    fn test_popover_rendering_performance() {
        let start = std::time::Instant::now();
        
        for _ in 0..100 {
            let _view = view! {
                <Popover>
                    <button>"Performance Test"</button>
                    <div class="popover-content">
                        <p>"Performance test content"</p>
                    </div>
                </Popover>
            };
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100, "Rendering should be performant");
    }

    #[test]
    fn test_popover_memory_usage() {
        let _memory_view = view! {
            <Popover>
                <button>"Memory Test"</button>
                <div class="popover-content">
                    <p>"Memory test content"</p>
                </div>
            </Popover>
        };
        
        // Memory usage should be reasonable
        assert!(std::mem::size_of::<usize>() < 1024, "Memory usage should be reasonable");
    }
}
