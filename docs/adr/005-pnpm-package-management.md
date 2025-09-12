# ADR-005: PNPM Package Management Strategy

## Status
**ACCEPTED** - 2024-09-08

## Context
As a data engineering consultancy, we need a reliable, efficient, and consistent package management strategy for our JavaScript/TypeScript projects, including our Playwright tests, build tools, and frontend components.

## Decision
We adopt **PNPM as our standard package manager** for all JavaScript/TypeScript projects to ensure consistency, efficiency, and reliability across all our development work.

### PNPM Benefits
- **Disk efficiency**: Shared dependency storage reduces disk usage
- **Speed**: Faster installation and resolution times
- **Strict dependency management**: Prevents phantom dependencies
- **Monorepo support**: Excellent support for monorepo architectures
- **Security**: Built-in security features and audit capabilities

## Implementation

### PNPM Configuration
```json
// .npmrc - Project configuration
registry=https://registry.npmjs.org/
shamefully-hoist=false
strict-peer-dependencies=false
auto-install-peers=true
prefer-frozen-lockfile=true
```

### Package.json Configuration
```json
{
  "name": "leptos-consultancy",
  "version": "1.0.0",
  "packageManager": "pnpm@8.15.0",
  "engines": {
    "node": ">=18.0.0",
    "pnpm": ">=8.0.0"
  },
  "scripts": {
    "install": "pnpm install --frozen-lockfile",
    "build": "pnpm run build:css && pnpm run build:wasm",
    "build:css": "sass src/style/main.scss src/style/main.css",
    "build:wasm": "wasm-pack build --target web --out-dir pkg",
    "dev": "concurrently \"pnpm run watch:css\" \"pnpm run serve\"",
    "watch:css": "sass --watch src/style/main.scss:src/style/main.css",
    "serve": "python3 -m http.server 8080",
    "test": "pnpm run test:unit && pnpm run test:integration && pnpm run test:e2e",
    "test:unit": "cargo test",
    "test:integration": "cargo test --test integration",
    "test:e2e": "playwright test",
    "test:e2e:ui": "playwright test --ui",
    "test:e2e:headed": "playwright test --headed",
    "test:all": "pnpm run test && pnpm run test:e2e",
    "test:watch": "playwright test --watch",
    "playwright:install": "playwright install",
    "playwright:install-deps": "playwright install-deps"
  },
  "devDependencies": {
    "sass": "^1.69.5",
    "concurrently": "^8.2.2",
    "@playwright/test": "^1.40.0",
    "playwright": "^1.40.0"
  }
}
```

### PNPM Workspace Configuration
```yaml
# pnpm-workspace.yaml - For monorepo projects
packages:
  - 'packages/*'
  - 'apps/*'
  - 'tools/*'
  - 'tests/*'
```

### CI/CD Integration
```yaml
# .github/workflows/ci.yml
name: CI/CD Pipeline
on: [push, pull_request]

jobs:
  install-and-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '18'
          
      - name: Setup PNPM
        uses: pnpm/action-setup@v2
        with:
          version: 8
          
      - name: Get PNPM store directory
        shell: bash
        run: |
          echo "STORE_PATH=$(pnpm store path --silent)" >> $GITHUB_ENV
          
      - name: Setup PNPM cache
        uses: actions/cache@v3
        with:
          path: ${{ env.STORE_PATH }}
          key: ${{ runner.os }}-pnpm-store-${{ hashFiles('**/pnpm-lock.yaml') }}
          restore-keys: |
            ${{ runner.os }}-pnpm-store-
            
      - name: Install dependencies
        run: pnpm install --frozen-lockfile
        
      - name: Run tests
        run: pnpm test
        
      - name: Run Playwright tests
        run: pnpm run test:e2e
```

## Quality Standards

### Package Management Requirements
- **Lockfile**: Always commit `pnpm-lock.yaml`
- **Version pinning**: Use exact versions for critical dependencies
- **Security**: Regular security audits with `pnpm audit`
- **Updates**: Regular dependency updates with `pnpm update`

### Development Workflow
1. **Installation**: Use `pnpm install` for dependency installation
2. **Adding packages**: Use `pnpm add` for new dependencies
3. **Scripts**: Use `pnpm run` for script execution
4. **Auditing**: Regular `pnpm audit` for security checks

## Tools and Technologies

### PNPM Features
- **Workspaces**: Monorepo support
- **Filters**: Selective package operations
- **Patching**: Package patching capabilities
- **Audit**: Security vulnerability scanning

### Integration Tools
- **GitHub Actions**: CI/CD integration
- **Docker**: Containerization support
- **VS Code**: Editor integration
- **ESLint**: Code quality tools

## Related ADRs
- ADR-001: Test-Driven Development (TDD) First Approach
- ADR-002: Testing Pyramid Strategy
- ADR-003: Playwright Testing for Demos
