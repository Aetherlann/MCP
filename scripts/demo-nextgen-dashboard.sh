#!/bin/bash
# Demo: Complete dashboard with layout, widgets, and streaming

set -e

echo "=== Next-Gen Terminal: Complete Dashboard Demo ==="
echo
echo "This demo creates a full dashboard interface showing"
echo "how all the next-gen features work together."
echo
sleep 3

# Create complex layout using JSON spec
echo "Creating dashboard layout..."
cat > /tmp/dashboard-layout.json <<'EOF'
{
  "type": "split",
  "direction": "horizontal",
  "ratio": 0.25,
  "first": {
    "type": "single",
    "content": {"type": "terminal"}
  },
  "second": {
    "type": "split",
    "direction": "vertical",
    "ratio": 0.6,
    "first": {
      "type": "split",
      "direction": "horizontal",
      "ratio": 0.5,
      "first": {
        "type": "single",
        "content": {"type": "widget", "widget_id": "chart1"}
      },
      "second": {
        "type": "single",
        "content": {"type": "widget", "widget_id": "table1"}
      }
    },
    "second": {
      "type": "single",
      "content": {"type": "gallery", "gallery_id": "dashboard-gallery"}
    }
  }
}
EOF

# Apply layout (in real implementation)
# echo -ne "\e]1339;region=layout;spec=$(cat /tmp/dashboard-layout.json | jq -c)\a"

echo "✓ Layout created"
echo "  - Top 25%: Command terminal"
echo "  - Middle 45%: Chart (left) + Status table (right)"
echo "  - Bottom 30%: Image gallery"
sleep 2

# Create widgets
echo
echo "Creating dashboard widgets..."

# Chart widget
echo "  → Response time chart"
echo -ne "\e]1339;widget=chart;id=chart1;type=line;title=API Response Time\a"
echo -ne "\e]1339;widget=chart_data;chart=chart1;labels=10:00,10:05,10:10,10:15,10:20;values=95,102,98,105,99\a"

# Table widget
echo "  → System status table"
echo -ne "\e]1339;widget=table;id=table1;columns=Service,Status,Uptime\a"
echo -ne "\e]1339;widget=table_row;table=table1;data=API Server,✓ Running,99.9%\a"
echo -ne "\e]1339;widget=table_row;table=table1;data=Database,✓ Running,100%\a"
echo -ne "\e]1339;widget=table_row;table=table1;data=Cache,✓ Running,99.5%\a"
echo -ne "\e]1339;widget=table_row;table=table1;data=Queue,✓ Running,98.2%\a"

sleep 1

# Start streaming gallery
echo "  → Starting image gallery stream"
echo -ne "\e]1339;stream=start;type=gallery;id=dashboard-gallery;mode=masonry\a"

# Stream some images
for i in {1..4}; do
    echo -ne "\e]1339;stream=add;stream=dashboard-gallery;title=Screenshot $i\a"
    sleep 0.5
done

echo -ne "\e]1339;stream=end;stream=dashboard-gallery\a"
sleep 1

# Add progress indicator
echo
echo "Simulating live data updates..."
echo -ne "\e]1339;widget=progress;id=sync-prog;value=0.0;label=Syncing data\a"

for i in {1..5}; do
    value=$(echo "scale=2; $i / 5" | bc)
    echo -ne "\e]1339;update=widget;id=sync-prog;value=$value\a"
    # Update table with new data
    echo -ne "\e]1339;update=table;id=table1;row=0;col=2;value=$(echo "scale=1; 99.9 + $RANDOM % 10 / 100" | bc)%\a"
    sleep 0.8
done

echo
echo
echo "=== Dashboard Complete ===$"
echo
echo "This dashboard demonstrates:"
echo "  ✓ Complex multi-region layouts"
echo "  ✓ Multiple widget types working together"
echo "  ✓ Streaming updates to galleries"
echo "  ✓ Real-time data updates"
echo "  ✓ Professional monitoring interface"
echo
echo "Perfect for:"
echo "  • System monitoring dashboards"
echo "  • CI/CD pipeline status"
echo "  • Data analysis workbenches"
echo "  • LLM-powered dev environments"
echo
