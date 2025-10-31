#!/bin/bash
# Demo: Split terminal into regions with different content

set -e

echo "=== Next-Gen Terminal: Split Layout Demo ==="
echo
echo "This demo shows how to split the terminal into multiple regions"
echo "and display different content in each region."
echo
sleep 2

# Split terminal vertically (left 40%, right 60%)
echo "Splitting terminal vertically..."
echo -ne "\e]1339;region=split;id=root;direction=vertical;ratio=0.4\a"
sleep 1

# Left region: Show some code
echo
echo "Left region (40%): Code Editor View"
echo -ne "\e]1339;region=content;id=left;type=code;language=rust\a"
sleep 1

# Right region: Split horizontally
echo "Splitting right region horizontally..."
echo -ne "\e]1339;region=split;id=right;direction=horizontal;ratio=0.6\a"
sleep 1

# Right top: Terminal output
echo
echo "Right top (36%): Terminal Output"
echo -ne "\e]1339;region=content;id=right_top;type=terminal\a"
sleep 1

# Right bottom: Gallery
echo
echo "Right bottom (24%): Image Gallery"
echo -ne "\e]1339;region=content;id=right_bottom;type=gallery;gallery_id=demo-gallery\a"
sleep 1

echo
echo "=== Layout Complete ===$"
echo
echo "You now have a 3-panel layout:"
echo "  - Left: Code editor (40%)"
echo "  - Right Top: Terminal output (36%)"
echo "  - Right Bottom: Gallery (24%)"
echo
echo "This is perfect for:"
echo "  ✓ Editing code while seeing output"
echo "  ✓ Viewing generated images alongside code"
echo "  ✓ Multi-tasking in a single terminal window"
echo
