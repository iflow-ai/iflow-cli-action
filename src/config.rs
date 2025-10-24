use clap::Parser;
use std::env;
use std::fs;

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
pub struct Cli {
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
    #[clap(long, env = "INPUT_MODEL", default_value = "Qwen3-Coder")]
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

impl Cli {
    /// Validates the CLI arguments
    pub fn validate(&self) -> Result<(), String> {
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
        if let Some(ref settings_json) = self.settings_json
            && !settings_json.is_empty()
        {
            serde_json::from_str::<serde_json::Value>(settings_json)
                .map_err(|e| format!("invalid settings_json provided: {}", e))?;
        }

        Ok(())
    }

    /// Configures iFlow settings
    pub fn configure(&self) -> Result<(), String> {
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

    /// Creates settings JSON from individual parameters
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

    /// Run iFlow using WebSocket client
    /// Returns Ok(Some(summary)) when a summary was generated, Ok(None) when nothing to summarize,
    /// or Err(...) on error.
    pub async fn run_websocket(&self) -> Result<Option<String>, String> {
        use crate::github_actions::{generate_summary_markdown, write_step_summary};
        use futures::stream::StreamExt;
        use iflow_cli_sdk_rust::error::IFlowError;
        use iflow_cli_sdk_rust::{IFlowClient, IFlowOptions, Message};
        use std::io::Write;

        // Holder to pass the generated summary out of the LocalSet closure
        let summary_holder = std::sync::Arc::new(std::sync::Mutex::new(None::<String>));

        // Initialize logging with environment variable support
        let log_level = if self.debug || env::var("ACTIONS_STEP_DEBUG").is_ok() {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        };

        tracing_subscriber::fmt().with_max_level(log_level).init();

        println!("🚀 Starting iFlow WebSocket client...");

        // Use LocalSet for spawn_local compatibility
        let local = tokio::task::LocalSet::new();
        let summary_holder_clone = summary_holder.clone();
        local
            .run_until(async move {
                // Configure client options with WebSocket configuration and custom timeout
                let custom_timeout_secs = self.timeout as f64;
                let mut process_config = iflow_cli_sdk_rust::types::ProcessConfig::new()
                    .enable_auto_start()
                    .start_port(8090);

                if self.debug {
                    process_config = process_config.enable_debug();
                }

                let options = IFlowOptions::new()
                    .with_websocket_config(iflow_cli_sdk_rust::types::WebSocketConfig::auto_start())
                    .with_timeout(custom_timeout_secs)
                    .with_process_config(process_config)
                    .with_permission_mode(iflow_cli_sdk_rust::types::PermissionMode::Auto);

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

                let summary_holder_for_task = summary_holder_clone.clone();
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
                                println!("🔧 Tool call: {} ({}) {:?}", id, name, status);

                                // Collect tool call messages for summary
                                collected_messages.push_str(&format!(
                                    "🔧 Tool call: {} ({}) {:?}",
                                    id, name, status
                                ));
                            }
                            Message::Plan { entries } => {
                                // Update plan entries
                                plan_entries.clear();
                                for entry in entries {
                                    plan_entries.push((entry.content, entry.status));
                                }

                                // Display all plan entries with status
                                if !plan_entries.is_empty() {
                                    println!("📋 Plan:");
                                    collected_messages.push_str("📋 Plan:");
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
                                        collected_messages.push_str(&format!(
                                            "{}. {} {}",
                                            i + 1,
                                            status_icon,
                                            content
                                        ));
                                    }
                                }
                            }
                            Message::TaskFinish { .. } => {
                                println!("✅ Task completed");
                                collected_messages.push_str("✅ Task completed");
                                break;
                            }
                            Message::Error {
                                code,
                                message: msg,
                                details: _,
                            } => {
                                eprintln!("❌ Error {}: {}", code, msg);
                                collected_messages.push_str(&format!("❌ Error {}: {}", code, msg));
                                break;
                            }
                            Message::User { content } => {
                                println!("👤 User message: {}", content);
                                collected_messages
                                    .push_str(&format!("👤 User message: {}", content));
                            }
                        }
                    }

                    collected_messages
                });

                // Send a prompt message
                let prompt = self.prompt.as_ref().unwrap();
                // println!("📤 User prompt: {}", prompt);

                // Handle the send_message result to catch timeout errors
                match client.send_message(prompt, None).await {
                    Ok(()) => {
                        println!("✅ Prompt message sent successfully");
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

                        // Prepare configuration map for summary generation
                        let mut config_map = std::collections::HashMap::new();
                        config_map.insert("isTimeout", serde_json::Value::Bool(false));
                        config_map.insert(
                            "timeout",
                            serde_json::Value::Number(serde_json::Number::from(self.timeout)),
                        );
                        config_map.insert("model", serde_json::Value::String(self.model.clone()));
                        config_map
                            .insert("baseURL", serde_json::Value::String(self.base_url.clone()));
                        config_map.insert(
                            "workingDir",
                            serde_json::Value::String(self.working_directory.clone()),
                        );
                        config_map.insert("prompt", serde_json::Value::String(prompt.clone()));

                        // Generate summary
                        let summary_content =
                            generate_summary_markdown(&collected_messages, 0, &config_map);

                        // Write collected messages to GitHub step summary if in GitHub Actions environment
                        if std::env::var("GITHUB_ACTIONS").is_ok()
                            && let Err(e) = write_step_summary(&summary_content)
                        {
                            eprintln!("⚠️  Warning: Failed to write step summary: {}", e);
                        }

                        // Store the generated summary into the shared holder so the outer
                        // function can access it after the LocalSet completes.
                        if let Ok(mut guard) = summary_holder_for_task.lock() {
                            *guard = Some(summary_content.clone());
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
                println!("🔌 Disconnecting...");
                client
                    .disconnect()
                    .await
                    .map_err(|e| format!("Failed to disconnect: {}", e))?;
                println!("👋 Disconnected from iFlow");

                message_result.map_err(|e| format!("WebSocket client error: {}", e))
            })
            .await
            .map_err(|e| format!("WebSocket client error: {}", e))?;

        // Extract the summary from the holder and return it
        let summary = summary_holder.lock().map(|g| g.clone()).unwrap_or(None);
        Ok(summary)
    }
}
