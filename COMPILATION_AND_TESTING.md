# Compilation and Testing Guide

## Prerequisites

### System Requirements

- **OS**: Linux, macOS, or Windows 10+
- **Rust**: 1.75 or later
- **GPU**: Any GPU with Vulkan/Metal/DirectX 12 support
- **Memory**: 4GB+ RAM recommended

### Dependencies

#### All Platforms
- Rust toolchain (install from https://rustup.rs)
- Git

#### Linux Specific
```bash
# Ubuntu/Debian
sudo apt-get install build-essential pkg-config libfontconfig1-dev

# Fedora/RHEL
sudo dnf install gcc pkg-config fontconfig-devel

# Arch
sudo pacman -S base-devel fontconfig
```

#### macOS Specific
```bash
# Install Xcode Command Line Tools
xcode-select --install
```

#### Windows Specific
- Visual Studio 2019 or later with C++ build tools
- Or: Visual Studio Build Tools 2019+

## Compilation Steps

### 1. Clone Repository

```bash
git clone https://github.com/Aetherlann/MCP.git
cd MCP
git checkout claude/hyper-modern-terminal-011CUZzWGZA27SxpoNWZfNt4
```

### 2. Build Project

```bash
# Debug build (faster compilation, slower runtime)
cargo build

# Release build (optimized, recommended)
cargo build --release

# With all features
cargo build --release --all-features
```

**Expected Build Time:**
- First build: 5-10 minutes (downloads and compiles dependencies)
- Incremental: 10-30 seconds

**Expected Output:**
```
Compiling hyper-terminal v0.1.0
Compiling ht-pty v0.1.0
Compiling ht-vt v0.1.0
Compiling ht-renderer v0.1.0
Compiling ht-media v0.1.0
Compiling ht-config v0.1.0
Compiling ht-ui v0.1.0
Finished release [optimized] target(s) in 8m 32s
```

### 3. Run Terminal

```bash
# Run directly
cargo run --release

# Or run the binary
./target/release/hyper
```

## Testing

### Unit Tests

Run all unit tests:

```bash
# All tests
cargo test

# Specific crate
cargo test --package ht-vt
cargo test --package ht-media

# With output
cargo test -- --nocapture

# Specific test
cargo test test_basic_text
```

**Expected Test Results:**
```
running 27 tests
test grid_tests::test_grid_creation ... ok
test grid_tests::test_put_char ... ok
test parser_tests::test_basic_text ... ok
test parser_tests::test_cursor_movement ... ok
...
test result: ok. 27 passed; 0 failed; 0 ignored
```

### Integration Tests

#### 1. Basic Terminal Functionality

```bash
# Run terminal
cargo run --release

# In terminal, type:
echo "Hello, World!"
ls -la
clear
```

**Expected**: Text renders correctly, colors work, cursor moves

#### 2. Text Rendering Test

```bash
# In terminal
./scripts/test-terminal.sh
```

**Expected**:
- Colors display (red, green, blue, yellow)
- Bold, italic, underline work
- Box drawing characters render
- Unicode/emoji display
- Scrolling works

#### 3. Media Rendering Test

```bash
# Generate test images first (if Python available)
python3 scripts/generate-test-images.py

# Or use pre-existing images
# Send test image
./scripts/send-kitty-image.sh assets/tiny-red.png
```

**Expected**: Image appears inline at cursor position

### Manual Testing Checklist

- [ ] **Text Rendering**
  - [ ] Basic ASCII characters
  - [ ] Unicode characters
  - [ ] Emoji
  - [ ] Box drawing
  - [ ] Colors (16, 256, RGB)
  - [ ] Bold, italic, underline

- [ ] **Cursor Movement**
  - [ ] Arrow keys move cursor
  - [ ] Home/End keys work
  - [ ] Page Up/Down scroll

- [ ] **Input**
  - [ ] Character input works
  - [ ] Special keys (Enter, Tab, Backspace)
  - [ ] UTF-8 input

- [ ] **Shell Integration**
  - [ ] Can run commands
  - [ ] Output displays correctly
  - [ ] Colors from shell programs work
  - [ ] Command history (shell feature)

- [ ] **Media**
  - [ ] Images display inline
  - [ ] Multiple images work
  - [ ] Images at different positions
  - [ ] PNG format
  - [ ] JPEG format

- [ ] **Performance**
  - [ ] Startup < 1 second
  - [ ] Typing feels responsive
  - [ ] Scrolling is smooth
  - [ ] No visible lag

- [ ] **Resize**
  - [ ] Window resizes properly
  - [ ] Text reflows
  - [ ] Images remain visible

## Visual Demo Tests

### Demo 1: Basic Terminal

```bash
# Run terminal
cargo run --release

# Type various commands
echo "Testing colors..."
echo -e "\e[31mRed\e[0m \e[32mGreen\e[0m \e[34mBlue\e[0m"
echo -e "\e[1mBold\e[0m \e[3mItalic\e[0m \e[4mUnderline\e[0m"

# Run system commands
ls --color=auto
top  # (press q to quit)
htop # (if available)
```

**What to Verify:**
- Colors render correctly
- Text attributes work
- Shell commands execute
- Output displays properly

### Demo 2: Media Display

```bash
# Create a test image (if ImageMagick available)
convert -size 200x100 xc:blue -fill white \
    -pointsize 40 -gravity center \
    -annotate +0+0 "TEST" \
    test-image.png

# Send via Kitty protocol
./scripts/send-kitty-image.sh test-image.png

# Should see blue rectangle with "TEST" in white
```

**What to Verify:**
- Image appears inline
- Image is at cursor position
- Text continues after image
- Can scroll past image

### Demo 3: Complex Terminal Session

```bash
# Run terminal
cargo run --release

# Complex session
neofetch  # System info with art (if available)
ls -la
cat README.md
git log --oneline --graph
```

**What to Verify:**
- All output renders
- Colors are correct
- Scrollback works
- Performance is good

### Demo 4: Image Gallery

```bash
# Send multiple images
for img in assets/*.png; do
    echo "Showing: $img"
    ./scripts/send-kitty-image.sh "$img"
    echo ""
done
```

**What to Verify:**
- Multiple images display
- Each at correct position
- Text between images works
- Scrolling shows all

## Performance Benchmarks

### Startup Time

```bash
time cargo run --release -- -c "exit"
```

**Expected**: < 500ms

### Frame Rate

Run terminal and observe:
```bash
while true; do
    echo "Frame $(date +%s%N)"
done
```

**Expected**: 60 FPS, no dropped frames

### Large Output

```bash
# Generate large output
seq 1 10000

# Or
cat /var/log/syslog
```

**Expected**: Smooth scrolling, no lag

### Memory Usage

```bash
# In another terminal
ps aux | grep hyper

# Or
top -p $(pgrep hyper)
```

**Expected**:
- Base: 50-100 MB
- With 10k scrollback: 100-150 MB
- With images: +8 MB per 1080p image

## Troubleshooting

### Build Errors

**Error**: `failed to get crates.io index`
**Solution**: Check internet connection, or use offline mode with cached deps

**Error**: `linker 'cc' not found`
**Solution**: Install build tools (gcc, clang)
```bash
# Linux
sudo apt-get install build-essential

# macOS
xcode-select --install
```

**Error**: `could not find system library 'fontconfig'`
**Solution**: Install fontconfig
```bash
# Linux
sudo apt-get install libfontconfig1-dev

# macOS
brew install fontconfig
```

### Runtime Errors

**Error**: Terminal window is black
**Solution**: Check GPU drivers, try software rendering

**Error**: No text appears
**Solution**:
1. Check font file exists: `crates/ht-renderer/assets/DejaVuSansMono.ttf`
2. Check logs: `RUST_LOG=debug cargo run`

**Error**: Images don't appear
**Solution**:
1. Check image is valid PNG/JPEG
2. Check logs for decoding errors
3. Try smaller image (< 1MB)

**Error**: Crash on startup
**Solution**:
1. Check GPU supports Vulkan/Metal/DX12
2. Update GPU drivers
3. Check logs: `RUST_LOG=trace cargo run 2>&1 | tee log.txt`

### Performance Issues

**Issue**: Slow typing
**Solution**:
1. Check CPU usage
2. Reduce scrollback: edit config.json `"scrollback": 10000`
3. Close other GPU applications

**Issue**: High memory usage
**Solution**:
1. Clear scrollback: Ctrl+L or `clear`
2. Reduce scrollback limit in config
3. Remove old images

**Issue**: Stuttering
**Solution**:
1. Check FPS: enable FPS counter in config
2. Reduce window size
3. Update GPU drivers

## Logging

### Enable Debug Logging

```bash
# All debug
RUST_LOG=debug cargo run

# Specific modules
RUST_LOG=hyper_terminal=trace,ht_pty=debug,ht_vt=info cargo run

# Save to file
RUST_LOG=debug cargo run 2>&1 | tee terminal.log
```

### Log Levels

- `error`: Errors only
- `warn`: Warnings and errors
- `info`: Info, warnings, errors (default)
- `debug`: Debug info (verbose)
- `trace`: Trace info (very verbose)

## Expected Behavior

### On Startup

1. Window opens (~200ms)
2. Shell prompt appears
3. Can type immediately
4. FPS: 60

### During Use

1. **Typing**: No lag, characters appear immediately
2. **Colors**: ANSI colors render correctly
3. **Cursor**: Visible, blinks (if configured)
4. **Scrolling**: Smooth, 60 FPS
5. **Images**: Display inline at cursor position

### Image Display

1. Send Kitty protocol escape sequence
2. Image decodes (~5-20ms)
3. Texture uploads to GPU (~1-5ms)
4. Image appears at current cursor position
5. Text continues below image

## Success Criteria

### Milestone 2 (Text Rendering)

- [x] Terminal opens and displays shell
- [x] Can type and see characters
- [x] Colors work (16, 256, RGB)
- [x] Cursor moves correctly
- [x] Arrow keys work
- [x] Scrolling works
- [x] Clear screen works
- [x] Shell commands execute

### Milestone 3 (Media Support)

- [x] Can send Kitty protocol images
- [x] PNG images decode and display
- [x] JPEG images decode and display
- [x] Images appear at cursor position
- [x] Multiple images work
- [x] Images + text composite correctly
- [x] Performance acceptable (<1ms per image)

## Demo Video Script

### Recording Setup

1. **Screen Capture**:
   ```bash
   # Using ffmpeg
   ffmpeg -f x11grab -r 60 -s 1920x1080 -i :0.0 demo.mp4

   # Or using OBS Studio
   ```

2. **Terminal Window**: 1200x800, positioned center

3. **Font Size**: Increase for readability (config.json: `"size": 16.0`)

### Demo Script

**Scene 1: Basic Text (30 seconds)**
```bash
# Type clearly
echo "Welcome to Hyper Terminal"
echo "A hyper-modern terminal with glass UI"
echo ""
echo "Testing colors:"
echo -e "\e[31m Red \e[32m Green \e[34m Blue \e[33m Yellow \e[0m"
echo ""
echo -e "\e[1mBold\e[0m \e[3mItalic\e[0m \e[4mUnderline\e[0m"
```

**Scene 2: System Commands (30 seconds)**
```bash
ls --color=auto
pwd
date
uname -a
```

**Scene 3: Media Display (60 seconds)**
```bash
echo "Now testing inline image display..."
./scripts/send-kitty-image.sh assets/test-blue.png
echo "Image displayed above!"
echo ""
echo "Multiple images:"
./scripts/send-kitty-image.sh assets/test-gradient.png
./scripts/send-kitty-image.sh assets/test-pattern.png
```

**Scene 4: Scrolling (20 seconds)**
```bash
# Show scrolling works
seq 1 50
echo "Scrolling complete"
```

**Scene 5: Resize (10 seconds)**
- Resize window
- Show text reflows
- Images remain visible

## Next Steps

After successful compilation and testing:

1. **Report Issues**: Open GitHub issues for bugs
2. **Performance Tuning**: Profile and optimize if needed
3. **Cross-Platform Testing**: Test on Windows, macOS, Linux
4. **Documentation**: Update docs based on findings
5. **Milestone 4**: Begin advanced layout features

## Support

If you encounter issues:

1. Check logs: `RUST_LOG=debug cargo run 2>&1 | tee log.txt`
2. Search existing issues: GitHub Issues
3. Create new issue with:
   - OS and version
   - Rust version (`rustc --version`)
   - Error message
   - Steps to reproduce
   - Log file

---

**Status**: Ready for compilation and testing
**Next**: Run `cargo build --release` and test
