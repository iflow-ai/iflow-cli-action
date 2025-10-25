use clap::Parser;
use std::env;

mod cli;
mod executor;
mod github;
mod iflow;
mod version_mgr;

use cli::args::CliArgs;
use executor::execute_precmd;
use github::outputs::write_github_output;
use iflow::acp_client::{AcpClientParams, communicate_with_iflow_cli_via_acp};
use iflow::config::IFlowConfig;
use version_mgr::{install_specific_versions, print_version_info};

#[tokio::main]
async fn main() -> Result<(), String> {
    // Check if we're running in GitHub Actions environment
    let is_github_actions = env::var("GITHUB_ACTIONS").is_ok();

    // Parse CLI arguments
    let cli = CliArgs::parse();

    // Validate the arguments
    if let Err(e) = cli::validation::validate_args(
        cli.prompt.as_ref(),
        cli.api_key.as_ref(),
        cli.settings_json.as_ref(),
        cli.timeout,
    ) {
        eprintln!("Validation Error: {}", e);
        std::process::exit(1);
    }

    // Install specific versions if requested
    if let Err(e) = install_specific_versions(&cli.gh_version, &cli.iflow_version) {
        eprintln!("Installation Error: {}", e);
        std::process::exit(1);
    }

    // Print version information (after installing specific versions)
    print_version_info();

    // Configure iFlow settings
    let iflow_config = IFlowConfig {
        base_url: cli.base_url.clone(),
        model: cli.model.clone(),
    };

    if let Err(e) = iflow_config.configure(
        cli.settings_json.as_ref(),
        cli.api_key.as_ref().unwrap_or(&String::new()),
        cli.settings_file_path.as_ref(),
    ) {
        eprintln!("Configuration Error: {}", e);
        std::process::exit(1);
    }

    // Execute pre-command if specified
    if let Err(e) = execute_precmd(&cli.precmd, &cli.working_directory) {
        eprintln!("Pre-command Error: {}", e);
        std::process::exit(1);
    }

    // Run ACP client in GitHub Actions environment or when explicitly requested
    if is_github_actions {
        // Skip actual execution in dry-run mode
        if cli.dry_run {
            println!("DRY RUN: Would execute communicate_with_iflow_cli_via_acp()");
            // In dry-run, write empty outputs with exit_code 0
            let _ = write_github_output("result", "");
            let _ = write_github_output("exit_code", "0");
            return Ok(());
        }

        // Run and capture summary (if any)
        match communicate_with_iflow_cli_via_acp(AcpClientParams {
            prompt: cli.prompt.as_ref().unwrap(),
            base_url: &cli.base_url,
            model: &cli.model,
            working_directory: &cli.working_directory,
            timeout: cli.timeout,
            debug: cli.debug,
        })
        .await
        {
            Ok(maybe_summary) => {
                if let Some(summary_content) = maybe_summary {
                    // Write outputs: result (may be multiline) and exit_code=0
                    if let Err(e) = write_github_output("result", &summary_content) {
                        eprintln!("Warning: failed to write result output: {}", e);
                    }
                } else {
                    // No summary produced
                    let _ = write_github_output("result", "");
                }

                if let Err(e) = write_github_output("exit_code", "0") {
                    eprintln!("Warning: failed to write exit_code output: {}", e);
                }

                return Ok(());
            }
            Err(err_msg) => {
                // On error, write result with the error message and exit_code 1
                let _ = write_github_output("result", &format!("ERROR: {}", err_msg));
                let _ = write_github_output("exit_code", "1");
                eprintln!("ACP client error: {}", err_msg);
                std::process::exit(1);
            }
        }
    }

    // Print the parsed arguments for verification
    println!("Parsed arguments:");
    println!("  prompt: {:?}", cli.prompt);
    println!("  api_key: {:?}", cli.api_key);
    println!("  settings_json: {:?}", cli.settings_json);
    println!("  base_url: {}", cli.base_url);
    println!("  model: {}", cli.model);
    println!("  working_directory: {}", cli.working_directory);
    println!("  timeout: {}", cli.timeout);
    println!("  precmd: {:?}", cli.precmd);
    println!("  gh_version: {:?}", cli.gh_version);
    println!("  iflow_version: {:?}", cli.iflow_version);
    println!("  dry_run: {}", cli.dry_run);

    Ok(())
}
