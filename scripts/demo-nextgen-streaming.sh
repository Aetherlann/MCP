#!/bin/bash
# Demo: Streaming updates (gallery and table)

set -e

echo "=== Next-Gen Terminal: Streaming Updates Demo ==="
echo
echo "This demo shows how LLMs can stream results in real-time"
echo "rather than waiting for all results before displaying."
echo
sleep 2

# Start streaming gallery
echo "Starting streaming gallery..."
echo -ne "\e]1339;stream=start;type=gallery;id=stream-gallery;mode=grid\a"
sleep 1

echo
echo "Streaming images as they're generated..."

# Simulate streaming images
for i in {1..6}; do
    echo "  → Adding image $i..."
    echo -ne "\e]1339;stream=add;stream=stream-gallery;title=Generated Image $i;desc=AI-generated artwork\a"
    # In real use, this would be followed by actual image data via Kitty protocol
    sleep 0.8
done

# Finalize gallery
echo
echo "Finalizing gallery..."
echo -ne "\e]1339;stream=end;stream=stream-gallery\a"
sleep 1

echo
echo "=== Streaming Gallery Complete ==="
sleep 1

# Now demo streaming table
echo
echo
echo "Starting streaming log table..."
echo -ne "\e]1339;stream=start;type=table;id=stream-logs;columns=Time,Level,Message\a"
sleep 1

# Simulate streaming log entries
echo
echo "Streaming log entries in real-time..."
for i in {1..10}; do
    time=$(date +"%H:%M:%S")
    levels=("INFO" "DEBUG" "WARN" "ERROR")
    level=${levels[$((RANDOM % 4))]}
    messages=("Request received" "Processing data" "Cache miss" "Query executed" "Response sent")
    msg=${messages[$((RANDOM % 5))]}

    echo "  → [$time] $level: $msg"
    echo -ne "\e]1339;stream=add;stream=stream-logs;data=$time,$level,$msg\a"
    sleep 0.5
done

echo
echo "Finalizing log table..."
echo -ne "\e]1339;stream=end;stream=stream-logs\a"
sleep 1

echo
echo "=== Streaming Demo Complete ===$"
echo
echo "Streaming enables:"
echo "  ✓ Real-time updates as results are generated"
echo "  ✓ No waiting for all results before seeing anything"
echo "  ✓ Better UX for long-running operations"
echo "  ✓ Live monitoring of logs and events"
echo
echo "Perfect for LLMs that generate multiple results!"
echo
