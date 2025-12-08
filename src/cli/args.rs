use clap::Parser;

/// iFlow CLI Action Command Line Interface
#[derive(Parser, Debug)]
#[clap(
    name = "iflow-cli-action",
    version = "2.0.0",
    about = "A GitHub Action ACP CLI for iFlow CLI that provides intelligent code assistance.",
    long_about = r#"A GitHub Action ACP CLI for iFlow CLI that provides intelligent code assistance.

This tool can run in two modes:
1. GitHub Actions mode: Uses environment variables (INPUT_*) for configuration
2. CLI mode: Uses command-line flags for configuration"#
)]
pub struct CliArgs {
    /// The prompt to send to iFlow CLI (required in CLI mode)
    #[clap(short, long, env = "INPUT_PROMPT")]
    pub prompt: Option<String>,

    /// API key for iFlow authentication
    #[clap(long, env = "INPUT_API_KEY")]
    pub api_key: Option<String>,

    /// Enable debug logging
    #[clap(long, env = "INPUT_DEBUG")]
    pub debug: bool,

    /// Complete settings JSON configuration
    #[clap(long, env = "INPUT_SETTINGS_JSON")]
    pub settings_json: Option<String>,

    /// Base URL for the iFlow API
    #[clap(
        long,
        env = "INPUT_BASE_URL",
        default_value = "https://apis.iflow.cn/v1"
    )]
    pub base_url: String,

    /// Model name to use
    #[clap(long, env = "INPUT_MODEL", default_value = "qwen3-coder-plus")]
    pub model: String,

    /// Working directory for execution
    #[clap(long, env = "INPUT_WORKING_DIRECTORY", default_value = ".")]
    pub working_directory: String,

    /// Timeout in seconds (1-86400)
    #[clap(long, env = "INPUT_TIMEOUT", default_value = "3600")]
    pub timeout: u32,

    /// Shell command(s) to execute before running iFlow CLI
    #[clap(long, env = "INPUT_PRECMD")]
    pub precmd: Option<String>,

    /// Version of GitHub CLI to install
    #[clap(long, env = "INPUT_GH_VERSION")]
    pub gh_version: Option<String>,

    /// Version of iFlow CLI to install
    #[clap(long, env = "INPUT_IFLOW_VERSION")]
    pub iflow_version: Option<String>,

    /// Path to the settings file (for testing purposes)
    #[clap(long, env = "SETTINGS_FILE_PATH")]
    pub settings_file_path: Option<String>,

    /// Dry run mode for E2E testing (skips actual execution)
    #[clap(long, env = "INPUT_DRY_RUN")]
    pub dry_run: bool,
}
