#!/bin/bash
# Gallery Grid Demo - Simple 3-image grid
# Run this inside Hyper Terminal after: cargo run --release

echo "============================================"
echo "  Hyper Terminal - Gallery Grid Demo"
echo "============================================"
echo ""

echo "This demo shows a simple 3-image gallery using grid mode."
echo ""

# Check if test images exist
if [ ! -f "assets/tiny-red.png" ]; then
    echo "Error: Test image not found!"
    echo "Please ensure assets/tiny-red.png exists."
    exit 1
fi

echo "Starting gallery with grid layout..."
echo ""

# Start gallery with grid mode
echo -ne "\e]1338;gallery=start;id=demo1;mode=grid;title=Test Images\a"

# Add image 1
echo "Adding Image 1..."
echo -ne "\e]1338;media=item;gallery=demo1;title=Image 1;desc=First test image;tags=test,demo\a"
printf '\e_Gf=100,a=T,t=d;%s\e\\' "$(base64 < assets/tiny-red.png | tr -d '\n')"
echo ""

# Add image 2
echo "Adding Image 2..."
echo -ne "\e]1338;media=item;gallery=demo1;title=Image 2;desc=Second test image;tags=test,demo\a"
printf '\e_Gf=100,a=T,t=d;%s\e\\' "$(base64 < assets/tiny-red.png | tr -d '\n')"
echo ""

# Add image 3
echo "Adding Image 3..."
echo -ne "\e]1338;media=item;gallery=demo1;title=Image 3;desc=Third test image;tags=test,demo\a"
printf '\e_Gf=100,a=T,t=d;%s\e\\' "$(base64 < assets/tiny-red.png | tr -d '\n')"
echo ""

# End gallery
echo -ne "\e]1338;gallery=end;id=demo1\a"

echo ""
echo "Gallery created! You should see:"
echo "  - 3 images in a grid layout"
echo "  - Title: 'Test Images'"
echo "  - Each image with its own title and description"
echo "  - Interactive hover effects"
echo ""
echo "============================================"
echo "  Demo Complete!"
echo "============================================"
