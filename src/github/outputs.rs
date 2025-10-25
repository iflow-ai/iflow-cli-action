use std::fs;
use std::io::Write;

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
    fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&summary_file)
        .map_err(|e| format!("failed to open step summary file: {}", e))
        .and_then(|mut file| {
            file.write_all(content.as_bytes())
                .map_err(|e| format!("failed to write to step summary: {}", e))
        })
}

/// Writes a key=value pair to GITHUB_OUTPUT (GitHub Actions outputs)
/// Appends to the file specified by the GITHUB_OUTPUT environment variable.
pub fn write_github_output(key: &str, value: &str) -> Result<(), String> {
    // Only proceed if running in GitHub Actions
    if std::env::var("GITHUB_ACTIONS").is_err() {
        return Ok(());
    }

    let output_file = match std::env::var("GITHUB_OUTPUT") {
        Ok(f) => f,
        Err(_) => return Err("GITHUB_OUTPUT not set".to_string()),
    };

    fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&output_file)
        .map_err(|e| format!("failed to open GITHUB_OUTPUT file: {}", e))
        .and_then(|mut file| {
            // Use the key=value format. If value contains newlines, use the
            // multiline GitHub Actions syntax (key<<EOF\n...\nEOF)
            if value.contains('\n') {
                // Write as multiline
                let payload = format!("{}<<EOF\n{}\nEOF\n", key, value);
                file.write_all(payload.as_bytes())
                    .map_err(|e| format!("failed to write to GITHUB_OUTPUT: {}", e))
            } else {
                let payload = format!("{}={}\n", key, value);
                file.write_all(payload.as_bytes())
                    .map_err(|e| format!("failed to write to GITHUB_OUTPUT: {}", e))
            }
        })
}
