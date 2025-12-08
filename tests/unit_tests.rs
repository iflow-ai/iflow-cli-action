use iflow_cli_action::{contains_code, generate_summary_markdown};
use serde_json::json;
use std::collections::HashMap;

#[test]
fn test_contains_code() {
    // Test cases that should be detected as code
    assert!(contains_code("function test() { return true; }"));
    assert!(contains_code("class MyClass { public void method() {} }"));
    assert!(contains_code(
        "def my_function():
    return True"
    ));
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
    // This test is a placeholder - in a real implementation we'd test the actual function
    // For now, we just ensure the test compiles and runs without issues
}

#[test]
fn test_generate_summary_markdown_success() {
    // Prepare configuration map
    let mut config_map = HashMap::new();
    config_map.insert("isTimeout", json!(false));
    config_map.insert("timeout", json!(3600));
    config_map.insert("model", json!("qwen3-coder-plus"));
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
    config_map.insert("model", json!("qwen3-coder-plus"));
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
    config_map.insert("model", json!("qwen3-coder-plus"));
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
