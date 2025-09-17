---
description: Repository Information Overview
alwaysApply: true
---

# Textual Framework Information

## Summary
Textual is a modern Text User Interface (TUI) framework for Python that allows building cross-platform terminal applications with a simple API. It combines Python with web development concepts to create responsive terminal UIs that can also be served in web browsers.

## Structure
- **src/textual**: Core framework source code
- **tests**: Extensive test suite with unit and integration tests
- **examples**: Sample applications demonstrating framework features
- **docs**: Documentation source files
- **tools**: Utility scripts for development and documentation

## Language & Runtime
**Language**: Python
**Version**: Python 3.8.1+
**Build System**: Poetry
**Package Manager**: Poetry

## Dependencies
**Main Dependencies**:
- rich (>=13.3.3): Terminal formatting and rendering
- markdown-it-py (>=2.1.0): Markdown processing
- typing-extensions (^4.4.0): Type hinting extensions
- platformdirs (>=3.6.0,<5): Platform-specific directory handling
- pygments (^2.19.2): Syntax highlighting

**Optional Dependencies**:
- tree-sitter (>=0.25.0): Syntax highlighting (Python 3.10+ only)
- Various tree-sitter language parsers

**Development Dependencies**:
- pytest (^8.3.1): Testing framework
- mypy (^1.0.0): Type checking
- black (24.4.2): Code formatting
- mkdocs (^1.3.0): Documentation generation
- textual-dev (^1.7.0): Development tools

## Build & Installation
```bash
# Install with pip
pip install textual textual-dev

# Development setup with Poetry
poetry install
poetry install --extras syntax  # For syntax highlighting support

# Run tests
poetry run pytest tests/ -n 16 --dist=loadgroup

# Build package
poetry build
```

## Testing
**Framework**: pytest
**Test Location**: tests/ directory
**Naming Convention**: test_*.py files
**Configuration**: pytest.ini_options in pyproject.toml
**Run Command**:
```bash
poetry run pytest tests/ -n 16 --dist=loadgroup
# or
make test
```

## Documentation
**System**: MkDocs with Material theme
**Build Command**:
```bash
# Serve documentation locally
make docs-serve

# Build documentation
make docs-build
```

## Demo & Examples
**Run Demo**:
```bash
# Run built-in demo
python -m textual

# Run examples
python -m textual.examples.calculator
```

## Web Integration
**Web Serving**:
```bash
# Serve any Textual app in a browser
textual serve "python -m textual"
```