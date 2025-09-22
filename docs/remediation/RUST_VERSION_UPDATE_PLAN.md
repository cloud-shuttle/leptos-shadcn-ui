# 🦀 Rust Version Update Plan
**Priority 3: Update to Latest Rust & Dependencies**

## 📊 Current Version Analysis

### **Current State (September 20, 2025)**
- **Rust Version**: 1.89.0 (August 4, 2025)
- **Leptos**: 0.8.9 (workspace dependency)
- **Edition**: 2024 (latest)
- **Target Architecture**: wasm32-unknown-unknown ✅

### **Target State**
- **Rust Version**: 1.90.0+ (September 2025)
- **Leptos**: 0.8.10+ (if available)
- **Dependencies**: Latest stable versions
- **Security**: All dependencies audited

---

## 🔍 Dependency Analysis

### **Core Dependencies Status**

| Dependency | Current Version | Latest Version | Status | Action Required |
|------------|----------------|----------------|--------|-----------------|
| leptos | 0.8.9 | 0.8.10+ | ⚠️ Check | Update if available |
| leptos-router | 0.8.9 | 0.8.10+ | ⚠️ Check | Update if available |
| wasm-bindgen | 0.2 | 0.2.95+ | ⚠️ Check | Update to latest |
| web-sys | 0.3 | 0.3.70+ | ⚠️ Check | Update to latest |
| serde | 1.0 | 1.0.200+ | ⚠️ Check | Update to latest |
| tokio | 1.47.1 | 1.50+ | ⚠️ Check | Update to latest |
| clap | 4.5.46 | 4.5.50+ | ⚠️ Check | Update to latest |

### **WASM-Specific Dependencies**

| Dependency | Current Version | Latest Version | Status | Action Required |
|------------|----------------|----------------|--------|-----------------|
| getrandom | 0.2 | 0.2.15+ | ⚠️ Check | Update to latest |
| uuid | 1.0 | 1.10+ | ⚠️ Check | Update to latest |
| console_error_panic_hook | 0.1 | 0.1.7+ | ⚠️ Check | Update to latest |
| wasm-bindgen-test | 0.3 | 0.3.40+ | ⚠️ Check | Update to latest |

---

## 🚀 Update Strategy

### **Phase 1: Rust Version Update (Week 1)**

#### **1. Update Rust Toolchain**
```bash
# Check current version
rustc --version

# Update to latest stable
rustup update stable

# Verify update
rustc --version
```

#### **2. Update rust-toolchain.toml**
```toml
# Create/update rust-toolchain.toml
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy", "rust-src"]
targets = ["wasm32-unknown-unknown"]
```

#### **3. Verify WASM Compatibility**
```bash
# Test WASM compilation
cargo check --target wasm32-unknown-unknown

# Test WASM demo
cd wasm-demo
cargo build --target wasm32-unknown-unknown
```

### **Phase 2: Dependency Updates (Week 2)**

#### **1. Update Workspace Dependencies**
```toml
# Cargo.toml workspace dependencies
[workspace.dependencies]
leptos = "0.8.10"  # Update if available
leptos-router = "0.8.10"  # Update if available
getrandom = { version = "0.2.15", features = ["js"] }
serde = { version = "1.0.200", features = ["derive"] }
serde_json = "1.0.200"
wasm-bindgen = "0.2.95"
web-sys = "0.3.70"
js-sys = "0.3.70"
wasm-bindgen-test = "0.3.40"
console_error_panic_hook = "0.1.7"
uuid = { version = "1.10", features = ["v4", "js"] }
tokio = { version = "1.50", features = ["full"] }
clap = { version = "4.5.50", features = ["derive"] }
```

#### **2. Update Individual Package Dependencies**
```bash
# Update all packages
cargo update

# Check for outdated dependencies
cargo outdated

# Audit for security vulnerabilities
cargo audit
```

#### **3. Test Compatibility**
```bash
# Test all packages
cargo test --workspace

# Test WASM packages specifically
cargo test --workspace --target wasm32-unknown-unknown

# Test published packages
cargo test -p leptos-shadcn-ui-wasm
```

### **Phase 3: Security & Performance (Week 3)**

#### **1. Security Audit**
```bash
# Run security audit
cargo audit

# Check for known vulnerabilities
cargo audit --deny warnings

# Update vulnerable dependencies
cargo update
```

#### **2. Performance Testing**
```bash
# Run performance benchmarks
cargo bench

# Test bundle sizes
cd wasm-demo
cargo build --release --target wasm32-unknown-unknown
du -h pkg/*.wasm
```

#### **3. Compatibility Testing**
```bash
# Test with different Rust versions
rustup toolchain install beta
cargo +beta test --workspace

# Test with different targets
cargo test --workspace --target x86_64-unknown-linux-gnu
cargo test --workspace --target wasm32-unknown-unknown
```

---

## 🛠️ Implementation Plan

### **Week 1: Rust Version Update**
- [ ] Update Rust toolchain to latest stable
- [ ] Create/update rust-toolchain.toml
- [ ] Verify WASM compatibility
- [ ] Test all packages with new Rust version

### **Week 2: Dependency Updates**
- [ ] Update workspace dependencies
- [ ] Update individual package dependencies
- [ ] Run cargo update and cargo outdated
- [ ] Test compatibility across all packages

### **Week 3: Security & Performance**
- [ ] Run security audit
- [ ] Update vulnerable dependencies
- [ ] Run performance benchmarks
- [ ] Test bundle sizes and performance

### **Week 4: Validation & Documentation**
- [ ] Validate all tests pass
- [ ] Update documentation
- [ ] Create update guidelines
- [ ] Document any breaking changes

---

## 📈 Success Metrics

### **Version Targets**
- **Rust 1.90.0+** (latest stable)
- **All dependencies** updated to latest stable
- **Zero security vulnerabilities** (cargo audit clean)
- **All tests passing** with new versions

### **Performance Metrics**
- **Bundle size** maintained or improved
- **Compilation time** maintained or improved
- **Test execution time** maintained or improved
- **WASM compatibility** maintained

### **Quality Metrics**
- **No breaking changes** in public APIs
- **Backward compatibility** maintained
- **Documentation** updated for new versions
- **Team training** completed on new versions

---

## 🔧 Update Tools & Commands

### **Version Checking**
```bash
# Check Rust version
rustc --version

# Check dependency versions
cargo outdated

# Check for security issues
cargo audit
```

### **Update Commands**
```bash
# Update Rust
rustup update stable

# Update dependencies
cargo update

# Update specific dependency
cargo update -p dependency-name
```

### **Testing Commands**
```bash
# Test all packages
cargo test --workspace

# Test WASM packages
cargo test --workspace --target wasm32-unknown-unknown

# Test specific package
cargo test -p package-name
```

---

## 🚨 Risk Mitigation

### **Breaking Changes**
- **Test thoroughly** before updating
- **Maintain backward compatibility** where possible
- **Document breaking changes** clearly
- **Provide migration guides** for users

### **WASM Compatibility**
- **Test WASM compilation** after each update
- **Verify WASM demo** still works
- **Check bundle sizes** for regressions
- **Test in browsers** for compatibility

### **Performance Regressions**
- **Run benchmarks** before and after updates
- **Monitor bundle sizes** for increases
- **Test compilation times** for regressions
- **Profile performance** if issues arise

---

## 📋 Update Checklist

### **Pre-Update**
- [ ] Backup current working state
- [ ] Document current versions
- [ ] Run full test suite
- [ ] Check current performance metrics

### **During Update**
- [ ] Update Rust toolchain
- [ ] Update dependencies incrementally
- [ ] Test after each major update
- [ ] Fix any compilation errors

### **Post-Update**
- [ ] Run full test suite
- [ ] Check performance metrics
- [ ] Update documentation
- [ ] Notify team of changes

---

## 🚀 Next Steps

1. **Check latest Rust version** availability
2. **Update Rust toolchain** to latest stable
3. **Update workspace dependencies** incrementally
4. **Test compatibility** across all packages
5. **Document any breaking changes** or migration requirements

This update plan ensures the repository stays current with the latest Rust and dependency versions while maintaining stability and performance.

---

*Plan created: September 20, 2025*  
*Target completion: October 11, 2025*
