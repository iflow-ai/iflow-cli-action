package app

import (
	"context"
	"fmt"
	"os"
	"os/exec"
	"strings"
	"time"

	"github.com/iflow-ai/iflow-cli-action/internal/config"
	"github.com/iflow-ai/iflow-cli-action/internal/errors"
	"github.com/iflow-ai/iflow-cli-action/internal/github"
	"github.com/iflow-ai/iflow-cli-action/internal/iflow"
)

// App represents the main application
type App struct {
	config    *config.Config
	ghActions github.Actions
	iflowClient *iflow.Client
}

// NewApp creates a new application instance
func NewApp(cfg *config.Config, ghActions github.Actions) *App {
	return &App{
		config:      cfg,
		ghActions:   ghActions,
		iflowClient: iflow.NewClient(cfg),
	}
}

// Run executes the main application logic
func (a *App) Run() error {
	// Print version information
	a.printVersionInfo()

	// Load configuration from environment if needed
	if a.config.UseEnvVars || config.IsGitHubActions() {
		if err := a.config.LoadFromEnv(); err != nil {
			return err
		}
	}

	// Validate configuration
	if err := a.config.Validate(); err != nil {
		return err
	}

	// Install specific versions if requested
	if err := a.iflowClient.InstallSpecificVersions(); err != nil {
		return err
	}

	// Setup working directory
	if err := a.setupWorkingDirectory(); err != nil {
		return err
	}

	// Configure iFlow
	if err := a.iflowClient.Configure(); err != nil {
		return err
	}

	// Execute pre-command if specified
	if err := a.iflowClient.ExecutePreCmd(); err != nil {
		return err
	}

	// Execute iFlow CLI command
	result, exitCode, err := a.executeIFlow()
	if err != nil {
		return err
	}

	// Handle outputs
	if err := a.handleOutputs(result, exitCode); err != nil {
		return err
	}

	if exitCode != 0 {
		return errors.NewExecutionError(fmt.Sprintf("iFlow CLI exited with code %d", exitCode), nil, nil)
	}

	return nil
}

func (a *App) printVersionInfo() {
	// Print iFlow CLI version
	if version, err := a.getCommandVersion("iflow"); err == nil {
		a.ghActions.Info(fmt.Sprintf("iFlow CLI version: %s", version))
	}

	// Print GitHub CLI version
	if version, err := a.getCommandVersion("gh"); err == nil {
		a.ghActions.Info(fmt.Sprintf("GitHub CLI version: %s", version))
	}
}

func (a *App) getCommandVersion(command string) (string, error) {
	cmd := exec.Command(command, "--version")
	output, err := cmd.CombinedOutput()
	if err != nil {
		return "", err
	}
	return strings.TrimSpace(string(output)), nil
}

func (a *App) setupWorkingDirectory() error {
	if a.config.WorkingDir != "." && a.config.WorkingDir != "" {
		if err := os.Chdir(a.config.WorkingDir); err != nil {
			return errors.NewConfigurationError("failed to change working directory", err)
		}
	}
	return nil
}

func (a *App) executeIFlow() (string, int, error) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Duration(a.config.Timeout)*time.Second)
	defer cancel()

	a.ghActions.Info(fmt.Sprintf("Executing iFlow CLI prompt with --prompt and --yolo: %s", a.config.Prompt))
	a.ghActions.Info(fmt.Sprintf("Command timeout set to: %d seconds", a.config.Timeout))

	return a.iflowClient.Execute(ctx, a.config.Prompt)
}

func (a *App) handleOutputs(result string, exitCode int) error {
	// Set outputs (GitHub Actions mode) or print results (CLI mode)
	if a.config.UseEnvVars || config.IsGitHubActions() {
		a.ghActions.SetOutput("result", result)
		a.ghActions.SetOutput("exit_code", fmt.Sprintf("%d", exitCode))

		fmt.Println(result)

		// Write to GitHub Actions step summary
		configMap := map[string]interface{}{
			"isTimeout":    a.config.IsTimeout,
			"timeout":      a.config.Timeout,
			"model":        a.config.Model,
			"baseURL":      a.config.BaseURL,
			"workingDir":   a.config.WorkingDir,
			"extraArgs":    a.config.ExtraArgs,
			"prompt":       a.config.Prompt,
		}
		
		summaryContent := github.GenerateSummaryMarkdown(result, exitCode, configMap)
		a.ghActions.WriteStepSummary(summaryContent)
	} else {
		fmt.Printf("Exit Code: %d\n", exitCode)
		fmt.Printf("Result:\n%s\n", result)
	}

	if exitCode != 0 {
		if a.config.UseEnvVars || config.IsGitHubActions() {
			a.ghActions.SetFailed(fmt.Sprintf("iFlow CLI exited with code %d", exitCode))
		}
	}

	return nil
}