# Build and Installation Guide

Complete guide to building and installing Hyper Terminal on all supported platforms.

## Table of Contents
- [Quick Build](#quick-build)
- [Prerequisites by Platform](#prerequisites-by-platform)
- [Building from Source](#building-from-source)
- [Running the Terminal](#running-the-terminal)
- [Testing the Build](#testing-the-build)
- [Troubleshooting](#troubleshooting)
- [Development Build](#development-build)

## Quick Build

If you just want to get started quickly:

```bash
# Clone the repository
git clone https://github.com/Aetherlann/MCP.git
cd MCP
git checkout claude/hyper-modern-terminal-011CUZzWGZA27SxpoNWZfNt4

# Build release version (optimized, takes 5-10 minutes first time)
cargo build --release

# Run it!
./target/release/hyper-terminal
```

For detailed platform-specific instructions, continue reading below.

## Prerequisites by Platform

### All Platforms
- **Rust**: 1.75 or newer (stable channel)
- **Git**: For cloning the repository
- **Internet connection**: For downloading Rust crates

Install Rust from [rustup.rs](https://rustup.rs/):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Windows

#### Required
- **Windows 10 or later** (for ConPTY support)
- **Visual Studio 2019 or later** with C++ tools:
  - Install from [Visual Studio](https://visualstudio.microsoft.com/)
  - Or install just the build tools: [Build Tools for Visual Studio](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022)
  - Select "Desktop development with C++" workload

#### Optional
- **PowerShell 7+** for best shell experience
- **Windows Terminal** for comparison testing

### macOS

#### Required
- **macOS 10.15 (Catalina) or later**
- **Xcode Command Line Tools**:
  ```bash
  xcode-select --install
  ```

#### Optional
- **Homebrew** for installing additional tools:
  ```bash
  /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
  ```

### Linux

#### Ubuntu/Debian
```bash
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    pkg-config \
    libfontconfig1-dev \
    libfreetype6-dev \
    libxcb1-dev \
    libxkbcommon-dev \
    libwayland-dev \
    libvulkan-dev
```

#### Fedora/RHEL
```bash
sudo dnf install -y \
    gcc \
    g++ \
    pkg-config \
    fontconfig-devel \
    freetype-devel \
    libxcb-devel \
    libxkbcommon-devel \
    wayland-devel \
    vulkan-loader-devel
```

#### Arch Linux
```bash
sudo pacman -S \
    base-devel \
    pkg-config \
    fontconfig \
    freetype2 \
    libxcb \
    libxkbcommon \
    wayland \
    vulkan-icd-loader
```

## Building from Source

### 1. Clone the Repository

```bash
git clone https://github.com/Aetherlann/MCP.git
cd MCP
```

### 2. Checkout the Branch

```bash
git checkout claude/hyper-modern-terminal-011CUZzWGZA27SxpoNWZfNt4
```

### 3. Build

#### Release Build (Recommended)
Optimized for performance:
```bash
cargo build --release
```

**Build time**: 5-10 minutes (first time), 10-30 seconds (incremental)

The binary will be at:
- **Linux/macOS**: `./target/release/hyper-terminal`
- **Windows**: `.\target\release\hyper-terminal.exe`

#### Debug Build (For Development)
Faster compilation, slower runtime, includes debug symbols:
```bash
cargo build
```

The binary will be at:
- **Linux/macOS**: `./target/debug/hyper-terminal`
- **Windows**: `.\target\debug\hyper-terminal.exe`

## Running the Terminal

### From the Build Directory

After building, you can run directly:

```bash
# Release build
cargo run --release

# Or run the binary directly
./target/release/hyper-terminal
```

### Installing System-Wide (Optional)

#### Linux/macOS
```bash
# Install to ~/.cargo/bin (automatically in PATH if you used rustup)
cargo install --path crates/hyper-terminal

# Or manually copy to /usr/local/bin
sudo cp target/release/hyper-terminal /usr/local/bin/hyper

# Now you can run from anywhere
hyper-terminal
```

#### Windows
```powershell
# Install to %USERPROFILE%\.cargo\bin (automatically in PATH if you used rustup)
cargo install --path crates\hyper-terminal

# Or manually copy to a directory in your PATH
copy target\release\hyper-terminal.exe C:\Windows\System32\hyper.exe

# Now you can run from anywhere
hyper-terminal
```

## Testing the Build

### 1. Basic Test
Just run the terminal and check if it starts:
```bash
cargo run --release
```

You should see a terminal window with your shell running.

### 2. Run Unit Tests
Test the individual crates:
```bash
# Test all crates
cargo test --all

# Test specific crate
cargo test -p ht-vt
cargo test -p ht-gallery
```

### 3. Run Gallery Demos
Test the gallery system with demo scripts:

```bash
# First, start the terminal
cargo run --release

# In the terminal, run demos:
./scripts/demo-gallery-grid.sh
./scripts/demo-gallery-comparison.sh
./scripts/demo-gallery-all-modes.sh
```

### 4. Run All Demos
Comprehensive demo of all features:
```bash
# Inside the running terminal
./scripts/run-all-demos.sh
```

## Troubleshooting

### Build Errors

#### "Failed to get crates.io index"
**Problem**: Network issue accessing crates.io
**Solution**:
```bash
# Wait a moment and try again
sleep 5 && cargo build --release

# Or use a different registry mirror
export CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
cargo build --release
```

#### "linker 'cc' not found" (Linux)
**Problem**: C compiler not installed
**Solution**:
```bash
# Ubuntu/Debian
sudo apt-get install build-essential

# Fedora/RHEL
sudo dnf install gcc gcc-c++

# Arch
sudo pacman -S base-devel
```

#### "LINK : fatal error LNK1181" (Windows)
**Problem**: Visual Studio C++ tools not installed
**Solution**:
1. Install Visual Studio Build Tools
2. Select "Desktop development with C++"
3. Restart your terminal
4. Run `cargo build --release` again

#### "Cannot find -lfontconfig" (Linux)
**Problem**: Fontconfig development headers not installed
**Solution**:
```bash
# Ubuntu/Debian
sudo apt-get install libfontconfig1-dev

# Fedora/RHEL
sudo dnf install fontconfig-devel

# Arch
sudo pacman -S fontconfig
```

### Runtime Errors

#### Window doesn't appear
**Problem**: Graphics driver or wgpu issue
**Solution**:
```bash
# Try running with specific backend
WGPU_BACKEND=vulkan cargo run --release  # Linux
WGPU_BACKEND=metal cargo run --release   # macOS
WGPU_BACKEND=dx12 cargo run --release    # Windows

# Or fallback to software rendering
WGPU_BACKEND=gl cargo run --release
```

#### "ConPTY not supported" (Windows)
**Problem**: Windows version too old
**Solution**: Upgrade to Windows 10 (1809 or later) or Windows 11

#### Slow performance
**Problem**: Running debug build
**Solution**: Always use release build for actual usage:
```bash
cargo build --release
cargo run --release
```

#### Glass effect doesn't work (Linux)
**Problem**: Compositor doesn't support blur
**Solution**:
- Use KDE Plasma or GNOME with blur extension
- Or disable glass in config: `"enabled": false`

## Development Build

For active development with faster iteration:

### 1. Development Build
```bash
# Faster compilation, but slower runtime
cargo build

# Run with debug logging
RUST_LOG=debug cargo run
```

### 2. Watch Mode (Auto-rebuild)
Install cargo-watch:
```bash
cargo install cargo-watch

# Auto-rebuild on file changes
cargo watch -x build
```

### 3. Check Without Building
Faster than full build:
```bash
cargo check
```

### 4. Format and Lint
```bash
# Format code
cargo fmt

# Run clippy for lints
cargo clippy --all-targets --all-features
```

## Build Profiles

The project uses custom build profiles defined in `Cargo.toml`:

### Release Profile
- **Optimization level**: 3 (maximum)
- **LTO**: Thin (faster build than full LTO)
- **Codegen units**: 1 (better optimization)
- **Strip**: Yes (smaller binary)

### Dev Profile
- **Optimization level**: 1 (faster compilation)
- **Debug info**: Yes (for debugging)

## Binary Size

Expected binary sizes after release build:

- **Linux**: ~15-25 MB (stripped)
- **macOS**: ~20-30 MB (stripped)
- **Windows**: ~18-28 MB (stripped)

To reduce size further:
```bash
# After building
strip target/release/hyper-terminal

# Or use UPX compression (install from upx.github.io)
upx --best --lzma target/release/hyper-terminal
```

## Next Steps

After building successfully:

1. **Read the docs**: See [DEMO_QUICKSTART.md](DEMO_QUICKSTART.md) for feature demos
2. **Try the gallery system**: See [GALLERY_COMPLETE.md](GALLERY_COMPLETE.md)
3. **Configure**: Copy `config.example.json` to your config directory
4. **Contribute**: See [ARCHITECTURE.md](ARCHITECTURE.md) for code structure

## Getting Help

If you encounter issues:

1. Check [COMPILATION_AND_TESTING.md](COMPILATION_AND_TESTING.md) for detailed troubleshooting
2. Search [existing issues](https://github.com/Aetherlann/MCP/issues)
3. Open a new issue with:
   - Your OS and version
   - Rust version (`rustc --version`)
   - Full error message
   - Steps to reproduce

## Summary

**Minimum viable build:**
```bash
git clone https://github.com/Aetherlann/MCP.git
cd MCP
git checkout claude/hyper-modern-terminal-011CUZzWGZA27SxpoNWZfNt4
cargo build --release
./target/release/hyper-terminal
```

**That's it!** You now have a fully functional hyper-modern terminal with gallery system support.
