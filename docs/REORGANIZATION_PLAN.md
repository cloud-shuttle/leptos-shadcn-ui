# Documentation Reorganization Plan

## Current Issues
- 50+ markdown files scattered across root directory and docs/
- Inconsistent naming conventions (UPPERCASE vs lowercase)
- Poor organization with related content spread across folders
- Duplicate and outdated content
- Missing clear entry points for different user types

## Proposed Structure

```
docs/
├── README.md                          # Main entry point
├── getting-started/
│   ├── README.md                      # Quick start guide
│   ├── installation.md                # Installation instructions
│   ├── first-component.md             # Creating your first component
│   └── examples/                      # Basic examples
├── architecture/
│   ├── README.md                      # Architecture overview
│   ├── design-decisions/              # ADRs (Architecture Decision Records)
│   ├── migration-guides/              # Version migration guides
│   ├── coverage/                      # Test coverage documentation
│   └── performance/                   # Performance analysis
├── components/
│   ├── README.md                      # Component library overview
│   ├── api-reference/                 # Component API docs
│   ├── examples/                      # Component examples
│   ├── accessibility/                 # Accessibility guides
│   └── theming/                       # Theming and customization
├── testing/
│   ├── README.md                      # Testing overview
│   ├── unit-testing.md                # Unit testing guide
│   ├── integration-testing.md         # Integration testing
│   ├── e2e-testing.md                 # End-to-end testing
│   └── test-generation.md             # Automated test generation
├── releases/
│   ├── README.md                      # Release process
│   ├── changelog.md                   # Version history
│   ├── migration-guides/              # Breaking changes
│   └── release-notes/                 # Detailed release notes
├── roadmap/
│   ├── README.md                      # Roadmap overview
│   ├── v1.0-plan.md                   # Version 1.0 planning
│   └── future-features.md             # Planned features
└── contributing/
    ├── README.md                      # Contributing overview
    ├── development-setup.md           # Development environment
    ├── coding-standards.md            # Code style and standards
    ├── pull-request-process.md        # PR guidelines
    └── adr/                           # Architecture Decision Records
```

## Migration Plan

### Phase 1: Create New Structure
- [x] Create new folder structure
- [ ] Move and rename files according to new structure
- [ ] Update internal links and references

### Phase 2: Consolidate Content
- [ ] Merge duplicate content
- [ ] Remove outdated files
- [ ] Create comprehensive README files for each section

### Phase 3: Improve Content
- [ ] Add missing documentation
- [ ] Improve existing content quality
- [ ] Add cross-references and navigation

### Phase 4: Final Cleanup
- [ ] Remove old files
- [ ] Update all references
- [ ] Test all links

## File Mapping

### Root Level Files to Move:
- `README.md` → Keep as main entry point
- `CONTRIBUTING.md` → `docs/contributing/README.md`
- `ADR_ADHERENCE_REPORT.md` → `docs/architecture/design-decisions/adr-adherence-report.md`
- `COMPETITOR_ANALYSIS_2024.md` → `docs/roadmap/competitor-analysis-2024.md`
- `MARKET_POSITIONING_2025.md` → `docs/roadmap/market-positioning-2025.md`
- `PERFORMANCE_BENCHMARKS_2025.md` → `docs/architecture/performance/benchmarks-2025.md`
- `REACT_NEXTJS_ECOSYSTEM_COMPARISON_2025.md` → `docs/roadmap/react-nextjs-comparison-2025.md`
- `LEPTOS_*.md` → `docs/architecture/migration-guides/`
- `PHASE_*.md` → `docs/releases/phase-completion-summaries/`
- `PUBLISHING_*.md` → `docs/releases/publishing/`
- `RELEASE_*.md` → `docs/releases/release-notes/`
- `TDD_*.md` → `docs/testing/tdd/`
- `VALIDATION_GUIDE.md` → `docs/testing/validation-guide.md`
- `REMEDIATION_PLAN.md` → `docs/architecture/coverage/remediation-plan.md`

### Existing docs/ Structure:
- `docs/adr/` → `docs/contributing/adr/`
- `docs/architecture/` → Keep, but reorganize subfolders
- `docs/components/` → Keep, but improve organization
- `docs/development/` → `docs/contributing/development/`
- `docs/examples/` → `docs/getting-started/examples/`
- `docs/performance-audit/` → `docs/architecture/performance/`
- `docs/quality/` → `docs/contributing/quality/`
- `docs/releases/` → Keep, but reorganize
- `docs/tdd/` → `docs/testing/tdd/`
- `docs/testing/` → Keep, but reorganize
- `docs/v1.0-roadmap/` → `docs/roadmap/v1.0/`

## Benefits
1. **Clear Navigation**: Easy to find relevant documentation
2. **User-Focused**: Organized by user journey (getting started → advanced)
3. **Maintainable**: Clear structure makes updates easier
4. **Professional**: Consistent naming and organization
5. **Scalable**: Structure can grow with the project
