#!/bin/bash
# Gallery All Modes Demo - Showcase all gallery display modes
# Run this inside Hyper Terminal after: cargo run --release

echo "╔══════════════════════════════════════════════════════╗"
echo "║                                                      ║"
echo "║   Hyper Terminal - Gallery System Demo              ║"
echo "║   All Display Modes                                  ║"
echo "║                                                      ║"
echo "╚══════════════════════════════════════════════════════╝"
echo ""

# Check if test images exist
if [ ! -f "assets/tiny-red.png" ]; then
    echo "Error: Test image not found!"
    echo "Please ensure assets/tiny-red.png exists."
    exit 1
fi

echo "This comprehensive demo will showcase all gallery modes:"
echo "  1. Grid Mode"
echo "  2. Masonry Mode"
echo "  3. Filmstrip Mode"
echo "  4. Comparison Mode"
echo "  5. Auto Mode"
echo ""
echo "Press Enter to begin..."
read

# Demo 1: Grid Mode
echo ""
echo "═══════════════════════════════════════════════════════"
echo "DEMO 1: Grid Mode - Uniform layout"
echo "═══════════════════════════════════════════════════════"
echo ""

echo -ne "\e]1338;gallery=start;id=grid_demo;mode=grid;title=Grid Gallery;columns=2\a"

for i in {1..4}; do
    echo "Adding image $i to grid..."
    echo -ne "\e]1338;media=item;gallery=grid_demo;title=Grid Image $i;desc=Item $i in grid layout\a"
    printf '\e_Gf=100,a=T,t=d;%s\e\\' "$(base64 < assets/tiny-red.png | tr -d '\n')"
    echo ""
done

echo -ne "\e]1338;gallery=end;id=grid_demo\a"

echo "✓ Grid gallery created (2x2 layout)"
echo ""
echo "Press Enter for next demo..."
read

# Demo 2: Masonry Mode
echo ""
echo "═══════════════════════════════════════════════════════"
echo "DEMO 2: Masonry Mode - Pinterest-style"
echo "═══════════════════════════════════════════════════════"
echo ""

echo -ne "\e]1338;gallery=start;id=masonry_demo;mode=masonry;title=Masonry Gallery\a"

for i in {1..6}; do
    echo "Adding image $i to masonry..."
    echo -ne "\e]1338;media=item;gallery=masonry_demo;title=Masonry $i;desc=Flexible layout item $i\a"
    printf '\e_Gf=100,a=T,t=d;%s\e\\' "$(base64 < assets/tiny-red.png | tr -d '\n')"
    echo ""
done

echo -ne "\e]1338;gallery=end;id=masonry_demo\a"

echo "✓ Masonry gallery created (balanced columns)"
echo ""
echo "Press Enter for next demo..."
read

# Demo 3: Filmstrip Mode
echo ""
echo "═══════════════════════════════════════════════════════"
echo "DEMO 3: Filmstrip Mode - Sequential navigation"
echo "═══════════════════════════════════════════════════════"
echo ""

echo -ne "\e]1338;gallery=start;id=filmstrip_demo;mode=filmstrip;title=Process Steps\a"

for i in {1..5}; do
    echo "Adding step $i..."
    echo -ne "\e]1338;media=item;gallery=filmstrip_demo;title=Step $i;desc=Process stage $i\a"
    printf '\e_Gf=100,a=T,t=d;%s\e\\' "$(base64 < assets/tiny-red.png | tr -d '\n')"
    echo ""
done

echo -ne "\e]1338;gallery=end;id=filmstrip_demo\a"

echo "✓ Filmstrip gallery created (horizontal scroll)"
echo ""
echo "Press Enter for next demo..."
read

# Demo 4: Comparison Mode
echo ""
echo "═══════════════════════════════════════════════════════"
echo "DEMO 4: Comparison Mode - Side-by-side"
echo "═══════════════════════════════════════════════════════"
echo ""

echo -ne "\e]1338;gallery=start;id=comparison_demo;mode=comparison;title=Before vs After\a"

echo "Adding 'Before' image..."
echo -ne "\e]1338;media=item;gallery=comparison_demo;title=Before;desc=Original;tags=before\a"
printf '\e_Gf=100,a=T,t=d;%s\e\\' "$(base64 < assets/tiny-red.png | tr -d '\n')"
echo ""

echo "Adding 'After' image..."
echo -ne "\e]1338;media=item;gallery=comparison_demo;title=After;desc=Enhanced;tags=after\a"
printf '\e_Gf=100,a=T,t=d;%s\e\\' "$(base64 < assets/tiny-red.png | tr -d '\n')"
echo ""

echo -ne "\e]1338;gallery=end;id=comparison_demo\a"

echo "✓ Comparison gallery created (2-up view)"
echo ""
echo "Press Enter for next demo..."
read

# Demo 5: Auto Mode
echo ""
echo "═══════════════════════════════════════════════════════"
echo "DEMO 5: Auto Mode - Smart detection"
echo "═══════════════════════════════════════════════════════"
echo ""

echo -ne "\e]1338;gallery=start;id=auto_demo;mode=auto;title=Smart Gallery\a"

for i in {1..3}; do
    echo "Adding image $i (auto mode will choose best layout)..."
    echo -ne "\e]1338;media=item;gallery=auto_demo;title=Auto Image $i;desc=Auto-layout item $i\a"
    printf '\e_Gf=100,a=T,t=d;%s\e\\' "$(base64 < assets/tiny-red.png | tr -d '\n')"
    echo ""
done

echo -ne "\e]1338;gallery=end;id=auto_demo\a"

echo "✓ Auto gallery created (smart mode selection)"
echo ""

# Summary
echo ""
echo "╔══════════════════════════════════════════════════════╗"
echo "║                                                      ║"
echo "║   All Gallery Modes Demonstrated!                    ║"
echo "║                                                      ║"
echo "╚══════════════════════════════════════════════════════╝"
echo ""
echo "Summary of what was shown:"
echo "  ✓ Grid Mode: Uniform 2x2 layout"
echo "  ✓ Masonry Mode: Flexible, balanced columns"
echo "  ✓ Filmstrip Mode: Horizontal scrolling"
echo "  ✓ Comparison Mode: Side-by-side view"
echo "  ✓ Auto Mode: Intelligent mode selection"
echo ""
echo "Each gallery supports:"
echo "  • Rich metadata (titles, descriptions, tags)"
echo "  • Interactive hover effects"
echo "  • Action buttons (save, copy, zoom)"
echo "  • Keyboard navigation"
echo "  • Smooth animations"
echo ""
echo "Try interacting with the galleries above!"
echo ""
