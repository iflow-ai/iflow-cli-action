use clap::Parser;
use std::fs;
use std::env;
use iflow_cli_action::generate_summary_markdown;

/// iFlow CLI Action wrapper
#[derive(Parser, Debug)]
#[clap(
    name = "iflow-action",
    version = "0.1.0",
    about = "A GitHub Action ACP CLI for iFlow CLI that provides intelligent code assistance.",
    long_about = r#"A GitHub Action ACP CLI for iFlow CLI that provides intelligent code assistance.

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

    /// Enable debug logging
    #[clap(long, env = "INPUT_DEBUG")]
    debug: bool,

    /// Complete settings JSON configuration
    #[clap(long, env = "INPUT_SETTINGS_JSON")]
    settings_json: Option<String>,

    /// Base URL for the iFlow API
    #[clap(
        long,
        env = "INPUT_BASE_URL",
        default_value = "https://apis.iflow.cn/v1"
    )]
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

    /// Path to the settings file (for testing purposes)
    #[clap(long, env = "SETTINGS_FILE_PATH")]
    settings_file_path: Option<String>,

    /// Dry run mode for E2E testing (skips actual execution)
    #[clap(long, env = "DRY_RUN")]
    dry_run: bool,
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
            return Err(
                "timeout value is out of range. Must be between 1 and 86400 seconds".to_string(),
            );
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
        if let Some(parent) = std::path::Path::new(&settings_file_path).parent() {
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

    /// Gets the version of a command by running it with --version flag
    fn get_command_version(command: &str) -> Result<String, String> {
        let output = std::process::Command::new(command)
            .arg("--version")
            .output()
            .map_err(|e| format!("failed to execute {} --version: {}", command, e))?;

        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout);
            Ok(version.trim().to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(format!("failed to get {} version: {}", command, error))
        }
    }

    /// Prints version information as GitHub Actions annotations
    fn print_version_info(&self) {
        // Check if we're in GitHub Actions environment
        let is_github_actions = std::env::var("GITHUB_ACTIONS").is_ok();

        // Print iFlow CLI version
        match Self::get_command_version("iflow") {
            Ok(version) => {
                if is_github_actions {
                    println!("::notice::iFlow CLI version: {}", version);
                } else {
                    println!("INFO: iFlow CLI version: {}", version);
                }
            }
            Err(e) => {
                if is_github_actions {
                    println!("::warning::Failed to get iFlow CLI version: {}", e);
                } else {
                    eprintln!("WARNING: Failed to get iFlow CLI version: {}", e);
                }
            }
        }

        // Print GitHub CLI version
        match Self::get_command_version("gh") {
            Ok(version) => {
                if is_github_actions {
                    println!("::notice::GitHub CLI version: {}", version);
                } else {
                    println!("INFO: GitHub CLI version: {}", version);
                }
            }
            Err(e) => {
                if is_github_actions {
                    println!("::warning::Failed to get GitHub CLI version: {}", e);
                } else {
                    eprintln!("WARNING: Failed to get GitHub CLI version: {}", e);
                }
            }
        }
    }

    /// Executes pre-command if specified
    fn execute_precmd(&self) -> Result<(), String> {
        if let Some(ref precmd) = self.precmd {
            if !precmd.is_empty() {
                // Split the precmd into lines and execute each line
                let commands: Vec<&str> = precmd.split('\n').collect();

                for command in commands {
                    // Skip empty lines
                    let command = command.trim();
                    if command.is_empty() {
                        continue;
                    }

                    println!("Executing pre-command: {}", command);

                    // Create a command to execute the pre-command
                    let output = std::process::Command::new("sh")
                        .arg("-c")
                        .arg(command)
                        .current_dir(&self.working_directory)
                        .output()
                        .map_err(|e| format!("failed to execute pre-command '{}': {}", command, e))?;

                    // Print stdout and stderr
                    if !output.stdout.is_empty() {
                        println!("{}", String::from_utf8_lossy(&output.stdout));
                    }
                    if !output.stderr.is_empty() {
                        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                    }

                    // Check if command failed
                    if !output.status.success() {
                        return Err(format!(
                            "pre-command '{}' failed with exit code: {:?}",
                            command,
                            output.status.code()
                        ));
                    }
                }
            }
        }

        Ok(())
    }

    /// Installs specific versions of GitHub CLI and iFlow CLI if requested
    fn install_specific_versions(&self) -> Result<(), String> {
        // Install specific GitHub CLI version if requested
        if let Some(ref gh_version) = self.gh_version {
            if !gh_version.is_empty() {
                println!("Installing GitHub CLI version: {}", gh_version);
                
                let install_cmd = format!(
                    "curl -fsSL https://github.com/cli/cli/releases/download/v{0}/gh_{0}_linux_amd64.tar.gz | tar xz && cp gh_{0}_linux_amd64/bin/gh /usr/local/bin/ && rm -rf gh_{0}_linux_amd64",
                    gh_version
                );
                
                let output = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(&install_cmd)
                    .output()
                    .map_err(|e| format!("failed to execute GitHub CLI installation command: {}", e))?;

                if !output.status.success() {
                    let error = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("failed to install GitHub CLI version {}: {}", gh_version, error));
                }
                
                println!("✅ Successfully installed GitHub CLI version: {}", gh_version);
            }
        }

        // Install specific iFlow CLI version if requested
        if let Some(ref iflow_version) = self.iflow_version {
            if !iflow_version.is_empty() {
                println!("Installing iFlow CLI version: {}", iflow_version);
                
                let output = std::process::Command::new("npm")
                    .arg("install")
                    .arg("-g")
                    .arg(format!("@iflow-ai/iflow-cli@{}", iflow_version))
                    .output()
                    .map_err(|e| format!("failed to execute iFlow CLI installation command: {}", e))?;

                if !output.status.success() {
                    let error = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("failed to install iFlow CLI version {}: {}", iflow_version, error));
                }
                
                println!("✅ Successfully installed iFlow CLI version: {}", iflow_version);
            }
        }

        Ok(())
    }

    /// Writes content to GitHub Actions step summary
    pub fn write_step_summary(content: &str) -> Result<(), String> {
        // Check if we're in GitHub Actions environment
        if std::env::var("GITHUB_ACTIONS").is_err() {
            // Not in GitHub Actions environment, nothing to do
            return Ok(());
        }

        // Get the summary file path from environment variable
        let summary_file = match std::env::var("GITHUB_STEP_SUMMARY") {
            Ok(file) => file,
            Err(_) => {
                // Summary not supported or not available
                return Ok(());
            }
        };

        // Write content to the summary file
        std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(&summary_file)
            .map_err(|e| format!("failed to open step summary file: {}", e))
            .and_then(|mut file| {
                use std::io::Write;
                file.write_all(content.as_bytes())
                    .map_err(|e| format!("failed to write to step summary: {}", e))
            })
    }

    /// Run iFlow using WebSocket client
    async fn run_websocket(&self) -> Result<(), String> {
        use futures::stream::StreamExt;
        use iflow_cli_sdk_rust::error::IFlowError;
        use iflow_cli_sdk_rust::{IFlowClient, IFlowOptions, Message};
        use std::io::Write;

        // Initialize logging with environment variable support
        let log_level = if self.debug {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        };
        
        tracing_subscriber::fmt()
            .with_max_level(log_level)
            .init();

        println!("🚀 Starting iFlow WebSocket client...");

        // Use LocalSet for spawn_local compatibility
        let local = tokio::task::LocalSet::new();
        local
            .run_until(async {
                // Configure client options with WebSocket configuration and custom timeout
                let custom_timeout_secs = self.timeout as f64;
                let options = IFlowOptions::new()
                    .with_websocket_config(iflow_cli_sdk_rust::types::WebSocketConfig::auto_start())
                    .with_timeout(custom_timeout_secs)
                    .with_process_config(
                        iflow_cli_sdk_rust::types::ProcessConfig::new()
                            .enable_auto_start()
                            .start_port(8090),
                    );

                // Create and connect client
                let mut client = IFlowClient::new(Some(options));

                println!("🔗 Connecting to iFlow via WebSocket...");
                client
                    .connect()
                    .await
                    .map_err(|e| format!("Failed to connect: {}", e))?;
                println!("✅ Connected to iFlow via WebSocket");

                // Receive and process responses
                println!("📥 Receiving responses...");
                let mut message_stream = client.messages();

                // Store plan entries to track progress
                let mut plan_entries: Vec<(String, iflow_cli_sdk_rust::types::PlanStatus)> =
                    Vec::new();

                let message_task = tokio::task::spawn_local(async move {
                    let mut stdout = std::io::stdout();
                    let mut collected_messages = String::new();

                    while let Some(message) = message_stream.next().await {
                        match message {
                            Message::Assistant { content } => {
                                print!("🤖 Assistant: {}", content);
                                if let Err(err) = stdout.flush() {
                                    eprintln!("❌ Error flushing stdout: {}", err);
                                    break;
                                }
                                
                                // Collect assistant messages for summary
                                collected_messages.push_str(&format!("🤖 Assistant: {}", content));
                            }
                            Message::ToolCall { id, name, status } => {
                                println!("\n🔧 Tool call: {} ({}): {:?}", id, name, status);
                                
                                // Collect tool call messages for summary
                                collected_messages.push_str(&format!("\n🔧 Tool call: {} ({}): {:?}", id, name, status));
                            }
                            Message::Plan { entries } => {
                                // Update plan entries
                                plan_entries.clear();
                                for entry in entries {
                                    plan_entries.push((entry.content, entry.status));
                                }

                                // Display all plan entries with status
                                if !plan_entries.is_empty() {
                                    println!("\n📋 Plan:");
                                    collected_messages.push_str("\n📋 Plan:");
                                    for (i, (content, status)) in plan_entries.iter().enumerate() {
                                        let status_icon = match status {
                                            iflow_cli_sdk_rust::types::PlanStatus::Pending => "⏳",
                                            iflow_cli_sdk_rust::types::PlanStatus::InProgress => {
                                                "🔄"
                                            }
                                            iflow_cli_sdk_rust::types::PlanStatus::Completed => {
                                                "✅"
                                            }
                                        };
                                        println!("  {}. {} {}", i + 1, status_icon, content);
                                        collected_messages.push_str(&format!("\n  {}. {} {}", i + 1, status_icon, content));
                                    }
                                }
                            }
                            Message::TaskFinish { .. } => {
                                println!("\n✅ Task completed");
                                collected_messages.push_str("\n✅ Task completed");
                                break;
                            }
                            Message::Error {
                                code,
                                message: msg,
                                details: _,
                            } => {
                                eprintln!("\n❌ Error {}: {}", code, msg);
                                collected_messages.push_str(&format!("\n❌ Error {}: {}", code, msg));
                                break;
                            }
                            Message::User { content } => {
                                println!("\n👤 User message: {}", content);
                                collected_messages.push_str(&format!("\n👤 User message: {}", content));
                            }
                        }
                    }

                    collected_messages
                });

                // Send a message
                let prompt = self.prompt.as_ref().unwrap();
                println!("📤 User prompt: {}", prompt);

                // Handle the send_message result to catch timeout errors
                match client.send_message(prompt, None).await {
                    Ok(()) => {
                        println!("\n✅ Prompt message sent successfully");
                    }
                    Err(IFlowError::Timeout(msg)) => {
                        eprintln!("⏰ Timeout error occurred: {}", msg);
                        eprintln!("This may be due to processing delays.");
                        eprintln!("Consider increasing the timeout or checking the iFlow process.");
                    }
                    Err(e) => {
                        eprintln!("❌ Error sending message: {}", e);
                        return Err(format!("{}", e));
                    }
                }

                // Wait for the message handling task to finish with a timeout
                let message_result = match tokio::time::timeout(
                    std::time::Duration::from_secs_f64(custom_timeout_secs),
                    message_task,
                )
                .await
                {
                    Ok(Ok(collected_messages)) => {
                        println!("✅ Message handling completed successfully");
                        
                        // Write collected messages to GitHub step summary if in GitHub Actions environment
                        if std::env::var("GITHUB_ACTIONS").is_ok() {
                            // Prepare configuration map for summary generation
                            let mut config_map = std::collections::HashMap::new();
                            config_map.insert("isTimeout", serde_json::Value::Bool(false));
                            config_map.insert("timeout", serde_json::Value::Number(serde_json::Number::from(self.timeout)));
                            config_map.insert("model", serde_json::Value::String(self.model.clone()));
                            config_map.insert("baseURL", serde_json::Value::String(self.base_url.clone()));
                            config_map.insert("workingDir", serde_json::Value::String(self.working_directory.clone()));
                            config_map.insert("extraArgs", serde_json::Value::String(self.extra_args.clone().unwrap_or_default()));
                            config_map.insert("prompt", serde_json::Value::String(prompt.clone()));

                            // Generate and write summary
                            let summary_content = generate_summary_markdown(&collected_messages, 0, &config_map);
                            if let Err(e) = Self::write_step_summary(&summary_content) {
                                eprintln!("⚠️  Warning: Failed to write step summary: {}", e);
                            }
                        }
                        
                        Ok(())
                    }
                    Ok(Err(err)) => {
                        eprintln!("❌ Error in message handling: {}", err);
                        Err(format!("Error in message handling: {}", err))
                    }
                    Err(_) => {
                        println!("⏰ Timeout waiting for message handling to complete");
                        Err("Timeout waiting for message handling to complete".to_string())
                    }
                };

                // Disconnect
                println!("\n🔌 Disconnecting...");
                client
                    .disconnect()
                    .await
                    .map_err(|e| format!("Failed to disconnect: {}", e))?;
                println!("👋 Disconnected from iFlow");

                message_result.map_err(|e| format!("WebSocket client error: {}", e))
            })
            .await
            .map_err(|e| format!("WebSocket client error: {}", e))?;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), String> {
    // Check if we're running in GitHub Actions environment
    let is_github_actions = env::var("GITHUB_ACTIONS").is_ok();
    
    // Parse CLI arguments
    let cli = Cli::parse();

    // Validate the arguments
    if let Err(e) = cli.validate() {
        eprintln!("Validation Error: {}", e);
        std::process::exit(1);
    }

    // Install specific versions if requested
    if let Err(e) = cli.install_specific_versions() {
        eprintln!("Installation Error: {}", e);
        std::process::exit(1);
    }

    // Print version information (after installing specific versions)
    cli.print_version_info();

    // Configure iFlow settings
    if let Err(e) = cli.configure() {
        eprintln!("Configuration Error: {}", e);
        std::process::exit(1);
    }

    // Execute pre-command if specified
    if let Err(e) = cli.execute_precmd() {
        eprintln!("Pre-command Error: {}", e);
        std::process::exit(1);
    }

    // Run WebSocket client in GitHub Actions environment or when explicitly requested
    if is_github_actions {
        // Skip actual execution in dry-run mode
        if cli.dry_run {
            println!("DRY RUN: Would execute run_websocket()");
            return Ok(());
        }
        cli.run_websocket().await?;
        return Ok(());
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
    println!("  dry_run: {}", cli.dry_run);
    
    // If we're in GitHub Actions, print a notice
    if is_github_actions {
        println!("ℹ️  Running in GitHub Actions mode with WebSocket client enabled by default");
    }

    Ok(())
}
