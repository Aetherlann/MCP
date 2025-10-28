# Demo Scripts

This directory contains comprehensive demonstration scripts for showcasing Hyper Terminal's features.

## Quick Start

```bash
# 1. Build Hyper Terminal
cargo build --release

# 2. Run terminal
cargo run --release

# 3. Inside terminal, run all demos:
./scripts/run-all-demos.sh
```

## Available Scripts

### Demo Scripts

| Script | Duration | Purpose |
|--------|----------|---------|
| `run-all-demos.sh` | 5-10 min | Master runner - executes all demos |
| `demo-basic.sh` | 2 min | Basic terminal features (M1+M2) |
| `demo-media.sh` | 3 min | Inline image rendering (M3) |
| `demo-performance.sh` | 2-3 min | Performance benchmarks |

### Utility Scripts

| Script | Purpose |
|--------|---------|
| `send-kitty-image.sh` | Send images via Kitty protocol |
| `test-media.sh` | Quick media rendering test |
| `test-terminal.sh` | VT parser and grid tests |
| `generate-test-images.py` | Create test images (requires Pillow) |

## Usage Examples

### Run All Demos (Interactive)

```bash
./scripts/run-all-demos.sh

# Follow prompts, press Enter to continue between demos
```

### Run All Demos (Automatic)

```bash
./scripts/run-all-demos.sh --auto

# Runs all demos without pausing
# Useful for screen recording
```

### Run Individual Demos

```bash
# Basic features
./scripts/demo-basic.sh

# Media rendering
./scripts/demo-media.sh

# Performance tests
./scripts/demo-performance.sh
```

### Send Custom Images

```bash
# Send any PNG/JPEG image
./scripts/send-kitty-image.sh path/to/your/image.png

# Works with various formats
./scripts/send-kitty-image.sh photo.jpg
./scripts/send-kitty-image.sh diagram.png
./scripts/send-kitty-image.sh screenshot.png
```

### Generate Test Images

```bash
# If Python + Pillow installed:
python3 scripts/generate-test-images.py

# Creates in assets/:
# - test-blue.png (200x200)
# - test-gradient.png (300x200)
# - test-pattern.png (400x300)
# - test-emoji.png (150x150)
# - test-small.png (64x64)
```

### Manual Testing

```bash
# Quick media test
./scripts/test-media.sh

# VT parser tests (run outside terminal)
./scripts/test-terminal.sh
```

## Demo Script Details

### `run-all-demos.sh`

Comprehensive master script that runs all demos in sequence.

**Features:**
- Interactive mode (default): Pauses between demos
- Automatic mode (`--auto`): Runs continuously
- Summary of all features at end
- Performance metrics display

**What it tests:**
- All M1, M2, and M3 features
- Text rendering (GPU accelerated)
- ANSI colors (16, 256, RGB)
- Text attributes (bold, italic, underline)
- Unicode and emoji
- Box drawing characters
- Inline image display (Kitty protocol)
- Performance benchmarks

### `demo-basic.sh`

Tests fundamental terminal features.

**Demonstrates:**
1. Plain text output
2. ANSI color rendering (16, 256, RGB)
3. Text attributes (bold, dim, italic, underline, strikethrough)
4. 256-color palette display
5. Custom RGB colors
6. Box drawing characters
7. Unicode characters and emoji
8. Cursor movement control
9. System command execution (`ls`)

**Expected Results:**
- All colors render correctly
- Text styling works
- Box characters form complete boxes
- Unicode displays clearly
- Cursor moves precisely

### `demo-media.sh`

Tests inline image rendering via Kitty Graphics Protocol.

**Demonstrates:**
1. Basic image display (1x1 pixel)
2. Image at specific cursor position
3. Text + image composition
4. Multiple simultaneous images
5. Dynamic image generation (if ImageMagick available)
6. Scrolling behavior with images

**Generated Images** (if ImageMagick present):
- `test-blue.png`: Blue square with "TEST"
- `test-gradient.png`: Color gradient
- `test-pattern.png`: Checkerboard

**Expected Results:**
- Images appear inline at cursor
- Multiple images display independently
- Text flows around images correctly
- Images scroll with content

### `demo-performance.sh`

Benchmarks rendering performance under various workloads.

**Tests:**
1. Rapid text output (1000 lines)
2. Color-intensive rendering (100 colored lines)
3. Large scrollback (5000 lines with markers)
4. Unicode rendering (multiple scripts)
5. Complex box drawing (nested tables)
6. Rapid cursor movement
7. Clear/redraw cycles (10 iterations)

**Performance Targets:**
- 1000 lines: <2000ms
- Frame rate: 60 FPS sustained
- Scrollback: All 5000 lines accessible
- No visual stuttering or lag

**Expected Results:**
- Smooth rendering throughout
- No frame drops
- All text remains readable
- Cursor moves accurately

## Screen Recording

For recording demos, use automatic mode:

```bash
# Start recording (ffmpeg example)
ffmpeg -f x11grab -r 60 -s 1920x1080 -i :0.0 \
    -c:v libx264 -preset ultrafast -crf 18 \
    demo.mp4 &

# Run terminal
cargo run --release

# Run demos without pauses
./scripts/run-all-demos.sh --auto

# Stop recording
pkill ffmpeg
```

See `VISUAL_DEMOS.md` for detailed recording instructions.

## Troubleshooting

### Scripts Won't Execute

```bash
# Make executable
chmod +x scripts/*.sh

# Or run with bash explicitly
bash scripts/demo-basic.sh
```

### Images Not Found

```bash
# Verify assets exist
ls -l assets/

# Create minimal test image
echo "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg==" | base64 -d > assets/tiny-red.png
```

### ImageMagick Not Available

Demos will skip generated images but still work with existing assets.

Install ImageMagick:
```bash
# Ubuntu/Debian
sudo apt-get install imagemagick

# macOS
brew install imagemagick

# Fedora
sudo dnf install ImageMagick
```

### Performance Issues

- Close background applications
- Update GPU drivers
- Reduce terminal window size
- Check CPU/memory usage: `top`

## Creating Custom Demos

### Template Script

```bash
#!/bin/bash
# My custom demo

echo "=== Custom Demo ==="
echo ""

echo "Testing custom feature..."
# Your commands here

echo ""
echo "Demo complete!"
```

### Using send-kitty-image.sh

```bash
# Display any image inline
./scripts/send-kitty-image.sh /path/to/image.png

# Create test image first
convert -size 400x300 xc:purple -fill white \
    -pointsize 60 -gravity center \
    -annotate +0+0 "Custom" custom.png

./scripts/send-kitty-image.sh custom.png
```

## Integration with CI/CD

Demo scripts can be used in automated testing:

```yaml
# .github/workflows/demo.yml
name: Demo Tests
on: [push]
jobs:
  demo:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Build
        run: cargo build --release
      - name: Run demos
        run: |
          timeout 300 cargo run --release &
          sleep 5
          bash scripts/demo-basic.sh
          bash scripts/demo-media.sh
```

## Documentation

For comprehensive information:
- **`VISUAL_DEMOS.md`**: Complete visual demo guide
- **`COMPILATION_AND_TESTING.md`**: Build and test instructions
- **`TESTING.md`**: Unit and integration tests
- **`M3_MEDIA_SUPPORT.md`**: Media rendering technical details

## Support

Issues with demos:
1. Check script is executable: `ls -l scripts/`
2. Verify in repository root: `pwd`
3. Check assets exist: `ls assets/`
4. Review logs: `RUST_LOG=debug cargo run`
5. Open GitHub issue with error details

---

**Quick command reference:**

```bash
# Build
cargo build --release

# Run all demos
cargo run --release
./scripts/run-all-demos.sh

# Run specific demo
./scripts/demo-basic.sh
./scripts/demo-media.sh
./scripts/demo-performance.sh

# Send custom image
./scripts/send-kitty-image.sh image.png
```

Happy demoing! 🎉
