# IFLOW.md

<!-- TOC start -->
## Table of Contents

- [Project Overview](#project-overview)
  - [Key Capabilities](#key-capabilities)
- [Architecture](#architecture)
  - [Core Components](#core-components)
  - [Key Features](#key-features)
- [Development Commands](#development-commands)
  - [Building](#building)
  - [Testing](#testing)
  - [Running](#running)
    - [CLI Mode](#cli-mode)
    - [GitHub Actions Mode](#github-actions-mode)
- [Code Structure](#code-structure)
  - [Configuration Management](#configuration-management)
  - [iFlow CLI Integration](#iflow-cli-integration)
  - [GitHub Actions Features](#github-actions-features)
- [Configuration](#configuration)
  - [Input Parameters](#input-parameters)
  - [Output Variables](#output-variables)
- [Advanced Features](#advanced-features)
  - [Pre-execution Commands (`precmd`)](#pre-execution-commands-precmd)
  - [Extra Arguments (`extra_args`)](#extra-arguments-extra_args)
  - [Custom Settings JSON](#custom-settings-json)
- [Common Development Tasks](#common-development-tasks)
  - [Adding New Configuration Options](#adding-new-configuration-options)
  - [Modifying iFlow Execution](#modifying-iflow-execution)
  - [Enhancing GitHub Summary Output](#enhancing-github-summary-output)
  - [Docker Image Updates](#docker-image-updates)
- [Testing Changes](#testing-changes)
  - [Local Testing](#local-testing)
    - [Go Binary Testing](#go-binary-testing)
    - [Docker Testing](#docker-testing)
  - [GitHub Actions Testing](#github-actions-testing)
    - [Workflow Testing](#workflow-testing)
- [GitHub Actions Integration](#github-actions-integration)
  - [Available Workflows](#available-workflows)
  - [Integration Patterns](#integration-patterns)
    - [Basic Integration](#basic-integration)
    - [Advanced Integration](#advanced-integration)
- [Troubleshooting](#troubleshooting)
  - [Common Issues](#common-issues)
    - [Timeout Errors](#timeout-errors)
    - [Authentication Errors](#authentication-errors)
    - [Docker Issues](#docker-issues)
    - [Command Execution Issues](#command-execution-issues)
  - [Debug Mode](#debug-mode)
  - [Performance Optimization](#performance-optimization)
  - [Security Considerations](#security-considerations)

[Back to Table of Contents](README.md#table-of-contents)
<!-- TOC end -->

This file provides comprehensive guidance for iFlow CLI when working with code in this repository.

## Project Overview

This repository is a production-ready GitHub Action that wraps the iFlow CLI, enabling users to run intelligent code analysis and automation commands within GitHub workflows. The action is built with Go and packaged in a multi-stage Docker container that includes Node.js 24, uv (ultra-fast Python package manager), GitHub CLI, and the iFlow CLI.

### Key Capabilities

- **Intelligent Code Analysis**: Run AI-powered code reviews, documentation generation, and issue triage
- **Workflow Automation**: Automate GitHub issues, PR reviews, and project management tasks
- **Flexible Configuration**: Support for both CLI flags and GitHub Actions environment variables
- **Rich Reporting**: Generate detailed execution summaries with GitHub Actions integration
- **Multi-language Support**: Pre-configured environment for Go, Python, Node.js, and more

## Architecture

### Core Components

1. **Entry Points**:
   - `main.go`: Minimal entry point that delegates to Cobra command structure
   - `cmd/root.go`: Comprehensive command implementation with full feature set

2. **Docker Infrastructure**:
   - **Multi-stage build**: Optimized for size and performance
   - **Ubuntu 22.04 base**: Production-ready with security updates
   - **Pre-installed tools**: Node.js 24, npm, uv, GitHub CLI, Go 1.23.2, xmake
   - **Non-root user**: Runs as `iflow` user for security

3. **GitHub Actions Integration**:
   - `action.yml`: Complete action definition with 10+ configurable inputs
   - Rich step summaries with emoji indicators and troubleshooting hints
   - Automatic output handling for GitHub Actions workflows

### Key Features

- **Docker-based action** with pre-installed development toolchain
- **Configurable authentication** with iFlow API (API key or settings JSON)
- **Advanced timeout handling** (1-86400 seconds) with graceful degradation
- **Pre-execution commands** support for environment setup
- **Extra arguments** support for iFlow CLI customization
- **Real-time output streaming** during execution
- **Comprehensive error handling** with actionable troubleshooting guidance

## Development Commands

### Building

```bash
# Build the Go binary locally
go build -o iflow-action .

# Build Docker image for testing
docker build -t iflow-cli-action .

# Build with specific Go version
go build -ldflags="-s -w" -o iflow-action .
```

### Testing

```bash
# Run all Go tests
go test ./...

# Run tests with verbose output
go test -v ./...

# Run specific test
go test -v ./cmd/...
```

### Running

#### CLI Mode

```bash
# Basic execution
./iflow-action --prompt "Analyze this code" --api-key YOUR_API_KEY

# With custom model and timeout
./iflow-action --prompt "Review this PR" --api-key YOUR_API_KEY --model "Qwen3-Coder" --timeout 1800

# With pre-execution commands
./iflow-action --prompt "Generate docs" --api-key YOUR_API_KEY --precmd "npm install && npm run build"

# With extra arguments
./iflow-action --prompt "Analyze code" --api-key YOUR_API_KEY --extra-args "--verbose --format json"
```

#### GitHub Actions Mode

```bash
# Test with environment variables
INPUT_PROMPT="Analyze this code" INPUT_API_KEY=your-key ./iflow-action --use-env-vars=true

# With pre-execution setup
INPUT_PROMPT="Run tests" INPUT_API_KEY=your-key INPUT_PRECMD="npm ci" ./iflow-action --use-env-vars=true

# With custom settings JSON
INPUT_SETTINGS_JSON='{"theme":"Default","selectedAuthType":"iflow","apiKey":"your-key"}' ./iflow-action --use-env-vars=true
```

## Code Structure

### Configuration Management

- **Dual-mode configuration**: CLI flags and GitHub Actions environment variables
- **Validation**: Comprehensive input validation with helpful error messages
- **Settings JSON**: Support for complete custom configuration via JSON
- **Environment detection**: Automatic GitHub Actions environment detection

### iFlow CLI Integration

- **Automatic installation**: Pre-installed via npm in Docker image
- **Settings configuration**: Automatic `~/.iflow/settings.json` generation
- **Command execution**: Non-interactive mode with `--yolo` and `--prompt` flags
- **Output handling**: Real-time streaming with buffer capture

### GitHub Actions Features

- **Output variables**: `result` and `exit_code` via `GITHUB_OUTPUT`
- **Step summaries**: Rich Markdown summaries with emoji indicators
- **Error annotations**: GitHub Actions compatible error and notice messages
- **Timeout handling**: Graceful timeout detection with troubleshooting hints

## Configuration

### Input Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `prompt` | string | - | **Required** The prompt to execute with iFlow CLI |
| `api_key` | string | - | iFlow API key for authentication |
| `settings_json` | string | - | Complete iFlow settings.json content (overrides other config) |
| `base_url` | string | `https://apis.iflow.cn/v1` | Custom base URL for iFlow API |
| `model` | string | `Qwen3-Coder` | Model name to use |
| `working_directory` | string | `.` | Working directory to run from |
| `timeout` | int | `3600` | Timeout in seconds (1-86400) |
| `extra_args` | string | - | Additional CLI arguments for iFlow |
| `precmd` | string | - | Shell commands to run before iFlow |

### Output Variables

| Variable | Description |
|----------|-------------|
| `result` | Complete output from iFlow CLI execution |
| `exit_code` | Exit code from iFlow CLI (0 = success) |

## Advanced Features

### Pre-execution Commands (`precmd`)

Execute shell commands before running iFlow CLI:

```yaml
- uses: iflow-ai/iflow-cli-action@main
  with:
    prompt: "Analyze the codebase"
    precmd: |
      npm install
      npm run build
      echo "Environment ready"
```

### Extra Arguments (`extra_args`)

Pass additional arguments to iFlow CLI:

```yaml
- uses: iflow-ai/iflow-cli-action@main
  with:
    prompt: "Review code"
    extra_args: "--verbose --format markdown --max-tokens 2000"
```

### Custom Settings JSON

Provide complete configuration via JSON:

```yaml
- uses: iflow-ai/iflow-cli-action@main
  with:
    prompt: "Generate documentation"
    settings_json: |
      {
        "theme": "Default",
        "selectedAuthType": "iflow",
        "apiKey": "${{ secrets.IFLOW_API_KEY }}",
        "baseUrl": "https://custom-api.example.com",
        "modelName": "Custom-Model"
      }
```

## Common Development Tasks

### Adding New Configuration Options

1. **Add CLI flag** in `cmd/root.go` init function
2. **Update Config struct** with new field
3. **Add environment variable support** in `loadConfigFromEnv`
4. **Update action.yml** with new input definition
5. **Add validation logic** in `validateConfig`
6. **Update documentation** in README files

### Modifying iFlow Execution

1. **Update `executeIFlow`** function for command changes
2. **Modify argument parsing** in `parseExtraArgs` if needed
3. **Update output handling** in `executeIFlow`
4. **Test with various scenarios** (success, timeout, error)

### Enhancing GitHub Summary Output

1. **Update `generateSummaryMarkdown`** function
2. **Add new sections** for metrics or troubleshooting
3. **Improve formatting** with better Markdown structure
4. **Add emoji indicators** for better visual feedback

### Docker Image Updates

1. **Modify Dockerfile** for new dependencies
2. **Test multi-stage build** locally
3. **Update base image** versions carefully
4. **Verify non-root user** permissions
5. **Test with GitHub Actions** environment

## Testing Changes

### Local Testing

#### Go Binary Testing

```bash
# Build and test
make build
./iflow-action --prompt "Test prompt" --api-key test-key --use-env-vars=false

# Test with timeout
./iflow-action --prompt "Long running task" --api-key test-key --timeout 60

# Test pre-execution commands
./iflow-action --prompt "Test" --api-key test-key --precmd "echo 'Setup complete'"
```

#### Docker Testing

```bash
# Build test image
docker build -t iflow-cli-action:test .

# Test basic functionality
docker run -e INPUT_PROMPT="Test prompt" -e INPUT_API_KEY=test-key iflow-cli-action:test

# Test with volume mounting
docker run -v $(pwd):/workspace -e INPUT_PROMPT="Analyze /workspace" -e INPUT_API_KEY=test-key -e INPUT_WORKING_DIRECTORY=/workspace iflow-cli-action:test

# Test timeout handling
docker run -e INPUT_PROMPT="Test" -e INPUT_API_KEY=test-key -e INPUT_TIMEOUT=5 iflow-cli-action:test

# Test pre-execution commands
docker run -e INPUT_PROMPT="Test" -e INPUT_API_KEY=test-key -e INPUT_PRECMD="ls -la && pwd" iflow-cli-action:test
```

### GitHub Actions Testing

#### Workflow Testing

```yaml
# Create test workflow
name: Test iFlow CLI Action
on: [push, pull_request]

jobs:
  test-action:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: ./
        with:
          prompt: "Test the action"
          api_key: ${{ secrets.IFLOW_API_KEY }}
          precmd: |
            echo "Testing pre-execution"
            ls -la
```

## GitHub Actions Integration

### Available Workflows

The repository includes several example workflows:

1. **Issue Triage** (`issue-triage.yaml`): Automatically label new issues
2. **Issue Killer** (`issue-killer.yml`): Implement features based on GitHub issues
3. **PR Review** (`pr-review.yml`): Automated code review for pull requests
4. **PR Review Killer** (`pr-review-killer.yml`): Direct code modifications via PR comments
5. **Vibe Ideas** (`vibe-ideas.yml`): Generate project ideas and suggestions

### Integration Patterns

#### Basic Integration

```yaml
- uses: iflow-ai/iflow-cli-action@main
  with:
    prompt: "Analyze the codebase for security issues"
    api_key: ${{ secrets.IFLOW_API_KEY }}
```

#### Advanced Integration

```yaml
- uses: iflow-ai/iflow-cli-action@main
  id: analysis
  with:
    prompt: "Generate comprehensive documentation"
    api_key: ${{ secrets.IFLOW_API_KEY }}
    model: "Qwen3-Coder"
    timeout: 7200
    precmd: |
      npm install
      npm run build
    extra_args: "--verbose --format markdown"

- name: Use analysis results
  run: |
    echo "Exit code: ${{ steps.analysis.outputs.exit_code }}"
    echo "Results: ${{ steps.analysis.outputs.result }}"
```

## Troubleshooting

### Common Issues

#### Timeout Errors

- **Symptom**: Exit code 124, execution stops abruptly
- **Solution**: Increase timeout value or optimize prompt
- **Debug**: Check step summary for timeout troubleshooting hints

#### Authentication Errors

- **Symptom**: API key invalid or missing
- **Solution**: Verify API key in repository secrets
- **Debug**: Test with minimal configuration first

#### Docker Issues

- **Symptom**: Container fails to start
- **Solution**: Check Docker image availability and permissions
- **Debug**: Test locally with `docker run`

#### Command Execution Issues

- **Symptom**: Pre-execution commands fail
- **Solution**: Check command syntax and working directory
- **Debug**: Test commands in local Docker container

### Debug Mode

Enable verbose logging by setting environment variables:

```bash
# Local debugging
export INPUT_PROMPT="Debug test"
export INPUT_API_KEY="your-key"
export GITHUB_ACTIONS="true"  # Simulate GitHub Actions
./iflow-action --use-env-vars=true
```

### Performance Optimization

- **Timeout tuning**: Start with shorter timeouts, increase as needed
- **Model selection**: Choose appropriate model for task complexity
- **Pre-execution optimization**: Cache dependencies when possible
- **Output handling**: Use `extra_args` to limit output size

### Security Considerations

- **API key management**: Always use GitHub secrets for API keys
- **Command injection**: Validate all user inputs in `precmd`
- **File permissions**: Ensure proper file permissions in Docker
- **Network security**: Use HTTPS endpoints for API calls
