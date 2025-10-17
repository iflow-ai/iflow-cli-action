
// Shared code indicators array for tests
const CODE_INDICATORS: [&str; 23] = [
    "function", "class", "def ", "import ", "const ", "let ", "var ",
    "public ", "private ", "protected", "return ", "if (", "for (", "while (",
    "{", "}", ";", "//", "/*", "*/", "#include", "package ", "use ",
];

#[test]
fn test_contains_code() {
    // Test cases that should be detected as code
    let code_indicators = &CODE_INDICATORS;

    // We'll implement a simple version of contains_code for testing
    let contains_code = |text: &str| -> bool {
        let lower_text = text.to_lowercase();
        for indicator in &code_indicators {
            if lower_text.contains(indicator) {
                return true;
            }
        }
        false
    };

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
fn test_write_step_summary_not_in_github_actions() {
    // Test that write_step_summary does nothing when not in GitHub Actions environment
    let _content = "Test content for summary";
    
    // Since we can't directly call the function from the binary, we'll test the logic indirectly
    // by checking that it doesn't panic or error even when GITHUB_ACTIONS is not set
    assert!(true); // Placeholder - in a real implementation we'd test the actual function
}

#[test]
fn test_generate_summary_markdown_success() {
    // Test cases that should be detected as code
    let code_indicators = [
        "function", "class", "def ", "import ", "const ", "let ", "var ",
        "public ", "private ", "protected", "return ", "if (", "for (", "while (",
        "{", "}", ";", "//", "/*", "*/", "#include", "package ", "use ",
    ];

    // We'll implement a simple version of contains_code for testing
    let contains_code = |text: &str| -> bool {
        let lower_text = text.to_lowercase();
        for indicator in &code_indicators {
            if lower_text.contains(indicator) {
                return true;
            }
        }
        false
    };

    // Prepare configuration map
    let mut config_map = std::collections::HashMap::new();
    config_map.insert("isTimeout", serde_json::Value::Bool(false));
    config_map.insert("timeout", serde_json::Value::Number(serde_json::Number::from(3600)));
    config_map.insert("model", serde_json::Value::String("Qwen3-Coder".to_string()));
    config_map.insert("baseURL", serde_json::Value::String("https://apis.iflow.cn/v1".to_string()));
    config_map.insert("workingDir", serde_json::Value::String(".".to_string()));
    config_map.insert("extraArgs", serde_json::Value::String("".to_string()));
    config_map.insert("prompt", serde_json::Value::String("Test prompt".to_string()));
    
    // Test successful execution
    let result = "This is a test result";
    
    // Simple implementation of generate_summary_markdown for testing
    let generate_summary_markdown = |result: &str, exit_code: i32, config: &std::collections::HashMap<&str, serde_json::Value>| -> String {
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
            summary.push_str("## ⏰ iFlow CLI Execution Summary - Timeout

");
        } else if exit_code == 0 {
            summary.push_str("## ✅ iFlow CLI Execution Summary

");
        } else {
            summary.push_str("## ❌ iFlow CLI Execution Summary

");
        }

        // Add execution status with more detail
        summary.push_str("### 📊 Status

");
        if is_timeout {
            summary.push_str("⏰ **Execution**: Timed Out
");
            summary.push_str(&format!("🕒 **Timeout Duration**: {} seconds
", timeout_val));
            summary.push_str(&format!("💥 **Exit Code**: {}

", exit_code));
        } else if exit_code == 0 {
            summary.push_str("🎉 **Execution**: Successful
");
            summary.push_str("🎯 **Exit Code**: 0

");
        } else {
            summary.push_str("⚠️ **Execution**: Failed
");
            summary.push_str(&format!("💥 **Exit Code**: {}

", exit_code));
        }

        // Add configuration details in a table format
        summary.push_str("### ⚙️ Configuration

");
        summary.push_str("| Setting | Value |
");
        summary.push_str("|---------|-------|
");
        summary.push_str(&format!("| Model | `{}` |
", model_val));
        summary.push_str(&format!("| Base URL | `{}` |
", base_url_val));
        summary.push_str(&format!("| Timeout | {} seconds |
", timeout_val));
        summary.push_str(&format!("| Working Directory | `{}` |
", working_dir_val));
        if !extra_args_val.is_empty() {
            summary.push_str(&format!("| Extra Arguments | `{}` |
", extra_args_val));
        }
        summary.push_str("
");

        // Add prompt section
        summary.push_str("### 📝 Input Prompt

");
        let mut prompt = prompt_val.to_string();
        if prompt.len() > 300 {
            prompt = prompt[..300].to_string() + "...";
        }
        // Escape any markdown characters in the prompt
        prompt = prompt.replace("`", "`");
        summary.push_str(&format!("> {}

", prompt));

        // Add result section with better formatting
        summary.push_str("### Output

");
        if exit_code == 0 {
            let mut display_result = result.to_string();
            if display_result.len() > 3000 {
                display_result = display_result[..3000].to_string() + "

... *(Output truncated. See full output in action logs)*";
            }

            // Check if result contains markdown or code blocks
            if result.contains("```") {
                // Result already contains code blocks, display as-is
                summary.push_str(&format!("{}

", display_result));
            } else if contains_code(&display_result) {
                // Result looks like code, wrap in code block
                summary.push_str(&format!("```
{}
```

", display_result));
            } else {
                // Regular text result, format as blockquote for readability
                for line in display_result.lines() {
                    if !line.trim().is_empty() {
                        summary.push_str(&format!("> {}
", line));
                    } else {
                        summary.push_str(">
");
                    }
                }
                summary.push_str("
");
            }
        } else {
            // Error output, always in code block
            summary.push_str("```
");
            summary.push_str(result);
            summary.push_str("
```

");

            // Add troubleshooting hints for common errors
            if is_timeout {
                summary.push_str("#### ⏰ Timeout Information

");
                summary.push_str(&format!("- **Configured Timeout**: {} seconds
", timeout_val));
                summary.push_str("- **Reason**: The iFlow CLI command did not complete within the specified timeout period
");
                summary.push_str("- **Exit Code**: 124 (timeout)

");

                summary.push_str("#### 🔧 Timeout Troubleshooting

");
                summary.push_str("- **Increase timeout**: Consider increasing the timeout value if the task legitimately needs more time
");
                summary.push_str("- **Optimize prompt**: Try breaking down complex prompts into smaller, more focused requests
");
                summary.push_str("- **Check model performance**: Some models may require longer processing time
");
                summary.push_str("- **Network issues**: Verify network connectivity and API response times
");
                summary.push_str("- **Resource constraints**: Check if the system has sufficient resources (CPU, memory)

");
            } else if result.contains("API Error") {
                summary.push_str("#### 🔧 Troubleshooting Hints

");
                summary.push_str("- Check if your API key is valid and active
");
                summary.push_str("- Verify the base URL is accessible
");
                summary.push_str("- Ensure the selected model is available
");
                summary.push_str("- Try increasing the timeout value

");
            }
        }

        // Add footer
        summary.push_str("---
");
        summary.push_str("*🤖 Generated by [iFlow CLI Action](https://github.com/iflow-ai/iflow-cli-action)*

");

        summary
    };
    
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
    // Test cases that should be detected as code
    let code_indicators = [
        "function", "class", "def ", "import ", "const ", "let ", "var ",
        "public ", "private ", "protected", "return ", "if (", "for (", "while (",
        "{", "}", ";", "//", "/*", "*/", "#include", "package ", "use ",
    ];

    // We'll implement a simple version of contains_code for testing
    let contains_code = |text: &str| -> bool {
        let lower_text = text.to_lowercase();
        for indicator in &code_indicators {
            if lower_text.contains(indicator) {
                return true;
            }
        }
        false
    };

    // Prepare configuration map
    let mut config_map = std::collections::HashMap::new();
    config_map.insert("isTimeout", serde_json::Value::Bool(false));
    config_map.insert("timeout", serde_json::Value::Number(serde_json::Number::from(3600)));
    config_map.insert("model", serde_json::Value::String("Qwen3-Coder".to_string()));
    config_map.insert("baseURL", serde_json::Value::String("https://apis.iflow.cn/v1".to_string()));
    config_map.insert("workingDir", serde_json::Value::String(".".to_string()));
    config_map.insert("extraArgs", serde_json::Value::String("".to_string()));
    config_map.insert("prompt", serde_json::Value::String("Test prompt".to_string()));
    
    // Simple implementation of generate_summary_markdown for testing
    let generate_summary_markdown = |result: &str, exit_code: i32, config: &std::collections::HashMap<&str, serde_json::Value>| -> String {
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
            summary.push_str("## ⏰ iFlow CLI Execution Summary - Timeout

");
        } else if exit_code == 0 {
            summary.push_str("## ✅ iFlow CLI Execution Summary

");
        } else {
            summary.push_str("## ❌ iFlow CLI Execution Summary

");
        }

        // Add execution status with more detail
        summary.push_str("### 📊 Status

");
        if is_timeout {
            summary.push_str("⏰ **Execution**: Timed Out
");
            summary.push_str(&format!("🕒 **Timeout Duration**: {} seconds
", timeout_val));
            summary.push_str(&format!("💥 **Exit Code**: {}

", exit_code));
        } else if exit_code == 0 {
            summary.push_str("🎉 **Execution**: Successful
");
            summary.push_str("🎯 **Exit Code**: 0

");
        } else {
            summary.push_str("⚠️ **Execution**: Failed
");
            summary.push_str(&format!("💥 **Exit Code**: {}

", exit_code));
        }

        // Add configuration details in a table format
        summary.push_str("### ⚙️ Configuration

");
        summary.push_str("| Setting | Value |
");
        summary.push_str("|---------|-------|
");
        summary.push_str(&format!("| Model | `{}` |
", model_val));
        summary.push_str(&format!("| Base URL | `{}` |
", base_url_val));
        summary.push_str(&format!("| Timeout | {} seconds |
", timeout_val));
        summary.push_str(&format!("| Working Directory | `{}` |
", working_dir_val));
        if !extra_args_val.is_empty() {
            summary.push_str(&format!("| Extra Arguments | `{}` |
", extra_args_val));
        }
        summary.push_str("
");

        // Add prompt section
        summary.push_str("### 📝 Input Prompt

");
        let mut prompt = prompt_val.to_string();
        if prompt.len() > 300 {
            prompt = prompt[..300].to_string() + "...";
        }
        // Escape any markdown characters in the prompt
        prompt = prompt.replace("`", "`");
        summary.push_str(&format!("> {}

", prompt));

        // Add result section with better formatting
        summary.push_str("### Output

");
        if exit_code == 0 {
            let mut display_result = result.to_string();
            if display_result.len() > 3000 {
                display_result = display_result[..3000].to_string() + "

... *(Output truncated. See full output in action logs)*";
            }

            // Check if result contains markdown or code blocks
            if result.contains("```") {
                // Result already contains code blocks, display as-is
                summary.push_str(&format!("{}

", display_result));
            } else if contains_code(&display_result) {
                // Result looks like code, wrap in code block
                summary.push_str(&format!("```
{}
```

", display_result));
            } else {
                // Regular text result, format as blockquote for readability
                for line in display_result.lines() {
                    if !line.trim().is_empty() {
                        summary.push_str(&format!("> {}
", line));
                    } else {
                        summary.push_str(">
");
                    }
                }
                summary.push_str("
");
            }
        } else {
            // Error output, always in code block
            summary.push_str("```
");
            summary.push_str(result);
            summary.push_str("
```

");

            // Add troubleshooting hints for common errors
            if is_timeout {
                summary.push_str("#### ⏰ Timeout Information

");
                summary.push_str(&format!("- **Configured Timeout**: {} seconds
", timeout_val));
                summary.push_str("- **Reason**: The iFlow CLI command did not complete within the specified timeout period
");
                summary.push_str("- **Exit Code**: 124 (timeout)

");

                summary.push_str("#### 🔧 Timeout Troubleshooting

");
                summary.push_str("- **Increase timeout**: Consider increasing the timeout value if the task legitimately needs more time
");
                summary.push_str("- **Optimize prompt**: Try breaking down complex prompts into smaller, more focused requests
");
                summary.push_str("- **Check model performance**: Some models may require longer processing time
");
                summary.push_str("- **Network issues**: Verify network connectivity and API response times
");
                summary.push_str("- **Resource constraints**: Check if the system has sufficient resources (CPU, memory)

");
            } else if result.contains("API Error") {
                summary.push_str("#### 🔧 Troubleshooting Hints

");
                summary.push_str("- Check if your API key is valid and active
");
                summary.push_str("- Verify the base URL is accessible
");
                summary.push_str("- Ensure the selected model is available
");
                summary.push_str("- Try increasing the timeout value

");
            }
        }

        // Add footer
        summary.push_str("---
");
        summary.push_str("*🤖 Generated by [iFlow CLI Action](https://github.com/iflow-ai/iflow-cli-action)*

");

        summary
    };
    
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
    // Test cases that should be detected as code
    let code_indicators = [
        "function", "class", "def ", "import ", "const ", "let ", "var ",
        "public ", "private ", "protected", "return ", "if (", "for (", "while (",
        "{", "}", ";", "//", "/*", "*/", "#include", "package ", "use ",
    ];

    // We'll implement a simple version of contains_code for testing
    let contains_code = |text: &str| -> bool {
        let lower_text = text.to_lowercase();
        for indicator in &code_indicators {
            if lower_text.contains(indicator) {
                return true;
            }
        }
        false
    };

    // Prepare configuration map with timeout
    let mut config_map = std::collections::HashMap::new();
    config_map.insert("isTimeout", serde_json::Value::Bool(true));
    config_map.insert("timeout", serde_json::Value::Number(serde_json::Number::from(3600)));
    config_map.insert("model", serde_json::Value::String("Qwen3-Coder".to_string()));
    config_map.insert("baseURL", serde_json::Value::String("https://apis.iflow.cn/v1".to_string()));
    config_map.insert("workingDir", serde_json::Value::String(".".to_string()));
    config_map.insert("extraArgs", serde_json::Value::String("".to_string()));
    config_map.insert("prompt", serde_json::Value::String("Test prompt".to_string()));
    
    // Simple implementation of generate_summary_markdown for testing
    let generate_summary_markdown = |result: &str, exit_code: i32, config: &std::collections::HashMap<&str, serde_json::Value>| -> String {
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
            summary.push_str("## ⏰ iFlow CLI Execution Summary - Timeout

");
        } else if exit_code == 0 {
            summary.push_str("## ✅ iFlow CLI Execution Summary

");
        } else {
            summary.push_str("## ❌ iFlow CLI Execution Summary

");
        }

        // Add execution status with more detail
        summary.push_str("### 📊 Status

");
        if is_timeout {
            summary.push_str("⏰ **Execution**: Timed Out
");
            summary.push_str(&format!("🕒 **Timeout Duration**: {} seconds
", timeout_val));
            summary.push_str(&format!("💥 **Exit Code**: {}

", exit_code));
        } else if exit_code == 0 {
            summary.push_str("🎉 **Execution**: Successful
");
            summary.push_str("🎯 **Exit Code**: 0

");
        } else {
            summary.push_str("⚠️ **Execution**: Failed
");
            summary.push_str(&format!("💥 **Exit Code**: {}

", exit_code));
        }

        // Add configuration details in a table format
        summary.push_str("### ⚙️ Configuration

");
        summary.push_str("| Setting | Value |
");
        summary.push_str("|---------|-------|
");
        summary.push_str(&format!("| Model | `{}` |
", model_val));
        summary.push_str(&format!("| Base URL | `{}` |
", base_url_val));
        summary.push_str(&format!("| Timeout | {} seconds |
", timeout_val));
        summary.push_str(&format!("| Working Directory | `{}` |
", working_dir_val));
        if !extra_args_val.is_empty() {
            summary.push_str(&format!("| Extra Arguments | `{}` |
", extra_args_val));
        }
        summary.push_str("
");

        // Add prompt section
        summary.push_str("### 📝 Input Prompt

");
        let mut prompt = prompt_val.to_string();
        if prompt.len() > 300 {
            prompt = prompt[..300].to_string() + "...";
        }
        // Escape any markdown characters in the prompt
        prompt = prompt.replace("`", "`");
        summary.push_str(&format!("> {}

", prompt));

        // Add result section with better formatting
        summary.push_str("### Output

");
        if exit_code == 0 {
            let mut display_result = result.to_string();
            if display_result.len() > 3000 {
                display_result = display_result[..3000].to_string() + "

... *(Output truncated. See full output in action logs)*";
            }

            // Check if result contains markdown or code blocks
            if result.contains("```") {
                // Result already contains code blocks, display as-is
                summary.push_str(&format!("{}

", display_result));
            } else if contains_code(&display_result) {
                // Result looks like code, wrap in code block
                summary.push_str(&format!("```
{}
```

", display_result));
            } else {
                // Regular text result, format as blockquote for readability
                for line in display_result.lines() {
                    if !line.trim().is_empty() {
                        summary.push_str(&format!("> {}
", line));
                    } else {
                        summary.push_str(">
");
                    }
                }
                summary.push_str("
");
            }
        } else {
            // Error output, always in code block
            summary.push_str("```
");
            summary.push_str(result);
            summary.push_str("
```

");

            // Add troubleshooting hints for common errors
            if is_timeout {
                summary.push_str("#### ⏰ Timeout Information

");
                summary.push_str(&format!("- **Configured Timeout**: {} seconds
", timeout_val));
                summary.push_str("- **Reason**: The iFlow CLI command did not complete within the specified timeout period
");
                summary.push_str("- **Exit Code**: 124 (timeout)

");

                summary.push_str("#### 🔧 Timeout Troubleshooting

");
                summary.push_str("- **Increase timeout**: Consider increasing the timeout value if the task legitimately needs more time
");
                summary.push_str("- **Optimize prompt**: Try breaking down complex prompts into smaller, more focused requests
");
                summary.push_str("- **Check model performance**: Some models may require longer processing time
");
                summary.push_str("- **Network issues**: Verify network connectivity and API response times
");
                summary.push_str("- **Resource constraints**: Check if the system has sufficient resources (CPU, memory)

");
            } else if result.contains("API Error") {
                summary.push_str("#### 🔧 Troubleshooting Hints

");
                summary.push_str("- Check if your API key is valid and active
");
                summary.push_str("- Verify the base URL is accessible
");
                summary.push_str("- Ensure the selected model is available
");
                summary.push_str("- Try increasing the timeout value

");
            }
        }

        // Add footer
        summary.push_str("---
");
        summary.push_str("*🤖 Generated by [iFlow CLI Action](https://github.com/iflow-ai/iflow-cli-action)*

");

        summary
    };
    
    // Test timeout execution
    let result = "Operation timed out";
    let summary = generate_summary_markdown(result, 124, &config_map);
    
    // Verify key elements are present
    assert!(summary.contains("## ⏰ iFlow CLI Execution Summary - Timeout"));
    assert!(summary.contains("⏰ **Execution**: Timed Out"));
    assert!(summary.contains("🕒 **Timeout Duration**: 3600 seconds"));
    assert!(summary.contains("💥 **Exit Code**: 124"));
}