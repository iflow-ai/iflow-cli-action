use std::process::Command;

/// Executes pre-command if specified
pub fn execute_precmd(precmd: &Option<String>, working_directory: &str) -> Result<(), String> {
    if let Some(precmd) = precmd
        && !precmd.is_empty()
    {
        // Split the precmd into lines and execute each line
        let commands: Vec<&str> = precmd.split('\n').collect();

        for command in commands {
            // Skip empty lines
            let command = command.trim();
            if command.is_empty() {
                continue;
            }

            println!("Executing pre-command: {}", command);

            // Create a command to execute the pre-command
            let output = Command::new("sh")
                .arg("-c")
                .arg(command)
                .current_dir(working_directory)
                .output()
                .map_err(|e| format!("failed to execute pre-command '{}': {}", command, e))?;

            // Print stdout and stderr
            if !output.stdout.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            if !output.stderr.is_empty() {
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            }

            // Check if command failed
            if !output.status.success() {
                return Err(format!(
                    "pre-command '{}' failed with exit code: {:?}",
                    command,
                    output.status.code()
                ));
            }
        }
    }

    Ok(())
}
