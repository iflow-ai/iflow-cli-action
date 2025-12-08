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
            "--dry-run",
            "--settings-file-path",
            settings_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute test");

    // Check that the command succeeded
    assert!(
        output.status.success(),
        "Command failed with stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

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
            r#"{"theme":"Default","selectedAuthType":"iflow","apiKey":"test-key","baseUrl":"https://apis.iflow.cn/v1","modelName":"qwen3-coder-plus","searchApiKey":"test-key"}"#,
            "--dry-run",
            "--settings-file-path",
            settings_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute test");

    // Check that the command succeeded
    assert!(
        output.status.success(),
        "Command failed with stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Check that settings file was created
    assert!(settings_file.exists(), "Settings file was not created");

    // Check the content of the settings file
    let content = fs::read_to_string(&settings_file).expect("Failed to read settings file");
    assert!(content.contains("\"apiKey\": \"test-key\""));
    assert!(content.contains("\"selectedAuthType\": \"iflow\""));
    assert!(content.contains("\"baseUrl\": \"https://apis.iflow.cn/v1\""));
    assert!(content.contains("\"modelName\": \"qwen3-coder-plus\""));
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
            r#"{"theme":"Default","selectedAuthType":"iflow","apiKey":"test-key","baseUrl":"https://apis.iflow.cn/v1","modelName":"qwen3-coder-plus","searchApiKey":"test-key","customField":"value\nwith\nnewlines"}"#,
            "--dry-run",
            "--settings-file-path",
            settings_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute test");

    // Check that the command succeeded
    assert!(
        output.status.success(),
        "Command failed with stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

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
            "--dry-run",
            "--settings-file-path",
            settings_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute test");

    // Check that the command failed
    assert!(
        !output.status.success(),
        "Command should have failed but succeeded"
    );

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
            "--dry-run",
            "--settings-file-path",
            settings_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute test");

    // Check that the command failed
    assert!(
        !output.status.success(),
        "Command should have failed but succeeded"
    );

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
            "--dry-run",
            "--settings-file-path",
            settings_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute test");

    // Check that the command failed
    assert!(
        !output.status.success(),
        "Command should have failed but succeeded"
    );

    // Check the error message
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Validation Error: invalid settings_json provided"));
}

#[test]
fn test_precmd_execution() {
    // Create a temporary directory for testing
    let temp_dir = tempfile::Builder::new()
        .prefix("iflow_cli_test")
        .tempdir()
        .expect("Failed to create temporary directory");

    let temp_path = temp_dir.path();
    let settings_file = temp_path.join("settings.json");

    // Create a test file to verify precmd execution
    let test_file = temp_path.join("test_precmd.txt");

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
            "--precmd",
            &format!("echo 'precmd executed' > {}", test_file.to_str().unwrap()),
            "--dry-run",
            "--settings-file-path",
            settings_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute test");

    // Check that the command succeeded
    assert!(
        output.status.success(),
        "Command failed with stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Check that the precmd was executed by verifying the test file was created
    assert!(
        test_file.exists(),
        "PreCmd did not execute - test file was not created"
    );

    // Check the content of the test file
    let content = fs::read_to_string(&test_file).expect("Failed to read test file");
    assert_eq!(content.trim(), "precmd executed");
}

#[test]
fn test_precmd_execution_multiple_commands() {
    // Create a temporary directory for testing
    let temp_dir = tempfile::Builder::new()
        .prefix("iflow_cli_test")
        .tempdir()
        .expect("Failed to create temporary directory");

    let temp_path = temp_dir.path();
    let settings_file = temp_path.join("settings.json");

    // Create test files to verify precmd execution
    let test_file1 = temp_path.join("test_precmd1.txt");
    let test_file2 = temp_path.join("test_precmd2.txt");

    let precmd = format!(
        "echo 'first command' > {}\necho 'second command' > {}",
        test_file1.to_str().unwrap(),
        test_file2.to_str().unwrap()
    );

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
            "--precmd",
            &precmd,
            "--dry-run",
            "--settings-file-path",
            settings_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute test");

    // Check that the command succeeded
    assert!(
        output.status.success(),
        "Command failed with stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Check that both precmd commands were executed
    assert!(
        test_file1.exists(),
        "First precmd command did not execute - test file was not created"
    );
    assert!(
        test_file2.exists(),
        "Second precmd command did not execute - test file was not created"
    );

    // Check the content of the test files
    let content1 = fs::read_to_string(&test_file1).expect("Failed to read first test file");
    assert_eq!(content1.trim(), "first command");

    let content2 = fs::read_to_string(&test_file2).expect("Failed to read second test file");
    assert_eq!(content2.trim(), "second command");
}

#[test]
fn test_precmd_execution_fails() {
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
            "--precmd",
            "exit 1", // This command will fail
            "--dry-run",
            "--settings-file-path",
            settings_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute test");

    // Check that the command failed
    assert!(
        !output.status.success(),
        "Command should have failed but succeeded"
    );

    // Check the error message
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Pre-command Error: pre-command 'exit 1' failed with exit code: Some(1)")
    );
}

#[test]
fn test_dry_run_mode() {
    // Create a temporary directory for testing
    let temp_dir = tempfile::Builder::new()
        .prefix("iflow_cli_test")
        .tempdir()
        .expect("Failed to create temporary directory");

    let temp_path = temp_dir.path();
    let settings_file = temp_path.join("settings.json");

    let output = Command::new("cargo")
        .env("GITHUB_ACTIONS", "true") // Enable GitHub Actions mode to force WebSocket usage
        .args([
            "run",
            "--bin",
            "iflow-cli-action",
            "--",
            "--prompt",
            "test prompt",
            "--api-key",
            "test-api-key",
            "--dry-run",
            "--settings-file-path",
            settings_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute test");

    // Check that the command succeeded
    assert!(
        output.status.success(),
        "Command failed with stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Check that settings file was created
    assert!(settings_file.exists(), "Settings file was not created");

    // Check that dry run message was printed
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("DRY RUN: Would execute communicate_with_iflow_cli_via_acp()"));
}

#[test]
fn test_github_actions_outputs_written() {
    // Create a temporary directory for testing
    let temp_dir = tempfile::Builder::new()
        .prefix("iflow_cli_test")
        .tempdir()
        .expect("Failed to create temporary directory");

    let temp_path = temp_dir.path();
    let settings_file = temp_path.join("settings.json");

    // Temp files for GITHUB_OUTPUT and GITHUB_STEP_SUMMARY
    let github_output = temp_path.join("github_output.txt");
    let github_summary = temp_path.join("github_summary.md");

    let output = Command::new("cargo")
        .env("GITHUB_ACTIONS", "true")
        .env("GITHUB_OUTPUT", &github_output)
        .env("GITHUB_STEP_SUMMARY", &github_summary)
        .args([
            "run",
            "--bin",
            "iflow-cli-action",
            "--",
            "--prompt",
            "test prompt",
            "--api-key",
            "test-api-key",
            "--dry-run",
            "--settings-file-path",
            settings_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute test");

    // Command should succeed
    assert!(
        output.status.success(),
        "Command failed: stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Read GITHUB_OUTPUT file
    let content = std::fs::read_to_string(&github_output).expect("Failed to read GITHUB_OUTPUT");

    // Should contain exit_code and result keys
    assert!(
        content.contains("exit_code=0") || content.contains("exit_code<<EOF"),
        "GITHUB_OUTPUT missing exit_code: {}",
        content
    );
    assert!(
        content.contains("result=") || content.contains("result<<EOF"),
        "GITHUB_OUTPUT missing result: {}",
        content
    );
}

// This test only runs on GitHub Actions. Locally it will be skipped.
#[test]
fn test_only_on_github_actions() {
    use std::io::{BufRead, BufReader};
    use std::thread;

    // If not running in GitHub Actions, skip the test by returning early.
    if std::env::var("GITHUB_ACTIONS").ok().as_deref() != Some("true") {
        eprintln!("Skipping test_only_on_github_actions: not running in GitHub Actions");
        return;
    }

    let api_key =
        std::env::var("CI_IFLOW_API_KEY").expect("IFLOW_API_KEY not set in GitHub Actions");

    let mut child = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "iflow-cli-action",
            "--",
            "--prompt",
            "简单介绍下当前项目的功能，作为 Rust 和 GitHub Actions 专家，分析计划下如何优化这个项目",
            "--api-key",
            api_key.as_str(),
        ])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute test");

    // Handle stdout in real-time
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        thread::spawn(move || {
            for line in reader.lines() {
                match line {
                    Ok(line) => println!("STDOUT: {}", line),
                    Err(e) => eprintln!("Error reading stdout: {}", e),
                }
            }
        });
    }

    // Handle stderr in real-time
    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);
        thread::spawn(move || {
            for line in reader.lines() {
                match line {
                    Ok(line) => eprintln!("STDERR: {}", line),
                    Err(e) => eprintln!("Error reading stderr: {}", e),
                }
            }
        });
    }

    // Wait for the process to complete
    let status = child.wait().expect("Failed to wait for test process");

    assert!(
        status.success(),
        "Command failed with exit code: {:?}",
        status.code()
    );
}
