# Documentation Index

Complete guide to all Hyper Terminal documentation organized by topic.

## 🚀 Quick Start

**New users start here:**

1. **[README.md](README.md)** - Project overview, features, and quick start
2. **[BUILD.md](BUILD.md)** - Complete build and installation guide
3. **[DEMO_QUICKSTART.md](DEMO_QUICKSTART.md)** - Run your first demos

**Quick reference:**
```bash
# Build the project
cargo build --release

# Run the terminal
cargo run --release

# Try the demo launcher (inside terminal)
./scripts/demo-launcher.sh
```

## 📚 Documentation by Topic

### Getting Started

| Document | Description |
|----------|-------------|
| [README.md](README.md) | Main project overview with features and highlights |
| [BUILD.md](BUILD.md) | Complete build instructions for all platforms |
| [DEMO_QUICKSTART.md](DEMO_QUICKSTART.md) | Quick reference for running demos |
| [VISUAL_DEMOS.md](VISUAL_DEMOS.md) | Visual demo guide with screen recording |

### Gallery System (NEW!)

The world-class gallery system for LLM-optimized media display:

| Document | Description |
|----------|-------------|
| [GALLERY_COMPLETE.md](GALLERY_COMPLETE.md) | ⭐ Complete integration summary and status |
| [GALLERY_VISION.md](GALLERY_VISION.md) | Original design vision and philosophy |
| [LLM_GALLERY_PROTOCOL.md](LLM_GALLERY_PROTOCOL.md) | OSC 1338 protocol specification |
| [GALLERY_INTEGRATION_COMPLETE.md](GALLERY_INTEGRATION_COMPLETE.md) | Technical integration details |
| [GALLERY_IMPLEMENTATION_SUMMARY.md](GALLERY_IMPLEMENTATION_SUMMARY.md) | Implementation summary and code metrics |

**Gallery Quick Links:**
- **Display Modes**: Grid, Masonry, Filmstrip, Comparison, Deck, FileExplorer, Auto
- **Protocol**: OSC 1338 escape sequences
- **Demos**: `./scripts/demo-gallery-*.sh`

### Architecture and Technical Details

| Document | Description |
|----------|-------------|
| [ARCHITECTURE.md](ARCHITECTURE.md) | Detailed architecture and design decisions |
| [M3_MEDIA_SUPPORT.md](M3_MEDIA_SUPPORT.md) | Media rendering technical details |
| [COMPILATION_AND_TESTING.md](COMPILATION_AND_TESTING.md) | Build system and testing guide |
| [TESTING.md](TESTING.md) | Unit and integration testing documentation |

### Development History

| Document | Description |
|----------|-------------|
| [PHASE2_SUMMARY.md](PHASE2_SUMMARY.md) | Milestone 2: VT Parity completion |
| [PHASE3_SUMMARY.md](PHASE3_SUMMARY.md) | Milestone 3: Media Support completion |
| [PHASE4_DEMO_COMPLETION.md](PHASE4_DEMO_COMPLETION.md) | Demo infrastructure completion |

### Configuration

| Document | Description |
|----------|-------------|
| [config.example.json](config.example.json) | Example configuration file |

**Config locations:**
- Windows: `%APPDATA%\HyperTerminal\config.json`
- macOS: `~/Library/Application Support/HyperTerminal/config.json`
- Linux: `~/.config/HyperTerminal/config.json`

### Demo Scripts

Located in `scripts/` directory:

| Script | Description |
|--------|-------------|
| `demo-launcher.sh` | ⭐ Interactive demo menu launcher |
| `demo-gallery-grid.sh` | Simple 3-image grid gallery |
| `demo-gallery-comparison.sh` | Before/after comparison gallery |
| `demo-gallery-all-modes.sh` | All 5 gallery modes demonstration |
| `demo-images.sh` | Basic image display (Kitty/iTerm2) |
| `demo-multiple-images.sh` | Multiple simultaneous images |
| `demo-large-image.sh` | High-resolution image testing |
| `demo-colors.sh` | Color and text attributes |
| `demo-unicode.sh` | Unicode, emoji, box drawing |
| `run-all-demos.sh` | Run all demos in sequence |

## 📖 Documentation by User Type

### For New Users

Start with these in order:

1. [README.md](README.md) - Understand what Hyper Terminal is
2. [BUILD.md](BUILD.md) - Build and install it
3. [DEMO_QUICKSTART.md](DEMO_QUICKSTART.md) - Try the features
4. [GALLERY_COMPLETE.md](GALLERY_COMPLETE.md) - Learn about the gallery system

### For Developers

Technical documentation for contributors:

1. [ARCHITECTURE.md](ARCHITECTURE.md) - Understand the code structure
2. [COMPILATION_AND_TESTING.md](COMPILATION_AND_TESTING.md) - Set up development environment
3. [TESTING.md](TESTING.md) - Write and run tests
4. [M3_MEDIA_SUPPORT.md](M3_MEDIA_SUPPORT.md) - Media rendering internals
5. [GALLERY_INTEGRATION_COMPLETE.md](GALLERY_INTEGRATION_COMPLETE.md) - Gallery system internals

### For LLM/AI Integration

Documentation for AI assistants integrating with Hyper Terminal:

1. [LLM_GALLERY_PROTOCOL.md](LLM_GALLERY_PROTOCOL.md) - OSC 1338 protocol specification
2. [GALLERY_VISION.md](GALLERY_VISION.md) - Design philosophy for LLM usage
3. [GALLERY_COMPLETE.md](GALLERY_COMPLETE.md) - Complete feature reference
4. Demo scripts in `scripts/` - Working examples to learn from

### For Visual Demos

Create recordings and showcase the terminal:

1. [VISUAL_DEMOS.md](VISUAL_DEMOS.md) - Screen recording guide
2. [DEMO_QUICKSTART.md](DEMO_QUICKSTART.md) - Quick demo reference
3. `scripts/demo-launcher.sh` - Interactive demo selector

## 🎯 Common Tasks

### I want to...

#### Build the project
→ See [BUILD.md](BUILD.md)

#### Run demos
→ See [DEMO_QUICKSTART.md](DEMO_QUICKSTART.md) or run `./scripts/demo-launcher.sh`

#### Create galleries
→ See [LLM_GALLERY_PROTOCOL.md](LLM_GALLERY_PROTOCOL.md) for protocol details

#### Understand the code
→ See [ARCHITECTURE.md](ARCHITECTURE.md)

#### Write tests
→ See [TESTING.md](TESTING.md)

#### Configure the terminal
→ See [config.example.json](config.example.json) and README.md Configuration section

#### Troubleshoot build issues
→ See [BUILD.md](BUILD.md) Troubleshooting section

#### Record a demo video
→ See [VISUAL_DEMOS.md](VISUAL_DEMOS.md)

## 📊 Project Statistics

### Code Metrics
- **Total crates**: 8
- **Gallery system**: ~4,000 lines (implementation + docs)
- **Unit tests**: 27+ tests
- **Demo scripts**: 10 scripts
- **Documentation**: 15+ markdown files

### Milestones Completed
- ✅ M1: Minimal Terminal
- ✅ M2: VT Parity
- ✅ M3: Media Support
- ✅ M3.5: Gallery System ⭐ NEW

### Next Milestones
- 🔄 M4: Advanced Layout (tabs, splits, columns)
- 🔄 M5: Polish (video, audio, search, hyperlinks)

## 🏗️ Project Structure

```
MCP/
├── crates/                 # Rust workspace crates
│   ├── hyper-terminal/    # Main binary
│   ├── ht-pty/            # PTY layer (ConPTY/openpty)
│   ├── ht-vt/             # VT/ANSI parser
│   ├── ht-renderer/       # GPU text rendering
│   ├── ht-media/          # Media decoding
│   ├── ht-gallery/        # Gallery system ⭐ NEW
│   ├── ht-config/         # Configuration
│   └── ht-ui/             # Window & glass effects
│
├── scripts/               # Demo scripts
│   ├── demo-launcher.sh   # Interactive menu ⭐ NEW
│   ├── demo-gallery-*.sh  # Gallery demos ⭐ NEW
│   └── demo-*.sh          # Feature demos
│
├── assets/                # Demo assets (images, etc.)
│
├── README.md              # Main documentation
├── BUILD.md               # Build guide ⭐ NEW
├── DOCS.md                # This file ⭐ NEW
├── GALLERY_*.md           # Gallery documentation ⭐ NEW
└── *.md                   # Additional documentation
```

## 🔗 External Links

- **Repository**: https://github.com/Aetherlann/MCP
- **Issues**: https://github.com/Aetherlann/MCP/issues
- **Rust**: https://www.rust-lang.org/
- **wgpu**: https://wgpu.rs/
- **Kitty Protocol**: https://sw.kovidgoyal.net/kitty/graphics-protocol/
- **iTerm2 Images**: https://iterm2.com/documentation-images.html

## ❓ Getting Help

1. **Check documentation**: Search this index for relevant docs
2. **Search issues**: https://github.com/Aetherlann/MCP/issues
3. **Open new issue**: Include OS, Rust version, error message

## 🎨 Features Overview

### Core Terminal
- Full VT/ANSI escape sequence support
- True color (24-bit RGB)
- Unicode and emoji
- 120k+ line scrollback
- Hyperlinks (OSC 8)
- Clipboard (OSC 52)

### Graphics
- Kitty Graphics Protocol
- iTerm2 Inline Images
- Multiple simultaneous images
- PNG, JPEG, GIF, WebP support

### Gallery System ⭐
- 7 display modes
- Rich metadata
- Auto-detection
- Mouse/keyboard navigation
- Smooth animations
- OSC 1338 protocol

### UI
- Glass effects (Acrylic/NSVisualEffectView)
- GPU-accelerated rendering
- 60+ FPS
- <10ms input latency

## 📝 Version Information

- **Version**: 0.1.0
- **Rust Edition**: 2021
- **Minimum Rust**: 1.75+
- **Branch**: claude/hyper-modern-terminal-011CUZzWGZA27SxpoNWZfNt4

## 🚀 Quick Commands Reference

```bash
# Build
cargo build --release

# Run
cargo run --release

# Test
cargo test --all

# Run specific demo
./scripts/demo-gallery-grid.sh

# Run demo launcher (interactive)
./scripts/demo-launcher.sh

# Check code
cargo check

# Format code
cargo fmt

# Run lints
cargo clippy --all-targets --all-features
```

## 📅 Last Updated

This documentation index was last updated with the completion of the Gallery System (Milestone 3.5).

---

**Ready to get started?** → [BUILD.md](BUILD.md)

**Want to see it in action?** → Run `./scripts/demo-launcher.sh` inside the terminal

**Need help?** → Check the relevant docs above or open an issue on GitHub
