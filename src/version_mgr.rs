use std::process::Command;

/// Gets the version of a command by running it with --version flag
pub fn get_command_version(command: &str) -> Result<String, String> {
    let output = Command::new(command)
        .arg("--version")
        .output()
        .map_err(|e| format!("failed to execute {} --version: {}", command, e))?;

    if output.status.success() {
        let version = String::from_utf8_lossy(&output.stdout);
        Ok(version.trim().to_string())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("failed to get {} version: {}", command, error))
    }
}

/// Prints version information as GitHub Actions annotations
pub fn print_version_info() {
    // Check if we're in GitHub Actions environment
    let is_github_actions = std::env::var("GITHUB_ACTIONS").is_ok();

    // Print iFlow CLI version
    match get_command_version("iflow") {
        Ok(version) => {
            if is_github_actions {
                println!("::notice::iFlow CLI version: {}", version);
            } else {
                println!("INFO: iFlow CLI version: {}", version);
            }
        }
        Err(e) => {
            if is_github_actions {
                println!("::warning::Failed to get iFlow CLI version: {}", e);
            } else {
                eprintln!("WARNING: Failed to get iFlow CLI version: {}", e);
            }
        }
    }

    // Print GitHub CLI version
    match get_command_version("gh") {
        Ok(version) => {
            if is_github_actions {
                println!("::notice::GitHub CLI version: {}", version);
            } else {
                println!("INFO: GitHub CLI version: {}", version);
            }
        }
        Err(e) => {
            if is_github_actions {
                println!("::warning::Failed to get GitHub CLI version: {}", e);
            } else {
                eprintln!("WARNING: Failed to get GitHub CLI version: {}", e);
            }
        }
    }
}

/// Installs specific versions of GitHub CLI and iFlow CLI if requested
pub fn install_specific_versions(gh_version: &Option<String>, iflow_version: &Option<String>) -> Result<(), String> {
    // Install specific GitHub CLI version if requested
    if let Some(gh_version) = gh_version {
        if !gh_version.is_empty() {
            println!("Installing GitHub CLI version: {}", gh_version);

            let install_cmd = format!(
                "curl -fsSL https://github.com/cli/cli/releases/download/v{0}/gh_{0}_linux_amd64.tar.gz | tar xz && cp gh_{0}_linux_amd64/bin/gh /usr/local/bin/ && rm -rf gh_{0}_linux_amd64",
                gh_version
            );

            let output = std::process::Command::new("sh")
                .arg("-c")
                .arg(&install_cmd)
                .output()
                .map_err(|e| {
                    format!("failed to execute GitHub CLI installation command: {}", e)
                })?;

            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(format!(
                    "failed to install GitHub CLI version {}: {}",
                    gh_version, error
                ));
            }

            println!(
                "✅ Successfully installed GitHub CLI version: {}",
                gh_version
            );
        }
    }

    // Install specific iFlow CLI version if requested
    if let Some(iflow_version) = iflow_version {
        if !iflow_version.is_empty() {
            println!("Installing iFlow CLI version: {}", iflow_version);

            let output = std::process::Command::new("npm")
                .arg("install")
                .arg("-g")
                .arg(format!("@iflow-ai/iflow-cli@{}", iflow_version))
                .output()
                .map_err(|e| {
                    format!("failed to execute iFlow CLI installation command: {}", e)
                })?;

            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(format!(
                    "failed to install iFlow CLI version {}: {}",
                    iflow_version, error
                ));
            }

            println!(
                "✅ Successfully installed iFlow CLI version: {}",
                iflow_version
            );
        }
    }

    Ok(())
}