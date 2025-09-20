#[cfg(test)]
mod visual_regression_tests {
    use leptos::prelude::*;
    use wasm_bindgen_test::*;
    use web_sys;
    use crate::visual_testing::{VisualTestRunner, VisualTestResult, VisualRegression};
    use crate::default::{Button, Input, Card, CardHeader, CardTitle, CardContent};

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_button_visual_regression() {
        let mut runner = VisualTestRunner::new();
        
        mount_to_body(|| {
            view! {
                <div id="button-test-container">
                    <Button id="test-button" class="visual-test-button">
                        "Test Button"
                    </Button>
                </div>
            }
        });

        // Capture screenshot
        let screenshot = runner.capture_screenshot("button-test-container", "button_default_state")
            .expect("Failed to capture screenshot");
        
        // Create test result
        let result = VisualTestResult {
            test_name: "button_default_state".to_string(),
            component_name: "Button".to_string(),
            screenshot_data: screenshot.clone(),
            timestamp: current_timestamp(),
            viewport_width: 1920,
            viewport_height: 1080,
            pixel_difference: None,
            visual_similarity: None,
        };
        
        runner.results.push(result);
        
        // Compare with baseline (if exists)
        let regression = runner.compare_with_baseline("button_default_state", &screenshot)
            .expect("Failed to compare with baseline");
        
        if let Some(regression) = regression {
            panic!("Visual regression detected: {:?}", regression);
        }
    }

    #[wasm_bindgen_test]
    fn test_button_variants_visual_regression() {
        let mut runner = VisualTestRunner::new();
        
        let variants = vec!["default", "destructive", "outline", "secondary", "ghost", "link"];
        
        for variant in variants {
            mount_to_body(move || {
                view! {
                    <div id=format!("button-{}-test", variant)>
                        <Button variant=variant>
                            {format!("{} Button", variant)}
                        </Button>
                    </div>
                }
            });
            
            let test_name = format!("button_{}_variant", variant);
            let screenshot = runner.capture_screenshot(&format!("button-{}-test", variant), &test_name)
                .expect("Failed to capture screenshot");
            
            let result = VisualTestResult {
                test_name: test_name.clone(),
                component_name: "Button".to_string(),
                screenshot_data: screenshot.clone(),
                timestamp: current_timestamp(),
                viewport_width: 1920,
                viewport_height: 1080,
                pixel_difference: None,
                visual_similarity: None,
            };
            
            runner.results.push(result);
            
            // Compare with baseline
            let regression = runner.compare_with_baseline(&test_name, &screenshot)
                .expect("Failed to compare with baseline");
            
            if let Some(regression) = regression {
                panic!("Visual regression detected for {} variant: {:?}", variant, regression);
            }
        }
    }

    #[wasm_bindgen_test]
    fn test_input_visual_regression() {
        let mut runner = VisualTestRunner::new();
        
        mount_to_body(|| {
            view! {
                <div id="input-test-container">
                    <Input 
                        id="test-input"
                        placeholder="Test input"
                        class="visual-test-input"
                    />
                </div>
            }
        });

        let screenshot = runner.capture_screenshot("input-test-container", "input_default_state")
            .expect("Failed to capture screenshot");
        
        let result = VisualTestResult {
            test_name: "input_default_state".to_string(),
            component_name: "Input".to_string(),
            screenshot_data: screenshot.clone(),
            timestamp: current_timestamp(),
            viewport_width: 1920,
            viewport_height: 1080,
            pixel_difference: None,
            visual_similarity: None,
        };
        
        runner.results.push(result);
        
        let regression = runner.compare_with_baseline("input_default_state", &screenshot)
            .expect("Failed to compare with baseline");
        
        if let Some(regression) = regression {
            panic!("Visual regression detected: {:?}", regression);
        }
    }

    #[wasm_bindgen_test]
    fn test_card_visual_regression() {
        let mut runner = VisualTestRunner::new();
        
        mount_to_body(|| {
            view! {
                <div id="card-test-container">
                    <Card class="visual-test-card">
                        <CardHeader>
                            <CardTitle>"Test Card"</CardTitle>
                        </CardHeader>
                        <CardContent>
                            "This is a test card for visual regression testing."
                        </CardContent>
                    </Card>
                </div>
            }
        });

        let screenshot = runner.capture_screenshot("card-test-container", "card_default_state")
            .expect("Failed to capture screenshot");
        
        let result = VisualTestResult {
            test_name: "card_default_state".to_string(),
            component_name: "Card".to_string(),
            screenshot_data: screenshot.clone(),
            timestamp: current_timestamp(),
            viewport_width: 1920,
            viewport_height: 1080,
            pixel_difference: None,
            visual_similarity: None,
        };
        
        runner.results.push(result);
        
        let regression = runner.compare_with_baseline("card_default_state", &screenshot)
            .expect("Failed to compare with baseline");
        
        if let Some(regression) = regression {
            panic!("Visual regression detected: {:?}", regression);
        }
    }

    #[wasm_bindgen_test]
    fn test_responsive_visual_regression() {
        let mut runner = VisualTestRunner::new();
        
        let viewports = vec![
            (320, 568, "mobile"),
            (768, 1024, "tablet"),
            (1920, 1080, "desktop"),
        ];
        
        for (width, height, device) in viewports {
            mount_to_body(move || {
                view! {
                    <div id=format!("responsive-test-{}", device) class="responsive-test-container">
                        <Button class="responsive-button">
                            {format!("{} Button", device)}
                        </Button>
                        <Input placeholder={format!("{} Input", device)} />
                        <Card>
                            <CardHeader>
                                <CardTitle>{format!("{} Card", device)}</CardTitle>
                            </CardHeader>
                            <CardContent>
                                {format!("Responsive test for {} viewport", device)}
                            </CardContent>
                        </Card>
                    </div>
                }
            });
            
            let test_name = format!("responsive_{}_layout", device);
            let screenshot = runner.capture_screenshot(&format!("responsive-test-{}", device), &test_name)
                .expect("Failed to capture screenshot");
            
            let result = VisualTestResult {
                test_name: test_name.clone(),
                component_name: "ResponsiveLayout".to_string(),
                screenshot_data: screenshot.clone(),
                timestamp: current_timestamp(),
                viewport_width: width,
                viewport_height: height,
                pixel_difference: None,
                visual_similarity: None,
            };
            
            runner.results.push(result);
            
            let regression = runner.compare_with_baseline(&test_name, &screenshot)
                .expect("Failed to compare with baseline");
            
            if let Some(regression) = regression {
                panic!("Visual regression detected for {} viewport: {:?}", device, regression);
            }
        }
    }

    #[wasm_bindgen_test]
    fn test_dark_mode_visual_regression() {
        let mut runner = VisualTestRunner::new();
        
        let themes = vec!["light", "dark"];
        
        for theme in themes {
            mount_to_body(move || {
                view! {
                    <div id=format!("theme-test-{}", theme) class=format!("theme-{}", theme)>
                        <Button class="theme-button">
                            {format!("{} Theme Button", theme)}
                        </Button>
                        <Input placeholder={format!("{} Theme Input", theme)} />
                        <Card>
                            <CardHeader>
                                <CardTitle>{format!("{} Theme Card", theme)}</CardTitle>
                            </CardHeader>
                            <CardContent>
                                {format!("Test card in {} theme", theme)}
                            </CardContent>
                        </Card>
                    </div>
                }
            });
            
            let test_name = format!("theme_{}_mode", theme);
            let screenshot = runner.capture_screenshot(&format!("theme-test-{}", theme), &test_name)
                .expect("Failed to capture screenshot");
            
            let result = VisualTestResult {
                test_name: test_name.clone(),
                component_name: "Theme".to_string(),
                screenshot_data: screenshot.clone(),
                timestamp: current_timestamp(),
                viewport_width: 1920,
                viewport_height: 1080,
                pixel_difference: None,
                visual_similarity: None,
            };
            
            runner.results.push(result);
            
            let regression = runner.compare_with_baseline(&test_name, &screenshot)
                .expect("Failed to compare with baseline");
            
            if let Some(regression) = regression {
                panic!("Visual regression detected for {} theme: {:?}", theme, regression);
            }
        }
    }

    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}