use clap::Parser;
use std::fs;
use std::path::Path;

/// iFlow CLI Action wrapper
#[derive(Parser, Debug)]
#[clap(
    name = "iflow-action",
    version = "0.1.0",
    about = "A GitHub Action wrapper for iFlow CLI that provides intelligent code assistance.",
    long_about = r#"A GitHub Action wrapper for iFlow CLI that provides intelligent code assistance.

This tool can run in two modes:
1. GitHub Actions mode: Uses environment variables (INPUT_*) for configuration
2. CLI mode: Uses command-line flags for configuration"#
)]
struct Cli {
    /// The prompt to send to iFlow CLI (required in CLI mode)
    #[clap(short, long, env = "INPUT_PROMPT")]
    prompt: Option<String>,

    /// API key for iFlow authentication
    #[clap(long, env = "INPUT_API_KEY")]
    api_key: Option<String>,

    /// Complete settings JSON configuration
    #[clap(long, env = "INPUT_SETTINGS_JSON")]
    settings_json: Option<String>,

    /// Base URL for the iFlow API
    #[clap(long, env = "INPUT_BASE_URL", default_value = "https://apis.iflow.cn/v1")]
    base_url: String,

    /// Model name to use
    #[clap(long, env = "INPUT_MODEL", default_value = "Qwen3-Coder")]
    model: String,

    /// Working directory for execution
    #[clap(long, env = "INPUT_WORKING_DIRECTORY", default_value = ".")]
    working_directory: String,

    /// Timeout in seconds (1-86400)
    #[clap(long, env = "INPUT_TIMEOUT", default_value = "3600")]
    timeout: u32,

    /// Additional command line arguments to pass to iFlow CLI
    #[clap(long, env = "INPUT_EXTRA_ARGS")]
    extra_args: Option<String>,

    /// Shell command(s) to execute before running iFlow CLI
    #[clap(long, env = "INPUT_PRECMD")]
    precmd: Option<String>,

    /// Version of GitHub CLI to install
    #[clap(long, env = "INPUT_GH_VERSION")]
    gh_version: Option<String>,

    /// Version of iFlow CLI to install
    #[clap(long, env = "INPUT_IFLOW_VERSION")]
    iflow_version: Option<String>,

    /// Use environment variables for configuration (GitHub Actions mode)
    #[clap(long, env = "USE_ENV_VARS")]
    use_env_vars: bool,
    
    /// Path to the settings file (for testing purposes)
    #[clap(long, env = "SETTINGS_FILE_PATH")]
    settings_file_path: Option<String>,
}

impl Cli {
    /// Validates the CLI arguments according to the same rules as the Go implementation
    fn validate(&self) -> Result<(), String> {
        // Validate required inputs
        if self.prompt.is_none() || self.prompt.as_ref().unwrap().is_empty() {
            return Err("prompt input is required and cannot be empty".to_string());
        }

        if self.api_key.is_none() && self.settings_json.is_none() {
            return Err("api_key input is required and cannot be empty".to_string());
        }

        // Validate timeout range (1 second to 24 hours)
        if self.timeout < 1 || self.timeout > 86400 {
            return Err("timeout value is out of range. Must be between 1 and 86400 seconds".to_string());
        }

        // Validate settings_json if provided
        if let Some(ref settings_json) = self.settings_json {
            if !settings_json.is_empty() {
                serde_json::from_str::<serde_json::Value>(settings_json)
                    .map_err(|e| format!("invalid settings_json provided: {}", e))?;
            }
        }

        Ok(())
    }

    /// Configures iFlow settings, similar to the Go implementation
    fn configure(&self) -> Result<(), String> {
        // Determine the settings file path
        let settings_file_path = if let Some(ref path) = self.settings_file_path {
            path.clone()
        } else {
            // Get home directory
            let home_dir = dirs::home_dir().ok_or("failed to get home directory")?;
            
            // Create .iflow directory
            let iflow_dir = home_dir.join(".iflow");
            fs::create_dir_all(&iflow_dir)
                .map_err(|e| format!("failed to create .iflow directory: {}", e))?;
            
            // Path to settings.json file
            let settings_file = iflow_dir.join("settings.json");
            settings_file.to_string_lossy().to_string()
        };
        
        let settings_data = if let Some(ref settings_json) = self.settings_json {
            if !settings_json.is_empty() {
                // Use provided settings JSON directly
                // Pretty format the JSON
                let parsed: serde_json::Value = serde_json::from_str(settings_json)
                    .map_err(|e| format!("invalid settings_json provided: {}", e))?;
                serde_json::to_string_pretty(&parsed)
                    .map_err(|e| format!("failed to format settings JSON: {}", e))?
            } else {
                // Create settings from individual parameters
                self.create_settings_from_params()?
            }
        } else {
            // Create settings from individual parameters
            self.create_settings_from_params()?
        };
        
        // Write settings to file
        // Ensure the parent directory exists
        if let Some(parent) = Path::new(&settings_file_path).parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("failed to create parent directory: {}", e))?;
        }
        
        fs::write(&settings_file_path, settings_data)
            .map_err(|e| format!("failed to write settings file: {}", e))?;
        
        Ok(())
    }
    
    /// Creates settings JSON from individual parameters, similar to the Go implementation
    fn create_settings_from_params(&self) -> Result<String, String> {
        let settings = serde_json::json!({
            "theme": "Default",
            "selectedAuthType": "iflow",
            "apiKey": self.api_key.as_ref().unwrap_or(&String::new()),
            "baseUrl": self.base_url,
            "modelName": self.model,
            "searchApiKey": self.api_key.as_ref().unwrap_or(&String::new())
        });
        
        serde_json::to_string_pretty(&settings)
            .map_err(|e| format!("failed to marshal settings: {}", e))
    }
}

fn main() {
    let cli = Cli::parse();
    
    // Validate the arguments
    if let Err(e) = cli.validate() {
        eprintln!("Validation Error: {}", e);
        std::process::exit(1);
    }
    
    // Configure iFlow settings
    if let Err(e) = cli.configure() {
        eprintln!("Configuration Error: {}", e);
        std::process::exit(1);
    }
    
    // Print the parsed arguments for verification
    println!("Parsed arguments:");
    println!("  prompt: {:?}", cli.prompt);
    println!("  api_key: {:?}", cli.api_key);
    println!("  settings_json: {:?}", cli.settings_json);
    println!("  base_url: {}", cli.base_url);
    println!("  model: {}", cli.model);
    println!("  working_directory: {}", cli.working_directory);
    println!("  timeout: {}", cli.timeout);
    println!("  extra_args: {:?}", cli.extra_args);
    println!("  precmd: {:?}", cli.precmd);
    println!("  gh_version: {:?}", cli.gh_version);
    println!("  iflow_version: {:?}", cli.iflow_version);
    println!("  use_env_vars: {}", cli.use_env_vars);
}