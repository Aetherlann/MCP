#!/bin/bash
# Performance testing and benchmarking
# Run this inside the Hyper Terminal after: cargo run --release

echo "============================================"
echo "  Hyper Terminal - Performance Demo"
echo "============================================"
echo ""

echo "This demo tests rendering performance with various workloads."
echo ""

echo "1. Testing rapid text output..."
echo "   Generating 1000 lines..."
start_time=$(date +%s%N)
for i in {1..1000}; do
    echo "Line $i - Testing rapid text rendering performance with GPU acceleration"
done
end_time=$(date +%s%N)
elapsed=$(( ($end_time - $start_time) / 1000000 ))
echo ""
echo "   Completed in ${elapsed}ms"
echo "   Target: <2000ms for smooth 60 FPS rendering"
echo ""

echo "2. Testing color-heavy output..."
echo "   Generating colored output..."
for i in {1..100}; do
    color=$((16 + RANDOM % 216))
    echo -e "   \e[38;5;${color}mColored line $i with random color code $color\e[0m"
done
echo ""
echo "   All colors should render smoothly"
echo ""

echo "3. Testing large scrollback..."
echo "   Generating 5000 lines for scrollback test..."
echo "   (You can scroll up to verify all lines are present)"
for i in {1..5000}; do
    if [ $((i % 100)) -eq 0 ]; then
        echo "   === Marker at line $i ==="
    else
        echo "   Scrollback line $i"
    fi
done
echo ""
echo "   Try scrolling up - you should see markers at every 100 lines"
echo ""

echo "4. Testing Unicode rendering..."
echo "   Various Unicode blocks:"
echo "   Latin Extended: Ā ā Ă ă Ą ą Ć ć Ĉ ĉ"
echo "   Greek: Α Β Γ Δ Ε Ζ Η Θ Ι Κ Λ Μ Ν"
echo "   Cyrillic: А Б В Г Д Е Ж З И К"
echo "   CJK: 你好 こんにちは 안녕하세요"
echo "   Symbols: ← ↑ → ↓ ⇐ ⇑ ⇒ ⇓ ∀ ∂ ∃ ∅"
echo "   Math: ∫ ∑ ∏ √ ∞ ≈ ≠ ≤ ≥ ± × ÷"
echo ""

echo "5. Testing box drawing stress test..."
echo "   Complex terminal UI:"
echo "   ╔════════════════════════════════════════╗"
echo "   ║  Hyper Terminal Performance Test      ║"
echo "   ╠════════════════════════════════════════╣"
echo "   ║  ┌──────────────┬──────────────────┐  ║"
echo "   ║  │ Cell 1       │ Cell 2           │  ║"
echo "   ║  ├──────────────┼──────────────────┤  ║"
echo "   ║  │ Cell 3       │ Cell 4           │  ║"
echo "   ║  └──────────────┴──────────────────┘  ║"
echo "   ╚════════════════════════════════════════╝"
echo ""

echo "6. Testing rapid cursor movement..."
echo "   Moving cursor across screen..."
for i in {0..10}; do
    col=$((i * 7))
    echo -e "\e[1A\e[${col}C*"
    sleep 0.05
done
echo ""
echo ""
echo "   All cursor positions should be accurate"
echo ""

echo "7. Testing clear and redraw performance..."
echo "   This will clear and redraw 10 times..."
for i in {1..10}; do
    clear
    echo "Redraw iteration $i/10"
    echo "Testing clear screen performance"
    for j in {1..20}; do
        echo "Line $j"
    done
    sleep 0.2
done
clear
echo ""

echo "============================================"
echo "  Performance Demo Complete!"
echo "============================================"
echo ""
echo "Expected performance characteristics:"
echo "  • Startup time: <500ms"
echo "  • Frame rate: 60 FPS sustained"
echo "  • Text rendering: <1ms per frame"
echo "  • Image rendering: <1ms per image"
echo "  • Memory usage: 50-150 MB base"
echo "  • Scrollback: Up to 10,000 lines efficiently"
echo ""
echo "Monitor system resources:"
echo "  ps aux | grep hyper          # Check memory usage"
echo "  top -p \$(pgrep hyper)        # Monitor CPU/RAM"
echo ""
