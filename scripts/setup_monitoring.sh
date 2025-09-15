#!/bin/bash
# Performance Monitoring Setup Script
# 
# This script sets up performance monitoring infrastructure for the
# leptos-shadcn-ui project, including alerts and dashboards.

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
MONITORING_DIR="monitoring"
ALERTS_DIR="$MONITORING_DIR/alerts"
DASHBOARDS_DIR="$MONITORING_DIR/dashboards"
CONFIG_DIR="$MONITORING_DIR/config"

echo -e "${BLUE}🚀 Setting up Performance Monitoring Infrastructure${NC}"
echo "=================================================="

# Create monitoring directory structure
echo -e "${YELLOW}📁 Creating monitoring directory structure...${NC}"
mkdir -p "$ALERTS_DIR"
mkdir -p "$DASHBOARDS_DIR"
mkdir -p "$CONFIG_DIR"

# Create performance monitoring configuration
echo -e "${YELLOW}⚙️  Creating performance monitoring configuration...${NC}"
cat > "$CONFIG_DIR/performance_config.toml" << 'EOF'
[monitoring]
# Performance contract thresholds
bundle_size_warning_kb = 400
bundle_size_critical_kb = 500
render_time_warning_ms = 12
render_time_critical_ms = 16
memory_warning_mb = 50
memory_critical_mb = 100

# Monitoring intervals
check_interval_seconds = 30
alert_cooldown_minutes = 5
report_interval_hours = 24

# Alert channels
[alerts]
slack_webhook_url = ""
discord_webhook_url = ""
email_recipients = []
pagerduty_integration_key = ""

# Components to monitor
[components]
include = [
    "button", "input", "card", "dialog", "form", "table",
    "calendar", "date-picker", "pagination", "tooltip", "popover",
    "accordion", "alert", "badge", "breadcrumb", "checkbox",
    "collapsible", "combobox", "command", "context-menu",
    "dropdown-menu", "hover-card", "label", "menubar",
    "navigation-menu", "progress", "radio-group", "scroll-area",
    "select", "separator", "sheet", "skeleton", "slider",
    "switch", "tabs", "textarea", "toast", "toggle"
]

# Performance baselines
[baselines]
button_bundle_size_kb = 45
input_bundle_size_kb = 38
card_bundle_size_kb = 52
dialog_bundle_size_kb = 78
form_bundle_size_kb = 95
table_bundle_size_kb = 120
EOF

# Create alert templates
echo -e "${YELLOW}📧 Creating alert templates...${NC}"

# Slack alert template
cat > "$ALERTS_DIR/slack_template.json" << 'EOF'
{
  "text": "🚨 Performance Contract Violation",
  "blocks": [
    {
      "type": "header",
      "text": {
        "type": "plain_text",
        "text": "Performance Contract Violation"
      }
    },
    {
      "type": "section",
      "fields": [
        {
          "type": "mrkdwn",
          "text": "*Component:* {{component}}"
        },
        {
          "type": "mrkdwn",
          "text": "*Violation:* {{violation_type}}"
        },
        {
          "type": "mrkdwn",
          "text": "*Current Value:* {{current_value}}"
        },
        {
          "type": "mrkdwn",
          "text": "*Threshold:* {{threshold}}"
        },
        {
          "type": "mrkdwn",
          "text": "*Severity:* {{severity}}"
        },
        {
          "type": "mrkdwn",
          "text": "*Timestamp:* {{timestamp}}"
        }
      ]
    },
    {
      "type": "actions",
      "elements": [
        {
          "type": "button",
          "text": {
            "type": "plain_text",
            "text": "View Details"
          },
          "url": "{{details_url}}"
        }
      ]
    }
  ]
}
EOF

# Email alert template
cat > "$ALERTS_DIR/email_template.html" << 'EOF'
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Performance Contract Violation</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        .header { background-color: #ff4444; color: white; padding: 20px; border-radius: 5px; }
        .content { margin: 20px 0; }
        .metric { background-color: #f5f5f5; padding: 10px; margin: 10px 0; border-radius: 3px; }
        .critical { border-left: 5px solid #ff4444; }
        .high { border-left: 5px solid #ff8800; }
        .medium { border-left: 5px solid #ffaa00; }
        .low { border-left: 5px solid #ffdd00; }
    </style>
</head>
<body>
    <div class="header">
        <h1>🚨 Performance Contract Violation</h1>
    </div>
    
    <div class="content">
        <h2>Violation Details</h2>
        <div class="metric {{severity_class}}">
            <strong>Component:</strong> {{component}}<br>
            <strong>Violation Type:</strong> {{violation_type}}<br>
            <strong>Current Value:</strong> {{current_value}}<br>
            <strong>Threshold:</strong> {{threshold}}<br>
            <strong>Severity:</strong> {{severity}}<br>
            <strong>Timestamp:</strong> {{timestamp}}
        </div>
        
        <h2>Recommended Actions</h2>
        <ul>
            <li>Review component implementation for optimization opportunities</li>
            <li>Check for unnecessary dependencies or imports</li>
            <li>Consider code splitting or lazy loading</li>
            <li>Update performance baselines if appropriate</li>
        </ul>
        
        <p><a href="{{details_url}}">View detailed performance report</a></p>
    </div>
</body>
</html>
EOF

# Create Grafana dashboard configuration
echo -e "${YELLOW}📊 Creating Grafana dashboard configuration...${NC}"
cat > "$DASHBOARDS_DIR/performance_dashboard.json" << 'EOF'
{
  "dashboard": {
    "id": null,
    "title": "Leptos ShadCN UI Performance Monitoring",
    "tags": ["leptos", "shadcn", "performance"],
    "timezone": "browser",
    "panels": [
      {
        "id": 1,
        "title": "Bundle Size Trends",
        "type": "graph",
        "targets": [
          {
            "expr": "leptos_component_bundle_size_kb",
            "legendFormat": "{{component}}"
          }
        ],
        "yAxes": [
          {
            "label": "Bundle Size (KB)",
            "min": 0,
            "max": 600
          }
        ],
        "thresholds": [
          {
            "value": 400,
            "colorMode": "critical",
            "op": "gt"
          },
          {
            "value": 500,
            "colorMode": "critical",
            "op": "gt"
          }
        ]
      },
      {
        "id": 2,
        "title": "Render Time Trends",
        "type": "graph",
        "targets": [
          {
            "expr": "leptos_component_render_time_ms",
            "legendFormat": "{{component}}"
          }
        ],
        "yAxes": [
          {
            "label": "Render Time (ms)",
            "min": 0,
            "max": 20
          }
        ],
        "thresholds": [
          {
            "value": 12,
            "colorMode": "critical",
            "op": "gt"
          },
          {
            "value": 16,
            "colorMode": "critical",
            "op": "gt"
          }
        ]
      },
      {
        "id": 3,
        "title": "Performance Contract Violations",
        "type": "stat",
        "targets": [
          {
            "expr": "sum(leptos_performance_violations_total)",
            "legendFormat": "Total Violations"
          }
        ],
        "colorMode": "value",
        "thresholds": [
          {
            "value": 0,
            "color": "green"
          },
          {
            "value": 1,
            "color": "yellow"
          },
          {
            "value": 5,
            "color": "red"
          }
        ]
      }
    ],
    "time": {
      "from": "now-1h",
      "to": "now"
    },
    "refresh": "30s"
  }
}
EOF

# Create monitoring service script
echo -e "${YELLOW}🔧 Creating monitoring service script...${NC}"
cat > "$MONITORING_DIR/start_monitoring.sh" << 'EOF'
#!/bin/bash
# Start Performance Monitoring Service

set -euo pipefail

# Load configuration
source monitoring/config/performance_config.toml

echo "🚀 Starting Performance Monitoring Service"
echo "=========================================="

# Check if monitoring is already running
if pgrep -f "performance_monitor" > /dev/null; then
    echo "⚠️  Performance monitoring is already running"
    echo "   PID: $(pgrep -f performance_monitor)"
    exit 1
fi

# Start the monitoring service
echo "📊 Starting performance monitor..."
cargo run --package leptos-shadcn-contract-testing --bin performance_monitor monitor 30 &

MONITOR_PID=$!
echo "✅ Performance monitoring started with PID: $MONITOR_PID"

# Save PID for later use
echo $MONITOR_PID > monitoring/monitor.pid

echo "📈 Monitoring service is now running"
echo "   - Check interval: 30 seconds"
echo "   - Logs: monitoring/monitor.log"
echo "   - PID file: monitoring/monitor.pid"
echo ""
echo "To stop monitoring: ./monitoring/stop_monitoring.sh"
echo "To view logs: tail -f monitoring/monitor.log"
EOF

chmod +x "$MONITORING_DIR/start_monitoring.sh"

# Create stop monitoring script
cat > "$MONITORING_DIR/stop_monitoring.sh" << 'EOF'
#!/bin/bash
# Stop Performance Monitoring Service

set -euo pipefail

echo "🛑 Stopping Performance Monitoring Service"
echo "=========================================="

if [ -f monitoring/monitor.pid ]; then
    MONITOR_PID=$(cat monitoring/monitor.pid)
    
    if kill -0 $MONITOR_PID 2>/dev/null; then
        echo "📊 Stopping performance monitor (PID: $MONITOR_PID)..."
        kill $MONITOR_PID
        
        # Wait for graceful shutdown
        sleep 2
        
        if kill -0 $MONITOR_PID 2>/dev/null; then
            echo "⚠️  Force killing monitor process..."
            kill -9 $MONITOR_PID
        fi
        
        echo "✅ Performance monitoring stopped"
    else
        echo "⚠️  Monitor process not running"
    fi
    
    rm -f monitoring/monitor.pid
else
    echo "⚠️  No PID file found. Trying to kill by process name..."
    pkill -f performance_monitor || echo "No monitoring processes found"
fi

echo "🏁 Monitoring service stopped"
EOF

chmod +x "$MONITORING_DIR/stop_monitoring.sh"

# Create health check script
cat > "$MONITORING_DIR/health_check.sh" << 'EOF'
#!/bin/bash
# Performance Monitoring Health Check

set -euo pipefail

echo "🏥 Performance Monitoring Health Check"
echo "====================================="

# Check if monitoring is running
if [ -f monitoring/monitor.pid ]; then
    MONITOR_PID=$(cat monitoring/monitor.pid)
    if kill -0 $MONITOR_PID 2>/dev/null; then
        echo "✅ Monitoring service is running (PID: $MONITOR_PID)"
    else
        echo "❌ Monitoring service is not running (stale PID file)"
        rm -f monitoring/monitor.pid
    fi
else
    echo "❌ No monitoring PID file found"
fi

# Check recent performance violations
echo ""
echo "📊 Recent Performance Status:"
cargo run --package leptos-shadcn-contract-testing --bin performance_monitor check

# Check configuration
echo ""
echo "⚙️  Configuration Status:"
if [ -f monitoring/config/performance_config.toml ]; then
    echo "✅ Configuration file exists"
else
    echo "❌ Configuration file missing"
fi

# Check alert templates
echo ""
echo "📧 Alert Templates Status:"
if [ -f monitoring/alerts/slack_template.json ]; then
    echo "✅ Slack template exists"
else
    echo "❌ Slack template missing"
fi

if [ -f monitoring/alerts/email_template.html ]; then
    echo "✅ Email template exists"
else
    echo "❌ Email template missing"
fi

echo ""
echo "🏁 Health check complete"
EOF

chmod +x "$MONITORING_DIR/health_check.sh"

# Create README for monitoring
cat > "$MONITORING_DIR/README.md" << 'EOF'
# Performance Monitoring Infrastructure

This directory contains the performance monitoring infrastructure for the leptos-shadcn-ui project.

## Quick Start

```bash
# Start monitoring
./monitoring/start_monitoring.sh

# Check health
./monitoring/health_check.sh

# Stop monitoring
./monitoring/stop_monitoring.sh
```

## Configuration

Edit `config/performance_config.toml` to customize:
- Performance thresholds
- Monitoring intervals
- Alert channels
- Components to monitor

## Alert Channels

### Slack Integration
1. Create a Slack webhook URL
2. Add it to `config/performance_config.toml`
3. Restart monitoring service

### Email Alerts
1. Configure SMTP settings
2. Add recipient emails to config
3. Restart monitoring service

### Grafana Dashboard
1. Import `dashboards/performance_dashboard.json`
2. Configure Prometheus data source
3. Set up alerting rules

## Manual Commands

```bash
# Check performance contracts once
cargo run --package leptos-shadcn-contract-testing --bin performance_monitor check

# Generate performance report
cargo run --package leptos-shadcn-contract-testing --bin performance_monitor report

# Start continuous monitoring
cargo run --package leptos-shadcn-contract-testing --bin performance_monitor monitor 30
```

## Troubleshooting

- Check logs: `tail -f monitoring/monitor.log`
- Verify configuration: `./monitoring/health_check.sh`
- Restart service: `./monitoring/stop_monitoring.sh && ./monitoring/start_monitoring.sh`
EOF

# Create .gitignore for monitoring
cat > "$MONITORING_DIR/.gitignore" << 'EOF'
# Monitoring runtime files
monitor.pid
monitor.log
*.log

# Sensitive configuration
config/secrets.toml
config/webhooks.toml

# Temporary files
*.tmp
*.temp
EOF

echo -e "${GREEN}✅ Performance monitoring infrastructure setup complete!${NC}"
echo ""
echo -e "${BLUE}📋 Next Steps:${NC}"
echo "1. Configure alert channels in $CONFIG_DIR/performance_config.toml"
echo "2. Start monitoring: ./$MONITORING_DIR/start_monitoring.sh"
echo "3. Check health: ./$MONITORING_DIR/health_check.sh"
echo "4. View dashboard: Import $DASHBOARDS_DIR/performance_dashboard.json into Grafana"
echo ""
echo -e "${YELLOW}📚 Documentation: $MONITORING_DIR/README.md${NC}"
