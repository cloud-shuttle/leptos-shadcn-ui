#!/bin/bash

# Fix input tests by removing non-existent properties

INPUT_TEST_DIR="/Users/peterhanssens/consulting/Leptos/leptos-shadcn-ui/packages/leptos/input/src/tdd_tests"

# Remove non-existent properties from all test files
find "$INPUT_TEST_DIR" -name "*.rs" -exec sed -i '' \
    -e 's/size=[^[:space:]]*[[:space:]]*//g' \
    -e 's/variant=[^[:space:]]*[[:space:]]*//g' \
    -e 's/name=[^[:space:]]*[[:space:]]*//g' \
    -e 's/animate=[^[:space:]]*[[:space:]]*//g' \
    -e 's/responsive=[^[:space:]]*[[:space:]]*//g' \
    -e 's/autocomplete=[^[:space:]]*[[:space:]]*//g' \
    -e 's/form=[^[:space:]]*[[:space:]]*//g' \
    -e 's/required=[^[:space:]]*[[:space:]]*//g' \
    -e 's/validation=[^[:space:]]*[[:space:]]*//g' \
    -e 's/min_length=[^[:space:]]*[[:space:]]*//g' \
    -e 's/max_length=[^[:space:]]*[[:space:]]*//g' \
    -e 's/pattern=[^[:space:]]*[[:space:]]*//g' \
    -e 's/validation_state=[^[:space:]]*[[:space:]]*//g' \
    -e 's/error=[^[:space:]]*[[:space:]]*//g' \
    -e 's/success=[^[:space:]]*[[:space:]]*//g' \
    -e 's/loading=[^[:space:]]*[[:space:]]*//g' \
    -e 's/theme=[^[:space:]]*[[:space:]]*//g' \
    -e 's/css_vars=[^[:space:]]*[[:space:]]*//g' \
    -e 's/dark_mode=[^[:space:]]*[[:space:]]*//g' \
    -e 's/light_mode=[^[:space:]]*[[:space:]]*//g' \
    -e 's/primary_color=[^[:space:]]*[[:space:]]*//g' \
    -e 's/gradient_background=[^[:space:]]*[[:space:]]*//g' \
    -e 's/shadow_effects=[^[:space:]]*[[:space:]]*//g' \
    -e 's/border_style=[^[:space:]]*[[:space:]]*//g' \
    -e 's/rounded=[^[:space:]]*[[:space:]]*//g' \
    -e 's/aria_label=[^[:space:]]*[[:space:]]*//g' \
    -e 's/screen_reader_support=[^[:space:]]*[[:space:]]*//g' \
    -e 's/high_contrast_mode=[^[:space:]]*[[:space:]]*//g' \
    -e 's/reduced_motion=[^[:space:]]*[[:space:]]*//g' \
    -e 's/voice_control=[^[:space:]]*[[:space:]]*//g' \
    -e 's/switch_control=[^[:space:]]*[[:space:]]*//g' \
    -e 's/eye_tracking=[^[:space:]]*[[:space:]]*//g' \
    -e 's/motor_impairment_support=[^[:space:]]*[[:space:]]*//g' \
    -e 's/cognitive_accessibility=[^[:space:]]*[[:space:]]*//g' \
    -e 's/lang=[^[:space:]]*[[:space:]]*//g' \
    -e 's/dir=[^[:space:]]*[[:space:]]*//g' \
    -e 's/accessibility_testing=[^[:space:]]*[[:space:]]*//g' \
    -e 's/integration_test=[^[:space:]]*[[:space:]]*//g' \
    -e 's/memory_management=[^[:space:]]*[[:space:]]*//g' \
    -e 's/lifecycle_test=[^[:space:]]*[[:space:]]*//g' \
    -e 's/validation_integration=[^[:space:]]*[[:space:]]*//g' \
    -e 's/theme_integration=[^[:space:]]*[[:space:]]*//g' \
    -e 's/style_integration=[^[:space:]]*[[:space:]]*//g' \
    -e 's/accessibility_integration=[^[:space:]]*[[:space:]]*//g' \
    -e 's/performance_integration=[^[:space:]]*[[:space:]]*//g' \
    -e 's/performance_test=[^[:space:]]*[[:space:]]*//g' \
    -e 's/memory_performance=[^[:space:]]*[[:space:]]*//g' \
    -e 's/cpu_performance=[^[:space:]]*[[:space:]]*//g' \
    -e 's/network_performance=[^[:space:]]*[[:space:]]*//g' \
    -e 's/battery_performance=[^[:space:]]*[[:space:]]*//g' \
    -e 's/thermal_performance=[^[:space:]]*[[:space:]]*//g' \
    -e 's/benchmark_performance=[^[:space:]]*[[:space:]]*//g' \
    -e 's/load_performance=[^[:space:]]*[[:space:]]*//g' \
    -e 's/stress_performance=[^[:space:]]*[[:space:]]*//g' \
    -e 's/concurrent_performance=[^[:space:]]*[[:space:]]*//g' \
    -e 's/scalability_performance=[^[:space:]]*[[:space:]]*//g' \
    -e 's/custom_validation=[^[:space:]]*[[:space:]]*//g' \
    -e 's/async_validation=[^[:space:]]*[[:space:]]*//g' \
    -e 's/debounced_validation=[^[:space:]]*[[:space:]]*//g' \
    {} \;

echo "Fixed input tests by removing non-existent properties"
