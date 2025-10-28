# Hyper Terminal - Demo Quick Start

## 🚀 Fastest Path to Demo

```bash
# 1. Build (first time only, takes 5-10 minutes)
cargo build --release

# 2. Run terminal
cargo run --release

# 3. Inside terminal, run comprehensive demo:
./scripts/run-all-demos.sh
```

That's it! The demo will showcase all features implemented in M1-M3.

---

## 📋 Quick Command Reference

### Build & Run

```bash
cargo build --release          # Build (5-10 min first time)
cargo run --release            # Run terminal
./target/release/hyper         # Or run binary directly
```

### Run Demos

```bash
./scripts/run-all-demos.sh              # All demos (interactive)
./scripts/run-all-demos.sh --auto      # All demos (no pauses)
./scripts/demo-basic.sh                # Basic features only
./scripts/demo-media.sh                # Media rendering only
./scripts/demo-performance.sh          # Performance tests only
```

### Test Custom Images

```bash
./scripts/send-kitty-image.sh image.png     # Display any image
./scripts/send-kitty-image.sh photo.jpg     # Works with JPEG too
```

### Generate Test Images

```bash
python3 scripts/generate-test-images.py     # If Pillow installed
# OR
convert -size 200x200 xc:blue test.png      # If ImageMagick installed
```

---

## ✨ What Will You See?

### Demo 1: Basic Terminal (2 min)
- ✅ GPU-accelerated text rendering
- ✅ ANSI colors (16, 256, RGB)
- ✅ Text attributes (bold, italic, underline)
- ✅ Unicode & emoji
- ✅ Box drawing characters
- ✅ Cursor control

### Demo 2: Media Rendering (3 min)
- ✅ Kitty Graphics Protocol
- ✅ Inline images at cursor
- ✅ Multiple simultaneous images
- ✅ Text + image composition
- ✅ Scrolling behavior

### Demo 3: Performance (2-3 min)
- ✅ 1000 lines rapid output
- ✅ Large scrollback (5000 lines)
- ✅ Color-intensive rendering
- ✅ Complex Unicode
- ✅ Clear/redraw performance

**Total Time**: ~5-10 minutes for all demos

---

## 🎬 Recording a Demo Video

### Simple Recording

```bash
# Linux with ffmpeg
ffmpeg -f x11grab -r 60 -s 1920x1080 -i :0.0 demo.mp4 &
cargo run --release
# Run demos...
pkill ffmpeg

# macOS with QuickTime
# 1. Open QuickTime Player
# 2. File → New Screen Recording
# 3. Run terminal and demos
# 4. Stop recording

# Windows with Game Bar
# 1. Press Win+G
# 2. Click record button
# 3. Run terminal and demos
# 4. Win+Alt+R to stop
```

---

## 🐛 Troubleshooting

### Build Fails

```bash
# Check Rust version (need 1.75+)
rustc --version

# Install/update Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install platform dependencies (Linux)
sudo apt-get install build-essential pkg-config libfontconfig1-dev
```

### Scripts Won't Run

```bash
# Make executable
chmod +x scripts/*.sh

# Run from repo root
cd /path/to/MCP
./scripts/demo-basic.sh
```

### No Images Appear

```bash
# Check test image exists
ls -l assets/tiny-red.png

# Create if missing
echo "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg==" | base64 -d > assets/tiny-red.png

# Check logs
RUST_LOG=debug cargo run
```

---

## 📚 Full Documentation

- **`VISUAL_DEMOS.md`**: Complete demo guide with recording tips
- **`COMPILATION_AND_TESTING.md`**: Build instructions & troubleshooting
- **`scripts/README.md`**: Detailed script documentation
- **`TESTING.md`**: Unit and integration tests
- **`M3_MEDIA_SUPPORT.md`**: Media implementation details

---

## 🎯 Success Criteria

After running demos, you should see:

✅ **Text**: All colors, styles, and Unicode render correctly
✅ **Images**: Display inline at cursor position
✅ **Performance**: Smooth 60 FPS, no lag
✅ **Shell**: Commands execute and display properly
✅ **Cursor**: Moves precisely to correct positions
✅ **Scrolling**: Smooth with all content visible

---

## 🚦 Next Steps

1. ✅ Run demos (you're here!)
2. 📊 Test with your own images
3. 🐛 Report any issues on GitHub
4. ⭐ Star the repository
5. 🔄 Share your experience

---

## 💡 Pro Tips

**Best Terminal Size**: 1200x800 or 1920x1080
**Best Font Size**: 14-16pt for demos
**Best Recording FPS**: 60 FPS for smooth playback
**Best Image Formats**: PNG, JPEG (GIF, WebP also supported)

---

## 🆘 Need Help?

1. Check `COMPILATION_AND_TESTING.md`
2. Review error logs: `RUST_LOG=debug cargo run 2>&1 | tee log.txt`
3. Search existing issues: https://github.com/Aetherlann/MCP/issues
4. Open new issue with:
   - OS and version
   - Rust version
   - Error message
   - Steps to reproduce

---

**Ready?** Let's go! 🚀

```bash
cargo run --release
./scripts/run-all-demos.sh
```

---

*Hyper Terminal - A hyper-modern, GPU-accelerated terminal with glass UI and native media support*
