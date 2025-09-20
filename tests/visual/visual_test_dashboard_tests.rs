#[cfg(test)]
mod visual_test_dashboard_tests {
    use leptos::prelude::*;
    use wasm_bindgen_test::*;
    use web_sys;
    use crate::visual_testing::{VisualTestRunner, VisualTestResult, VisualRegression};

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_visual_test_dashboard() {
        let mut runner = VisualTestRunner::new();
        let test_results = RwSignal::new(Vec::<VisualTestResult>::new());
        let regressions = RwSignal::new(Vec::<VisualRegression>::new());
        let selected_test = RwSignal::new(None::<String>);
        let show_baselines = RwSignal::new(false);

        // Add some test data
        let sample_result = VisualTestResult {
            test_name: "button_default_state".to_string(),
            component_name: "Button".to_string(),
            screenshot_data: "sample_screenshot_data".to_string(),
            timestamp: current_timestamp(),
            viewport_width: 1920,
            viewport_height: 1080,
            pixel_difference: Some(0.0),
            visual_similarity: Some(1.0),
        };
        
        test_results.set(vec![sample_result]);

        mount_to_body(move || {
            view! {
                <div class="visual-test-dashboard">
                    <div class="dashboard-header">
                        <h1>"Visual Regression Test Dashboard"</h1>
                        <div class="controls">
                            <Button 
                                on_click=Callback::new(move || {
                                    test_results.set(runner.get_results().clone());
                                    regressions.set(runner.get_regressions().clone());
                                })
                            >
                                "Refresh Results"
                            </Button>
                            <Button 
                                on_click=Callback::new(move || show_baselines.set(!show_baselines.get()))
                            >
                                {if show_baselines.get() { "Hide Baselines" } else { "Show Baselines" }}
                            </Button>
                        </div>
                    </div>

                    <div class="dashboard-content">
                        <div class="test-results-section">
                            <h2>"Test Results"</h2>
                            <div class="results-grid">
                                {for test_results.get().iter().map(|result| {
                                    let result = result.clone();
                                    let selected_test = selected_test.clone();
                                    
                                    view! {
                                        <div 
                                            class="result-card" 
                                            class:selected=selected_test.get() == Some(result.test_name.clone())
                                            on_click=Callback::new(move || selected_test.set(Some(result.test_name.clone())))
                                        >
                                            <div class="result-header">
                                                <h3>{result.test_name.clone()}</h3>
                                                <span class="component-name">{result.component_name.clone()}</span>
                                            </div>
                                            <div class="result-screenshot">
                                                <img src=format!("data:image/png;base64,{}", result.screenshot_data) alt="Screenshot" />
                                            </div>
                                            <div class="result-metrics">
                                                <div class="metric">
                                                    <span class="metric-label">"Similarity:"</span>
                                                    <span class="metric-value">{format!("{:.2}%", result.visual_similarity.unwrap_or(0.0) * 100.0)}</span>
                                                </div>
                                                <div class="metric">
                                                    <span class="metric-label">"Viewport:"</span>
                                                    <span class="metric-value">{format!("{}x{}", result.viewport_width, result.viewport_height)}</span>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                })}
                            </div>
                        </div>

                        <div class="regressions-section">
                            <h2>"Visual Regressions"</h2>
                            <div class="regressions-list">
                                {for regressions.get().iter().map(|regression| {
                                    let regression = regression.clone();
                                    
                                    view! {
                                        <div class="regression-item" class:critical=regression.similarity_score < 0.5>
                                            <div class="regression-header">
                                                <h3>{regression.test_name.clone()}</h3>
                                                <span class="severity">{regression.similarity_score}</span>
                                            </div>
                                            <div class="regression-comparison">
                                                <div class="comparison-image">
                                                    <h4>"Baseline"</h4>
                                                    <img src=format!("data:image/png;base64,{}", regression.baseline_screenshot) alt="Baseline" />
                                                </div>
                                                <div class="comparison-image">
                                                    <h4>"Current"</h4>
                                                    <img src=format!("data:image/png;base64,{}", regression.current_screenshot) alt="Current" />
                                                </div>
                                                <div class="comparison-image">
                                                    <h4>"Diff"</h4>
                                                    <img src=format!("data:image/png;base64,{}", regression.diff_screenshot) alt="Diff" />
                                                </div>
                                            </div>
                                            <div class="regression-details">
                                                <p>{format!("Similarity: {:.2}% (Threshold: {:.2}%)", regression.similarity_score * 100.0, regression.threshold * 100.0)}</p>
                                                <p>{format!("Pixel Differences: {}", regression.pixel_differences)}</p>
                                            </div>
                                        </div>
                                    }
                                })}
                            </div>
                        </div>

                        {if show_baselines.get() {
                            view! {
                                <div class="baselines-section">
                                    <h2>"Baselines"</h2>
                                    <div class="baselines-list">
                                        <p>"Baseline management interface would go here"</p>
                                    </div>
                                </div>
                            }
                        } else {
                            view! { <div></div> }
                        }}
                    </div>
                </div>
            }
        });

        let document = web_sys::window().unwrap().document().unwrap();
        
        // Test dashboard functionality
        let refresh_button = document.query_selector("button").unwrap().unwrap()
            .unchecked_into::<web_sys::HtmlButtonElement>();
        if refresh_button.text_content().unwrap().contains("Refresh Results") {
            refresh_button.click();
        }

        // Verify dashboard sections
        let results_section = document.query_selector(".test-results-section").unwrap();
        assert!(results_section.is_some(), "Test results section should be displayed");

        let regressions_section = document.query_selector(".regressions-section").unwrap();
        assert!(regressions_section.is_some(), "Regressions section should be displayed");

        // Test result selection
        let result_cards = document.query_selector_all(".result-card").unwrap();
        if result_cards.length() > 0 {
            let first_card = result_cards.item(0).unwrap();
            first_card.click();
            
            let selected_card = document.query_selector(".result-card.selected").unwrap();
            assert!(selected_card.is_some(), "Result card should be selectable");
        }
    }

    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}