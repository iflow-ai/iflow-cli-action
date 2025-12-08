use serde_json::Value;
use std::collections::HashMap;

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
        .unwrap_or("qwen3-coder-plus");
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
    summary.push('\n');

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
            summary.push('\n');
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
