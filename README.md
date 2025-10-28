# Hyper Terminal

A hyper-modern, GPU-accelerated terminal with glass UI aesthetics and native media rendering capabilities. Built in Rust for maximum performance and safety.

## Features

### Glass Aesthetic
- **Windows**: Acrylic/Mica effects with backdrop blur, noise, and tint
- **macOS**: NSVisualEffectView vibrancy with material selection
- **Linux**: Compositor-aware blur (KWin/Mutter)
- Adjustable opacity, blur radius, tint, and transparency settings

### Native Media Support
Display images, videos, audio, SVG, PDF, and 3D models directly in your terminal:
- **Image formats**: PNG, JPEG, GIF, WEBP
- **Video**: MP4, WebM, H.264/H.265 (with hardware decode)
- **SVG**: Vector graphics with full rendering
- **PDF**: Page previews and inline viewing
- **3D**: glTF/GLB with orbit controls

### Graphics Protocols
- Kitty Graphics Protocol
- iTerm2 Inline Images
- Sixel (coming soon)

### Modern Terminal Features
- Full VT/ANSI escape sequence support
- True color (24-bit RGB)
- Unicode support with proper width handling
- Hyperlinks (OSC 8)
- Clipboard integration (OSC 52)
- 120k+ line scrollback
- Multiple profiles

### Performance
- GPU-accelerated text rendering via wgpu
- Cross-platform (Windows, macOS, Linux)
- Low latency input (<10ms target)
- 60+ FPS rendering
- Efficient dirty-rect batching

## Architecture

The project is organized as a Rust workspace with modular crates:

```
hyper-terminal/          # Main binary
├── ht-pty/             # PTY/ConPTY host layer
├── ht-vt/              # VT/ANSI parser & terminal state
├── ht-renderer/        # GPU text & media rendering
├── ht-media/           # Media decoding & composition
├── ht-config/          # Configuration management
└── ht-ui/              # Window management & glass effects
```

### Data Flow

```
Shell (cmd/bash/zsh)
  ↓
PTY/ConPTY (ht-pty)
  ↓
VT Parser (ht-vt) → Graphics Protocol Decoder
  ↓                          ↓
Terminal Grid            Media Manager (ht-media)
  ↓                          ↓
GPU Renderer (ht-renderer) ←─┘
  ↓
Window (ht-ui)
```

## Building

### Prerequisites

- Rust 1.75+ (stable)
- Platform-specific dependencies:
  - **Windows**: Windows 10+ (ConPTY support)
  - **macOS**: macOS 10.15+
  - **Linux**: GTK3/Qt, working compositor

### Build Commands

```bash
# Clone the repository
git clone https://github.com/Aetherlann/MCP.git
cd MCP

# Build debug version
cargo build

# Build release version (optimized)
cargo build --release

# Run
cargo run --release
```

### Optional Features

```bash
# Build with video support (requires FFmpeg)
cargo build --release --features video

# Build with all features
cargo build --release --all-features
```

## Configuration

Configuration is stored in JSON format at:
- **Windows**: `%APPDATA%\HyperTerminal\config.json`
- **macOS**: `~/Library/Application Support/HyperTerminal/config.json`
- **Linux**: `~/.config/HyperTerminal/config.json`

See `config.example.json` for a full example configuration.

### Example Configuration

```json
{
  "appearance": {
    "theme": "Everglass Dark",
    "glass": {
      "enabled": true,
      "opacity": 0.8,
      "blur_radius": 24,
      "tint": "#0b0f19"
    },
    "font": {
      "family": "Cascadia Code",
      "size": 13.5,
      "ligatures": true
    }
  },
  "profiles": [
    {
      "name": "PowerShell",
      "shell": "pwsh.exe",
      "args": [],
      "env": [["TERM", "xterm-256color"]]
    }
  ]
}
```

## Usage

### Displaying Images

Using the Kitty graphics protocol:

```bash
# Using kitty's icat (if installed)
kitty +kitten icat image.png

# Or send raw escape sequences
printf '\e_Gf=100,a=T,t=d;%s\e\\' "$(base64 < image.png)"
```

Using iTerm2 inline images:

```bash
printf '\e]1337;File=inline=1:%s\a' "$(base64 < image.png)"
```

### Shell Integration

Add to your shell profile for best experience:

**PowerShell** (`$PROFILE`):
```powershell
$env:TERM = "xterm-256color"
```

**Bash/Zsh** (`~/.bashrc` or `~/.zshrc`):
```bash
export TERM=xterm-256color
```

## Development Roadmap

### Milestone 1: Minimal Terminal ✅
- [x] PTY host (ConPTY/openpty)
- [x] Basic window & text rendering
- [x] VT parser foundation
- [x] Configuration system

### Milestone 2: VT Parity (In Progress)
- [x] Full SGR support
- [x] Cursor control
- [x] Glass UI foundation
- [ ] Complete text rendering pipeline
- [ ] Tabs and splits

### Milestone 3: Media Support (Planned)
- [x] Kitty & iTerm2 protocol parsing
- [ ] Image rendering (PNG/JPEG/WEBP)
- [ ] Video playback with audio
- [ ] Side-panel docking

### Milestone 4: Advanced Layout (Planned)
- [ ] Columnar text mode
- [ ] Pane tiling manager
- [ ] Layout persistence
- [ ] Workspace management

### Milestone 5: Polish (Planned)
- [ ] Keybinding editor
- [ ] Find-in-scrollback
- [ ] Hyperlink handling
- [ ] Hover previews
- [ ] Performance optimization

## Contributing

Contributions are welcome! Areas that need help:

1. **Text Rendering**: Complete GPU text pipeline with DirectWrite/CoreText/HarfBuzz
2. **Media Decoders**: FFmpeg integration for video/audio
3. **Glass Effects**: Platform-specific improvements (especially Linux compositor hints)
4. **Performance**: Profiling and optimization
5. **Testing**: Cross-platform testing and vttest compliance

## Technical Details

### PTY Layer (`ht-pty`)
- **Windows**: Uses ConPTY API (`CreatePseudoConsole`)
- **POSIX**: Uses `forkpty`/`openpty`
- Async I/O via Tokio
- Resize propagation, UTF-8 support

### VT Parser (`ht-vt`)
- State machine-based parser
- ECMA-48 compliant
- SGR (colors, attributes), cursor control, screen manipulation
- Graphics protocol extraction (Kitty/iTerm2)
- OSC sequences (hyperlinks, clipboard)

### Renderer (`ht-renderer`)
- wgpu for cross-platform GPU access
- Glyph atlas caching
- Dirty-rect optimization
- Sub-pixel positioning (planned)

### Media (`ht-media`)
- Image decoding via `image` crate
- SVG rendering via `resvg`
- Video/audio via FFmpeg (optional)
- Platform decoders (WMF/AVFoundation/VAAPI) planned

### UI (`ht-ui`)
- winit for windowing
- Platform-specific glass effects
- Transparent window support

## Performance Targets

- **Input latency**: <10ms
- **Frame rate**: 60+ FPS (120 FPS capable)
- **Startup time**: <150ms on SSD
- **Memory**: <100MB base, scales with scrollback
- **Scrollback**: 120k lines default (configurable)

## License

MIT License - see LICENSE file for details

## Acknowledgments

Built with inspiration from:
- **Alacritty**: GPU-accelerated terminal
- **Kitty**: Graphics protocol design
- **Windows Terminal**: ConPTY and modern UX
- **iTerm2**: Inline images and shell integration

## Links

- [Repository](https://github.com/Aetherlann/MCP)
- [Issues](https://github.com/Aetherlann/MCP/issues)
- [Kitty Graphics Protocol](https://sw.kovidgoyal.net/kitty/graphics-protocol/)
- [iTerm2 Inline Images](https://iterm2.com/documentation-images.html)

---

**Status**: Alpha - Core functionality implemented, many features in progress.

For questions or feedback, please open an issue on GitHub.