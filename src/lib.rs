use std::collections::HashMap;
use serde_json::Value;

/// Generates a comprehensive summary markdown
pub fn generate_summary_markdown(
    result: &str,
    exit_code: i32,
    config: &HashMap<&str, Value>,
) -> String {
    let mut summary = String::new();

    let is_timeout = config.get("isTimeout").and_then(|v| v.as_bool()).unwrap_or(false);
    let timeout_val = config.get("timeout").and_then(|v| v.as_i64()).unwrap_or(3600) as i32;
    let model_val = config.get("model").and_then(|v| v.as_str()).unwrap_or("Qwen3-Coder");
    let base_url_val = config.get("baseURL").and_then(|v| v.as_str()).unwrap_or("https://apis.iflow.cn/v1");
    let working_dir_val = config.get("workingDir").and_then(|v| v.as_str()).unwrap_or(".");
    let extra_args_val = config.get("extraArgs").and_then(|v| v.as_str()).unwrap_or("");
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
        summary.push_str(&format!("🕒 **Timeout Duration**: {} seconds\n", timeout_val));
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
    if !extra_args_val.is_empty() {
        summary.push_str(&format!("| Extra Arguments | `{}` |\n", extra_args_val));
    }
    summary.push_str("\n");

    // Add prompt section
    summary.push_str("### 📝 Input Prompt\n\n");
    let mut prompt = prompt_val.to_string();
    if prompt.len() > 300 {
        prompt = prompt[..300].to_string() + "...";
    }
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
            summary.push_str(&format!("- **Configured Timeout**: {} seconds\n", timeout_val));
            summary.push_str("- **Reason**: The iFlow CLI command did not complete within the specified timeout period\n");
            summary.push_str("- **Exit Code**: 124 (timeout)\n\n");

            summary.push_str("#### 🔧 Timeout Troubleshooting\n\n");
            summary.push_str("- **Increase timeout**: Consider increasing the timeout value if the task legitimately needs more time\n");
            summary.push_str("- **Optimize prompt**: Try breaking down complex prompts into smaller, more focused requests\n");
            summary.push_str("- **Check model performance**: Some models may require longer processing time\n");
            summary.push_str("- **Network issues**: Verify network connectivity and API response times\n");
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
    summary.push_str("*🤖 Generated by [iFlow CLI Action](https://github.com/iflow-ai/iflow-cli-action)*\n\n");

    summary
}

/// Detects if text looks like code
pub fn contains_code(text: &str) -> bool {
    let code_indicators = [
        "function", "class", "def ", "import ", "const ", "let ", "var ",
        "public ", "private ", "protected", "return ", "if (", "for (", "while (",
        "{", "}", ";", "//", "/*", "*/", "#include", "package ", "use ",
    ];

    let lower_text = text.to_lowercase();
    for indicator in &code_indicators {
        if lower_text.contains(indicator) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_contains_code() {
        // Test cases that should be detected as code
        assert!(contains_code("function test() { return true; }"));
        assert!(contains_code("class MyClass { public void method() {} }"));
        assert!(contains_code("def my_function():
    return True"));
        assert!(contains_code("const x = 10;"));
        assert!(contains_code("let y = 20;"));
        assert!(contains_code("var z = 30;"));
        assert!(contains_code("public static void main(String[] args)"));
        assert!(contains_code("if (condition) { do_something(); }"));
        assert!(contains_code("#include <stdio.h>"));
        assert!(contains_code("package main"));
        
        // Test cases that should not be detected as code
        assert!(!contains_code("This is a regular text message"));
        assert!(!contains_code("Hello, this is a simple message"));
        assert!(!contains_code("Just some text without code indicators"));
        assert!(!contains_code(""));
    }

    #[test]
    fn test_generate_summary_markdown_success() {
        // Prepare configuration map
        let mut config_map = HashMap::new();
        config_map.insert("isTimeout", json!(false));
        config_map.insert("timeout", json!(3600));
        config_map.insert("model", json!("Qwen3-Coder"));
        config_map.insert("baseURL", json!("https://apis.iflow.cn/v1"));
        config_map.insert("workingDir", json!("."));
        config_map.insert("extraArgs", json!(""));
        config_map.insert("prompt", json!("Test prompt"));
        
        // Test successful execution
        let result = "This is a test result";
        let summary = generate_summary_markdown(result, 0, &config_map);
        
        // Verify key elements are present
        assert!(summary.contains("## ✅ iFlow CLI Execution Summary"));
        assert!(summary.contains("🎉 **Execution**: Successful"));
        assert!(summary.contains("🎯 **Exit Code**: 0"));
        assert!(summary.contains("Test prompt"));
        assert!(summary.contains("This is a test result"));
    }

    #[test]
    fn test_generate_summary_markdown_failure() {
        // Prepare configuration map
        let mut config_map = HashMap::new();
        config_map.insert("isTimeout", json!(false));
        config_map.insert("timeout", json!(3600));
        config_map.insert("model", json!("Qwen3-Coder"));
        config_map.insert("baseURL", json!("https://apis.iflow.cn/v1"));
        config_map.insert("workingDir", json!("."));
        config_map.insert("extraArgs", json!(""));
        config_map.insert("prompt", json!("Test prompt"));
        
        // Test failed execution
        let result = "Error: Something went wrong";
        let summary = generate_summary_markdown(result, 1, &config_map);
        
        // Verify key elements are present
        assert!(summary.contains("## ❌ iFlow CLI Execution Summary"));
        assert!(summary.contains("⚠️ **Execution**: Failed"));
        assert!(summary.contains("💥 **Exit Code**: 1"));
        assert!(summary.contains("Error: Something went wrong"));
    }

    #[test]
    fn test_generate_summary_markdown_timeout() {
        // Prepare configuration map with timeout
        let mut config_map = HashMap::new();
        config_map.insert("isTimeout", json!(true));
        config_map.insert("timeout", json!(3600));
        config_map.insert("model", json!("Qwen3-Coder"));
        config_map.insert("baseURL", json!("https://apis.iflow.cn/v1"));
        config_map.insert("workingDir", json!("."));
        config_map.insert("extraArgs", json!(""));
        config_map.insert("prompt", json!("Test prompt"));
        
        // Test timeout execution
        let result = "Operation timed out";
        let summary = generate_summary_markdown(result, 124, &config_map);
        
        // Verify key elements are present
        assert!(summary.contains("## ⏰ iFlow CLI Execution Summary - Timeout"));
        assert!(summary.contains("⏰ **Execution**: Timed Out"));
        assert!(summary.contains("🕒 **Timeout Duration**: 3600 seconds"));
        assert!(summary.contains("💥 **Exit Code**: 124"));
    }
}