# Easy Installation Guide for TUI Framework

This guide provides **multiple easy options** to install notcurses for the TUI Framework, solving the installation friction problem.

## Quick Start (Recommended)

### Option 1: Python Installer (Easiest)
```bash
pip install tui-framework-installer
tui-install
```

This is the **easiest method** - just like `npm install` for JavaScript:
- **Cross-platform**: Linux, macOS, Windows (MSYS2)
- **Beautiful UI**: Rich progress bars and status messages
- **Smart detection**: Automatically detects your platform
- **Dependency management**: Installs everything you need
- **Error handling**: Clear error messages and troubleshooting

### Option 2: One-Line Shell Script
```bash
curl -sSL https://raw.githubusercontent.com/entrepeneur4lyf/tui-framework/main/install-notcurses.sh | bash
```

This script:
- **Detects your OS** automatically (Ubuntu, Fedora, Arch, macOS, Windows)
- **Installs dependencies** for your platform
- **Uses local source** if available (faster for developers)
- **Builds notcurses 3.0.11** from source
- **Tests installation** automatically
- **Runs framework test** to verify everything works

### Option 3: Docker (Zero Dependencies)
```bash
# Build and run in Docker
docker build -t tui-framework .
docker run -it tui-framework

# Or use pre-built image (when available)
docker run -it ghcr.io/entrepeneur4lyf/tui-framework:latest
```

### Option 4: GitHub Releases (Coming Soon)
```bash
# Download pre-built notcurses for your platform
wget https://github.com/entrepeneur4lyf/tui-framework/releases/download/v0.1.0/notcurses-3.0.11-linux-x86_64.tar.gz
tar -xzf notcurses-3.0.11-linux-x86_64.tar.gz
sudo cp -r usr/* /usr/local/
sudo ldconfig
```

## 🛠 **Manual Installation**

If the automated options don't work, see [INSTALLATION.md](INSTALLATION.md) for detailed manual instructions.

## Verification

After installation, verify everything works:

```bash
# Check notcurses version
pkg-config --modversion notcurses
# Should output: 3.0.11

# Test the framework
git clone https://github.com/entrepeneur4lyf/tui-framework.git
cd tui-framework
cargo run --example backend_test --features notcurses
```

## Success!

You should see:
```
Testing TUI Framework Backend
==============================

1. Testing Placeholder Backend:
  - Placeholder backend initialized
  - Terminal size: 80x24
  - Virtual DOM rendered successfully

2. Testing Libnotcurses Backend:
  - Libnotcurses backend initialized
  - Terminal size: 80x30
  - Virtual DOM rendered to terminal
  - Event handling works

All backend tests completed successfully!
```

## Troubleshooting

### Common Issues:

**"pkg-config not found"**
```bash
# Ubuntu/Debian
sudo apt-get install pkg-config

# Fedora
sudo dnf install pkgconfig

# macOS
brew install pkg-config
```

**"Library not found"**
```bash
# Run ldconfig to update library cache
sudo ldconfig

# Or add to your shell profile:
export LD_LIBRARY_PATH="/usr/local/lib:$LD_LIBRARY_PATH"
```

**"Permission denied"**
```bash
# Make install script executable
chmod +x install-notcurses.sh
./install-notcurses.sh
```

## Support

- **Documentation**: [INSTALLATION.md](INSTALLATION.md)
- **Issues**: [GitHub Issues](https://github.com/entrepeneur4lyf/tui-framework/issues)
- **Discussions**: [GitHub Discussions](https://github.com/entrepeneur4lyf/tui-framework/discussions)

---

**The goal**: Make installation as easy as `npm install` for JavaScript developers!
