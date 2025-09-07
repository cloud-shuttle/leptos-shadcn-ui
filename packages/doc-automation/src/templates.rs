//! Handlebars template helpers for documentation generation

use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext};
use pulldown_cmark::{Parser, Options, html};

/// Template for component API documentation
pub const API_DOC_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{component.name}} - leptos-shadcn-ui API Documentation</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; }
        .container { max-width: 1200px; margin: 0 auto; padding: 20px; }
        .component-header { border-bottom: 2px solid #e5e7eb; padding-bottom: 20px; margin-bottom: 30px; }
        .props-table { width: 100%; border-collapse: collapse; margin: 20px 0; }
        .props-table th, .props-table td { border: 1px solid #e5e7eb; padding: 12px; text-align: left; }
        .props-table th { background-color: #f9fafb; font-weight: 600; }
        .code-block { background-color: #f3f4f6; padding: 16px; border-radius: 6px; overflow-x: auto; }
        .example-section { margin: 30px 0; padding: 20px; border: 1px solid #e5e7eb; border-radius: 8px; }
        .accessibility-info { background-color: #ecfdf5; padding: 16px; border-radius: 6px; margin: 20px 0; }
        .performance-info { background-color: #fef3c7; padding: 16px; border-radius: 6px; margin: 20px 0; }
        .test-coverage { background-color: #e0e7ff; padding: 16px; border-radius: 6px; margin: 20px 0; }
    </style>
</head>
<body>
    <div class="container">
        <header class="component-header">
            <h1>{{component.name}}</h1>
            {{#if component.description}}
            <div class="description">
                {{{markdown component.description}}}
            </div>
            {{/if}}
        </header>

        <section class="props-section">
            <h2>Props</h2>
            {{#if component.props}}
            <table class="props-table">
                <thead>
                    <tr>
                        <th>Name</th>
                        <th>Type</th>
                        <th>Required</th>
                        <th>Default</th>
                        <th>Description</th>
                    </tr>
                </thead>
                <tbody>
                    {{#each component.props}}
                    <tr>
                        <td><code>{{name}}</code></td>
                        <td><code>{{prop_type}}</code></td>
                        <td>{{#if required}}Yes{{else}}No{{/if}}</td>
                        <td>{{#if default_value}}<code>{{default_value}}</code>{{else}}-{{/if}}</td>
                        <td>{{#if description}}{{{markdown description}}}{{else}}-{{/if}}</td>
                    </tr>
                    {{/each}}
                </tbody>
            </table>
            {{else}}
            <p>No props defined.</p>
            {{/if}}
        </section>

        {{#if component.events}}
        <section class="events-section">
            <h2>Events</h2>
            <table class="props-table">
                <thead>
                    <tr>
                        <th>Name</th>
                        <th>Type</th>
                        <th>Description</th>
                    </tr>
                </thead>
                <tbody>
                    {{#each component.events}}
                    <tr>
                        <td><code>{{name}}</code></td>
                        <td><code>{{event_type}}</code></td>
                        <td>{{#if description}}{{{markdown description}}}{{else}}-{{/if}}</td>
                    </tr>
                    {{/each}}
                </tbody>
            </table>
        </section>
        {{/if}}

        {{#if component.examples}}
        <section class="examples-section">
            <h2>Examples</h2>
            {{#each component.examples}}
            <div class="example-section">
                <h3>{{title}}</h3>
                {{#if description}}
                <p>{{{markdown description}}}</p>
                {{/if}}
                <div class="code-block">
                    <pre><code>{{{format_code code}}}</code></pre>
                </div>
            </div>
            {{/each}}
        </section>
        {{/if}}

        <section class="accessibility-section">
            <h2>Accessibility</h2>
            <div class="accessibility-info">
                <p><strong>WCAG Level:</strong> {{component.accessibility.wcag_level}}</p>
                <p><strong>Keyboard Support:</strong> {{#if component.accessibility.keyboard_support}}Yes{{else}}No{{/if}}</p>
                <p><strong>Screen Reader Support:</strong> {{#if component.accessibility.screen_reader_support}}Yes{{else}}No{{/if}}</p>
                {{#if component.accessibility.aria_attributes}}
                <p><strong>ARIA Attributes:</strong></p>
                <ul>
                    {{#each component.accessibility.aria_attributes}}
                    <li><code>{{this}}</code></li>
                    {{/each}}
                </ul>
                {{/if}}
            </div>
        </section>

        {{#if component.performance}}
        <section class="performance-section">
            <h2>Performance</h2>
            <div class="performance-info">
                {{#if component.performance.render_time_ms}}
                <p><strong>Render Time:</strong> {{component.performance.render_time_ms}}ms</p>
                {{/if}}
                {{#if component.performance.bundle_size_kb}}
                <p><strong>Bundle Size:</strong> {{component.performance.bundle_size_kb}}KB</p>
                {{/if}}
                {{#if component.performance.memory_usage_mb}}
                <p><strong>Memory Usage:</strong> {{component.performance.memory_usage_mb}}MB</p>
                {{/if}}
            </div>
        </section>
        {{/if}}

        {{#if component.tests}}
        <section class="tests-section">
            <h2>Test Coverage</h2>
            <div class="test-coverage">
                <p><strong>Total Tests:</strong> {{component.tests.length}}</p>
                {{#each component.tests}}
                <div>
                    <strong>{{name}}</strong> ({{test_type}})
                    {{#if description}}: {{description}}{{/if}}
                </div>
                {{/each}}
            </div>
        </section>
        {{/if}}
    </div>
</body>
</html>
"#;

/// Template for component gallery
pub const GALLERY_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Component Gallery - leptos-shadcn-ui</title>
    <style>
        body { 
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            margin: 0;
            padding: 0;
            background-color: #f9fafb;
        }
        .header { 
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 60px 20px;
            text-align: center;
        }
        .container { max-width: 1400px; margin: 0 auto; padding: 40px 20px; }
        .component-grid { 
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));
            gap: 30px;
        }
        .component-card { 
            background: white;
            border-radius: 12px;
            padding: 24px;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.07);
            transition: transform 0.2s, box-shadow 0.2s;
        }
        .component-card:hover {
            transform: translateY(-2px);
            box-shadow: 0 8px 25px rgba(0, 0, 0, 0.1);
        }
        .component-name { 
            font-size: 1.5em;
            font-weight: 600;
            color: #111827;
            margin-bottom: 8px;
        }
        .component-description { 
            color: #6b7280;
            line-height: 1.5;
            margin-bottom: 16px;
        }
        .component-stats {
            display: flex;
            gap: 16px;
            margin: 16px 0;
            font-size: 0.875em;
        }
        .stat {
            background: #f3f4f6;
            padding: 6px 12px;
            border-radius: 6px;
            color: #374151;
        }
        .code-preview {
            background: #f3f4f6;
            border-radius: 6px;
            padding: 12px;
            font-family: 'SF Mono', Monaco, monospace;
            font-size: 0.875em;
            overflow-x: auto;
            margin-top: 16px;
        }
        .accessibility-badge {
            display: inline-block;
            background: #10b981;
            color: white;
            padding: 4px 8px;
            border-radius: 4px;
            font-size: 0.75em;
            font-weight: 500;
        }
        .search-box {
            max-width: 600px;
            margin: 0 auto 40px auto;
            position: relative;
        }
        .search-input {
            width: 100%;
            padding: 12px 16px;
            border: 2px solid #e5e7eb;
            border-radius: 8px;
            font-size: 16px;
            transition: border-color 0.2s;
        }
        .search-input:focus {
            outline: none;
            border-color: #667eea;
        }
        .filters {
            display: flex;
            gap: 12px;
            margin-bottom: 30px;
            flex-wrap: wrap;
        }
        .filter-button {
            padding: 8px 16px;
            border: 1px solid #d1d5db;
            background: white;
            border-radius: 6px;
            cursor: pointer;
            transition: all 0.2s;
        }
        .filter-button:hover, .filter-button.active {
            background: #667eea;
            color: white;
            border-color: #667eea;
        }
    </style>
</head>
<body>
    <header class="header">
        <h1>leptos-shadcn-ui Component Gallery</h1>
        <p>Interactive showcase of all {{components.length}} components</p>
        <p><em>Generated on {{generation_timestamp}}</em></p>
    </header>

    <div class="container">
        <div class="search-box">
            <input type="text" class="search-input" placeholder="Search components..." id="searchInput">
        </div>

        <div class="filters">
            <button class="filter-button active" data-filter="all">All Components</button>
            <button class="filter-button" data-filter="form">Form</button>
            <button class="filter-button" data-filter="layout">Layout</button>
            <button class="filter-button" data-filter="navigation">Navigation</button>
            <button class="filter-button" data-filter="feedback">Feedback</button>
        </div>

        <div class="component-grid" id="componentGrid">
            {{#each components}}
            <div class="component-card" data-category="{{category}}">
                <div class="component-name">{{name}}</div>
                
                {{#if description}}
                <div class="component-description">{{{markdown description}}}</div>
                {{/if}}

                <div class="component-stats">
                    <span class="stat">{{props.length}} Props</span>
                    <span class="stat">{{tests.length}} Tests</span>
                    {{#if accessibility.wcag_level}}
                    <span class="accessibility-badge">WCAG {{accessibility.wcag_level}}</span>
                    {{/if}}
                </div>

                {{#if examples}}
                <div class="code-preview">
                    <code>{{{format_code examples.[0].code}}}</code>
                </div>
                {{/if}}
            </div>
            {{/each}}
        </div>
    </div>

    <script>
        // Simple search and filter functionality
        const searchInput = document.getElementById('searchInput');
        const componentGrid = document.getElementById('componentGrid');
        const filterButtons = document.querySelectorAll('.filter-button');
        
        let currentFilter = 'all';
        
        searchInput.addEventListener('input', filterComponents);
        
        filterButtons.forEach(button => {
            button.addEventListener('click', () => {
                filterButtons.forEach(b => b.classList.remove('active'));
                button.classList.add('active');
                currentFilter = button.dataset.filter;
                filterComponents();
            });
        });
        
        function filterComponents() {
            const searchTerm = searchInput.value.toLowerCase();
            const cards = componentGrid.querySelectorAll('.component-card');
            
            cards.forEach(card => {
                const name = card.querySelector('.component-name').textContent.toLowerCase();
                const description = card.querySelector('.component-description')?.textContent.toLowerCase() || '';
                const category = card.dataset.category || '';
                
                const matchesSearch = name.includes(searchTerm) || description.includes(searchTerm);
                const matchesFilter = currentFilter === 'all' || category === currentFilter;
                
                card.style.display = matchesSearch && matchesFilter ? 'block' : 'none';
            });
        }
    </script>
</body>
</html>
"#;

/// Template for test reports
pub const TEST_REPORT_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Test Coverage Report - leptos-shadcn-ui</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; }
        .container { max-width: 1200px; margin: 0 auto; padding: 20px; }
        .header { text-align: center; border-bottom: 2px solid #e5e7eb; padding-bottom: 20px; }
        .summary { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px; margin: 30px 0; }
        .summary-card { background: #f9fafb; padding: 20px; border-radius: 8px; text-align: center; }
        .summary-number { font-size: 2em; font-weight: bold; color: #059669; }
        .coverage-table { width: 100%; border-collapse: collapse; margin: 20px 0; }
        .coverage-table th, .coverage-table td { border: 1px solid #e5e7eb; padding: 12px; text-align: left; }
        .coverage-table th { background-color: #f9fafb; }
        .coverage-high { background-color: #d1fae5; }
        .coverage-medium { background-color: #fef3c7; }
        .coverage-low { background-color: #fee2e2; }
    </style>
</head>
<body>
    <div class="container">
        <header class="header">
            <h1>Test Coverage Report</h1>
            <p>Generated on {{generation_timestamp}}</p>
        </header>

        <section class="summary">
            <div class="summary-card">
                <div class="summary-number">{{total_components}}</div>
                <div>Total Components</div>
            </div>
            <div class="summary-card">
                <div class="summary-number">{{total_tests}}</div>
                <div>Total Tests</div>
            </div>
            <div class="summary-card">
                <div class="summary-number">{{average_coverage}}%</div>
                <div>Average Coverage</div>
            </div>
            <div class="summary-card">
                <div class="summary-number">{{components_with_full_coverage}}</div>
                <div>100% Coverage</div>
            </div>
        </section>

        <section class="coverage-details">
            <h2>Component Coverage Details</h2>
            <table class="coverage-table">
                <thead>
                    <tr>
                        <th>Component</th>
                        <th>Unit Tests</th>
                        <th>Integration Tests</th>
                        <th>E2E Tests</th>
                        <th>Performance Tests</th>
                        <th>Total Coverage</th>
                    </tr>
                </thead>
                <tbody>
                    {{#each components}}
                    <tr>
                        <td><strong>{{name}}</strong></td>
                        <td>{{count_tests tests "unit"}}</td>
                        <td>{{count_tests tests "integration"}}</td>
                        <td>{{count_tests tests "e2e"}}</td>
                        <td>{{count_tests tests "performance"}}</td>
                        <td class="{{coverage_class tests.length}}">{{tests.length}} tests</td>
                    </tr>
                    {{/each}}
                </tbody>
            </table>
        </section>
    </div>
</body>
</html>
"#;

/// Handlebars helper for code formatting
pub fn format_code_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    if let Some(code) = h.param(0).and_then(|v| v.value().as_str()) {
        // Simple HTML escaping for code display
        let escaped = html_escape::encode_text(code);
        out.write(&escaped)?;
    }
    Ok(())
}

/// Handlebars helper for markdown rendering
pub fn markdown_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    if let Some(markdown) = h.param(0).and_then(|v| v.value().as_str()) {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        
        let parser = Parser::new_ext(markdown, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        
        out.write(&html_output)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use handlebars::Handlebars;

    #[test]
    fn test_format_code_helper() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("format_code", Box::new(format_code_helper));
        
        let template = "{{format_code code}}";
        handlebars.register_template_string("test", template).unwrap();
        
        let data = serde_json::json!({
            "code": "<button>Click me</button>"
        });
        
        let result = handlebars.render("test", &data).unwrap();
        assert!(result.contains("&lt;button&gt;"));
    }

    #[test]
    fn test_markdown_helper() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("markdown", Box::new(markdown_helper));
        
        let template = "{{{markdown text}}}";
        handlebars.register_template_string("test", template).unwrap();
        
        let data = serde_json::json!({
            "text": "# Heading\n\nThis is **bold** text."
        });
        
        let result = handlebars.render("test", &data).unwrap();
        assert!(result.contains("<h1>Heading</h1>"));
        assert!(result.contains("<strong>bold</strong>"));
    }

    #[test]
    fn test_api_doc_template_compilation() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("format_code", Box::new(format_code_helper));
        handlebars.register_helper("markdown", Box::new(markdown_helper));
        
        let result = handlebars.register_template_string("api_doc", API_DOC_TEMPLATE);
        assert!(result.is_ok());
    }

    #[test]
    fn test_gallery_template_compilation() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("format_code", Box::new(format_code_helper));
        handlebars.register_helper("markdown", Box::new(markdown_helper));
        
        let result = handlebars.register_template_string("gallery", GALLERY_TEMPLATE);
        assert!(result.is_ok());
    }

    #[test]
    fn test_test_report_template_compilation() {
        let mut handlebars = Handlebars::new();
        
        let result = handlebars.register_template_string("test_report", TEST_REPORT_TEMPLATE);
        assert!(result.is_ok());
    }
}