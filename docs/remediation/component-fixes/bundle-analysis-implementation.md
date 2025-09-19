# 🔧 Bundle Analysis Implementation Design

**Component**: `leptos-shadcn-performance-audit`  
**Priority**: 🟡 **HIGH**  
**Issues**: 3 todo! implementations in bundle analysis  
**Timeline**: 3-4 days

## 🚨 Stub Code Issues

### **Missing Implementations**
```rust
// File: performance-audit/src/bundle_analysis.rs
// Line 179:
todo!("Implement component bundle analysis")

// Line 185:
todo!("Implement single component analysis")

// Line 191:
todo!("Implement bundle size extraction")
```

## 🎯 Implementation Strategy

### **Phase 1: Component Bundle Analysis**

#### **1.1 Implement Bundle Analysis Core**
```rust
// File: performance-audit/src/bundle_analysis.rs
use std::path::Path;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentBundleInfo {
    pub component_name: String,
    pub bundle_size: u64,
    pub dependencies: Vec<String>,
    pub exports: Vec<String>,
    pub file_path: String,
}

impl BundleAnalyzer {
    pub fn analyze_component_bundles(&self, components_path: &Path) -> Result<Vec<ComponentBundleInfo>, BundleAnalysisError> {
        let mut bundle_info = Vec::new();
        
        // Scan for component directories
        let entries = fs::read_dir(components_path)?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                if let Some(component_name) = path.file_name().and_then(|n| n.to_str()) {
                    if component_name.starts_with("leptos-shadcn-") {
                        let info = self.analyze_single_component(&path, component_name)?;
                        bundle_info.push(info);
                    }
                }
            }
        }
        
        Ok(bundle_info)
    }
}
```

#### **1.2 Component Analysis Logic**
```rust
impl BundleAnalyzer {
    fn analyze_single_component(&self, component_path: &Path, component_name: &str) -> Result<ComponentBundleInfo, BundleAnalysisError> {
        let mut bundle_size = 0;
        let mut dependencies = Vec::new();
        let mut exports = Vec::new();
        
        // Analyze Cargo.toml for dependencies
        let cargo_toml = component_path.join("Cargo.toml");
        if cargo_toml.exists() {
            let cargo_content = fs::read_to_string(&cargo_toml)?;
            dependencies = self.extract_dependencies(&cargo_content);
        }
        
        // Analyze source files for exports
        let src_path = component_path.join("src");
        if src_path.exists() {
            exports = self.extract_exports(&src_path)?;
            bundle_size = self.calculate_bundle_size(&src_path)?;
        }
        
        Ok(ComponentBundleInfo {
            component_name: component_name.to_string(),
            bundle_size,
            dependencies,
            exports,
            file_path: component_path.to_string_lossy().to_string(),
        })
    }
}
```

### **Phase 2: Single Component Analysis**

#### **2.1 Detailed Component Analysis**
```rust
impl BundleAnalyzer {
    pub fn analyze_single_component_detailed(&self, component_path: &Path) -> Result<DetailedComponentAnalysis, BundleAnalysisError> {
        let mut analysis = DetailedComponentAnalysis::new();
        
        // Analyze source code structure
        analysis.source_files = self.analyze_source_files(component_path)?;
        analysis.dependencies = self.analyze_dependencies(component_path)?;
        analysis.exports = self.analyze_exports(component_path)?;
        analysis.imports = self.analyze_imports(component_path)?;
        
        // Calculate metrics
        analysis.total_lines = self.count_total_lines(&analysis.source_files);
        analysis.complexity_score = self.calculate_complexity(&analysis.source_files);
        analysis.bundle_size_estimate = self.estimate_bundle_size(&analysis);
        
        Ok(analysis)
    }
}
```

#### **2.2 Source File Analysis**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceFileAnalysis {
    pub file_path: String,
    pub lines_of_code: usize,
    pub functions: Vec<String>,
    pub structs: Vec<String>,
    pub enums: Vec<String>,
    pub imports: Vec<String>,
    pub exports: Vec<String>,
}

impl BundleAnalyzer {
    fn analyze_source_files(&self, component_path: &Path) -> Result<Vec<SourceFileAnalysis>, BundleAnalysisError> {
        let mut analyses = Vec::new();
        let src_path = component_path.join("src");
        
        if src_path.exists() {
            self.analyze_directory_recursive(&src_path, &mut analyses)?;
        }
        
        Ok(analyses)
    }
    
    fn analyze_directory_recursive(&self, dir: &Path, analyses: &mut Vec<SourceFileAnalysis>) -> Result<(), BundleAnalysisError> {
        let entries = fs::read_dir(dir)?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
                let analysis = self.analyze_rust_file(&path)?;
                analyses.push(analysis);
            } else if path.is_dir() {
                self.analyze_directory_recursive(&path, analyses)?;
            }
        }
        
        Ok(())
    }
}
```

### **Phase 3: Bundle Size Extraction**

#### **3.1 Bundle Size Calculation**
```rust
impl BundleAnalyzer {
    pub fn extract_bundle_sizes(&self, components_path: &Path) -> Result<BundleSizeReport, BundleAnalysisError> {
        let mut report = BundleSizeReport::new();
        
        // Analyze each component
        let components = self.analyze_component_bundles(components_path)?;
        
        for component in components {
            let size_info = BundleSizeInfo {
                component_name: component.component_name.clone(),
                estimated_size: component.bundle_size,
                dependencies_size: self.calculate_dependencies_size(&component.dependencies)?,
                total_size: component.bundle_size + self.calculate_dependencies_size(&component.dependencies)?,
                optimization_potential: self.assess_optimization_potential(&component),
            };
            
            report.components.push(size_info);
        }
        
        // Calculate totals
        report.total_bundle_size = report.components.iter().map(|c| c.total_size).sum();
        report.average_component_size = report.total_bundle_size / report.components.len() as u64;
        report.largest_component = report.components.iter().max_by_key(|c| c.total_size).cloned();
        
        Ok(report)
    }
}
```

#### **3.2 Size Estimation Logic**
```rust
impl BundleAnalyzer {
    fn calculate_bundle_size(&self, src_path: &Path) -> Result<u64, BundleAnalysisError> {
        let mut total_size = 0;
        
        // Count lines of code as proxy for bundle size
        let source_files = self.get_rust_files(src_path)?;
        
        for file in source_files {
            let content = fs::read_to_string(&file)?;
            let lines = content.lines().count();
            
            // Rough estimation: 1 line ≈ 50 bytes in compiled WASM
            total_size += lines as u64 * 50;
        }
        
        Ok(total_size)
    }
    
    fn calculate_dependencies_size(&self, dependencies: &[String]) -> Result<u64, BundleAnalysisError> {
        // Estimate dependency sizes based on known packages
        let mut total_size = 0;
        
        for dep in dependencies {
            let estimated_size = match dep.as_str() {
                "leptos" => 50000,  // ~50KB
                "leptos_router" => 20000,  // ~20KB
                "serde" => 15000,  // ~15KB
                "web-sys" => 10000,  // ~10KB
                _ => 5000,  // Default estimate
            };
            
            total_size += estimated_size;
        }
        
        Ok(total_size)
    }
}
```

## 📋 Implementation Steps

### **Step 1: Core Bundle Analysis (Day 1)**
```rust
// 1. Implement analyze_component_bundles method
// 2. Add ComponentBundleInfo struct
// 3. Add error handling
// 4. Test with sample components
```

### **Step 2: Single Component Analysis (Day 2)**
```rust
// 1. Implement analyze_single_component_detailed method
// 2. Add source file analysis
// 3. Add dependency analysis
// 4. Test detailed analysis
```

### **Step 3: Bundle Size Extraction (Day 3)**
```rust
// 1. Implement extract_bundle_sizes method
// 2. Add size calculation logic
// 3. Add optimization assessment
// 4. Test size reporting
```

### **Step 4: Integration and Testing (Day 4)**
```rust
// 1. Integrate all methods
// 2. Add comprehensive tests
// 3. Test CLI integration
// 4. Verify performance
```

## 🧪 Testing Strategy

### **Unit Tests**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_component_bundle_analysis() {
        let analyzer = BundleAnalyzer::new();
        let result = analyzer.analyze_component_bundles(Path::new("packages/leptos"));
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_single_component_analysis() {
        let analyzer = BundleAnalyzer::new();
        let result = analyzer.analyze_single_component_detailed(Path::new("packages/leptos/button"));
        assert!(result.is_ok());
    }
    
