# Hyper Terminal Architecture

This document describes the internal architecture and design decisions of Hyper Terminal.

## Overview

Hyper Terminal is built as a modular Rust workspace with clear separation of concerns. Each crate handles a specific aspect of terminal functionality.

## Crate Structure

### `hyper-terminal` (Binary)
**Purpose**: Main application entry point and event loop coordination

**Responsibilities**:
- Initialize logging and configuration
- Create window and event loop
- Coordinate between all subsystems
- Handle user input and window events
- Drive the render loop

**Key Types**:
- `main()`: Application entry point
- `Terminal`: Main coordinator struct

### `ht-pty` (PTY Host)
**Purpose**: Cross-platform PTY/ConPTY abstraction

**Responsibilities**:
- Spawn shell processes with PTY
- Handle I/O to/from shell
- Propagate resize events
- Process lifecycle management

**Platform Implementations**:
- **Windows**: `conpty.rs` - Uses Win32 ConPTY APIs
- **POSIX**: `openpty.rs` - Uses `forkpty`/`openpty`

**Key Types**:
- `Pty` trait: Platform-agnostic interface
- `PtySize`: Terminal dimensions
- `PtyEvent`: Data and exit events

**Design Notes**:
- Async-first design using Tokio
- Non-blocking I/O
- Automatic cleanup on drop

### `ht-vt` (VT Parser)
**Purpose**: Parse VT/ANSI escape sequences and maintain terminal state

**Responsibilities**:
- Tokenize byte stream into VT commands
- Parse SGR, cursor, erase sequences
- Extract graphics protocol commands
- Maintain terminal grid state

**Key Types**:
- `VtParser`: State machine parser
- `VtToken`: Parsed command enum
- `Grid`: Terminal cell buffer
- `GraphicsCommand`: Kitty/iTerm2/Sixel

**State Machine**:
```
Ground → Escape → CSI → Parameters → Execute
             ↓
            OSC → String → Handle
             ↓
            DCS → String → Handle (Graphics)
```

**Graphics Protocols**:
1. **Kitty Graphics**: `ESC_G<params>;<base64>ESC\`
2. **iTerm2 Inline**: `OSC 1337;File=<params>:<base64> BEL`
3. **Sixel**: DCS-based (planned)

**Grid Management**:
- Cell buffer with attributes (color, bold, etc.)
- Scrollback ring buffer
- Cursor tracking
- Wide character support

### `ht-renderer` (GPU Renderer)
**Purpose**: GPU-accelerated rendering of text and media

**Responsibilities**:
- wgpu setup and management
- Text glyph atlas
- Render pass orchestration
- Frame presentation

**Key Types**:
- `Renderer`: Main render coordinator
- `TextRenderer`: Text-specific rendering
- `GlyphCache`: Font rasterization cache

**Rendering Pipeline**:
```
Grid → Text Renderer → GPU Commands
                         ↓
Media → Media Renderer → GPU Commands
                         ↓
                    Frame Buffer → Present
```

**Optimization Strategies**:
- Glyph atlas caching
- Dirty-rect tracking
- Batch drawing
- Sub-pixel positioning (planned)

### `ht-media` (Media Subsystem)
**Purpose**: Decode and manage media surfaces

**Responsibilities**:
- Image decoding (PNG/JPEG/WEBP/SVG)
- Video decoding (FFmpeg/platform decoders)
- Media surface placement
- Playback control

**Key Types**:
- `MediaManager`: Surface coordinator
- `MediaSurface`: Individual media instance
- `SurfacePlacement`: Inline/Overlay/Docked

**Decoder Backends**:
- **Images**: `image` crate, `webp`, `resvg` for SVG
- **Video**: FFmpeg (optional), WMF/AVFoundation/VAAPI (planned)
- **PDF**: pdfium (planned)
- **3D**: tinygltf (planned)

**Surface Types**:
1. **Inline**: Anchored to grid cells, scrolls with text
2. **Overlay**: Fixed position within pane
3. **Docked**: Side panel with optional text sync

### `ht-config` (Configuration)
**Purpose**: Configuration loading, validation, and management

**Responsibilities**:
- JSON config loading/saving
- Schema validation
- Default values
- Live reload (planned)

**Key Types**:
- `Config`: Root configuration
- `AppearanceConfig`, `ProfileConfig`, etc.

**Storage Locations**:
- Windows: `%APPDATA%\HyperTerminal\config.json`
- macOS: `~/Library/Application Support/HyperTerminal/config.json`
- Linux: `~/.config/HyperTerminal/config.json`

### `ht-ui` (UI & Windowing)
**Purpose**: Window management and platform-specific effects

**Responsibilities**:
- Window creation (winit)
- Glass/blur effects
- Platform integration
- Input handling helpers

**Key Types**:
- `WindowManager`: Window state
- `GlassEffect`: Platform blur effects

**Glass Implementation**:
- **Windows**: DWM blur + acrylic (via undocumented APIs)
- **macOS**: NSVisualEffectView
- **Linux**: Compositor hints (_NET_WM_BLUR_BEHIND)

## Data Flow

### Startup Sequence
1. Load configuration
2. Create event loop and window
3. Apply glass effects
4. Initialize renderer (wgpu)
5. Spawn PTY with shell
6. Enter event loop

### Input Flow
```
User Input
  ↓
winit KeyboardInput
  ↓
Terminal::handle_key()
  ↓
Key → Escape Sequence
  ↓
Pty::write()
  ↓
Shell
```

### Output Flow
```
Shell Output
  ↓
PTY read
  ↓
VtParser::parse()
  ↓
VtToken stream
  ↓
Grid update / Media command
  ↓
Renderer::render()
  ↓
GPU → Screen
```

### Graphics Protocol Flow
```
Shell emits: ESC_G...
  ↓
PTY receives bytes
  ↓
VtParser extracts DCS/OSC
  ↓
GraphicsCommand parsed
  ↓
MediaManager::handle_graphics()
  ↓
Decode image/video
  ↓
Create MediaSurface
  ↓
Render in next frame
```

## Concurrency Model

### Async Runtime
- Tokio for async I/O
- PTY read/write on async tasks
- File I/O for config

### Threading
- Main thread: Event loop, rendering
- Tokio thread pool: PTY I/O, async tasks
- Spawn blocking: Heavy media decoding

### Synchronization
- No shared mutable state between threads
- Message passing for PTY events (planned: channels)
- Render state owned by main thread

## Performance Optimizations

### Text Rendering
1. **Glyph Atlas**: Pre-rasterize glyphs to GPU texture
2. **Dirty Rects**: Only update changed regions
3. **Batching**: Single draw call per frame
4. **Instancing**: GPU instancing for repeated glyphs

### Media Rendering
1. **Hardware Decode**: Use GPU video decoders
2. **Texture Streaming**: Upload frames as textures
3. **Async Decode**: Decode on background thread

### Parser
1. **Zero-copy**: Parse in-place where possible
2. **State machine**: No allocation for common paths
3. **Incremental**: Process byte-by-byte

## Extension Points

### Custom Protocols
Add new graphics protocols by:
1. Parse in `VtParser::handle_dcs()` or `handle_osc()`
2. Add variant to `GraphicsCommand`
3. Handle in `MediaManager::handle_graphics()`

### Custom Decoders
Register media decoders:
1. Implement `Decoder` trait
2. Register with `MediaManager`
3. Handle new MIME types

### Themes
Theme system (planned):
1. Define color schemes in config
2. Load from separate JSON files
3. Hot-reload support

## Testing Strategy

### Unit Tests
- `ht-vt`: Parser correctness, escape sequence handling
- `ht-pty`: Mock PTY for I/O tests
- `ht-config`: Config validation

### Integration Tests
- Full terminal with mock PTY
- Graphics protocol end-to-end
- Resize handling

### Acceptance Tests
- vttest compatibility
- Real shell integration
- Performance benchmarks

## Future Architecture Changes

### Multi-pane Support
- Split current `Terminal` into `Pane`
- `TerminalWindow` manages multiple `Pane`s
- Each pane has own PTY, grid, parser

### Layout Engine
- Binary split tree for panes
- Percentage-based sizing
- Drag-to-resize
- Serialize/deserialize layouts

### Command Palette
- Fuzzy search command registry
- Keybinding system
- Plugin commands

### Shell Integration
- Prompt marking (OSC 133)
- Semantic prompt/command/output
- Jump to prompt
- Command history

## Platform Considerations

### Windows
- ConPTY requires Windows 10+
- DirectWrite for text
- DWM for glass effects
- Windows Media Foundation for video

### macOS
- CoreText for text
- NSVisualEffectView for glass
- AVFoundation for video
- Metal backend for wgpu

### Linux
- HarfBuzz/Freetype for text
- Compositor-dependent blur
- VAAPI for video
- Vulkan/OpenGL backend for wgpu

## Dependencies

### Core
- `tokio`: Async runtime
- `wgpu`: GPU abstraction
- `winit`: Windowing
- `serde`/`serde_json`: Serialization

### Platform-Specific
- `windows`: Win32 APIs (Windows)
- `nix`: POSIX APIs (Unix)
- `core-text`: Text rendering (macOS)

### Media
- `image`: Image decoding
- `webp`: WebP support
- `resvg`: SVG rendering
- `ffmpeg-next`: Video (optional)

## Build Configuration

### Features
- `default`: Basic terminal + images
- `svg`: SVG rendering
- `video`: Video/audio playback
- `pdf`: PDF viewing

### Profiles
- `dev`: Fast compile, minimal optimization
- `release`: Full optimization, LTO, strip

### Cross-compilation
- Windows: MSVC toolchain required for `windows-rs`
- macOS: Xcode command-line tools
- Linux: Standard GCC/Clang

---

For questions about architecture, open an issue or discussion on GitHub.
