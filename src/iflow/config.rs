use serde_json;
use std::fs;

/// IFlow configuration
#[derive(Debug)]
pub struct IFlowConfig {
    /// Base URL for the iFlow API
    pub base_url: String,

    /// Model name to use
    pub model: String,
}

impl IFlowConfig {
    /// Creates settings JSON from individual parameters
    pub fn create_settings_from_params(&self, api_key: &str) -> Result<String, String> {
        let settings = serde_json::json!({
            "theme": "Default",
            "selectedAuthType": "iflow",
            "apiKey": api_key,
            "baseUrl": self.base_url,
            "modelName": self.model,
            "searchApiKey": api_key
        });

        serde_json::to_string_pretty(&settings)
            .map_err(|e| format!("failed to marshal settings: {}", e))
    }

    /// Configures iFlow settings
    pub fn configure(
        &self,
        settings_json: Option<&String>,
        api_key: &str,
        settings_file_path: Option<&String>,
    ) -> Result<(), String> {
        // Determine the settings file path
        let settings_file_path = if let Some(path) = settings_file_path {
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

        let settings_data = if let Some(settings_json) = settings_json {
            if !settings_json.is_empty() {
                // Use provided settings JSON directly
                // Pretty format the JSON
                let parsed: serde_json::Value = serde_json::from_str(settings_json)
                    .map_err(|e| format!("invalid settings_json provided: {}", e))?;
                serde_json::to_string_pretty(&parsed)
                    .map_err(|e| format!("failed to format settings JSON: {}", e))?
            } else {
                // Create settings from individual parameters
                self.create_settings_from_params(api_key)?
            }
        } else {
            // Create settings from individual parameters
            self.create_settings_from_params(api_key)?
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
}
