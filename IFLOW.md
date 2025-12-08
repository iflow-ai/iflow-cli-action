# IFLOW.md

This file provides comprehensive guidance for iFlow CLI when working with code in this repository.

## Project Overview

This repository is a production-ready GitHub Action that wraps the iFlow CLI, enabling users to run intelligent code analysis and automation commands within GitHub workflows. The action is built with Rust and packaged in a multi-stage Docker container that includes Node.js 24, uv (ultra-fast Python package manager), GitHub CLI, and the iFlow CLI.

### Key Capabilities

- **Intelligent Code Analysis**: Run AI-powered code reviews, documentation generation, and issue triage
- **Workflow Automation**: Automate GitHub issues, PR reviews, and project management tasks
- **Flexible Configuration**: Support for both CLI flags and GitHub Actions environment variables
- **Rich Reporting**: Generate detailed execution summaries with GitHub Actions integration
- **Multi-language Support**: Pre-configured environment for Rust, Python, Node.js, and more
- **Version Management**: Configurable versions for GitHub CLI and iFlow CLI

## Architecture

### Core Components

1. **Entry Points**:
   - `src/main.rs`: Main entry point that uses Clap for command-line argument parsing
   - `src/lib.rs`: Library code for generating GitHub Actions summaries

2. **Docker Infrastructure**:
   - **Multi-stage build**: Optimized for size and performance
   - **Ubuntu 22.04 base**: Production-ready with security updates
   - **Pre-installed tools**: Node.js 24, npm, uv, GitHub CLI, Rust 1.90, xmake
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
- **Real-time output streaming** during execution
- **Comprehensive error handling** with actionable troubleshooting guidance
- **Version management** for GitHub CLI and iFlow CLI

## Development Commands

### Building

```bash
# Build the Rust binary locally
cargo build --release

# Build Docker image for testing
docker build -t iflow-cli-action .

# Build with specific Rust version
cargo build --release

# Build with release optimizations
cargo build --release
```

### Testing

```bash
# Run all Rust tests
cargo test

# Run tests with verbose output
cargo test --verbose

# Run specific test
cargo test test_name

# Run tests with coverage
cargo tarpaulin
```

### Running

#### CLI Mode

```bash
# Basic execution
./target/release/iflow-cli-action --prompt "Analyze this code" --api-key YOUR_API_KEY

# With custom model and timeout
./target/release/iflow-cli-action --prompt "Review this PR" --api-key YOUR_API_KEY --model "qwen3-coder-plus" --timeout 1800

# With pre-execution commands
./target/release/iflow-cli-action --prompt "Generate docs" --api-key YOUR_API_KEY --precmd "npm install && npm run build"

# With specific versions
./target/release/iflow-cli-action --prompt "Test" --api-key YOUR_API_KEY --gh-version "2.76.2" --iflow-version "0.2.4"
```

#### GitHub Actions Mode

```bash
# Test with environment variables
INPUT_PROMPT="Analyze this code" INPUT_API_KEY=your-key ./target/release/iflow-cli-action

# With pre-execution setup
INPUT_PROMPT="Run tests" INPUT_API_KEY=your-key INPUT_PRECMD="npm ci" ./target/release/iflow-cli-action

# With custom settings JSON
INPUT_SETTINGS_JSON='{"theme":"Default","selectedAuthType":"iflow","apiKey":"your-key"}' ./target/release/iflow-cli-action

# With version specifications
INPUT_PROMPT="Test" INPUT_API_KEY=your-key INPUT_GH_VERSION="2.76.2" INPUT_IFLOW_VERSION="0.2.4" ./target/release/iflow-cli-action
```

## Code Structure

### Configuration Management

- **Dual-mode configuration**: CLI flags and GitHub Actions environment variables
- **Validation**: Comprehensive input validation with helpful error messages
- **Settings JSON**: Support for complete custom configuration via JSON
- **Environment detection**: Automatic GitHub Actions environment detection
- **Version management**: Configurable tool versions for flexible deployments

### iFlow CLI Integration

- **Automatic installation**: Pre-installed via npm in Docker image
- **Settings configuration**: Automatic `~/.iflow/settings.json` generation
- **Command execution**: Non-interactive mode with `--yolo` and `--prompt` flags
- **Output handling**: Real-time streaming with buffer capture
- **Version control**: Support for specific iFlow CLI versions

### GitHub Actions Features

- **Output variables**: `result` and `exit_code` via `GITHUB_OUTPUT`
- **Step summaries**: Rich Markdown summaries with emoji indicators
- **Error annotations**: GitHub Actions compatible error and notice messages
- **Timeout handling**: Graceful timeout detection with troubleshooting hints
- **Concurrent execution**: Safe concurrent workflow execution

## Configuration

### Input Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `prompt` | string | - | **Required** The prompt to execute with iFlow CLI |
| `api_key` | string | - | iFlow API key for authentication |
| `settings_json` | string | - | Complete iFlow settings.json content (overrides other config) |
| `base_url` | string | `https://apis.iflow.cn/v1` | Custom base URL for iFlow API |
| `model` | string | `qwen3-coder-plus` | Model name to use |
| `working_directory` | string | `.` | Working directory to run from |
| `timeout` | int | `3600` | Timeout in seconds (1-86400) |
| `precmd` | string | - | Shell commands to run before iFlow |
| `gh_version` | string | - | Version of GitHub CLI to install (e.g., "2.76.2") |
| `iflow_version` | string | - | Version of iFlow CLI to install (e.g., "0.2.4") |
| `debug` | boolean | `false` | Enable debug logging |
| `dry_run` | boolean | `false` | Dry run mode for E2E testing |

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

### Version Management

Specify exact versions for GitHub CLI and iFlow CLI:

```yaml
- uses: iflow-ai/iflow-cli-action@main
  with:
    prompt: "Analyze code"
    api_key: ${{ secrets.IFLOW_API_KEY }}
    gh_version: "2.76.2"
    iflow_version: "0.2.4"
```

## Common Development Tasks

### Adding New Configuration Options

1. **Add CLI flag** in `src/main.rs` using Clap attributes
2. **Update validation logic** in the `validate` method
3. **Update action.yml** with new input definition
4. **Update documentation** in README files

### Modifying iFlow Execution

1. **Update execution logic** in `src/main.rs` for command changes
2. **Modify argument parsing** if needed
3. **Update output handling** in execution functions
4. **Test with various scenarios** (success, timeout, error)

### Enhancing GitHub Summary Output

1. **Update `generate_summary_markdown`** function in `src/lib.rs`
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

#### Rust Binary Testing

```bash
# Build and test
cargo build --release
./target/release/iflow-cli-action --prompt "Test prompt" --api-key test-key

# Test with timeout
./target/release/iflow-cli-action --prompt "Long running task" --api-key test-key --timeout 60

# Test pre-execution commands
./target/release/iflow-cli-action --prompt "Test" --api-key test-key --precmd "echo 'Setup complete'"

# Test version specifications
./target/release/iflow-cli-action --prompt "Test" --api-key test-key --gh-version "2.76.2" --iflow-version "0.2.4"
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

# Test version specifications
docker run -e INPUT_PROMPT="Test" -e INPUT_API_KEY=test-key -e INPUT_GH_VERSION="2.76.2" -e INPUT_IFLOW_VERSION="0.2.4" iflow-cli-action:test
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
          gh_version: "2.76.2"
          iflow_version: "0.2.4"
```

## GitHub Actions Integration

### Available Workflows

The repository includes several example workflows:

1. **iFlow CLI Assistant** (`iflow-cli-assistant.yml`): AI-powered PR and issue assistance
2. **iFlow with MCP** (`iflow-with-mcp.yml`): Model Context Protocol integration
3. **Issue Triage** (`issue-triage.yaml`): Automatically label new issues
4. **Issue Killer** (`issue-killer.yml`): Implement features based on GitHub issues
5. **PR Review** (`pr-review.yml`): Automated code review for pull requests
6. **PR Review Killer** (`pr-review-killer.yml`): Direct code modifications via PR comments
7. **Vibe Ideas** (`vibe-ideas.yml`): Generate project ideas and suggestions
8. **Unit Tests** (`unittest.yml`): Automated testing
9. **Release CI Image** (`release-ci-image.yml`): Docker image building and publishing
10. **Deploy Homepage** (`deploy-homepage.yml`): Documentation deployment

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
    model: "qwen3-coder-plus"
    timeout: 7200
    precmd: |
      npm install
      npm run build
    gh_version: "2.76.2"
    iflow_version: "0.2.4"

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

#### Version Compatibility Issues

- **Symptom**: GitHub CLI or iFlow CLI version conflicts
- **Solution**: Use specific version parameters or default versions
- **Debug**: Check version compatibility matrix

### Debug Mode

Enable verbose logging by setting environment variables:

```bash
# Local debugging
export INPUT_PROMPT="Debug test"
export INPUT_API_KEY="your-key"
export GITHUB_ACTIONS="true"  # Simulate GitHub Actions
./target/release/iflow-cli-action

# With specific versions for testing
export INPUT_GH_VERSION="2.76.2"
export INPUT_IFLOW_VERSION="0.2.4"
```

### Performance Optimization

- **Timeout tuning**: Start with shorter timeouts, increase as needed
- **Model selection**: Choose appropriate model for task complexity
- **Pre-execution optimization**: Cache dependencies when possible
- **Version management**: Use compatible tool versions

### Security Considerations

- **API key management**: Always use GitHub secrets for API keys
- **Command injection**: Validate all user inputs in `precmd`
- **File permissions**: Ensure proper file permissions in Docker
- **Network security**: Use HTTPS endpoints for API calls
- **Version pinning**: Pin specific versions for reproducible builds
