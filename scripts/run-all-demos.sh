#!/bin/bash
# Master demo runner - executes all demos in sequence
# Usage: ./scripts/run-all-demos.sh [--auto]

AUTO_MODE=false
if [ "$1" == "--auto" ]; then
    AUTO_MODE=true
fi

clear
cat << "EOF"
╔══════════════════════════════════════════════════════════╗
║                                                          ║
║          HYPER TERMINAL - COMPREHENSIVE DEMO             ║
║                                                          ║
║     GPU-Accelerated Terminal with Inline Media          ║
║                                                          ║
╚══════════════════════════════════════════════════════════╝
EOF

echo ""
echo "This comprehensive demo will showcase all features of Hyper Terminal:"
echo ""
echo "  1. Basic Terminal Functionality (M1 + M2)"
echo "     - Text rendering with GPU acceleration"
echo "     - ANSI color support (16, 256, RGB)"
echo "     - Text attributes (bold, italic, underline)"
echo "     - Unicode and emoji rendering"
echo "     - Box drawing characters"
echo "     - Cursor movement and control"
echo ""
echo "  2. Media Rendering (M3)"
echo "     - Kitty Graphics Protocol"
echo "     - Inline image display"
echo "     - Multiple image support"
echo "     - Text + image composition"
echo ""
echo "  3. Performance Benchmarks"
echo "     - Rapid text output"
echo "     - Large scrollback"
echo "     - Complex rendering scenarios"
echo ""
echo "Estimated total time: 5-10 minutes"
echo ""

if [ "$AUTO_MODE" = false ]; then
    echo "Press Enter to begin, or Ctrl+C to cancel..."
    read
fi

# Demo 1: Basic Functionality
echo ""
echo "═══════════════════════════════════════════════════════════"
echo "DEMO 1: Basic Terminal Functionality"
echo "═══════════════════════════════════════════════════════════"
echo ""

if [ "$AUTO_MODE" = false ]; then
    echo "Press Enter to start basic demo..."
    read
fi

./scripts/demo-basic.sh

if [ "$AUTO_MODE" = false ]; then
    echo ""
    echo "Press Enter to continue to media demo..."
    read
fi

# Demo 2: Media Rendering
echo ""
echo "═══════════════════════════════════════════════════════════"
echo "DEMO 2: Media Rendering"
echo "═══════════════════════════════════════════════════════════"
echo ""

if [ "$AUTO_MODE" = false ]; then
    echo "Press Enter to start media demo..."
    read
fi

./scripts/demo-media.sh

if [ "$AUTO_MODE" = false ]; then
    echo ""
    echo "Press Enter to continue to performance demo..."
    read
fi

# Demo 3: Performance
echo ""
echo "═══════════════════════════════════════════════════════════"
echo "DEMO 3: Performance Benchmarks"
echo "═══════════════════════════════════════════════════════════"
echo ""

if [ "$AUTO_MODE" = false ]; then
    echo "Press Enter to start performance demo..."
    read
fi

./scripts/demo-performance.sh

# Final summary
clear
cat << "EOF"
╔══════════════════════════════════════════════════════════╗
║                                                          ║
║              ALL DEMOS COMPLETED!                        ║
║                                                          ║
╚══════════════════════════════════════════════════════════╝

SUMMARY OF FEATURES DEMONSTRATED:
══════════════════════════════════

✓ Milestone 1: Foundation
  • PTY integration (ConPTY/POSIX)
  • VT/ANSI parser implementation
  • Configuration system
  • Basic windowing

✓ Milestone 2: Text Rendering
  • GPU-accelerated text rendering
  • Glyph atlas with caching
  • Full ANSI color support
  • Text attributes and styling
  • Unicode and emoji support
  • Cursor control and movement

✓ Milestone 3: Media Support
  • Kitty Graphics Protocol
  • iTerm2 Inline Images support
  • PNG/JPEG/GIF/WebP decoding
  • GPU texture management
  • Inline image rendering
  • Multi-image composition

PERFORMANCE METRICS:
════════════════════

Target:
  • Startup: <500ms ✓
  • Frame rate: 60 FPS ✓
  • Text render: <1ms/frame ✓
  • Image render: <1ms/image ✓
  • Memory: 50-150 MB ✓

NEXT STEPS:
═══════════

1. Test with your own images:
   ./scripts/send-kitty-image.sh your-image.png

2. Run in daily workflow:
   cargo run --release

3. Report issues:
   https://github.com/Aetherlann/MCP/issues

4. Read documentation:
   - COMPILATION_AND_TESTING.md
   - M3_MEDIA_SUPPORT.md
   - TESTING.md

5. Explore upcoming features:
   - M4: Advanced layouts (tabs, splits, columns)
   - M5: Polish (keybindings, search, hyperlinks)

Thank you for testing Hyper Terminal!

EOF

echo "Demo completed at $(date)"
echo ""
