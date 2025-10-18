use serde_json::Value;
use std::collections::HashMap;
use std::fs;

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
            use std::io::Write;
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

    std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&output_file)
        .map_err(|e| format!("failed to open GITHUB_OUTPUT file: {}", e))
        .and_then(|mut file| {
            use std::io::Write;
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

/// Generates a comprehensive summary markdown
pub fn generate_summary_markdown(
    result: &str,
    exit_code: i32,
    config: &HashMap<&str, Value>,
) -> String {
    let mut summary = String::new();

    let is_timeout = config
        .get("isTimeout")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let timeout_val = config
        .get("timeout")
        .and_then(|v| v.as_i64())
        .unwrap_or(3600) as i32;
    let model_val = config
        .get("model")
        .and_then(|v| v.as_str())
        .unwrap_or("Qwen3-Coder");
    let base_url_val = config
        .get("baseURL")
        .and_then(|v| v.as_str())
        .unwrap_or("https://apis.iflow.cn/v1");
    let working_dir_val = config
        .get("workingDir")
        .and_then(|v| v.as_str())
        .unwrap_or(".");
    let prompt_val = config.get("prompt").and_then(|v| v.as_str()).unwrap_or("");

    // Add header with emoji based on status
    if is_timeout {
        summary.push_str("## ⏰ iFlow CLI Execution Summary - Timeout\n\n");
    } else if exit_code == 0 {
        summary.push_str("## ✅ iFlow CLI Execution Summary\n\n");
    } else {
        summary.push_str("## ❌ iFlow CLI Execution Summary\n\n");
    }

    // Add execution status with more detail
    summary.push_str("### 📊 Status\n\n");
    if is_timeout {
        summary.push_str("⏰ **Execution**: Timed Out\n");
        summary.push_str(&format!(
            "🕒 **Timeout Duration**: {} seconds\n",
            timeout_val
        ));
        summary.push_str(&format!("💥 **Exit Code**: {}\n\n", exit_code));
    } else if exit_code == 0 {
        summary.push_str("🎉 **Execution**: Successful\n");
        summary.push_str("🎯 **Exit Code**: 0\n\n");
    } else {
        summary.push_str("⚠️ **Execution**: Failed\n");
        summary.push_str(&format!("💥 **Exit Code**: {}\n\n", exit_code));
    }

    // Add configuration details in a table format
    summary.push_str("### ⚙️ Configuration\n\n");
    summary.push_str("| Setting | Value |\n");
    summary.push_str("|---------|-------|\n");
    summary.push_str(&format!("| Model | `{}` |\n", model_val));
    summary.push_str(&format!("| Base URL | `{}` |\n", base_url_val));
    summary.push_str(&format!("| Timeout | {} seconds |\n", timeout_val));
    summary.push_str(&format!("| Working Directory | `{}` |\n", working_dir_val));
    summary.push_str("\n");

    // Add prompt section
    summary.push_str("### 📝 Input Prompt\n\n");
    let mut prompt = prompt_val.to_string();
    // Escape any markdown characters in the prompt
    prompt = prompt.replace("`", "\\`");
    summary.push_str(&format!("> {}\n\n", prompt));

    // Add result section with better formatting
    summary.push_str("### Output\n\n");
    if exit_code == 0 {
        let display_result = result.to_string();
        // if display_result.chars().count() > 3000 {
        //     display_result = display_result.chars().take(3000).collect::<String>()
        //         + "\n\n... *(Output truncated. See full output in action logs)*";
        // }

        // Check if result contains markdown or code blocks
        if result.contains("```") {
            // Result already contains code blocks, display as-is
            summary.push_str(&format!("{}\n\n", display_result));
        } else if contains_code(&display_result) {
            // Result looks like code, wrap in code block
            summary.push_str(&format!("```\n{}\n```\n\n", display_result));
        } else {
            // Regular text result, format as blockquote for readability
            for line in display_result.lines() {
                if !line.trim().is_empty() {
                    summary.push_str(&format!("> {}\n", line));
                } else {
                    summary.push_str(">\n");
                }
            }
            summary.push_str("\n");
        }
    } else {
        // Error output, always in code block
        summary.push_str("```\n");
        summary.push_str(result);
        summary.push_str("\n```\n\n");

        // Add troubleshooting hints for common errors
        if is_timeout {
            summary.push_str("#### ⏰ Timeout Information\n\n");
            summary.push_str(&format!(
                "- **Configured Timeout**: {} seconds\n",
                timeout_val
            ));
            summary.push_str("- **Reason**: The iFlow CLI command did not complete within the specified timeout period\n");
            summary.push_str("- **Exit Code**: 124 (timeout)\n\n");

            summary.push_str("#### 🔧 Timeout Troubleshooting\n\n");
            summary.push_str("- **Increase timeout**: Consider increasing the timeout value if the task legitimately needs more time\n");
            summary.push_str("- **Optimize prompt**: Try breaking down complex prompts into smaller, more focused requests\n");
            summary.push_str(
                "- **Check model performance**: Some models may require longer processing time\n",
            );
            summary.push_str(
                "- **Network issues**: Verify network connectivity and API response times\n",
            );
            summary.push_str("- **Resource constraints**: Check if the system has sufficient resources (CPU, memory)\n\n");
        } else if result.contains("API Error") {
            summary.push_str("#### 🔧 Troubleshooting Hints\n\n");
            summary.push_str("- Check if your API key is valid and active\n");
            summary.push_str("- Verify the base URL is accessible\n");
            summary.push_str("- Ensure the selected model is available\n");
            summary.push_str("- Try increasing the timeout value\n\n");
        }
    }

    // Add footer
    summary.push_str("---\n");
    summary.push_str(
        "*🤖 Generated by [iFlow CLI Action](https://github.com/iflow-ai/iflow-cli-action)*\n\n",
    );

    summary
}

/// Detects if text looks like code
pub fn contains_code(text: &str) -> bool {
    let code_indicators = [
        "function",
        "class",
        "def ",
        "import ",
        "const ",
        "let ",
        "var ",
        "public ",
        "private ",
        "protected",
        "return ",
        "if (",
        "for (",
        "while (",
        "{",
        "}",
        ";",
        "//",
        "/*",
        "*/",
        "#include",
        "package ",
        "use ",
    ];

    let lower_text = text.to_lowercase();
    for indicator in &code_indicators {
        if lower_text.contains(indicator) {
            return true;
        }
    }
    false
}