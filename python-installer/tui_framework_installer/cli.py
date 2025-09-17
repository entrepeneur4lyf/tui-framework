#!/usr/bin/env python3
"""
TUI Framework Installer CLI

Easy installation of notcurses and TUI Framework dependencies.
"""

import click
import platform
import subprocess
import sys
import os
import shutil
import tempfile
from pathlib import Path
from rich.console import Console
from rich.progress import Progress, SpinnerColumn, TextColumn
from rich.panel import Panel
from rich.text import Text

console = Console()

def run_command(cmd, cwd=None, check=True):
    """Run a shell command with proper error handling."""
    try:
        result = subprocess.run(
            cmd, 
            shell=True, 
            cwd=cwd, 
            check=check,
            capture_output=True, 
            text=True
        )
        return result.returncode == 0, result.stdout, result.stderr
    except subprocess.CalledProcessError as e:
        return False, e.stdout, e.stderr

def check_command_exists(cmd):
    """Check if a command exists in PATH."""
    return shutil.which(cmd) is not None

def detect_platform():
    """Detect the current platform and package manager."""
    system = platform.system().lower()
    
    if system == "linux":
        if check_command_exists("apt-get"):
            return "ubuntu", "apt"
        elif check_command_exists("dnf"):
            return "fedora", "dnf"
        elif check_command_exists("pacman"):
            return "arch", "pacman"
        else:
            return "linux", "unknown"
    elif system == "darwin":
        return "macos", "brew"
    elif system == "windows":
        if check_command_exists("pacman"):  # MSYS2
            return "windows", "msys2"
        elif check_command_exists("vcpkg"):
            return "windows", "vcpkg"
        else:
            return "windows", "unknown"
    else:
        return "unknown", "unknown"

def install_dependencies(platform_name, package_manager):
    """Install platform-specific dependencies."""
    deps = {
        "ubuntu": [
            "build-essential", "cmake", "pkg-config",
            "libncurses-dev", "libunistring-dev", 
            "libavformat-dev", "libavutil-dev",
            "libswscale-dev", "libqrcodegen-dev", "git"
        ],
        "fedora": [
            "gcc-c++", "cmake", "pkgconfig",
            "ncurses-devel", "libunistring-devel",
            "ffmpeg-devel", "qrencode-devel", "git"
        ],
        "arch": ["notcurses"],  # Available in AUR
        "macos": ["notcurses"],  # Available via Homebrew
        "msys2": [
            "mingw-w64-x86_64-gcc", "mingw-w64-x86_64-cmake",
            "mingw-w64-x86_64-pkg-config", "mingw-w64-x86_64-ncurses",
            "mingw-w64-x86_64-ffmpeg", "git"
        ]
    }
    
    commands = {
        "apt": "sudo apt-get update && sudo apt-get install -y",
        "dnf": "sudo dnf install -y",
        "pacman": "sudo pacman -S --needed --noconfirm" if platform_name == "msys2" else "yay -S",
        "brew": "brew install"
    }
    
    if platform_name not in deps:
        return False, f"Unsupported platform: {platform_name}"
    
    if package_manager not in commands:
        return False, f"Unsupported package manager: {package_manager}"
    
    packages = " ".join(deps[platform_name])
    cmd = f"{commands[package_manager]} {packages}"
    
    console.print(f"[blue]Installing dependencies: {packages}[/blue]")
    success, stdout, stderr = run_command(cmd)
    
    if not success:
        return False, f"Failed to install dependencies: {stderr}"
    
    return True, "Dependencies installed successfully"

def build_notcurses():
    """Build notcurses 3.0.11 from source."""
    with tempfile.TemporaryDirectory() as temp_dir:
        console.print("[blue]Downloading notcurses source...[/blue]")
        
        # Clone notcurses
        success, _, stderr = run_command(
            "git clone https://github.com/dankamongmen/notcurses.git",
            cwd=temp_dir
        )
        if not success:
            return False, f"Failed to clone notcurses: {stderr}"
        
        notcurses_dir = os.path.join(temp_dir, "notcurses")
        
        # Checkout v3.0.11
        success, _, stderr = run_command(
            "git checkout v3.0.11",
            cwd=notcurses_dir
        )
        if not success:
            return False, f"Failed to checkout v3.0.11: {stderr}"
        
        # Create build directory
        build_dir = os.path.join(notcurses_dir, "build")
        os.makedirs(build_dir, exist_ok=True)
        
        console.print("[blue]Configuring build...[/blue]")
        success, _, stderr = run_command(
            "cmake .. -DCMAKE_BUILD_TYPE=Release",
            cwd=build_dir
        )
        if not success:
            return False, f"Failed to configure build: {stderr}"
        
        console.print("[blue]Building notcurses (this may take a few minutes)...[/blue]")
        cpu_count = os.cpu_count() or 4
        success, _, stderr = run_command(
            f"make -j{cpu_count}",
            cwd=build_dir
        )
        if not success:
            return False, f"Failed to build notcurses: {stderr}"
        
        console.print("[blue]Installing notcurses...[/blue]")
        success, _, stderr = run_command(
            "sudo make install",
            cwd=build_dir
        )
        if not success:
            return False, f"Failed to install notcurses: {stderr}"
        
        # Update library cache
        if platform.system().lower() == "linux":
            run_command("sudo ldconfig")
        
        return True, "notcurses 3.0.11 installed successfully"

@click.command()
@click.option("--skip-deps", is_flag=True, help="Skip dependency installation")
@click.option("--skip-build", is_flag=True, help="Skip building notcurses")
@click.option("--test", is_flag=True, help="Test installation after completion")
def main(skip_deps, skip_build, test):
    """Install TUI Framework and notcurses dependencies."""
    
    console.print(Panel.fit(
        "[bold blue]TUI Framework Installer[/bold blue]\n"
        "Installing notcurses 3.0.11 and dependencies...",
        border_style="blue"
    ))
    
    # Detect platform
    platform_name, package_manager = detect_platform()
    console.print(f"[green]Detected platform: {platform_name} ({package_manager})[/green]")
    
    if platform_name == "unknown":
        console.print("[red]ERROR: Unsupported platform detected[/red]")
        console.print("Supported platforms:")
        console.print("  - Linux (Ubuntu, Fedora, Arch)")
        console.print("  - macOS (with Homebrew)")
        console.print("  - Windows (with MSYS2)")
        sys.exit(1)
    
    # Check if notcurses is already installed
    success, stdout, _ = run_command("pkg-config --modversion notcurses", check=False)
    if success and "3.0.11" in stdout:
        console.print("[green]SUCCESS: notcurses 3.0.11 already installed[/green]")
        if not test:
            return
    
    # Install dependencies
    if not skip_deps:
        with Progress(
            SpinnerColumn(),
            TextColumn("[progress.description]{task.description}"),
            console=console,
        ) as progress:
            task = progress.add_task("Installing dependencies...", total=None)
            success, message = install_dependencies(platform_name, package_manager)
            progress.remove_task(task)
            
            if success:
                console.print(f"[green]SUCCESS: {message}[/green]")
            else:
                console.print(f"[red]ERROR: {message}[/red]")
                sys.exit(1)
    
    # Build notcurses (unless using package manager that provides it)
    if not skip_build and platform_name not in ["arch", "macos"]:
        with Progress(
            SpinnerColumn(),
            TextColumn("[progress.description]{task.description}"),
            console=console,
        ) as progress:
            task = progress.add_task("Building notcurses...", total=None)
            success, message = build_notcurses()
            progress.remove_task(task)
            
            if success:
                console.print(f"[green]SUCCESS: {message}[/green]")
            else:
                console.print(f"[red]ERROR: {message}[/red]")
                sys.exit(1)
    
    # Test installation
    if test:
        console.print("[blue]Testing installation...[/blue]")
        success, stdout, stderr = run_command("pkg-config --modversion notcurses", check=False)
        if success:
            version = stdout.strip()
            console.print(f"[green]SUCCESS: notcurses {version} detected[/green]")
        else:
            console.print("[red]ERROR: Installation verification failed[/red]")
            sys.exit(1)
    
    console.print(Panel.fit(
        "[bold green]Installation Complete![/bold green]\n\n"
        "Next steps:\n"
        "1. Clone the TUI Framework:\n"
        "   [cyan]git clone https://github.com/entrepeneur4lyf/tui-framework.git[/cyan]\n"
        "2. Test the framework:\n"
        "   [cyan]cd tui-framework && cargo run --example backend_test --features notcurses[/cyan]",
        border_style="green"
    ))

if __name__ == "__main__":
    main()
