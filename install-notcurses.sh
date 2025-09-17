#!/bin/bash
# Install script for notcurses 3.0.11+ for tui-framework
set -e

echo "🚀 Installing notcurses for tui-framework..."

# Check if already installed
if pkg-config --exists notcurses && [[ $(pkg-config --modversion notcurses) == "3.0.11" ]]; then
    echo "✅ notcurses 3.0.11 already installed!"
    exit 0
fi

# Function to build from source
build_from_source() {
    echo "� Building notcurses 3.0.11 from source..."

    # Use local source if available, otherwise clone
    if [[ -d "reference/notcurses-3.0.11" ]]; then
        echo "📁 Using local notcurses source..."
        cd reference/notcurses-3.0.11
        if [[ ! -d "build" ]]; then
            mkdir build
        fi
    else
        echo "� Downloading notcurses source..."
        cd /tmp
        if [[ -d "notcurses" ]]; then
            rm -rf notcurses
        fi
        git clone https://github.com/dankamongmen/notcurses.git
        cd notcurses
        git checkout v3.0.11
        mkdir build
    fi

    cd build
    cmake .. -DCMAKE_BUILD_TYPE=Release
    make -j$(nproc)
    sudo make install
    sudo ldconfig

    echo "✅ notcurses 3.0.11 installed successfully!"
}

# Detect OS and install dependencies
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    if command -v apt-get &> /dev/null; then
        # Ubuntu/Debian
        echo "📦 Installing dependencies for Ubuntu/Debian..."
        sudo apt-get update
        sudo apt-get install -y build-essential cmake pkg-config \
            libncurses-dev libunistring-dev libavformat-dev libavutil-dev \
            libswscale-dev libqrcodegen-dev git
        build_from_source

    elif command -v dnf &> /dev/null; then
        # Fedora
        echo "📦 Installing dependencies for Fedora..."
        sudo dnf install -y gcc-c++ cmake pkgconfig ncurses-devel \
            libunistring-devel ffmpeg-devel qrencode-devel git
        build_from_source

    elif command -v pacman &> /dev/null; then
        # Arch Linux
        echo "📦 Installing from AUR..."
        if command -v yay &> /dev/null; then
            yay -S notcurses
        else
            echo "❌ Please install yay or manually install notcurses from AUR"
            exit 1
        fi
    else
        echo "❌ Unsupported Linux distribution"
        echo "Please install build dependencies manually and run:"
        echo "  ./install-notcurses.sh"
        exit 1
    fi

elif [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    if command -v brew &> /dev/null; then
        echo "📦 Installing via Homebrew..."
        brew install notcurses
    else
        echo "❌ Please install Homebrew first: https://brew.sh"
        exit 1
    fi

elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" || "$OSTYPE" == "win32" ]]; then
    # Windows (MSYS2/Cygwin/Git Bash)
    echo "🪟 Windows detected - using MSYS2/vcpkg approach..."

    if command -v pacman &> /dev/null; then
        # MSYS2
        echo "📦 Installing via MSYS2..."
        pacman -S --needed --noconfirm \
            mingw-w64-x86_64-gcc \
            mingw-w64-x86_64-cmake \
            mingw-w64-x86_64-pkg-config \
            mingw-w64-x86_64-ncurses \
            mingw-w64-x86_64-ffmpeg \
            git
        build_from_source

    elif command -v vcpkg &> /dev/null; then
        # vcpkg
        echo "📦 Installing via vcpkg..."
        vcpkg install ncurses ffmpeg
        build_from_source

    else
        echo "❌ Windows requires MSYS2 or vcpkg"
        echo "Install MSYS2: https://www.msys2.org/"
        echo "Or vcpkg: https://github.com/Microsoft/vcpkg"
        exit 1
    fi

else
    echo "❌ Unsupported OS: $OSTYPE"
    echo "Supported platforms:"
    echo "  - Linux (Ubuntu, Fedora, Arch)"
    echo "  - macOS (with Homebrew)"
    echo "  - Windows (with MSYS2 or vcpkg)"
    echo ""
    echo "Please install notcurses 3.0.11+ manually or use Docker:"
    echo "  docker run -it ghcr.io/entrepeneur4lyf/tui-framework:latest"
    exit 1
fi

echo ""
echo "🎉 Installation complete!"
echo ""
echo "🧪 Testing installation..."
if pkg-config --exists notcurses; then
    VERSION=$(pkg-config --modversion notcurses)
    echo "✅ notcurses $VERSION detected"

    if [[ -f "Cargo.toml" ]]; then
        echo "🚀 Testing tui-framework..."
        cargo run --example backend_test --features notcurses
    fi
else
    echo "❌ Installation verification failed"
    exit 1
fi
