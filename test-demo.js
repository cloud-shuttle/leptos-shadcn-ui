#!/usr/bin/env node

/**
 * Simple Demo Page Test
 * Tests the demo page functionality without requiring full Playwright setup
 */

const http = require('http');
const { JSDOM } = require('jsdom');

const DEMO_URL = 'http://localhost:8081';

async function testDemoPage() {
  console.log('🚀 Testing leptos-shadcn-ui Demo Page');
  console.log('📊 Performance Champion showcase validation');
  console.log('');

  try {
    // Test 1: Page loads successfully
    console.log('✅ Test 1: Page loads successfully');
    const html = await fetchPage(DEMO_URL);
    const dom = new JSDOM(html);
    const document = dom.window.document;

    // Test 2: Title is correct
    console.log('✅ Test 2: Title is correct');
    const title = document.querySelector('title');
    if (!title || !title.textContent.includes('Performance Champion')) {
      throw new Error('Title does not contain "Performance Champion"');
    }

    // Test 3: Hero section exists
    console.log('✅ Test 3: Hero section exists');
    const hero = document.querySelector('section.gradient-bg');
    if (!hero) {
      throw new Error('Hero section not found');
    }

    // Test 4: Performance metrics are displayed
    console.log('✅ Test 4: Performance metrics are displayed');
    const performanceSection = document.querySelector('#performance');
    if (!performanceSection) {
      throw new Error('Performance section not found');
    }

    const performanceGrid = performanceSection.querySelector('.performance-grid');
    if (!performanceGrid) {
      throw new Error('Performance grid not found');
    }

    // Test 5: Component showcase exists
    console.log('✅ Test 5: Component showcase exists');
    const componentsSection = document.querySelector('#components');
    if (!componentsSection) {
      throw new Error('Components section not found');
    }

    const componentGrid = componentsSection.querySelector('.component-grid');
    if (!componentGrid) {
      throw new Error('Component grid not found');
    }

    // Test 6: Comparison table exists
    console.log('✅ Test 6: Comparison table exists');
    const comparisonSection = document.querySelector('#comparison');
    if (!comparisonSection) {
      throw new Error('Comparison section not found');
    }

    const comparisonTable = comparisonSection.querySelector('.comparison-table table');
    if (!comparisonTable) {
      throw new Error('Comparison table not found');
    }

    // Test 7: Interactive demo section exists
    console.log('✅ Test 7: Interactive demo section exists');
    const demoSection = document.querySelector('#demo');
    if (!demoSection) {
      throw new Error('Demo section not found');
    }

    // Test 8: Navigation links exist
    console.log('✅ Test 8: Navigation links exist');
    const navLinks = document.querySelectorAll('nav a');
    if (navLinks.length < 4) {
      throw new Error('Not enough navigation links found');
    }

    // Test 9: Performance messaging is present
    console.log('✅ Test 9: Performance messaging is present');
    const heroText = document.querySelector('section.gradient-bg p');
    if (!heroText || !heroText.textContent.includes('3-5x Faster')) {
      throw new Error('Performance messaging not found in hero section');
    }

    // Test 10: Call-to-action buttons exist
    console.log('✅ Test 10: Call-to-action buttons exist');
    const ctaButtons = document.querySelectorAll('button, a[href*="github"]');
    if (ctaButtons.length < 2) {
      throw new Error('Not enough call-to-action buttons found');
    }

    console.log('');
    console.log('🎉 All tests passed! Demo page is working correctly.');
    console.log('');
    console.log('📊 Demo Page Features Verified:');
    console.log('   ✅ Page loads successfully');
    console.log('   ✅ Performance Champion messaging');
    console.log('   ✅ Performance metrics display');
    console.log('   ✅ Component showcase');
    console.log('   ✅ Comparison table');
    console.log('   ✅ Interactive demo section');
    console.log('   ✅ Navigation structure');
    console.log('   ✅ Call-to-action elements');
    console.log('');
    console.log('🌐 Demo page is ready at: http://localhost:8081');
    console.log('🚀 Performance Champion showcase is live!');

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

