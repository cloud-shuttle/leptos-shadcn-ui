const { chromium } = require('@playwright/test');

async function testMinimalWasm() {
    const browser = await chromium.launch({ headless: false });
    const page = await browser.newPage();
    
    try {
        console.log('Testing minimal WASM at http://localhost:8082/test.html');
        await page.goto('http://localhost:8082/test.html', { waitUntil: 'networkidle' });
        
        // Wait a bit for WASM to load
        await page.waitForTimeout(3000);
        
        // Check if the loading screen is gone
        const loadingElement = await page.$('#loading');
        if (loadingElement) {
            console.log('❌ Loading screen still present - WASM failed to initialize');
            const loadingText = await page.evaluate(el => el.textContent, loadingElement);
            console.log('Loading text:', loadingText);
        } else {
            console.log('✅ Loading screen removed - WASM initialized successfully');
        }
        
        // Check the page content
        const bodyText = await page.evaluate(() => document.body.textContent);
        console.log('Page content:', bodyText);
        
        // Check for any error messages
        const errorElements = await page.$$('h1');
        for (const element of errorElements) {
            const text = await page.evaluate(el => el.textContent, element);
            if (text.includes('Error')) {
                console.log('❌ Error found:', text);
            }
        }
        
    } catch (error) {
        console.error('Error testing minimal WASM:', error);
    } finally {
        await browser.close();
    }
}

testMinimalWasm();