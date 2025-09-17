# TUI Framework Python Installer

The easiest way to install the TUI Framework and its notcurses dependencies.

## Quick Install

```bash
pip install tui-framework-installer
tui-install
```

That's it! The installer will:
- Detect your platform automatically
- Install all required dependencies
- Build and install notcurses 3.0.11
- Verify the installation works

## Supported Platforms

- **Linux**: Ubuntu, Fedora, Arch Linux
- **macOS**: With Homebrew
- **Windows**: With MSYS2 or vcpkg

## Options

```bash
# Skip dependency installation (if already installed)
tui-install --skip-deps

# Skip building notcurses (if using package manager version)
tui-install --skip-build

# Test installation after completion
tui-install --test

# Get help
tui-install --help
```

## Alternative: Direct Python Usage

```bash
# Clone and install locally
git clone https://github.com/entrepeneur4lyf/tui-framework.git
cd tui-framework/python-installer
pip install -e .
tui-install
```

## After Installation

Once installed, you can start using the TUI Framework:

```bash
# Clone the framework
git clone https://github.com/entrepeneur4lyf/tui-framework.git
cd tui-framework

# Test the backend
cargo run --example backend_test --features notcurses

# Start building your TUI app!
cargo new my-tui-app
cd my-tui-app
cargo add tui-framework --features notcurses
```

## Troubleshooting

If you encounter issues:

1. **Permission errors**: Make sure you have sudo access for system package installation
2. **Missing dependencies**: The installer will try to install them automatically
3. **Build failures**: Check that you have a working C/C++ compiler
4. **Windows issues**: Make sure you're using MSYS2 or have vcpkg installed

For more help, see the [main installation guide](../INSTALLATION.md).

## Contributing

This installer is part of the TUI Framework project. Contributions welcome!

- Report bugs: [GitHub Issues](https://github.com/entrepeneur4lyf/tui-framework/issues)
- Suggest features: [GitHub Discussions](https://github.com/entrepeneur4lyf/tui-framework/discussions)
