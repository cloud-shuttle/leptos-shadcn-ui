# ADR-006: Leptos Versioning and Latest Support Strategy

## Status
**ACCEPTED** - 2024-09-08

## Context
As a data engineering consultancy specializing in Rust and Leptos, we need a clear strategy for managing Leptos versions and ensuring we always support the latest features while maintaining stability for our clients.

## Decision
We implement a **proactive Leptos versioning strategy** that prioritizes the latest stable versions while maintaining backward compatibility and providing migration support.

### Versioning Strategy

#### Current Version Support
- **Primary**: Leptos v0.8.8 (latest stable)
- **LTS**: Maintain support for previous stable versions
- **Beta**: Evaluate beta releases for future adoption
- **Migration**: Provide migration services for version upgrades

#### Version Management Principles
- **Latest first**: Always target the latest stable version for new projects
- **Backward compatibility**: Maintain support for previous versions
- **Migration support**: Provide upgrade services for existing projects
- **Feature evaluation**: Continuously evaluate new features and capabilities

## Implementation

### Cargo.toml Configuration
```toml
# Example: Latest Leptos configuration
[package]
name = "leptos-consultancy"
version = "1.0.0"
edition = "2021"

[dependencies]
# Latest stable Leptos
leptos = { version = "0.8.8", features = ["csr"] }
leptos_router = "0.8.8"

# Supporting crates - always latest
wasm-bindgen = "0.2.101"
web-sys = "0.3.77"
console_error_panic_hook = "0.1.7"
console_log = "1.0.0"
log = "0.4.20"
serde = { version = "1.0", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.6", features = ["v4", "serde"] }

[dev-dependencies]
# Latest testing tools
wasm-pack = "0.12.1"
cargo-leptos = "0.2.43"
```

### Version Compatibility Matrix
```yaml
# leptos-versions.yml
leptos_versions:
  current:
    version: "0.8.8"
    status: "stable"
    features:
      - "CSR (Client-Side Rendering)"
      - "SSR (Server-Side Rendering)"
      - "Islands Architecture"
      - "Reactive Signals"
      - "Router"
      - "Forms"
      - "WebSocket Support"
    
  previous:
    version: "0.8.6"
    status: "lts"
    support_until: "2024-12-31"
    migration_path: "0.8.6 -> 0.8.8"
    
  beta:
    version: "0.9.0-beta.1"
    status: "beta"
    evaluation_status: "in_progress"
    expected_stable: "2024-10-15"
    
  roadmap:
    - version: "0.9.0"
      expected: "2024-10-15"
      features: ["Enhanced Performance", "New Router", "Improved DevX"]
    - version: "1.0.0"
      expected: "2025-01-15"
      features: ["Stable API", "Long-term Support"]
```

### Migration Strategy
```rust
// Example: Version migration helper
pub struct LeptosMigration {
    from_version: String,
    to_version: String,
    breaking_changes: Vec<BreakingChange>,
    migration_steps: Vec<MigrationStep>,
}

pub struct BreakingChange {
    description: String,
    impact: ImpactLevel,
    migration_guide: String,
    automated_fix: Option<String>,
}

pub enum ImpactLevel {
    Low,
    Medium,
    High,
    Critical,
}

pub struct MigrationStep {
    step_number: u32,
    description: String,
    code_example: String,
    automated: bool,
}

impl LeptosMigration {
    pub fn new(from: &str, to: &str) -> Self {
        Self {
            from_version: from.to_string(),
            to_version: to.to_string(),
            breaking_changes: Self::identify_breaking_changes(from, to),
            migration_steps: Self::generate_migration_steps(from, to),
        }
    }
    
    pub fn execute_migration(&self) -> Result<(), MigrationError> {
        for step in &self.migration_steps {
            if step.automated {
                self.execute_automated_step(step)?;
            } else {
                self.execute_manual_step(step)?;
            }
        }
        Ok(())
    }
}
```

### Version Testing Strategy
```rust
// Example: Multi-version testing
#[cfg(test)]
mod version_tests {
    use super::*;
    
    #[test]
    fn test_leptos_0_8_8_compatibility() {
        // Test current version features
        let app = create_leptos_app();
        assert!(app.is_compatible_with("0.8.8"));
    }
    
    #[test]
    fn test_migration_from_0_8_6() {
        // Test migration from previous version
        let migration = LeptosMigration::new("0.8.6", "0.8.8");
        assert!(migration.can_migrate());
        assert_eq!(migration.breaking_changes.len(), 0);
    }
    
    #[test]
    fn test_beta_version_evaluation() {
        // Test beta version features
        let beta_features = evaluate_beta_features("0.9.0-beta.1");
        assert!(beta_features.performance_improvements > 0);
        assert!(beta_features.new_features.len() > 0);
    }
}
```

### Playwright Version Testing
```typescript
// Example: Version compatibility testing
import { test, expect } from '@playwright/test';

test.describe('Leptos Version Compatibility', () => {
  test('should work with latest Leptos version', async ({ page }) => {
    await page.goto('/');
    
    // Test latest version features
    await expect(page.locator('[data-leptos-version="0.8.8"]')).toBeVisible();
    
    // Test reactive signals
    await page.click('[data-testid="increment-button"]');
    await expect(page.locator('[data-testid="counter"]')).toContainText('1');
    
    // Test router functionality
    await page.click('[data-testid="navigate-to-about"]');
    await expect(page).toHaveURL('/about');
  });

  test('should handle version migration gracefully', async ({ page }) => {
    // Test migration from previous version
    await page.goto('/migration-test');
    
    // Verify no breaking changes
    await expect(page.locator('[data-testid="migration-status"]'))
      .toContainText('Migration successful');
    
    // Test all features still work
    await expect(page.locator('[data-testid="all-features"]'))
      .toContainText('All features working');
  });
});
```

## Quality Standards

### Version Management Requirements
- **Latest adoption**: New projects must use latest stable version
- **Migration support**: Provide migration services for existing projects
- **Testing**: Comprehensive testing across all supported versions
- **Documentation**: Maintain version-specific documentation

### Client Support
- **Version recommendations**: Provide version recommendations based on project needs
- **Migration planning**: Create migration plans for version upgrades
- **Training**: Provide training on new version features
- **Support**: Ongoing support for all supported versions

## Tools and Technologies

### Version Management
- **Cargo**: Rust package management
- **wasm-pack**: WebAssembly packaging
- **cargo-leptos**: Leptos-specific build tools
- **Git**: Version control and branching

### Testing and Validation
- **Playwright**: End-to-end testing
- **wasm-bindgen-test**: WebAssembly testing
- **Criterion**: Performance benchmarking
- **Tarpaulin**: Code coverage analysis

### Migration Tools
- **cargo fix**: Automated code fixes
- **Custom migration scripts**: Automated migration tools
- **Version compatibility checker**: Automated compatibility validation
- **Migration documentation**: Comprehensive migration guides

## Monitoring and Metrics

### Version Adoption Metrics
- **Latest version usage**: Percentage of projects using latest version
- **Migration success rate**: Percentage of successful migrations
- **Version support requests**: Number of version-related support requests
- **Feature adoption**: Adoption rate of new features

### Quality Metrics
- **Migration time**: Average time to complete migrations
- **Breaking change impact**: Impact assessment of breaking changes
- **Client satisfaction**: Client feedback on version support
- **Performance improvements**: Performance gains from version upgrades

## Review and Updates

### Regular Reviews
- **Monthly**: Review latest version features and capabilities
- **Quarterly**: Evaluate version adoption and migration needs
- **Annually**: Strategic planning for major version upgrades

### Update Triggers
- **New stable release**: Immediate evaluation and adoption planning
- **Security updates**: Immediate implementation and client notification
- **Performance improvements**: Evaluation and adoption planning
- **Client requests**: Custom version support based on client needs

## Related ADRs
- ADR-001: Test-Driven Development (TDD) First Approach
- ADR-002: Testing Pyramid Strategy
- ADR-003: Playwright Testing for Demos
- ADR-004: API Contracts and Testing
