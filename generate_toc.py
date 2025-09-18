#!/usr/bin/env python3
"""
Script to generate and insert table of contents for all markdown files in the repository.
"""

import os
import re
import json
import argparse
from pathlib import Path


def load_ignore_config():
    """Load ignore configuration from .tocignore file or return defaults."""
    ignore_file = Path(".tocignore")
    default_ignores = {
        "files": [],
        "patterns": [".git/*", "node_modules/*", "*.min.md"],
    }

    if ignore_file.exists():
        try:
            with open(ignore_file, "r", encoding="utf-8") as f:
                config = json.load(f)
                return {**default_ignores, **config}
        except (json.JSONDecodeError, FileNotFoundError):
            print("Warning: Invalid .tocignore file, using defaults")

    return default_ignores


def should_ignore_file(filepath, ignore_config):
    """Check if file should be ignored based on ignore configuration."""
    file_path_str = str(filepath)

    # Check exact file matches
    if filepath.name in ignore_config.get("files", []):
        return True

    # Check pattern matches
    for pattern in ignore_config.get("patterns", []):
        if Path(file_path_str).match(pattern) or any(
            part in file_path_str for part in pattern.split("/")
        ):
            return True

    return False


def extract_headers(content):
    """Extract headers from markdown content."""
    headers = []
    lines = content.split("\n")
    for line in lines:
        # Match markdown headers (##, ###, ####, etc.)
        match = re.match(r"^(#{2,6})\s+(.+)", line)
        if match:
            level = len(match.group(1)) - 1  # Convert ## to 1, ### to 2, etc.
            title = match.group(2).strip()
            # Skip "Table of Contents" headers
            if title.lower() == "table of contents":
                continue
            # Create anchor link by lowercasing, removing special chars, replacing spaces with -
            anchor = re.sub(r"[^\w\s-]", "", title.lower())
            anchor = re.sub(r"[-\s]+", "-", anchor)
            headers.append((level, title, anchor))
    return headers


def generate_toc(headers):
    """Generate table of contents from headers."""
    if not headers:
        return ""

    toc_lines = ["<!-- TOC start -->", "<!-- TOC end -->", ""]
    return "\n".join(toc_lines)


def generate_toc_content(headers, filename):
    """Generate the actual table of contents content."""
    if not headers:
        return ""

    toc_lines = ["## Table of Contents", ""]
    for level, title, anchor in headers:
        indent = "  " * (level - 1)
        toc_lines.append(f"{indent}- [{title}](#{anchor})")

    # Add a link back to the top level TOC if not the main README
    if filename != "README.md":
        toc_lines.append("")
        toc_lines.append("[Back to Table of Contents](README.md#table-of-contents)")

    return "\n".join(toc_lines)


def insert_toc(filepath):
    """Insert table of contents into a markdown file."""
    with open(filepath, "r", encoding="utf-8") as f:
        content = f.read()

    # Extract headers
    headers = extract_headers(content)

    # Generate TOC content
    toc_content = generate_toc_content(headers, os.path.basename(filepath))

    # Check if TOC markers already exist
    if "<!-- TOC start -->" in content and "<!-- TOC end -->" in content:
        # Replace existing TOC
        content = re.sub(
            r"<!-- TOC start -->.*?<!-- TOC end -->",
            f"<!-- TOC start -->\n{toc_content}\n<!-- TOC end -->",
            content,
            flags=re.DOTALL,
        )
    else:
        # Insert TOC after the first header (title)
        lines = content.split("\n")
        new_lines = []
        title_found = False

        for line in lines:
            new_lines.append(line)
            # Insert TOC after the first header
            if (
                not title_found
                and line.startswith("# ")
                and not line.startswith("# Table of Contents")
            ):
                new_lines.append("")
                new_lines.append("<!-- TOC start -->")
                new_lines.append(toc_content)
                new_lines.append("<!-- TOC end -->")
                new_lines.append("")
                title_found = True

        content = "\n".join(new_lines)

    # Write back to file
    with open(filepath, "w", encoding="utf-8") as f:
        f.write(content)

    print(f"Updated {filepath}")


def main():
    """Main function to process all markdown files."""
    parser = argparse.ArgumentParser(
        description="Generate table of contents for markdown files"
    )
    parser.add_argument("--ignore-files", nargs="*", default=[], help="Files to ignore")
    parser.add_argument(
        "--ignore-patterns", nargs="*", default=[], help="Patterns to ignore"
    )
    parser.add_argument(
        "--config", default=".tocignore", help="Path to ignore config file"
    )

    args = parser.parse_args()

    # Load ignore configuration
    ignore_config = load_ignore_config()

    # Add command line ignores
    ignore_config["files"].extend(args.ignore_files)
    ignore_config["patterns"].extend(args.ignore_patterns)

    # Get all markdown files
    md_files = list(Path(".").rglob("*.md"))

    # Process each file
    processed_count = 0
    for md_file in md_files:
        # Skip ignored files
        if should_ignore_file(md_file, ignore_config):
            print(f"Skipping {md_file} (ignored)")
            continue

        print(f"Processing {md_file}")
        insert_toc(md_file)
        processed_count += 1

    print(f"\nProcessed {processed_count} files")


if __name__ == "__main__":
    main()
