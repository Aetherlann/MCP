#!/bin/bash
# Gallery Comparison Demo - Side-by-side comparison
# Run this inside Hyper Terminal after: cargo run --release

echo "============================================"
echo "  Hyper Terminal - Comparison Mode Demo"
echo "============================================"
echo ""

echo "This demo shows side-by-side image comparison."
echo ""

# Check if test images exist
if [ ! -f "assets/tiny-red.png" ]; then
    echo "Error: Test image not found!"
    echo "Please ensure assets/tiny-red.png exists."
    exit 1
fi

echo "Creating comparison gallery..."
echo ""

# Start gallery with comparison mode
echo -ne "\e]1338;gallery=start;id=compare1;mode=comparison;title=Before vs After\a"

# Add "before" image
echo "Adding 'Before' image..."
echo -ne "\e]1338;media=item;gallery=compare1;title=Before;desc=Original version\a"
printf '\e_Gf=100,a=T,t=d;%s\e\\' "$(base64 < assets/tiny-red.png | tr -d '\n')"
echo ""

# Add "after" image
echo "Adding 'After' image..."
echo -ne "\e]1338;media=item;gallery=compare1;title=After;desc=Enhanced version\a"
printf '\e_Gf=100,a=T,t=d;%s\e\\' "$(base64 < assets/tiny-red.png | tr -d '\n')"
echo ""

# End gallery
echo -ne "\e]1338;gallery=end;id=compare1\a"

echo ""
echo "Comparison gallery created! You should see:"
echo "  - Two images side-by-side"
echo "  - 'Before' on the left, 'After' on the right"
echo "  - Synchronized zoom controls"
echo "  - Optional slider for A/B comparison"
echo ""
echo "============================================"
echo "  Demo Complete!"
echo "============================================"
