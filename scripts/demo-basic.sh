#!/bin/bash
# Basic terminal functionality demo
# Run this inside the Hyper Terminal after: cargo run --release

echo "============================================"
echo "  Hyper Terminal - Basic Functionality Demo"
echo "============================================"
echo ""

echo "1. Testing basic text output..."
echo "   This is plain text."
echo ""

echo "2. Testing ANSI colors..."
echo -e "   \e[31mRed\e[0m \e[32mGreen\e[0m \e[33mYellow\e[0m \e[34mBlue\e[0m \e[35mMagenta\e[0m \e[36mCyan\e[0m \e[37mWhite\e[0m"
echo ""

echo "3. Testing text attributes..."
echo -e "   \e[1mBold\e[0m \e[2mDim\e[0m \e[3mItalic\e[0m \e[4mUnderline\e[0m \e[9mStrikethrough\e[0m"
echo ""

echo "4. Testing 256-color palette..."
printf "   "
for i in {0..15}; do
    printf "\e[48;5;${i}m  \e[0m"
done
echo ""
printf "   "
for i in {16..31}; do
    printf "\e[48;5;${i}m  \e[0m"
done
echo ""
echo ""

echo "5. Testing RGB colors..."
echo -e "   \e[38;2;255;100;50mCustom RGB Text\e[0m \e[48;2;50;100;255m Background \e[0m"
echo ""

echo "6. Testing box drawing characters..."
echo "   ┌─────────────────┐"
echo "   │  Box Drawing    │"
echo "   ├─────────────────┤"
echo "   │  Characters     │"
echo "   └─────────────────┘"
echo ""

echo "7. Testing Unicode and emoji..."
echo "   Unicode: α β γ δ ε → ← ↑ ↓ ★ ♠ ♣ ♥ ♦"
echo "   Emoji: 😀 🚀 🎨 💻 🔥 ✨"
echo ""

echo "8. Testing cursor movement..."
echo -n "   Start"
sleep 0.5
echo -e "\r   \e[10CMiddle"
sleep 0.5
echo -e "\e[1A\e[20CEnd"
echo ""
echo ""

echo "9. Running system command..."
echo "   Current directory contents:"
ls -lh --color=auto | head -10
echo ""

echo "============================================"
echo "  Basic Demo Complete!"
echo "============================================"
echo ""
echo "Next: Run './scripts/demo-media.sh' to test image rendering"
echo ""
