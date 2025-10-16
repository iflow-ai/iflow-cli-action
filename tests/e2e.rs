use std::fs;
use std::process::Command;

#[test]
fn test_basic_configuration_with_api_key() {
    // Create a temporary directory for testing
    let temp_dir = tempfile::Builder::new()
        .prefix("iflow_cli_test")
        .tempdir()
        .expect("Failed to create temporary directory");
    
    let temp_path = temp_dir.path();
    let settings_file = temp_path.join("settings.json");
    
    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "iflow-cli-action",
            "--",
            "--prompt",
            "test prompt",
            "--api-key",
            "test-api-key",
            "--settings-file-path",
            settings_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute test");
    
    // Check that the command succeeded
    assert!(output.status.success(), "Command failed with stderr: {}", String::from_utf8_lossy(&output.stderr));
    
    // Check that settings file was created
    assert!(settings_file.exists(), "Settings file was not created");
    
    // Check the content of the settings file
    let content = fs::read_to_string(&settings_file).expect("Failed to read settings file");
    assert!(content.contains("\"apiKey\": \"test-api-key\""));
    assert!(content.contains("\"selectedAuthType\": \"iflow\""));
}

#[test]
fn test_configuration_with_settings_json() {
    // Create a temporary directory for testing
    let temp_dir = tempfile::Builder::new()
        .prefix("iflow_cli_test")
        .tempdir()
        .expect("Failed to create temporary directory");
    
    let temp_path = temp_dir.path();
    let settings_file = temp_path.join("settings.json");
    
    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "iflow-cli-action",
            "--",
            "--prompt",
            "test prompt",
            "--settings-json",
            r#"{"theme":"Default","selectedAuthType":"iflow","apiKey":"test-key","baseUrl":"https://apis.iflow.cn/v1","modelName":"Qwen3-Coder","searchApiKey":"test-key"}"#,
            "--settings-file-path",
            settings_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute test");
    
    // Check that the command succeeded
    assert!(output.status.success(), "Command failed with stderr: {}", String::from_utf8_lossy(&output.stderr));
    
    // Check that settings file was created
    assert!(settings_file.exists(), "Settings file was not created");
    
    // Check the content of the settings file
    let content = fs::read_to_string(&settings_file).expect("Failed to read settings file");
    assert!(content.contains("\"apiKey\": \"test-key\""));
    assert!(content.contains("\"selectedAuthType\": \"iflow\""));
    assert!(content.contains("\"baseUrl\": \"https://apis.iflow.cn/v1\""));
    assert!(content.contains("\"modelName\": \"Qwen3-Coder\""));
}

#[test]
fn test_configuration_with_multiline_settings_json() {
    // Create a temporary directory for testing
    let temp_dir = tempfile::Builder::new()
        .prefix("iflow_cli_test")
        .tempdir()
        .expect("Failed to create temporary directory");
    
    let temp_path = temp_dir.path();
    let settings_file = temp_path.join("settings.json");
    
    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "iflow-cli-action",
            "--",
            "--prompt",
            "test prompt",
            "--settings-json",
            r#"{"theme":"Default","selectedAuthType":"iflow","apiKey":"test-key","baseUrl":"https://apis.iflow.cn/v1","modelName":"Qwen3-Coder","searchApiKey":"test-key","customField":"value\nwith\nnewlines"}"#,
            "--settings-file-path",
            settings_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute test");
    
    // Check that the command succeeded
    assert!(output.status.success(), "Command failed with stderr: {}", String::from_utf8_lossy(&output.stderr));
    
    // Check that settings file was created
    assert!(settings_file.exists(), "Settings file was not created");
    
    // Check the content of the settings file
    let content = fs::read_to_string(&settings_file).expect("Failed to read settings file");
    assert!(content.contains("\"customField\": \"value\\nwith\\nnewlines\""));
}

#[test]
fn test_validation_error_missing_prompt() {
    // Create a temporary directory for testing
    let temp_dir = tempfile::Builder::new()
        .prefix("iflow_cli_test")
        .tempdir()
        .expect("Failed to create temporary directory");
    
    let temp_path = temp_dir.path();
    let settings_file = temp_path.join("settings.json");
    
    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "iflow-cli-action",
            "--",
            "--api-key",
            "test-api-key",
            "--settings-file-path",
            settings_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute test");
    
    // Check that the command failed
    assert!(!output.status.success(), "Command should have failed but succeeded");
    
    // Check the error message
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Validation Error: prompt input is required and cannot be empty"));
}

#[test]
fn test_validation_error_missing_api_key_and_settings_json() {
    // Create a temporary directory for testing
    let temp_dir = tempfile::Builder::new()
        .prefix("iflow_cli_test")
        .tempdir()
        .expect("Failed to create temporary directory");
    
    let temp_path = temp_dir.path();
    let settings_file = temp_path.join("settings.json");
    
    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "iflow-cli-action",
            "--",
            "--prompt",
            "test prompt",
            "--settings-file-path",
            settings_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute test");
    
    // Check that the command failed
    assert!(!output.status.success(), "Command should have failed but succeeded");
    
    // Check the error message
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Validation Error: api_key input is required and cannot be empty"));
}

#[test]
fn test_validation_error_invalid_settings_json() {
    // Create a temporary directory for testing
    let temp_dir = tempfile::Builder::new()
        .prefix("iflow_cli_test")
        .tempdir()
        .expect("Failed to create temporary directory");
    
    let temp_path = temp_dir.path();
    let settings_file = temp_path.join("settings.json");
    
    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "iflow-cli-action",
            "--",
            "--prompt",
            "test prompt",
            "--settings-json",
            r#"{"invalid": json}"#,
            "--settings-file-path",
            settings_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute test");
    
    // Check that the command failed
    assert!(!output.status.success(), "Command should have failed but succeeded");
    
    // Check the error message
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Validation Error: invalid settings_json provided"));
}