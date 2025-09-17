# Installation Guide

This guide helps you install the required dependencies for the TUI Framework.

## Quick Install

For most Linux distributions, run our installation script:

```bash
curl -sSL https://raw.githubusercontent.com/entrepeneur4lyf/tui-framework/main/install-notcurses.sh | bash
```

## Manual Installation

### Ubuntu/Debian

```bash
# Install dependencies
sudo apt-get update
sudo apt-get install -y build-essential cmake pkg-config \
    libncurses-dev libunistring-dev libavformat-dev libavutil-dev \
    libswscale-dev libqrcodegen-dev git

# Build notcurses from source
cd /tmp
git clone https://github.com/dankamongmen/notcurses.git
cd notcurses
git checkout v3.0.11
mkdir build && cd build
cmake .. -DCMAKE_BUILD_TYPE=Release
make -j$(nproc)
sudo make install
sudo ldconfig
```

### Fedora

```bash
# Install dependencies
sudo dnf install -y gcc-c++ cmake pkgconfig ncurses-devel \
    libunistring-devel ffmpeg-devel qrencode-devel git

# Build notcurses from source (same as Ubuntu)
cd /tmp
git clone https://github.com/dankamongmen/notcurses.git
cd notcurses
git checkout v3.0.11
mkdir build && cd build
cmake .. -DCMAKE_BUILD_TYPE=Release
make -j$(nproc)
sudo make install
sudo ldconfig
```

### Arch Linux

```bash
# Using yay (AUR helper)
yay -S notcurses

# Or manually from AUR
git clone https://aur.archlinux.org/notcurses.git
cd notcurses
makepkg -si
```

### macOS

```bash
# Using Homebrew
brew install notcurses
```

## Verification

Test that notcurses is properly installed:

```bash
pkg-config --modversion notcurses
# Should output: 3.0.11 or higher
```

## Using the Framework

Once notcurses is installed:

```bash
# Clone the framework
git clone https://github.com/entrepeneur4lyf/tui-framework.git
cd tui-framework

# Test the backend
cargo run --example backend_test --features notcurses

# Run examples
cargo run --example hello_world --features notcurses
```

## Troubleshooting

### Library not found errors

If you get linking errors, ensure the library paths are configured:

```bash
# Add to ~/.bashrc or ~/.zshrc
export PKG_CONFIG_PATH="/usr/local/lib/pkgconfig:$PKG_CONFIG_PATH"
export LD_LIBRARY_PATH="/usr/local/lib:$LD_LIBRARY_PATH"
```

### Container/Docker usage

For containers, you may need to run `ldconfig` after installation:

```bash
sudo ldconfig
```

## Alternative: Use PlaceholderBackend

If you can't install notcurses, the framework includes a placeholder backend for development:

```bash
cargo run --example backend_test  # Uses PlaceholderBackend by default
```
