#!/bin/bash
# Test script for Hyper Terminal

echo "=== Hyper Terminal Test Script ==="
echo ""

# Test basic text
echo "Testing basic text output:"
echo "Hello, World!"
echo ""

# Test colors
echo "Testing colors:"
echo -e "\e[31mRed text\e[0m"
echo -e "\e[32mGreen text\e[0m"
echo -e "\e[34mBlue text\e[0m"
echo -e "\e[1;33mBold yellow text\e[0m"
echo ""

# Test cursor movement
echo "Testing cursor movement:"
echo -e "Line 1\nLine 2\nLine 3"
echo -e "\e[2AMoving up 2 lines"
echo ""

# Test clear line
echo "Testing clear operations:"
echo -e "This will be cleared\e[K"
sleep 1
echo ""

# Test RGB colors
echo "Testing RGB colors:"
echo -e "\e[38;2;255;100;50mRGB Orange text\e[0m"
echo -e "\e[48;2;50;100;200mRGB Blue background\e[0m"
echo ""

# Test attributes
echo "Testing text attributes:"
echo -e "\e[1mBold\e[0m"
echo -e "\e[3mItalic\e[0m"
echo -e "\e[4mUnderline\e[0m"
echo -e "\e[9mStrikethrough\e[0m"
echo -e "\e[1;3;4mBold italic underline\e[0m"
echo ""

# Test box drawing
echo "Testing box drawing:"
echo "┌─────────────┐"
echo "│   Box Test  │"
echo "└─────────────┘"
echo ""

# Test Unicode
echo "Testing Unicode:"
echo "Emoji: 🚀 🎉 ⭐"
echo "Math: π ≈ 3.14159"
echo "Arrows: → ↓ ← ↑"
echo ""

# Test scrolling
echo "Testing scrolling (generating many lines):"
for i in {1..30}; do
    echo "Line $i"
done
echo ""

echo "=== Test Complete ==="
