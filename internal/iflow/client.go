package iflow

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"sync"

	"github.com/iflow-ai/iflow-cli-action/internal/config"
	"github.com/iflow-ai/iflow-cli-action/internal/errors"
)

// Client represents an iFlow CLI client
type Client struct {
	config *config.Config
}

// NewClient creates a new iFlow client
func NewClient(cfg *config.Config) *Client {
	return &Client{
		config: cfg,
	}
}

// Configure sets up iFlow configuration
func (c *Client) Configure() error {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return errors.NewConfigurationError("failed to get home directory", err)
	}

	iflowDir := filepath.Join(homeDir, ".iflow")
	if err := os.MkdirAll(iflowDir, 0755); err != nil {
		return errors.NewConfigurationError("failed to create .iflow directory", err)
	}

	settingsFile := filepath.Join(iflowDir, "settings.json")

	var settingsData []byte

	if c.config.SettingsJSON != "" {
		// Use provided settings JSON directly
		// Validate that it's valid JSON
		var testSettings map[string]interface{}
		if err := json.Unmarshal([]byte(c.config.SettingsJSON), &testSettings); err != nil {
			return errors.NewConfigurationError("invalid settings_json provided", err)
		}

		// Pretty format the JSON
		var prettyJSON json.RawMessage = []byte(c.config.SettingsJSON)
		settingsData, err = json.MarshalIndent(prettyJSON, "", "  ")
		if err != nil {
			return errors.NewConfigurationError("failed to format settings JSON", err)
		}
	} else {
		// Create settings from individual parameters
		settings := config.IFlowSettings{
			Theme:            "Default",
			SelectedAuthType: "iflow",
			APIKey:           c.config.APIKey,
			BaseURL:          c.config.BaseURL,
			ModelName:        c.config.Model,
			SearchAPIKey:     c.config.APIKey,
		}

		settingsData, err = json.MarshalIndent(settings, "", "  ")
		if err != nil {
			return errors.NewConfigurationError("failed to marshal settings", err)
		}
	}

	if err := os.WriteFile(settingsFile, settingsData, 0644); err != nil {
		return errors.NewConfigurationError("failed to write settings file", err)
	}

	return nil
}

// Execute runs the iFlow CLI command
func (c *Client) Execute(ctx context.Context, prompt string) (string, int, error) {
	// Prepare the command with --prompt and --yolo flags by default
	args := []string{"--yolo", "--prompt", prompt}

	// Parse and add extra arguments if provided
	if c.config.ExtraArgs != "" {
		extraArgs := c.parseExtraArgs(c.config.ExtraArgs)
		args = append(args, extraArgs...)
	}

	cmd := exec.CommandContext(ctx, "iflow", args...)

	// Create pipes for real-time output streaming
	stdoutPipe, err := cmd.StdoutPipe()
	if err != nil {
		return "", 1, errors.NewExecutionError("failed to create stdout pipe", err, nil)
	}

	stderrPipe, err := cmd.StderrPipe()
	if err != nil {
		return "", 1, errors.NewExecutionError("failed to create stderr pipe", err, nil)
	}

	// Buffer to capture all output for GitHub summary
	var outputBuffer strings.Builder

	// Create multi-writers to write to both console and buffer
	stdoutWriter := io.MultiWriter(os.Stdout, &outputBuffer)
	stderrWriter := io.MultiWriter(os.Stderr, &outputBuffer)

	// Start the command
	if err := cmd.Start(); err != nil {
		return "", 1, errors.NewExecutionError("failed to start command", err, nil)
	}

	// Use WaitGroup to ensure both goroutines complete
	var wg sync.WaitGroup
	// Create channels for goroutines to report errors
	errorChan := make(chan error, 2)

	// Start goroutines to stream output in real-time
	wg.Add(2)
	go func() {
		defer wg.Done()
		_, err := io.Copy(stdoutWriter, stdoutPipe)
		errorChan <- err
	}()

	go func() {
		defer wg.Done()
		_, err := io.Copy(stderrWriter, stderrPipe)
		errorChan <- err
	}()

	// Wait for command completion
	err = cmd.Wait()

	// Wait for both output streaming goroutines to complete
	wg.Wait()
	close(errorChan)

	// Check for timeout first
	if ctx.Err() == context.DeadlineExceeded {
		c.config.IsTimeout = true
		return outputBuffer.String(), 124, errors.NewTimeoutError("command timed out", c.config.Timeout)
	}

	// Check for streaming errors (but don't fail if we got output)
	for streamErr := range errorChan {
		if streamErr != nil && streamErr != io.EOF {
			// Log streaming errors but continue
			fmt.Fprintf(os.Stderr, "Warning: output streaming error: %v\n", streamErr)
		}
	}

	exitCode := 0
	if err != nil {
		if exitError, ok := err.(*exec.ExitError); ok {
			exitCode = exitError.ExitCode()
		} else {
			// Non-exit error (e.g., command not found)
			return outputBuffer.String(), 1, errors.NewExecutionError("command execution failed", err, nil)
		}
	}

	return outputBuffer.String(), exitCode, nil
}

// ExecutePreCmd executes pre-command if specified
func (c *Client) ExecutePreCmd() error {
	if c.config.PreCmd == "" {
		return nil
	}

	// Split the precmd into lines and execute each line
	commands := strings.Split(c.config.PreCmd, "\n")

	for _, command := range commands {
		// Skip empty lines
		command = strings.TrimSpace(command)
		if command == "" {
			continue
		}

		// Create a command to execute the pre-command
		cmd := exec.Command("sh", "-c", command)

		// Set the working directory for the command
		cmd.Dir = c.config.WorkingDir

		// Connect the command's stdin, stdout, and stderr to the current process
		cmd.Stdin = os.Stdin
		cmd.Stdout = os.Stdout
		cmd.Stderr = os.Stderr

		// Execute the command and wait for it to complete
		if err := cmd.Run(); err != nil {
			return errors.NewExecutionError("pre-command failed", err, map[string]interface{}{
				"command": command,
			})
		}
	}

	return nil
}

// InstallSpecificVersions installs specific versions of GitHub CLI and iFlow CLI if requested
func (c *Client) InstallSpecificVersions() error {
	// Install specific GitHub CLI version if requested
	if c.config.GhVersion != "" {
		installCmd := exec.Command("sh", "-c", fmt.Sprintf(
			"curl -fsSL https://github.com/cli/cli/releases/download/v%s/gh_%s_linux_amd64.tar.gz | tar xz && cp gh_%s_linux_amd64/bin/gh /usr/local/bin/ && rm -rf gh_%s_linux_amd64",
			c.config.GhVersion, c.config.GhVersion, c.config.GhVersion, c.config.GhVersion,
		))
		if err := installCmd.Run(); err != nil {
			return errors.NewExecutionError("failed to install GitHub CLI version", err, map[string]interface{}{
				"version": c.config.GhVersion,
			})
		}
	}

	// Install specific iFlow CLI version if requested
	if c.config.IFlowVersion != "" {
		installCmd := exec.Command("npm", "install", "-g", fmt.Sprintf("@iflow-ai/iflow-cli@%s", c.config.IFlowVersion))
		if err := installCmd.Run(); err != nil {
			return errors.NewExecutionError("failed to install iFlow CLI version", err, map[string]interface{}{
				"version": c.config.IFlowVersion,
			})
		}
	}

	return nil
}

// parseExtraArgs parses a space-separated string of arguments into a slice
// Handles quoted arguments with spaces properly
func (c *Client) parseExtraArgs(extraArgs string) []string {
	if extraArgs == "" {
		return []string{}
	}

	var args []string
	var current strings.Builder
	inQuotes := false
	var quoteChar rune

	for i, char := range extraArgs {
		switch char {
		case '"', '\'':
			if !inQuotes {
				// Start of quoted string
				inQuotes = true
				quoteChar = char
			} else if char == quoteChar {
				// End of quoted string
				inQuotes = false
				quoteChar = 0
			} else {
				// Quote character inside different quotes
				current.WriteRune(char)
			}
		case ' ', '\t', '\n':
			if inQuotes {
				// Space inside quotes, add to current argument
				current.WriteRune(char)
			} else {
				// Space outside quotes, end current argument
				if current.Len() > 0 {
					args = append(args, current.String())
					current.Reset()
				}
			}
		default:
			current.WriteRune(char)
		}

		// Handle end of string
		if i == len(extraArgs)-1 && current.Len() > 0 {
			args = append(args, current.String())
		}
	}

	return args
}