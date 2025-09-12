# ADR-007: Rust Coding Standards and Latest Practices

## Status
**ACCEPTED** - 2024-09-08

## Context
As a data engineering consultancy specializing in Rust, we need to establish clear coding standards and practices that ensure high-quality, maintainable, and performant code that follows the latest Rust best practices and idioms.

## Decision
We adopt **comprehensive Rust coding standards** that prioritize the latest Rust features, best practices, and performance optimizations while maintaining code quality and consistency across all projects.

### Rust Standards

#### Version and Toolchain
- **Rust Version**: Always use the latest stable Rust (currently 1.89.0)
- **Edition**: Use Rust 2021 edition for all new projects
- **Toolchain**: Use `rustup` for toolchain management
- **Components**: Include `rustfmt`, `clippy`, and `rust-analyzer`

#### Code Quality Standards
- **Formatting**: Use `rustfmt` with default settings
- **Linting**: Use `clippy` with strict settings
- **Documentation**: Comprehensive documentation for all public APIs
- **Testing**: 100% test coverage for all public functions
- **Performance**: Benchmark critical code paths

## Implementation

### Cargo.toml Configuration
```toml
# Example: Rust project configuration
[package]
name = "leptos-consultancy"
version = "1.0.0"
edition = "2021"
rust-version = "1.89.0"

[dependencies]
# Latest stable versions
leptos = { version = "0.8.8", features = ["csr"] }
leptos_router = "0.8.8"
wasm-bindgen = "0.2.101"
web-sys = "0.3.77"
serde = { version = "1.0", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.6", features = ["v4", "serde"] }
tokio = { version = "1.0", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }

[dev-dependencies]
# Testing and benchmarking
criterion = { version = "0.5", features = ["html_reports"] }
proptest = "1.4"
mockall = "0.12"
wasm-bindgen-test = "0.3"

[profile.release]
# Optimize for performance
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"

[profile.dev]
# Optimize for compilation speed
opt-level = 0
debug = true
```

### Rustfmt Configuration
```toml
# rustfmt.toml
edition = "2021"
max_width = 100
tab_spaces = 4
newline_style = "Unix"
use_small_heuristics = "Default"
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
```

### Clippy Configuration
```toml
# .clippy.toml
# Allow some lints that are too strict for our use case
allow = [
    "clippy::too_many_arguments",
    "clippy::needless_pass_by_value",
]

# Deny important lints
deny = [
    "clippy::all",
    "clippy::pedantic",
    "clippy::nursery",
    "clippy::cargo",
]

# Set specific lint levels
warn = [
    "clippy::missing_docs_in_private_items",
    "clippy::missing_errors_doc",
    "clippy::missing_panics_doc",
]
```

### Code Standards Examples

#### Error Handling
```rust
// Example: Proper error handling with thiserror
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataProcessingError {
    #[error("Invalid data format: {0}")]
    InvalidFormat(String),
    
    #[error("Processing timeout after {0} seconds")]
    Timeout(u64),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

// Example: Result type usage
pub fn process_data(input: &str) -> Result<ProcessedData, DataProcessingError> {
    let parsed = parse_input(input)?;
    let processed = transform_data(parsed)?;
    Ok(processed)
}
```

#### Async/Await Best Practices
```rust
// Example: Proper async/await usage
use tokio::time::{timeout, Duration};

pub async fn fetch_data_with_timeout(
    url: &str,
    timeout_duration: Duration,
) -> Result<Data, DataProcessingError> {
    let client = reqwest::Client::new();
    
    let response = timeout(timeout_duration, client.get(url).send())
        .await
        .map_err(|_| DataProcessingError::Timeout(timeout_duration.as_secs()))?
        .map_err(|e| DataProcessingError::Io(e.into()))?;
    
    let data: Data = response
        .json()
        .await
        .map_err(DataProcessingError::Serialization)?;
    
    Ok(data)
}
```

#### Memory Management
```rust
// Example: Efficient memory management
use std::collections::HashMap;
use std::sync::Arc;

pub struct DataProcessor {
    cache: Arc<HashMap<String, ProcessedData>>,
    config: Arc<ProcessorConfig>,
}

impl DataProcessor {
    pub fn new(config: ProcessorConfig) -> Self {
        Self {
            cache: Arc::new(HashMap::new()),
            config: Arc::new(config),
        }
    }
    
    pub async fn process(&self, input: &str) -> Result<ProcessedData, DataProcessingError> {
        // Check cache first
        if let Some(cached) = self.cache.get(input) {
            return Ok(cached.clone());
        }
        
        // Process and cache result
        let result = self.process_internal(input).await?;
        self.cache.insert(input.to_string(), result.clone());
        
        Ok(result)
    }
}
```

#### Testing Standards
```rust
// Example: Comprehensive testing
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use mockall::predicate::*;
    
    #[test]
    fn test_data_processing_success() {
        let processor = DataProcessor::new(ProcessorConfig::default());
        let input = "test,data,here";
        
        let result = processor.process(input).unwrap();
        
        assert_eq!(result.record_count, 3);
        assert!(result.is_valid());
    }
    
    #[test]
    fn test_data_processing_invalid_format() {
        let processor = DataProcessor::new(ProcessorConfig::default());
        let input = "invalid format";
        
        let result = processor.process(input);
        
        assert!(matches!(result, Err(DataProcessingError::InvalidFormat(_))));
    }
    
    #[tokio::test]
    async fn test_async_data_processing() {
        let processor = DataProcessor::new(ProcessorConfig::default());
        let input = "async,test,data";
        
        let result = processor.process(input).await.unwrap();
        
        assert_eq!(result.record_count, 3);
    }
    
    // Property-based testing
    proptest! {
        #[test]
        fn test_data_processing_properties(input in "[a-zA-Z0-9,]+") {
            let processor = DataProcessor::new(ProcessorConfig::default());
            let result = processor.process(&input);
            
            // Properties that should always hold
            match result {
                Ok(data) => {
                    prop_assert!(data.record_count >= 0);
                    prop_assert!(data.is_valid());
                }
                Err(DataProcessingError::InvalidFormat(_)) => {
                    // Invalid format should be caught
                    prop_assert!(true);
                }
                _ => prop_assert!(false, "Unexpected error type"),
            }
        }
    }
}
```

#### Performance Benchmarking
```rust
// Example: Performance benchmarking with Criterion
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_data_processing(c: &mut Criterion) {
    let processor = DataProcessor::new(ProcessorConfig::default());
    let test_data = "benchmark,test,data,with,multiple,records";
    
    c.bench_function("data_processing", |b| {
        b.iter(|| {
            processor.process(black_box(test_data))
        })
    });
}

fn benchmark_async_data_processing(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let processor = DataProcessor::new(ProcessorConfig::default());
    let test_data = "async,benchmark,test,data";
    
    c.bench_function("async_data_processing", |b| {
        b.to_async(&rt).iter(|| async {
            processor.process(black_box(test_data)).await
        })
    });
}

criterion_group!(benches, benchmark_data_processing, benchmark_async_data_processing);
criterion_main!(benches);
```

### CI/CD Integration
```yaml
# .github/workflows/rust-ci.yml
name: Rust CI
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          components: rustfmt, clippy
          
      - name: Cache cargo registry
        uses: actions/cache@v3
        with:
          path: ~/.cargo/registry
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
          
      - name: Cache cargo index
        uses: actions/cache@v3
        with:
          path: ~/.cargo/git
          key: ${{ runner.os }}-cargo-index-${{ hashFiles('**/Cargo.lock') }}
          
      - name: Cache cargo build
        uses: actions/cache@v3
        with:
          path: target
          key: ${{ runner.os }}-cargo-build-target-${{ hashFiles('**/Cargo.lock') }}
          
      - name: Install cargo-tarpaulin
        run: cargo install cargo-tarpaulin
        
      - name: Run tests
        run: cargo test --verbose
        
      - name: Run clippy
        run: cargo clippy --all-targets --all-features -- -D warnings
        
      - name: Run rustfmt
        run: cargo fmt --all -- --check
        
      - name: Generate test coverage
        run: cargo tarpaulin --out Html --output-dir coverage
        
      - name: Upload coverage to Codecov
        uses: codecov/codecov-action@v3
        with:
          file: coverage/tarpaulin-report.html
```

## Quality Standards

### Code Quality Requirements
- **Formatting**: All code must be formatted with `rustfmt`
- **Linting**: All code must pass `clippy` with strict settings
- **Documentation**: All public APIs must be documented
- **Testing**: 100% test coverage for all public functions
- **Performance**: Critical paths must be benchmarked

### Development Workflow
1. **Setup**: Use latest Rust toolchain
2. **Development**: Follow coding standards and best practices
3. **Testing**: Write comprehensive tests
4. **Review**: Code review for quality and standards compliance
5. **CI/CD**: Automated testing and quality checks

## Tools and Technologies

### Development Tools
- **rustup**: Toolchain management
- **rustfmt**: Code formatting
- **clippy**: Linting and code analysis
- **rust-analyzer**: Language server
- **cargo**: Package management

### Testing Tools
- **cargo test**: Built-in testing framework
- **criterion**: Performance benchmarking
- **proptest**: Property-based testing
- **mockall**: Mocking framework
- **tarpaulin**: Code coverage analysis

### Quality Tools
- **cargo audit**: Security vulnerability scanning
- **cargo outdated**: Dependency update checking
- **cargo tree**: Dependency tree visualization
- **cargo expand**: Macro expansion

## Metrics and Monitoring

### Code Quality Metrics
- **Test coverage**: Percentage of code covered by tests
- **Clippy warnings**: Number of linting warnings
- **Documentation coverage**: Percentage of documented APIs
- **Performance benchmarks**: Execution time and memory usage

### Development Metrics
- **Build time**: Compilation and test execution time
- **Code complexity**: Cyclomatic complexity metrics
- **Technical debt**: Code quality and maintainability metrics
- **Security vulnerabilities**: Number of security issues found

## Review and Updates

### Regular Reviews
- **Weekly**: Review code quality metrics
- **Monthly**: Update dependencies and toolchain
- **Quarterly**: Review and update coding standards
- **Annually**: Strategic planning for tool and practice updates

### Update Triggers
- **New Rust release**: Immediate evaluation and adoption
- **New tool releases**: Evaluation and adoption planning
- **Security updates**: Immediate implementation
- **Performance improvements**: Evaluation and adoption

## Related ADRs
- ADR-001: Test-Driven Development (TDD) First Approach
- ADR-002: Testing Pyramid Strategy
- ADR-003: Playwright Testing for Demos
- ADR-004: API Contracts and Testing
- ADR-006: Leptos Versioning and Latest Support Strategy
