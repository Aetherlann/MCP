#!/bin/bash
# Media rendering demo for inline images
# Run this inside the Hyper Terminal after: cargo run --release

echo "============================================"
echo "  Hyper Terminal - Media Rendering Demo"
echo "============================================"
echo ""

# Check if we have test images
if [ ! -f "assets/tiny-red.png" ]; then
    echo "Error: Test images not found in assets/"
    echo "Please ensure you're running from the repository root."
    exit 1
fi

echo "This demo tests the Kitty Graphics Protocol implementation"
echo "for inline image rendering in the terminal."
echo ""
echo "Press Enter to continue..."
read

echo "1. Testing basic image display..."
echo "   Sending tiny-red.png (1x1 red pixel)..."
./scripts/send-kitty-image.sh assets/tiny-red.png
echo "   ↑ You should see a tiny red pixel above"
echo ""
sleep 2

echo "2. Testing image at cursor position..."
echo -n "   Image should appear here → "
./scripts/send-kitty-image.sh assets/tiny-red.png
echo " ← right there"
echo ""
sleep 2

echo "3. Testing text + image composition..."
echo "   Line of text before image"
./scripts/send-kitty-image.sh assets/tiny-red.png
echo "   Line of text after image"
echo ""
sleep 2

echo "4. Testing multiple images..."
echo "   Image 1:"
./scripts/send-kitty-image.sh assets/tiny-red.png
echo "   Image 2:"
./scripts/send-kitty-image.sh assets/tiny-red.png
echo "   Image 3:"
./scripts/send-kitty-image.sh assets/tiny-red.png
echo "   All three images should be visible"
echo ""
sleep 2

echo "5. Creating a test pattern with ImageMagick (if available)..."
if command -v convert &> /dev/null; then
    echo "   Generating test images..."

    # Create a blue square
    convert -size 200x200 xc:blue -fill white \
        -pointsize 40 -gravity center \
        -annotate +0+0 "TEST" \
        assets/test-blue.png 2>/dev/null

    if [ -f "assets/test-blue.png" ]; then
        echo "   Displaying test-blue.png (200x200)..."
        ./scripts/send-kitty-image.sh assets/test-blue.png
        echo "   ↑ Blue square with 'TEST' text"
        echo ""
        sleep 2

        # Create a gradient
        convert -size 300x150 gradient:blue-red assets/test-gradient.png 2>/dev/null
        echo "   Displaying test-gradient.png (300x150)..."
        ./scripts/send-kitty-image.sh assets/test-gradient.png
        echo "   ↑ Blue to red gradient"
        echo ""
        sleep 2

        # Create a pattern
        convert -size 200x200 pattern:checkerboard assets/test-pattern.png 2>/dev/null
        echo "   Displaying test-pattern.png (200x200)..."
        ./scripts/send-kitty-image.sh assets/test-pattern.png
        echo "   ↑ Checkerboard pattern"
        echo ""
    else
        echo "   Warning: Image generation failed"
    fi
else
    echo "   ImageMagick not found - skipping generated images"
    echo "   Install with: sudo apt-get install imagemagick"
fi
echo ""

echo "6. Testing scrolling behavior..."
echo "   Generating text to demonstrate image scrolling..."
for i in {1..20}; do
    echo "   Line $i - Images should scroll with text"
done
echo ""

echo "============================================"
echo "  Media Demo Complete!"
echo "============================================"
echo ""
echo "What was tested:"
echo "  ✓ Kitty Graphics Protocol parsing"
echo "  ✓ Base64 payload decoding"
echo "  ✓ Image format detection (PNG)"
echo "  ✓ GPU texture upload"
echo "  ✓ Inline rendering at cursor position"
echo "  ✓ Multiple image support"
echo "  ✓ Text + image composition"
echo "  ✓ Scrolling behavior"
echo ""
echo "Next steps:"
echo "  - Try your own images: ./scripts/send-kitty-image.sh <your-image.png>"
echo "  - Test JPEG: ./scripts/send-kitty-image.sh <your-image.jpg>"
echo "  - Test with larger images (up to 4K resolution)"
echo ""
