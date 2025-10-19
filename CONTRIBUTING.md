# Developer Guide

First, thank you for contributing to iflow-cli-action! The goal of this document is to provide everything you need to start contributing to iflow-cli-action. The following TOC is sorted progressively, starting with the basics and expanding into more specifics.

<!-- TOC start -->
## Table of Contents

- [Your First Contribution](#your-first-contribution)
- [Workflow](#workflow)
  - [Git Branches](#git-branches)
  - [GitHub Pull Requests](#github-pull-requests)
    - [Title](#title)
    - [Reviews and Approvals](#reviews-and-approvals)
  - [CI](#ci)
- [Development Setup](#development-setup)
  - [Running tests](#running-tests)
- [Documentation Tools](#documentation-tools)
  - [Table of Contents Generator](#table-of-contents-generator)
    - [Usage](#usage)
    - [Configuration](#configuration)
    - [Default Ignores](#default-ignores)

[Back to Table of Contents](README.md#table-of-contents)
<!-- TOC end -->

## Your First Contribution

1. Ensure your change has an issue! Find an [existing issue](https://github.com/iflow-ai/iflow-cli-action/issues) or [open a new issue](https://github.com/iflow-ai/iflow-cli-action/issues/new).
1. [Fork the iflow-cli-action repository](https://github.com/iflow-ai/iflow-cli-action/fork) in your own GitHub account.
1. [Create a new Git branch](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/creating-and-deleting-branches-within-your-repository).
1. Make your changes.
1. [Submit the branch as a pull request](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/creating-a-pull-request-from-a-fork) to the main iflow-cli-action repo. An iflow-cli-action team member should comment and/or review your pull request within a few days. Although, depending on the circumstances, it may take longer.

## Workflow

### Git Branches

*All* changes must be made in a branch and submitted as [pull requests](#github-pull-requests). iflow-cli-action does not adopt any type of branch naming style, but please use something descriptive of your changes.

### GitHub Pull Requests

Once your changes are ready you must submit your branch as a [pull request](https://github.com/iflow-ai/iflow-cli-action/pulls).

#### Title

The pull request title must follow the format outlined in the [conventional commits spec](https://www.conventionalcommits.org). [Conventional commits](https://www.conventionalcommits.org) is a standardized format for commit messages. iflow-cli-action only requires this format for commits on the `main` branch. And because iflow-cli-action squashes commits before merging branches, this means that only the pull request title must conform to this format.

The following are all good examples of pull request titles:

```text
feat(services/gcs): Add start-after support for list
docs: add hdfs classpath related troubleshoot
ci: Mark job as skipped if owner is not apache
fix(services/s3): Ignore prefix if it's empty
refactor: Polish the implementation of webhdfs
```

#### Reviews and Approvals

All pull requests should be reviewed by at least one iflow-cli-action committer.

### CI

Currently, iflow-cli-action uses GitHub Actions to run tests. The workflows are defined in `.github/workflows`.

## Development Setup

iflow-cli-action is primarily a Rust project. To build iflow-cli-action, you will need to set up Rust development first. We highly recommend using [rustup](https://rustup.rs/) for the setup process.

For Linux or macOS, use the following command:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

For Windows, download `rustup-init.exe` from [win here](https://win.rustup.rs/x86_64) instead.

Rustup will read iflow-cli-action's `rust-toolchain.toml` and set up everything else automatically. To ensure that everything works correctly, run `cargo version` under iflow-cli-action's root directory:

```shell
$ cargo version
cargo 1.85.0 (stable)
```

### Running tests

Once you have completed your work, you can run the tests by running the following command:

```shell
cargo test
```

Ensure all tests pass before submitting a pull request. And add new tests if necessary.

## Documentation Tools

This repository includes a Python script `generate_toc.py` to automatically generate and update table of contents for all markdown files. if you have any documentation changes, please run the script before submitting a pull request.

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
