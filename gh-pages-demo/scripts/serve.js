#!/usr/bin/env node

const express = require('express');
const cors = require('cors');
const path = require('path');
const portfinder = require('portfinder');

const app = express();

// Enable CORS for all routes
app.use(cors());

// Serve static files
app.use(express.static(path.join(__dirname, '..')));

// Health check endpoint
app.get('/health', (req, res) => {
  res.json({ 
    status: 'healthy', 
    timestamp: new Date().toISOString(),
    demo: 'leptos-shadcn-comprehensive-demo',
    version: '0.9.1'
  });
});

// API endpoint for demo information
app.get('/api/demo-info', (req, res) => {
  res.json({
    name: 'Leptos ShadCN UI Comprehensive Demo',
    version: '0.9.1',
    description: 'Showcasing all refactored Leptos ShadCN UI components',
    components: [
      {
        name: 'Drawer',
        status: 'refactored',
        size: '15k → 12k bytes',
        modules: 9
      },
      {
        name: 'Context Menu',
        status: 'refactored', 
        size: '13k → 14.8k bytes',
        modules: 8
      },
      {
        name: 'Alert Dialog',
        status: 'refactored',
        size: '12k → 9.5k bytes', 
        modules: 7
      },
      {
        name: 'Select',
        status: 'refactored',
        size: 'modularized',
        modules: 'improved'
      },
      {
        name: 'Command',
        status: 'refactored',
        size: 'compilation fixed',
        modules: 'improved'
      }
    ],
    achievements: {
      totalRefactored: 5,
      totalReviewed: 40,
      regressions: 0,
      published: true
    }
  });
});

// Start server on available port
const startServer = async () => {
  try {
    // Find an available port starting from 3000
    const port = await portfinder.getPortPromise({
      port: 3000,
      stopPort: 3100
    });

    app.listen(port, () => {
      console.log('🚀 Leptos ShadCN UI Comprehensive Demo Server');
      console.log('==========================================');
      console.log(`🌐 Server running at: http://localhost:${port}`);
      console.log(`📱 Demo available at: http://localhost:${port}`);
      console.log(`🔍 Health check: http://localhost:${port}/health`);
      console.log(`📊 API info: http://localhost:${port}/api/demo-info`);
      console.log('');
      console.log('🎯 Features:');
      console.log('  ✅ All refactored components showcased');
      console.log('  ✅ Interactive demos with reactive state');
      console.log('  ✅ Dark/light mode theme switching');
      console.log('  ✅ Real WASM components');
      console.log('  ✅ Production-ready packages from crates.io v0.9.1');
      console.log('');
      console.log('🛑 Press Ctrl+C to stop the server');
      
      // Export port for other processes
      process.env.DEMO_PORT = port;
    });

  } catch (error) {
    console.error('❌ Failed to start server:', error);
    process.exit(1);
  }
};

startServer();