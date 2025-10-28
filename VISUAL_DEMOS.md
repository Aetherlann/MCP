# Visual Demos Guide

## Overview

This guide provides comprehensive instructions for running, recording, and sharing visual demonstrations of Hyper Terminal's capabilities.

## Prerequisites

### Required

- Hyper Terminal successfully compiled (`cargo build --release`)
- Working directory: Repository root (`/path/to/MCP`)
- Terminal window: At least 1200x800 pixels

### Optional (for enhanced demos)

- **ImageMagick**: For generating test images
  ```bash
  # Ubuntu/Debian
  sudo apt-get install imagemagick

  # macOS
  brew install imagemagick

  # Fedora/RHEL
  sudo dnf install ImageMagick
  ```

- **Screen Recording**: For creating demo videos
  - Linux: `ffmpeg`, `obs-studio`, or `kazam`
  - macOS: QuickTime Player or ScreenFlow
  - Windows: OBS Studio or Windows Game Bar

## Quick Start

### Option 1: Run All Demos (Recommended)

```bash
# Start Hyper Terminal
cargo run --release

# In the terminal, run:
./scripts/run-all-demos.sh
```

This runs all three demo scripts in sequence:
1. Basic terminal functionality
2. Media rendering
3. Performance benchmarks

### Option 2: Run Individual Demos

```bash
# Start Hyper Terminal
cargo run --release

# Then run any of:
./scripts/demo-basic.sh         # Basic features
./scripts/demo-media.sh          # Image rendering
./scripts/demo-performance.sh    # Performance tests
```

## Demo Scripts

### 1. Basic Functionality Demo (`demo-basic.sh`)

**Duration**: ~2 minutes
**Purpose**: Showcase core terminal features

**What it demonstrates:**
- Plain text rendering
- ANSI colors (16, 256, RGB)
- Text attributes (bold, italic, underline, strikethrough)
- Box drawing characters
- Unicode and emoji support
- Cursor movement control
- System command execution

**Expected output:**
- All colors render correctly with proper RGB values
- Text attributes display as styled
- Box characters form continuous lines
- Unicode/emoji render clearly
- Cursor moves to exact positions
- `ls` command shows colored output

### 2. Media Rendering Demo (`demo-media.sh`)

**Duration**: ~3 minutes
**Purpose**: Showcase inline image support

**What it demonstrates:**
- Kitty Graphics Protocol implementation
- Base64 payload decoding
- Image display at cursor position
- Multiple simultaneous images
- Text + image composition
- Scrolling behavior with images
- Dynamic image generation (if ImageMagick available)

**Expected output:**
- Images appear inline at cursor position
- Text wraps around images correctly
- Multiple images display independently
- Images scroll with text
- No rendering artifacts or glitches

**Note**: If ImageMagick is installed, additional test images are generated:
- `test-blue.png`: Blue square with "TEST" text (200x200)
- `test-gradient.png`: Blue to red gradient (300x150)
- `test-pattern.png`: Checkerboard pattern (200x200)

### 3. Performance Demo (`demo-performance.sh`)

**Duration**: ~2-3 minutes
**Purpose**: Benchmark rendering performance

**What it demonstrates:**
- Rapid text output (1000 lines)
- Color-intensive rendering (100 colored lines)
- Large scrollback (5000 lines)
- Unicode rendering across multiple scripts
- Complex box drawing
- Rapid cursor movement
- Clear/redraw performance (10 iterations)

**Expected output:**
- 1000 lines render in <2 seconds
- All colors display smoothly
- Scrollback contains all 5000 lines with markers
- Unicode characters render correctly
- Cursor moves precisely
- No frame drops or stuttering

## Recording Demos

### Setup

1. **Window Size**: Set terminal to 1200x800 or 1920x1080 for best results
2. **Font Size**: Increase for better visibility (edit `config.json`: `"size": 16.0`)
3. **Background**: Use glass effect for visual appeal

### Method 1: Using ffmpeg (Linux/macOS)

```bash
# Find display
echo $DISPLAY  # Usually :0 or :1

# Record entire screen
ffmpeg -f x11grab -r 60 -s 1920x1080 -i :0.0 \
    -c:v libx264 -preset ultrafast -crf 18 \
    hyper-terminal-demo.mp4

# In another terminal, run demos
cargo run --release
# Then: ./scripts/run-all-demos.sh --auto

# Stop recording with Ctrl+C
```

### Method 2: Using OBS Studio (All Platforms)

1. **Setup**:
   - Add "Window Capture" source
   - Select Hyper Terminal window
   - Resolution: 1920x1080
   - FPS: 60

2. **Recording**:
   - Start Recording
   - Run: `cargo run --release`
   - Execute: `./scripts/run-all-demos.sh --auto`
   - Stop Recording

3. **Output**: File saved to OBS videos folder

### Method 3: Using asciinema (Terminal Recording)

```bash
# Install asciinema
pip install asciinema

# Record terminal session
asciinema rec hyper-terminal-demo.cast

# Inside recording:
cargo run --release
# Run demos...

# Stop with Ctrl+D

# Upload and share
asciinema upload hyper-terminal-demo.cast
```

## Demo Scenarios

### Scenario 1: Quick Overview (30 seconds)

```bash
cargo run --release

# In terminal:
echo "Hyper Terminal - GPU Accelerated"
echo -e "\e[31mColors\e[0m \e[32mwork\e[0m \e[34mgreat\e[0m"
./scripts/send-kitty-image.sh assets/tiny-red.png
echo "Inline images supported!"
```

### Scenario 2: Developer Workflow (2 minutes)

```bash
cargo run --release

# In terminal:
git status
git log --oneline --graph --color | head -20
cat README.md
ls -la --color=auto
./scripts/send-kitty-image.sh assets/tiny-red.png
```

### Scenario 3: Media Showcase (3 minutes)

```bash
cargo run --release

# Generate test images (if ImageMagick available)
convert -size 300x200 xc:blue -fill white \
    -pointsize 60 -gravity center \
    -annotate +0+0 "DEMO" demo1.png

convert -size 400x300 gradient:red-yellow demo2.png

convert -size 300x300 plasma: demo3.png

# Display them
echo "Image 1: Blue background"
./scripts/send-kitty-image.sh demo1.png
echo ""

echo "Image 2: Gradient"
./scripts/send-kitty-image.sh demo2.png
echo ""

echo "Image 3: Plasma pattern"
./scripts/send-kitty-image.sh demo3.png
echo ""

echo "All images rendered with GPU acceleration!"
```

### Scenario 4: Stress Test (5 minutes)

```bash
cargo run --release

# In terminal:
./scripts/demo-performance.sh

# Monitor resources in another terminal:
watch -n 1 'ps aux | grep hyper | grep -v grep'
```

## Manual Testing Checklist

Use this checklist while running demos:

### Text Rendering
- [ ] ASCII characters render clearly
- [ ] 16 basic colors display correctly
- [ ] 256 color palette renders
- [ ] RGB colors work (test: `echo -e "\e[38;2;255;128;64mCustom\e[0m"`)
- [ ] Bold text is visibly bolder
- [ ] Italic text is slanted
- [ ] Underline appears under text
- [ ] Strikethrough crosses text

### Unicode & Emoji
- [ ] Latin extended characters (Ā, ă, ę)
- [ ] Greek alphabet (Α, Β, Γ)
- [ ] Cyrillic alphabet (А, Б, В)
- [ ] CJK characters (你好, こんにちは)
- [ ] Math symbols (∫, ∑, √, ∞)
- [ ] Emoji (😀, 🚀, 💻)
- [ ] Box drawing (┌─┐│└┘)

### Cursor & Control
- [ ] Cursor visible and blinking
- [ ] Arrow keys move cursor
- [ ] Home/End keys work
- [ ] Backspace deletes correctly
- [ ] Enter creates new line
- [ ] Tab indents

### Shell Integration
- [ ] Can run commands (ls, pwd, echo)
- [ ] Command output displays correctly
- [ ] Colors from programs work (ls --color)
- [ ] Interactive programs work (top, htop)
- [ ] Can navigate directories (cd)

### Media Rendering
- [ ] Images display inline
- [ ] Images at correct cursor position
- [ ] Multiple images work
- [ ] Text + image composition correct
- [ ] Images scroll with text
- [ ] No visual artifacts around images
- [ ] PNG images work
- [ ] JPEG images work (if tested)

### Performance
- [ ] Startup time <1 second
- [ ] Typing feels responsive (no lag)
- [ ] Scrolling is smooth (60 FPS)
- [ ] 1000+ lines render quickly
- [ ] No memory leaks visible
- [ ] CPU usage reasonable (<20% idle)

### Window & Resize
- [ ] Window opens at correct size
- [ ] Can resize window
- [ ] Text reflows on resize
- [ ] Images remain visible after resize
- [ ] No rendering glitches

## Troubleshooting Demos

### Demo Script Errors

**Error**: `command not found`
```bash
# Ensure scripts are executable
chmod +x scripts/*.sh

# Run from repository root
cd /path/to/MCP
./scripts/demo-basic.sh
```

**Error**: `assets/tiny-red.png not found`
```bash
# Verify file exists
ls -l assets/

# If missing, asset was not committed
# Create minimal test:
echo "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg==" | base64 -d > assets/tiny-red.png
```

### Visual Issues

**Issue**: Colors look wrong
- Check terminal color scheme settings
- Verify GPU drivers are up to date
- Check `TERM` environment variable

**Issue**: Images don't appear
- Check logs: `RUST_LOG=debug cargo run`
- Verify PNG file is valid: `file image.png`
- Try smaller image (<1MB)
- Check Kitty protocol implementation

**Issue**: Performance is poor
- Close other GPU applications
- Reduce window size
- Check CPU/GPU usage
- Update graphics drivers

### Recording Issues

**Issue**: Recording is choppy
- Reduce recording resolution (1280x720)
- Lower FPS to 30
- Close background applications
- Use hardware encoding if available

**Issue**: Audio sync problems
- Record audio separately
- Sync in post-production
- Use professional recording software

## Sharing Demos

### GitHub

```bash
# Upload recording to GitHub release
gh release create v0.1.0-demo \
    --title "Hyper Terminal Demo" \
    --notes "Visual demonstration of M1-M3 features" \
    hyper-terminal-demo.mp4
```

### YouTube

1. Export recording in H.264 format
2. Upload to YouTube
3. Title: "Hyper Terminal - GPU Accelerated Terminal with Inline Media"
4. Description: Link to GitHub repo and feature list
5. Tags: terminal, gpu, rust, wgpu, kitty-protocol

### Social Media

**Twitter/X**:
```
🚀 Hyper Terminal Demo

✨ GPU-accelerated text rendering
🎨 Full ANSI color support
🖼️ Inline image display (Kitty protocol)
⚡ 60 FPS performance

Built with #Rust and wgpu

[video] [link to repo]
```

**Reddit** (r/rust, r/programming, r/commandline):
```
Title: Built a GPU-accelerated terminal with inline image support

Description of features, architecture, and technical highlights.
Demo video and repository link.
```

## Next Steps

After successful demos:

1. **Report Results**: Open GitHub issue with findings
2. **Performance Data**: Share benchmark numbers
3. **Bug Reports**: Document any issues encountered
4. **Feature Requests**: Suggest improvements for M4/M5
5. **Community**: Share demos in Rust and terminal communities

## Support

For issues with demos:
- Check `COMPILATION_AND_TESTING.md`
- Review `TESTING.md`
- Read `M3_MEDIA_SUPPORT.md`
- Open GitHub issue with demo logs

---

**Ready to demo?** Start with:
```bash
cargo run --release
./scripts/run-all-demos.sh
```

**Happy demoing!** 🚀
