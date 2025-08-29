# Changelog

<!-- TOC start -->
## Table of Contents

- [Unreleased](#unreleased)
  - [Added](#added-1)
  - [Changed](#changed-1)
- [1.3.0 - 2025-08-16](#130---2025-08-16)
  - [Added](#added-2)
  - [Changed](#changed-2)
- [1.2.0 - 2025-08-02](#120---2025-08-02)
  - [Added](#added-3)
  - [Changed](#changed-3)
- [1.1.0 - 2025-07-30](#110---2025-07-30)
  - [Added](#added-4)
  - [Changed](#changed-4)
  - [Refactored](#refactored)
- [1.0.0 - 2025-07-29](#100---2025-07-29)
  - [Added](#added-5)

[Back to Table of Contents](README.md#table-of-contents)
<!-- TOC end -->

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] {#unreleased}

### Added {#added-1}

- **Advanced GitHub Workflow Examples**: Added comprehensive examples for automated GitHub issue and PR management
  - Issue Killer: Implements features based on GitHub issues with automated PR creation
  - Issue Triage: Automatically labels new issues with appropriate tags
  - PR Review: Provides automated code review for pull requests
  - PR Review Killer: Enables direct code modifications based on PR comments
- **Enhanced Docker Image**: Embedded uv Python package manager in Docker image for faster dependency installation
- **Vibe Ideas Workflow**: Added new workflow for generating project ideas and suggestions
- **Pre-execution Commands Support**: New `precmd` input parameter allows running shell commands before executing iFlow CLI
- **Multi-line Command Support**: Enhanced `precmd` to support multiple shell commands separated by newlines
- **Code Search Enhancement**: Added ripgrep to Dockerfile for better code search capabilities
- **QuickStart Documentation**: Added QuickStart guide for iflow-cli-action
- **Commit Command Configuration**: Added commit command config and examples in README [zh]

### Changed {#changed-1}

- **Dockerfile Improvements**: Updated CI image configuration and Dockerfile reference
- **Repository Updates**: Updated repository URLs and references
- **Dependency Management**: Added more dependencies to Dockerfile as requested in issue #7
- **Documentation**: Updated README files and added comprehensive documentation for new workflow examples
- **Security**: Added id-token permission for deploy workflow
- **Performance**: Reduced the number of image layers in the Dockerfile
- **Configuration**: Extracted hard-coded bot name to configurable repo variable
- **Node.js Installation**: Added Node.js installation to Dockerfile for npm availability
- **Permission Management**: Removed unused id-token permission
- **Project Metadata**: Updated project metadata and CI settings

## [1.3.0] - 2025-08-16 {#130---2025-08-16}

### Added {#added-2}

- **Pre-execution Commands Support**: New `precmd` input parameter allows running shell commands before executing iFlow CLI, useful for setting up environment or installing dependencies
- **Multi-line Command Support**: Enhanced `precmd` to support multiple shell commands separated by newlines
- **Enhanced Documentation**: Comprehensive documentation and examples for using `precmd` and `extra_args` features
- **Gemini CLI Action Reference**: Added reference to Gemini CLI GitHub Action in documentation

### Changed {#changed-2}

- **Improved Examples**: Enhanced example workflows with better security controls and trust verification
- **Workflow Naming**: Renamed workflows for better clarity and organization
- **Documentation Updates**: Updated README files with detailed information about new features and usage patterns
- **PR Review Workflow**: Enhanced pull request review workflow with improved security checks and trust verification

## [1.2.0] - 2025-08-02 {#120---2025-08-02}

### Added {#added-3}

- **Extra Arguments Support**: New `extra_args` input parameter allows passing additional command-line arguments to iFlow CLI
- Enhanced argument parsing with support for quoted arguments containing spaces
- Dynamic argument support for workflow inputs
- Updated documentation with extra_args examples and use cases
- New example workflow demonstrating extra_args functionality
- iFlow CLI version display functionality
- GitHub Actions workflow for iFlow with MCP (Model Context Protocol) support

### Changed {#changed-3}

- Updated command execution logic to support additional arguments
- Enhanced GitHub Actions step summary to display extra arguments
- Improved configuration display in workflow summaries

## [1.1.0] - 2025-07-30 {#110---2025-07-30}

### Added {#added-4}

- Chinese documentation (README_zh.md)
- IFLOW.md development guide
- Examples for code review and documentation workflows

### Changed {#changed-4}

- Updated iFlow CLI version in Dockerfile
- Updated timeout format in deploy workflow
- Improved homepage generation instructions
- Updated deploy homepage workflow prompt
- Updated iflow-cli-action version in README.md

### Refactored {#refactored}

- Moved timeout check before error handling in executeIFlow function
- Updated timeout input range and added flag in config struct

## [1.0.0] - 2025-07-29 {#100---2025-07-29}

### Added {#added-5}

- Initial release of iFlow CLI GitHub Action
- Docker-based action with pre-installed Node.js 22 and iFlow CLI
- Support for configurable authentication with iFlow API
- Support for custom models and API endpoints
- Flexible command execution with timeout control
- GitHub Actions Summary integration for rich execution reports
- Action outputs for result and exit code
