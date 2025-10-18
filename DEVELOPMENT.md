# Developer Guide

<!-- TOC start -->
## Table of Contents

- [Development Setup](#development-setup)
- [Documentation Tools](#documentation-tools)
  - [Table of Contents Generator](#table-of-contents-generator)
    - [Usage](#usage)
    - [Configuration](#configuration)
    - [Default Ignores](#default-ignores)

[Back to Table of Contents](README.md#table-of-contents)
<!-- TOC end -->

## Development Setup

TODO:

## Documentation Tools

This repository includes a Python script `generate_toc.py` to automatically generate and update table of contents for all markdown files.

### Table of Contents Generator

The `generate_toc.py` script can automatically generate and update table of contents for all markdown files in the repository.

#### Usage

```bash
# Generate TOC for all markdown files
python3 generate_toc.py

# Ignore specific files via command line
python3 generate_toc.py --ignore-files README.md CHANGELOG.md

# Ignore patterns via command line
python3 generate_toc.py --ignore-patterns "docs/temp/*" "*.draft.md"

# Use custom config file
python3 generate_toc.py --config .mytocignore
```

#### Configuration

Create a `.tocignore` file in your repository root to configure which files to ignore:

```json
{
  "files": ["CHANGELOG.md", "README_zh.md"],
  "patterns": [".git/*", "node_modules/*", "*.min.md", "docs/temp/*", "examples/drafts/*"]
}
```

- **files**: Exact filenames to ignore
- **patterns**: Glob patterns to match files/directories to ignore

#### Default Ignores

By default, the script ignores:

- `.git/` directory
- `node_modules/` directory
- `*.min.md` files
