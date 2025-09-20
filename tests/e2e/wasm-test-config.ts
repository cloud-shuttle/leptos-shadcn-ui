/**
 * WASM Testing Configuration
 * 
 * Centralized configuration for WASM browser testing across different environments
 */

export interface WASMTestConfig {
  // Performance thresholds
  performance: {
    maxInitializationTime: number; // milliseconds
    maxFirstPaint: number; // milliseconds
    maxFirstContentfulPaint: number; // milliseconds
    maxInteractionLatency: number; // milliseconds
    maxMemoryIncrease: number; // percentage
  };
  
  // Browser-specific settings
  browsers: {
    [browserName: string]: {
      enabled: boolean;
      timeout: number;
      retries: number;
      specificThresholds?: Partial<WASMTestConfig['performance']>;
    };
  };
  
  // Test scenarios
  scenarios: {
    [scenarioName: string]: {
      enabled: boolean;
      description: string;
      testFunction: string;
    };
  };
  
  // Reporting
  reporting: {
    generateHtmlReport: boolean;
    generateJsonReport: boolean;
    generateMarkdownReport: boolean;
    outputDirectory: string;
  };
}

export const defaultWASMTestConfig: WASMTestConfig = {
  performance: {
    maxInitializationTime: 5000, // 5 seconds
    maxFirstPaint: 3000, // 3 seconds
    maxFirstContentfulPaint: 4000, // 4 seconds
    maxInteractionLatency: 100, // 100ms
    maxMemoryIncrease: 50, // 50% increase allowed
  },
  
  browsers: {
    chromium: {
      enabled: true,
      timeout: 30000,
      retries: 2,
    },
    firefox: {
      enabled: true,
      timeout: 35000, // Firefox can be slower
      retries: 2,
      specificThresholds: {
        maxInitializationTime: 6000, // Allow slightly more time for Firefox
      },
    },
    webkit: {
      enabled: true,
      timeout: 40000, // Safari can be slower
      retries: 3,
      specificThresholds: {
        maxInitializationTime: 7000, // Safari needs more time
        maxFirstPaint: 3500,
      },
    },
    'Mobile Chrome': {
      enabled: true,
      timeout: 45000, // Mobile can be slower
      retries: 2,
      specificThresholds: {
        maxInitializationTime: 8000, // Mobile needs more time
        maxFirstPaint: 4000,
        maxFirstContentfulPaint: 5000,
      },
    },
    'Mobile Safari': {
      enabled: true,
      timeout: 50000, // Mobile Safari can be slowest
      retries: 3,
      specificThresholds: {
        maxInitializationTime: 10000, // Mobile Safari needs most time
        maxFirstPaint: 5000,
        maxFirstContentfulPaint: 6000,
        maxInteractionLatency: 150, // Mobile interactions can be slower
      },
    },
  },
  
  scenarios: {
    'basic-initialization': {
      enabled: true,
      description: 'Basic WASM initialization and loading',
      testFunction: 'testBasicInitialization',
    },
    'memory-management': {
      enabled: true,
      description: 'Memory usage and leak detection',
      testFunction: 'testMemoryManagement',
    },
    'cross-browser-compatibility': {
      enabled: true,
      description: 'Cross-browser WASM compatibility',
      testFunction: 'testCrossBrowserCompatibility',
    },
    'performance-monitoring': {
      enabled: true,
      description: 'Performance benchmarks and monitoring',
      testFunction: 'testPerformanceMonitoring',
    },
    'error-handling': {
      enabled: true,
      description: 'Error handling and recovery',
      testFunction: 'testErrorHandling',
    },
    'bundle-analysis': {
      enabled: true,
      description: 'WASM bundle size and loading analysis',
      testFunction: 'testBundleAnalysis',
    },
  },
  
  reporting: {
    generateHtmlReport: true,
    generateJsonReport: true,
    generateMarkdownReport: true,
    outputDirectory: 'test-results/wasm-tests',
  },
};

/**
 * Get browser-specific configuration
 */
export function getBrowserConfig(browserName: string): WASMTestConfig['browsers'][string] {
  const config = defaultWASMTestConfig.browsers[browserName];
  if (!config) {
    throw new Error(`No configuration found for browser: ${browserName}`);
  }
  return config;
}

/**
 * Get performance thresholds for a specific browser
 */
export function getPerformanceThresholds(browserName: string): WASMTestConfig['performance'] {
  const baseThresholds = defaultWASMTestConfig.performance;
  const browserConfig = getBrowserConfig(browserName);
  
  if (browserConfig.specificThresholds) {
    return {
      ...baseThresholds,
      ...browserConfig.specificThresholds,
    };
  }
  
  return baseThresholds;
}

/**
 * Check if a scenario is enabled
 */
export function isScenarioEnabled(scenarioName: string): boolean {
  const scenario = defaultWASMTestConfig.scenarios[scenarioName];
  return scenario ? scenario.enabled : false;
}

/**
 * Get all enabled scenarios
 */
export function getEnabledScenarios(): string[] {
  return Object.keys(defaultWASMTestConfig.scenarios).filter(isScenarioEnabled);
}

/**
 * Get all enabled browsers
 */
export function getEnabledBrowsers(): string[] {
  return Object.keys(defaultWASMTestConfig.browsers).filter(
    browserName => defaultWASMTestConfig.browsers[browserName].enabled
  );
}

/**
 * Validate configuration
 */
export function validateConfig(config: WASMTestConfig): { valid: boolean; errors: string[] } {
  const errors: string[] = [];
  
  // Validate performance thresholds
  if (config.performance.maxInitializationTime <= 0) {
    errors.push('maxInitializationTime must be positive');
  }
  if (config.performance.maxFirstPaint <= 0) {
    errors.push('maxFirstPaint must be positive');
  }
  if (config.performance.maxFirstContentfulPaint <= 0) {
    errors.push('maxFirstContentfulPaint must be positive');
  }
  if (config.performance.maxInteractionLatency <= 0) {
    errors.push('maxInteractionLatency must be positive');
  }
  if (config.performance.maxMemoryIncrease < 0) {
    errors.push('maxMemoryIncrease must be non-negative');
  }
  
  // Validate browser configurations
  Object.entries(config.browsers).forEach(([browserName, browserConfig]) => {
    if (browserConfig.timeout <= 0) {
      errors.push(`Browser ${browserName}: timeout must be positive`);
    }
    if (browserConfig.retries < 0) {
      errors.push(`Browser ${browserName}: retries must be non-negative`);
    }
  });
  
  // Validate scenarios
  Object.entries(config.scenarios).forEach(([scenarioName, scenario]) => {
    if (!scenario.description || scenario.description.trim().length === 0) {
      errors.push(`Scenario ${scenarioName}: description is required`);
    }
    if (!scenario.testFunction || scenario.testFunction.trim().length === 0) {
      errors.push(`Scenario ${scenarioName}: testFunction is required`);
    }
  });
  
  return {
    valid: errors.length === 0,
    errors,
  };
}

/**
 * Load configuration from environment variables
 */
export function loadConfigFromEnv(): WASMTestConfig {
  const config = { ...defaultWASMTestConfig };
  
  // Override performance thresholds from environment
  if (process.env.WASM_MAX_INIT_TIME) {
    config.performance.maxInitializationTime = parseInt(process.env.WASM_MAX_INIT_TIME);
  }
  if (process.env.WASM_MAX_FIRST_PAINT) {
    config.performance.maxFirstPaint = parseInt(process.env.WASM_MAX_FIRST_PAINT);
  }
  if (process.env.WASM_MAX_FCP) {
    config.performance.maxFirstContentfulPaint = parseInt(process.env.WASM_MAX_FCP);
  }
  if (process.env.WASM_MAX_INTERACTION_LATENCY) {
    config.performance.maxInteractionLatency = parseInt(process.env.WASM_MAX_INTERACTION_LATENCY);
  }
  if (process.env.WASM_MAX_MEMORY_INCREASE) {
    config.performance.maxMemoryIncrease = parseInt(process.env.WASM_MAX_MEMORY_INCREASE);
  }
  
  // Override browser settings from environment
  if (process.env.WASM_ENABLED_BROWSERS) {
    const enabledBrowsers = process.env.WASM_ENABLED_BROWSERS.split(',');
    Object.keys(config.browsers).forEach(browserName => {
      config.browsers[browserName].enabled = enabledBrowsers.includes(browserName);
    });
  }
  
  // Override scenario settings from environment
  if (process.env.WASM_ENABLED_SCENARIOS) {
    const enabledScenarios = process.env.WASM_ENABLED_SCENARIOS.split(',');
    Object.keys(config.scenarios).forEach(scenarioName => {
      config.scenarios[scenarioName].enabled = enabledScenarios.includes(scenarioName);
    });
  }
  
  // Override reporting settings from environment
  if (process.env.WASM_OUTPUT_DIR) {
    config.reporting.outputDirectory = process.env.WASM_OUTPUT_DIR;
  }
  if (process.env.WASM_GENERATE_HTML_REPORT) {
    config.reporting.generateHtmlReport = process.env.WASM_GENERATE_HTML_REPORT === 'true';
  }
  if (process.env.WASM_GENERATE_JSON_REPORT) {
    config.reporting.generateJsonReport = process.env.WASM_GENERATE_JSON_REPORT === 'true';
  }
  if (process.env.WASM_GENERATE_MARKDOWN_REPORT) {
    config.reporting.generateMarkdownReport = process.env.WASM_GENERATE_MARKDOWN_REPORT === 'true';
  }
  
  return config;
}
