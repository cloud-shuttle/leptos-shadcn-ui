# Architecture Decision Records (ADRs)

This directory contains the Architecture Decision Records (ADRs) for our data engineering consultancy. These documents capture important architectural decisions, their context, and consequences.

## ADR Index

| ADR | Title | Status | Date |
|-----|-------|--------|------|
| [ADR-001](./001-tdd-first-approach.md) | Test-Driven Development (TDD) First Approach | ACCEPTED | 2024-09-08 |
| [ADR-002](./002-testing-pyramid-strategy.md) | Testing Pyramid Strategy | ACCEPTED | 2024-09-08 |
| [ADR-003](./003-playwright-testing-demos.md) | Playwright Testing for Demos and Applications | ACCEPTED | 2024-09-08 |
| [ADR-004](./004-api-contracts-and-testing.md) | API Contracts and Testing Strategy | ACCEPTED | 2024-09-08 |
| [ADR-005](./005-pnpm-package-management.md) | PNPM Package Management Strategy | ACCEPTED | 2024-09-08 |
| [ADR-006](./006-leptos-versioning-strategy.md) | Leptos Versioning and Latest Support Strategy | ACCEPTED | 2024-09-08 |
| [ADR-007](./007-rust-coding-standards.md) | Rust Coding Standards and Latest Practices | ACCEPTED | 2024-09-08 |
| [ADR-008](./008-competitive-analysis-strategy.md) | Competitive Analysis and Capability Matching Strategy | ACCEPTED | 2024-09-08 |

## ADR Template

When creating new ADRs, use the following template:

```markdown
# ADR-XXX: [Title]

## Status
**PROPOSED** / **ACCEPTED** / **DEPRECATED** / **SUPERSEDED** - YYYY-MM-DD

## Context
[Describe the context and problem statement]

## Decision
[State the architectural decision]

## Consequences

### Positive
[Describe the positive consequences]

### Negative
[Describe the negative consequences]

### Mitigation
[Describe how negative consequences will be mitigated]

## Implementation
[Describe the implementation details]

## Review and Updates
[Describe the review process and update triggers]

## Related ADRs
[List related ADRs]
```

## ADR Process

1. **Proposal**: Create ADR with PROPOSED status
2. **Review**: Team review and discussion
3. **Decision**: Accept, reject, or modify
4. **Implementation**: Implement the decision
5. **Review**: Regular review and updates

## ADR Maintenance

- **Regular Reviews**: ADRs are reviewed quarterly
- **Updates**: ADRs are updated when decisions change
- **Deprecation**: ADRs are deprecated when superseded
- **Archival**: Deprecated ADRs are archived but not deleted

## Contact

For questions about ADRs, contact the architecture team at architecture@dataengineeringpro.com.
