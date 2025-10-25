/// Validates CLI arguments
pub fn validate_args(
    prompt: Option<&String>,
    api_key: Option<&String>,
    settings_json: Option<&String>,
    timeout: u32,
) -> Result<(), String> {
    // Validate required inputs
    if prompt.is_none_or(|p| p.is_empty()) {
        return Err("prompt input is required and cannot be empty".to_string());
    }

    if api_key.is_none() && settings_json.is_none() {
        return Err("api_key input is required and cannot be empty".to_string());
    }

    // Validate timeout range (1 second to 24 hours)
    if !(1..=86400).contains(&timeout) {
        return Err(
            "timeout value is out of range. Must be between 1 and 86400 seconds".to_string(),
        );
    }

    // Validate settings_json if provided
    if let Some(settings_json) = settings_json
        && !settings_json.is_empty()
    {
        serde_json::from_str::<serde_json::Value>(settings_json)
            .map_err(|e| format!("invalid settings_json provided: {}", e))?;
    }

    Ok(())
}
