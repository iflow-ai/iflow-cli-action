package cmd

import (
	"errors"
	"fmt"
	"os"

	"github.com/iflow-ai/iflow-cli-action/internal/app"
	"github.com/iflow-ai/iflow-cli-action/internal/config"
	internalerrors "github.com/iflow-ai/iflow-cli-action/internal/errors"
	"github.com/iflow-ai/iflow-cli-action/internal/github"
	"github.com/spf13/cobra"
)

var cfg *config.Config

// rootCmd represents the base command when called without any subcommands
var rootCmd = &cobra.Command{
	Use:   "iflow-action",
	Short: "iFlow CLI Action wrapper",
	Long: `A GitHub Action wrapper for iFlow CLI that provides intelligent code assistance.
	
This tool can run in two modes:
1. GitHub Actions mode: Uses environment variables (INPUT_*) for configuration
2. CLI mode: Uses command-line flags for configuration`,
	RunE: func(cmd *cobra.Command, args []string) error {
		return runIFlowAction()
	},
}

// Execute adds all child commands to the root command and sets flags appropriately.
func Execute() {
	if err := rootCmd.Execute(); err != nil {
		var appErr *internalerrors.AppError
		if errors.As(err, &appErr) {
			handleAppError(appErr)
		} else {
			fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		}
		os.Exit(1)
	}
}

func init() {
	// Initialize config with defaults
	cfg = config.NewConfig()

	// Define flags
	rootCmd.Flags().StringVarP(&cfg.Prompt, "prompt", "p", "", "The prompt to send to iFlow CLI (required in CLI mode)")
	rootCmd.Flags().StringVar(&cfg.APIKey, "api-key", "", "API key for iFlow authentication")
	rootCmd.Flags().StringVar(&cfg.SettingsJSON, "settings-json", "", "Complete settings JSON configuration")
	rootCmd.Flags().StringVar(&cfg.BaseURL, "base-url", "https://apis.iflow.cn/v1", "Base URL for the iFlow API")
	rootCmd.Flags().StringVar(&cfg.Model, "model", "Qwen3-Coder", "Model name to use")
	rootCmd.Flags().StringVar(&cfg.WorkingDir, "working-directory", ".", "Working directory for execution")
	rootCmd.Flags().IntVar(&cfg.Timeout, "timeout", 3600, "Timeout in seconds (1-86400)")
	rootCmd.Flags().StringVar(&cfg.ExtraArgs, "extra-args", "", "Additional command line arguments to pass to iFlow CLI")
	rootCmd.Flags().StringVar(&cfg.PreCmd, "precmd", "", "Shell command(s) to execute before running iFlow CLI")
	rootCmd.Flags().StringVar(&cfg.GhVersion, "gh-version", "", "Version of GitHub CLI to install")
	rootCmd.Flags().StringVar(&cfg.IFlowVersion, "iflow-version", "", "Version of iFlow CLI to install")
	rootCmd.Flags().BoolVar(&cfg.UseACP, "use-acp", false, "Enable ACP (Agent Communication Protocol) mode")
	rootCmd.Flags().BoolVar(&cfg.UseEnvVars, "use-env-vars", false, "Use environment variables for configuration (GitHub Actions mode)")

	// Mark required flags only if not in GitHub Actions mode - this will be validated later
}

func runIFlowAction() error {
	// Create GitHub Actions handler
	ghActions := github.NewGitHubActions()

	// Create and run the application
	application := app.NewApp(cfg, ghActions)
	return application.Run()
}

// For testing purposes, expose the config
func GetConfig() *config.Config {
	return cfg
}

func handleAppError(err *internalerrors.AppError) {
	switch err.Type {
	case internalerrors.ErrTypeValidation:
		fmt.Fprintf(os.Stderr, "Validation Error: %s\n", err.Message)
	case internalerrors.ErrTypeTimeout:
		fmt.Fprintf(os.Stderr, "Timeout Error: %s\n", err.Message)
	case internalerrors.ErrTypeConfiguration:
		fmt.Fprintf(os.Stderr, "Configuration Error: %s\n", err.Message)
	default:
		fmt.Fprintf(os.Stderr, "Error: %s\n", err.Message)
	}
	
	if err.Context != nil {
		fmt.Fprintf(os.Stderr, "Context: %+v\n", err.Context)
	}
}