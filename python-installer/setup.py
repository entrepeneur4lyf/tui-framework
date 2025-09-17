#!/usr/bin/env python3
"""
TUI Framework Installer - Python-based installer for notcurses and tui-framework
"""

from setuptools import setup, find_packages
import os

here = os.path.abspath(os.path.dirname(__file__))

def read(fname):
    return open(os.path.join(here, fname)).read()

setup(
    name="tui-framework-installer",
    version="0.1.0",
    description="Easy installer for TUI Framework and notcurses dependencies",
    long_description=read("README.md") if os.path.exists("README.md") else "",
    long_description_content_type="text/markdown",
    author="TUI Framework Team",
    author_email="support@tui-framework.dev",
    url="https://github.com/entrepeneur4lyf/tui-framework",
    packages=find_packages(),
    classifiers=[
        "Development Status :: 4 - Beta",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Python :: 3.12",
        "Operating System :: OS Independent",
        "Topic :: Software Development :: Libraries",
        "Topic :: System :: Shells",
        "Topic :: Terminals",
    ],
    python_requires=">=3.8",
    install_requires=[
        "requests>=2.25.0",
        "click>=8.0.0",
        "rich>=10.0.0",
        "packaging>=21.0",
    ],
    entry_points={
        "console_scripts": [
            "tui-framework-install=tui_framework_installer.cli:main",
            "tui-install=tui_framework_installer.cli:main",
        ],
    },
    keywords="tui terminal ui framework notcurses rust installer",
    project_urls={
        "Bug Reports": "https://github.com/entrepeneur4lyf/tui-framework/issues",
        "Source": "https://github.com/entrepeneur4lyf/tui-framework",
        "Documentation": "https://github.com/entrepeneur4lyf/tui-framework/blob/main/README.md",
    },
)
