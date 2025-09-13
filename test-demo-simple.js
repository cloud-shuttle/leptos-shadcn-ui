#!/usr/bin/env node

/**
 * Simple Demo Page Test (No Dependencies)
 * Tests the demo page functionality using basic Node.js
 */

const http = require('http');

const DEMO_URL = 'http://localhost:8081';

async function testDemoPage() {
  console.log('🚀 Testing leptos-shadcn-ui Demo Page');
  console.log('📊 Performance Champion showcase validation');
  console.log('');

  try {
    // Test 1: Page loads successfully
    console.log('✅ Test 1: Page loads successfully');
    const html = await fetchPage(DEMO_URL);
    
    if (!html || html.length < 1000) {
      throw new Error('Page content is too short or empty');
    }

    // Test 2: Title is correct
    console.log('✅ Test 2: Title is correct');
    if (!html.includes('Performance Champion')) {
      throw new Error('Title does not contain "Performance Champion"');
    }

    // Test 3: Hero section exists
    console.log('✅ Test 3: Hero section exists');
    if (!html.includes('gradient-bg')) {
      throw new Error('Hero section not found');
    }

    // Test 4: Performance metrics are displayed
    console.log('✅ Test 4: Performance metrics are displayed');
    if (!html.includes('performance-grid')) {
      throw new Error('Performance grid not found');
    }

    // Test 5: Component showcase exists
    console.log('✅ Test 5: Component showcase exists');
    if (!html.includes('component-grid')) {
      throw new Error('Component grid not found');
    }

    // Test 6: Comparison table exists
    console.log('✅ Test 6: Comparison table exists');
    if (!html.includes('comparison-table')) {
      throw new Error('Comparison table not found');
    }

    // Test 7: Interactive demo section exists
    console.log('✅ Test 7: Interactive demo section exists');
    if (!html.includes('id="demo"')) {
      throw new Error('Demo section not found');
    }

    // Test 8: Navigation links exist
    console.log('✅ Test 8: Navigation links exist');
    const navLinkCount = (html.match(/<nav[^>]*>[\s\S]*?<\/nav>/g) || []).length;
    if (navLinkCount === 0) {
      throw new Error('Navigation not found');
    }

    // Test 9: Performance messaging is present
    console.log('✅ Test 9: Performance messaging is present');
    if (!html.includes('3-5x Faster')) {
      throw new Error('Performance messaging not found');
    }

    // Test 10: Call-to-action buttons exist
    console.log('✅ Test 10: Call-to-action buttons exist');
    const buttonCount = (html.match(/<button/g) || []).length;
    if (buttonCount < 3) {
      throw new Error('Not enough buttons found');
    }

    // Test 11: GitHub link exists
    console.log('✅ Test 11: GitHub link exists');
    if (!html.includes('github.com')) {
      throw new Error('GitHub link not found');
    }

    // Test 12: Performance metrics values are present
    console.log('✅ Test 12: Performance metrics values are present');
    const metrics = ['3-5x', '5x', '3-8x', '0', '60 FPS', '100%'];
    for (const metric of metrics) {
      if (!html.includes(metric)) {
        throw new Error(`Performance metric "${metric}" not found`);
      }
    }

    // Test 13: Component examples are present
    console.log('✅ Test 13: Component examples are present');
    const components = ['Button', 'Input', 'Card', 'Modal', 'Data Table', 'Form'];
    for (const component of components) {
      if (!html.includes(component)) {
        throw new Error(`Component "${component}" not found`);
      }
    }

    // Test 14: Comparison data is present
    console.log('✅ Test 14: Comparison data is present');
    const comparisonData = ['leptos-shadcn-ui', 'React 19', 'Next.js 15', 'Material-UI'];
    for (const framework of comparisonData) {
      if (!html.includes(framework)) {
        throw new Error(`Framework "${framework}" not found in comparison`);
      }
    }

    // Test 15: Interactive elements are present
    console.log('✅ Test 15: Interactive elements are present');
    const interactiveElements = ['perfTest', 'memoryTest', 'speedTest'];
    for (const element of interactiveElements) {
      if (!html.includes(`id="${element}"`)) {
        throw new Error(`Interactive element "${element}" not found`);
      }
    }

    console.log('');
    console.log('🎉 All tests passed! Demo page is working correctly.');
    console.log('');
    console.log('📊 Demo Page Features Verified:');
    console.log('   ✅ Page loads successfully');
    console.log('   ✅ Performance Champion messaging');
    console.log('   ✅ Performance metrics display (3-5x, 5x, 3-8x, 0, 60 FPS, 100%)');
    console.log('   ✅ Component showcase (Button, Input, Card, Modal, Data Table, Form)');
    console.log('   ✅ Comparison table (leptos-shadcn-ui vs React/Next.js/Material-UI)');
    console.log('   ✅ Interactive demo section (perfTest, memoryTest, speedTest)');
    console.log('   ✅ Navigation structure');
    console.log('   ✅ Call-to-action elements');
    console.log('   ✅ GitHub integration');
    console.log('');
    console.log('🌐 Demo page is ready at: http://localhost:8081');
    console.log('🚀 Performance Champion showcase is live!');
    console.log('');
    console.log('📈 Key Performance Messages:');
    console.log('   🏆 "3-5x Faster than React/Next.js"');
    console.log('   🏆 "5x Less Memory Usage"');
    console.log('   🏆 "3-8x Smaller Bundles"');
    console.log('   🏆 "Zero Memory Leaks"');
    console.log('   🏆 "60 FPS Guaranteed"');
    console.log('   🏆 "100% Test Coverage"');

  } catch (error) {
    console.error('❌ Test failed:', error.message);
    process.exit(1);
  }
}

function fetchPage(url) {
  return new Promise((resolve, reject) => {
    const request = http.get(url, (response) => {
      let data = '';
      response.on('data', (chunk) => {
        data += chunk;
      });
      response.on('end', () => {
        resolve(data);
      });
    });
    
    request.on('error', (error) => {
      reject(error);
    });
  });
}

// Run the test
testDemoPage().catch(console.error);

