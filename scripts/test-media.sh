#!/bin/bash
# Test script for media rendering

echo "=== Hyper Terminal Media Test ==="
echo ""

echo "Testing text output first..."
echo "If you can see this, text rendering works!"
echo ""

echo "Now testing image rendering..."
echo "Attempting to display test images..."
echo ""

# Check if we have test images
if [ -f "assets/test-image.png" ]; then
    echo "Sending test image via Kitty protocol..."
    ./scripts/send-kitty-image.sh assets/test-image.png
    echo "Image sent! You should see an image above."
else
    echo "Note: Test images not found in assets/"
    echo "You can create a test image with:"
    echo "  convert -size 100x100 xc:blue assets/test-image.png"
fi

echo ""
echo "=== Media Test Complete ==="
