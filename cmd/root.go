package cmd

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"os/exec"
	"path/filepath"
	"strconv"
	"strings"
	"sync"
	"time"

	"github.com/spf13/cobra"
)

// Config holds all configuration options
type Config struct {
	Prompt       string
	APIKey       string
	SettingsJSON string
	BaseURL      string
	Model        string
	WorkingDir   string
	Timeout      int
	ExtraArgs    string // Additional command line arguments for iFlow CLI
	PreCmd       string // Shell command(s) to execute before running iFlow CLI
	GhVersion    string // Version of GitHub CLI to install
	IFlowVersion string // Version of iFlow CLI to install
	UseEnvVars   bool   // Flag to indicate whether to use environment variables (GitHub Actions mode)
	IsTimeout    bool   // Flag to indicate if execution timed out
}

// IFlowSettings represents the iFlow configuration
type IFlowSettings struct {
	Theme            string `json:"theme"`
	SelectedAuthType string `json:"selectedAuthType"`
	APIKey           string `json:"apiKey"`
	BaseURL          string `json:"baseUrl"`
	ModelName        string `json:"modelName"`
	SearchAPIKey     string `json:"searchApiKey"`
}

var config Config

// For testing purposes, expose the config
func GetConfig() Config {
	return config
}

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
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}

func init() {
	// Define flags
	rootCmd.Flags().StringVarP(&config.Prompt, "prompt", "p", "", "The prompt to send to iFlow CLI (required in CLI mode)")
	rootCmd.Flags().StringVar(&config.APIKey, "api-key", "", "API key for iFlow authentication")
	rootCmd.Flags().StringVar(&config.SettingsJSON, "settings-json", "", "Complete settings JSON configuration")
	rootCmd.Flags().StringVar(&config.BaseURL, "base-url", "https://apis.iflow.cn/v1", "Base URL for the iFlow API")
	rootCmd.Flags().StringVar(&config.Model, "model", "Qwen3-Coder", "Model name to use")
	rootCmd.Flags().StringVar(&config.WorkingDir, "working-directory", ".", "Working directory for execution")
	rootCmd.Flags().IntVar(&config.Timeout, "timeout", 3600, "Timeout in seconds (1-86400)")
	rootCmd.Flags().StringVar(&config.ExtraArgs, "extra-args", "", "Additional command line arguments to pass to iFlow CLI")
	rootCmd.Flags().StringVar(&config.PreCmd, "precmd", "", "Shell command(s) to execute before running iFlow CLI")
	rootCmd.Flags().StringVar(&config.GhVersion, "gh-version", "", "Version of GitHub CLI to install")
	rootCmd.Flags().StringVar(&config.IFlowVersion, "iflow-version", "", "Version of iFlow CLI to install")
	rootCmd.Flags().BoolVar(&config.UseEnvVars, "use-env-vars", false, "Use environment variables for configuration (GitHub Actions mode)")

	// Mark required flags only if not in GitHub Actions mode - this will be validated later
}

func runIFlowAction() error {
	// Print iFlow CLI version and GitHub CLI version
	printIFlowVersion()
	printGhVersion()

	// If use-env-vars is set or we detect GitHub Actions environment, use environment variables
	if config.UseEnvVars || isGitHubActions() {
		if err := loadConfigFromEnv(); err != nil {
			return fmt.Errorf("failed to load config from environment: %w", err)
		}
	}

	// Validate configuration
	if err := validateConfig(); err != nil {
		return err
	}

	// Install specific versions if requested
	if err := installSpecificVersions(); err != nil {
		return fmt.Errorf("failed to install specific versions: %w", err)
	}

	// Setup working directory
	if config.WorkingDir != "." && config.WorkingDir != "" {
		if err := os.Chdir(config.WorkingDir); err != nil {
			return fmt.Errorf("failed to change working directory: %w", err)
		}
	}

	// iFlow CLI is pre-installed in Docker image
	info("iFlow CLI is pre-installed and ready to use")

	// Configure iFlow settings
	info("Configuring iFlow settings...")
	if err := configureIFlow(); err != nil {
		return fmt.Errorf("failed to configure iFlow: %w", err)
	}

	// Execute pre-command if specified
	if config.PreCmd != "" {
		info(fmt.Sprintf("Executing pre-command: %s", config.PreCmd))
		if err := executePreCmd(); err != nil {
			return fmt.Errorf("failed to execute pre-command: %w", err)
		}
	}

	// Execute iFlow CLI command with --prompt and --yolo flags
	info(fmt.Sprintf("Executing iFlow CLI prompt with --prompt and --yolo: %s", config.Prompt))
	info(fmt.Sprintf("Command timeout set to: %d seconds", config.Timeout))
	result, exitCode, err := executeIFlow()
	if err != nil {
		return fmt.Errorf("failed to execute iFlow CLI: %w", err)
	}

	// Set outputs (GitHub Actions mode) or print results (CLI mode)
	if config.UseEnvVars || isGitHubActions() {
		setOutput("result", result)
		setOutput("exit_code", fmt.Sprintf("%d", exitCode))

		fmt.Println(result)

		// Write to GitHub Actions step summary
		if err := writeStepSummary(result, exitCode); err != nil {
			info(fmt.Sprintf("Failed to write step summary: %v", err))
		}
	} else {
		fmt.Printf("Exit Code: %d\n", exitCode)
		fmt.Printf("Result:\n%s\n", result)
	}

	if exitCode != 0 {
		if config.UseEnvVars || isGitHubActions() {
			setFailed(fmt.Sprintf("iFlow CLI exited with code %d", exitCode))
		}
		return fmt.Errorf("iFlow CLI exited with code %d", exitCode)
	}

	info("iFlow CLI execution completed successfully")
	return nil
}

func isGitHubActions() bool {
	return os.Getenv("GITHUB_ACTIONS") == "true"
}

// LoadConfigFromEnv loads configuration from environment variables (GitHub Actions convention)
// This function is exported for testing purposes
func LoadConfigFromEnv() error {
	// Load configuration from environment variables (GitHub Actions convention)
	if prompt := getInput("prompt"); prompt != "" {
		config.Prompt = strings.TrimSpace(prompt)
	}
	if apiKey := getInput("api_key"); apiKey != "" {
		config.APIKey = apiKey
	}
	if settingsJSON := getInput("settings_json"); settingsJSON != "" {
		config.SettingsJSON = settingsJSON
	}
	if baseURL := getInput("base_url"); baseURL != "" {
		config.BaseURL = baseURL
	}
	if model := getInput("model"); model != "" {
		config.Model = model
	}
	if workingDir := getInput("working_directory"); workingDir != "" {
		config.WorkingDir = workingDir
	}
	if timeoutStr := getInput("timeout"); timeoutStr != "" {
		info(fmt.Sprintf("Parsing timeout value from input: '%s'", timeoutStr))
		timeout, err := strconv.Atoi(timeoutStr)
		if err != nil {
			if config.UseEnvVars || isGitHubActions() {
				setFailed(fmt.Sprintf("Invalid timeout value: '%s'. Timeout must be a valid integer between 1 and 86400 seconds.", timeoutStr))
			}
			return fmt.Errorf("invalid timeout value: '%s'. Timeout must be a valid integer between 1 and 86400 seconds", timeoutStr)
		}
		config.Timeout = timeout
		info(fmt.Sprintf("Timeout value set to: %d seconds", config.Timeout))
	}

	if extraArgs := getInput("extra_args"); extraArgs != "" {
		config.ExtraArgs = strings.TrimSpace(extraArgs)
		info(fmt.Sprintf("Extra arguments set to: '%s'", config.ExtraArgs))
	}

	if preCmd := getInput("precmd"); preCmd != "" {
		config.PreCmd = strings.TrimSpace(preCmd)
		info(fmt.Sprintf("Pre-command set to: '%s'", config.PreCmd))
	}

	if ghVersion := getInput("gh_version"); ghVersion != "" {
		config.GhVersion = strings.TrimSpace(ghVersion)
		info(fmt.Sprintf("GitHub CLI version set to: '%s'", config.GhVersion))
	}

	if iflowVersion := getInput("iflow_version"); iflowVersion != "" {
		config.IFlowVersion = strings.TrimSpace(iflowVersion)
		info(fmt.Sprintf("iFlow CLI version set to: '%s'", config.IFlowVersion))
	}

	return nil
}

// For backward compatibility
func loadConfigFromEnv() error {
	return LoadConfigFromEnv()
}

func validateConfig() error {
	// Validate required inputs
	if config.Prompt == "" {
		if config.UseEnvVars || isGitHubActions() {
			setFailed("prompt input is required and cannot be empty")
			return fmt.Errorf("prompt input is required and cannot be empty")
		}
		return fmt.Errorf("prompt is required and cannot be empty")
	}

	if config.APIKey == "" && config.SettingsJSON == "" {
		if config.UseEnvVars || isGitHubActions() {
			setFailed("api_key input is required and cannot be empty")
			return fmt.Errorf("api_key input is required and cannot be empty")
		}
		return fmt.Errorf("api-key is required when settings-json is not provided")
	}

	// Validate timeout range (1 second to 24 hours)
	if config.Timeout < 1 || config.Timeout > 86400 {
		if config.UseEnvVars || isGitHubActions() {
			setFailed(fmt.Sprintf("Timeout value %d is out of range. Timeout must be between 1 and 86400 seconds (24 hours).", config.Timeout))
		}
		return fmt.Errorf("timeout value %d is out of range. Timeout must be between 1 and 86400 seconds (24 hours)", config.Timeout)
	}

	return nil
}

// Helper functions for GitHub Actions
func getInput(name string) string {
	// GitHub Actions sets inputs as environment variables with INPUT_ prefix
	envName := "INPUT_" + strings.ToUpper(strings.ReplaceAll(name, "-", "_"))
	return os.Getenv(envName)
}

func setOutput(name, value string) {
	// GitHub Actions outputs can be set using the GITHUB_OUTPUT file
	if outputFile := os.Getenv("GITHUB_OUTPUT"); outputFile != "" {
		f, err := os.OpenFile(outputFile, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
		if err != nil {
			fmt.Printf("::error::Failed to open output file: %v\n", err)
			return
		}
		defer f.Close()

		// Use proper GitHub Actions output format with multiline support
		delimiter := fmt.Sprintf("EOF_%d", os.Getpid())
		_, err = f.WriteString(fmt.Sprintf("%s<<%s\n%s\n%s\n", name, delimiter, value, delimiter))
		if err != nil {
			fmt.Printf("::error::Failed to write output: %v\n", err)
		}
	} else {
		// Fallback to legacy format if GITHUB_OUTPUT is not available
		fmt.Printf("::set-output name=%s::%s\n", name, value)
	}
}

func info(message string) {
	if isGitHubActions() {
		fmt.Printf("::notice::%s\n", message)
	} else {
		fmt.Printf("INFO: %s\n", message)
	}
}

func setFailed(message string) {
	fmt.Printf("::error::%s\n", message)
	os.Exit(1)
}

func printIFlowVersion() {
	// Run iflow --version and print the output
	cmd := exec.Command("iflow", "--version")
	output, err := cmd.CombinedOutput()
	if err != nil {
		info(fmt.Sprintf("Warning: Failed to get iFlow version: %v", err))
		return
	}
	info(fmt.Sprintf("iFlow CLI version: %s", strings.TrimSpace(string(output))))
}

func printGhVersion() {
	// Run gh --version and print the output
	cmd := exec.Command("gh", "--version")
	output, err := cmd.CombinedOutput()
	if err != nil {
		info(fmt.Sprintf("Warning: Failed to get GitHub CLI version: %v", err))
		return
	}
	info(fmt.Sprintf("GitHub CLI version: %s", strings.TrimSpace(string(output))))
}

// installSpecificVersions installs specific versions of GitHub CLI and iFlow CLI if requested
func installSpecificVersions() error {
	// Install specific GitHub CLI version if requested
	if config.GhVersion != "" {
		info(fmt.Sprintf("Installing GitHub CLI version %s", config.GhVersion))
		// Download and install specific version of GitHub CLI
		installCmd := exec.Command("sh", "-c", fmt.Sprintf("curl -fsSL https://github.com/cli/cli/releases/download/v%s/gh_%s_linux_amd64.tar.gz | tar xz && cp gh_%s_linux_amd64/bin/gh /usr/local/bin/ && rm -rf gh_%s_linux_amd64", config.GhVersion, config.GhVersion, config.GhVersion, config.GhVersion))
		if err := installCmd.Run(); err != nil {
			return fmt.Errorf("failed to install GitHub CLI version %s: %w", config.GhVersion, err)
		}
		info(fmt.Sprintf("Successfully installed GitHub CLI version %s", config.GhVersion))
	}

	// Install specific iFlow CLI version if requested
	if config.IFlowVersion != "" {
		info(fmt.Sprintf("Installing iFlow CLI version %s", config.IFlowVersion))
		// Install specific version of iFlow CLI using npm
		installCmd := exec.Command("npm", "install", "-g", fmt.Sprintf("@iflow-ai/iflow-cli@%s", config.IFlowVersion))
		if err := installCmd.Run(); err != nil {
			return fmt.Errorf("failed to install iFlow CLI version %s: %w", config.IFlowVersion, err)
		}
		info(fmt.Sprintf("Successfully installed iFlow CLI version %s", config.IFlowVersion))
	}

	return nil
}

func configureIFlow() error {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return fmt.Errorf("failed to get home directory: %w", err)
	}

	iflowDir := filepath.Join(homeDir, ".iflow")
	if err := os.MkdirAll(iflowDir, 0755); err != nil {
		return fmt.Errorf("failed to create .iflow directory: %w", err)
	}

	settingsFile := filepath.Join(iflowDir, "settings.json")

	var settingsData []byte

	if config.SettingsJSON != "" {
		// Use provided settings JSON directly
		info("Using provided settings.json content")

		// Validate that it's valid JSON
		var testSettings map[string]interface{}
		if err := json.Unmarshal([]byte(config.SettingsJSON), &testSettings); err != nil {
			return fmt.Errorf("invalid settings_json provided: %w", err)
		}

		// Pretty format the JSON
		var prettyJSON json.RawMessage = []byte(config.SettingsJSON)
		settingsData, err = json.MarshalIndent(prettyJSON, "", "  ")
		if err != nil {
			return fmt.Errorf("failed to format settings JSON: %w", err)
		}
	} else {
		// Create settings from individual parameters
		info("Creating settings from individual parameters")
		settings := IFlowSettings{
			Theme:            "Default",
			SelectedAuthType: "iflow",
			APIKey:           config.APIKey,
			BaseURL:          config.BaseURL,
			ModelName:        config.Model,
			SearchAPIKey:     config.APIKey,
		}

		settingsData, err = json.MarshalIndent(settings, "", "  ")
		if err != nil {
			return fmt.Errorf("failed to marshal settings: %w", err)
		}
	}

	if err := os.WriteFile(settingsFile, settingsData, 0644); err != nil {
		return fmt.Errorf("failed to write settings file: %w", err)
	}

	info(fmt.Sprintf("iFlow settings configured at %s", settingsFile))
	return nil
}

func executePreCmd() error {
	// Split the precmd into lines and execute each line
	commands := strings.Split(config.PreCmd, "\n")

	for _, command := range commands {
		// Skip empty lines
		command = strings.TrimSpace(command)
		if command == "" {
			continue
		}

		info(fmt.Sprintf("Executing pre-command: %s", command))

		// Create a command to execute the pre-command
		cmd := exec.Command("sh", "-c", command)

		// Set the working directory for the command
		cmd.Dir = config.WorkingDir

		// Connect the command's stdin, stdout, and stderr to the current process
		cmd.Stdin = os.Stdin
		cmd.Stdout = os.Stdout
		cmd.Stderr = os.Stderr

		// Execute the command and wait for it to complete
		if err := cmd.Run(); err != nil {
			return fmt.Errorf("pre-command failed: %w", err)
		}
	}

	return nil
}

func executeIFlow() (string, int, error) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Duration(config.Timeout)*time.Second)
	defer cancel()

	// Prepare the command with --prompt and --yolo flags by default
	// Use --prompt and --yolo flags for all commands
	args := []string{"--yolo", "--prompt", config.Prompt}

	// Parse and add extra arguments if provided
	if config.ExtraArgs != "" {
		extraArgs := parseExtraArgs(config.ExtraArgs)
		args = append(args, extraArgs...)
		info(fmt.Sprintf("Using additional arguments: %v", extraArgs))
	}

	cmd := exec.CommandContext(ctx, "iflow", args...)

	// Create pipes for real-time output streaming
	stdoutPipe, err := cmd.StdoutPipe()
	if err != nil {
		return "", 1, fmt.Errorf("failed to create stdout pipe: %w", err)
	}

	stderrPipe, err := cmd.StderrPipe()
	if err != nil {
		return "", 1, fmt.Errorf("failed to create stderr pipe: %w", err)
	}

	// Buffer to capture all output for GitHub summary
	var outputBuffer strings.Builder

	// Create multi-writers to write to both console and buffer
	stdoutWriter := io.MultiWriter(os.Stdout, &outputBuffer)
	stderrWriter := io.MultiWriter(os.Stderr, &outputBuffer)

	// Start the command
	if err := cmd.Start(); err != nil {
		return "", 1, fmt.Errorf("failed to start command: %w", err)
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
		config.IsTimeout = true
		return outputBuffer.String(), 124, fmt.Errorf("command timed out after %d seconds", config.Timeout)
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
			return outputBuffer.String(), 1, err
		}
	}

	return outputBuffer.String(), exitCode, nil
}

// parseExtraArgs parses a space-separated string of arguments into a slice
// Handles quoted arguments with spaces properly
func parseExtraArgs(extraArgs string) []string {
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

func writeStepSummary(result string, exitCode int) error {
	summaryFile := os.Getenv("GITHUB_STEP_SUMMARY")
	if summaryFile == "" {
		// Not in GitHub Actions environment or summary not supported
		return nil
	}

	f, err := os.OpenFile(summaryFile, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		return fmt.Errorf("failed to open step summary file: %w", err)
	}
	defer f.Close()

	// Write summary in Markdown format
	summaryContent := generateSummaryMarkdown(result, exitCode)

	_, err = f.WriteString(summaryContent)
	if err != nil {
		return fmt.Errorf("failed to write to step summary: %w", err)
	}

	return nil
}

func generateSummaryMarkdown(result string, exitCode int) string {
	var summary strings.Builder

	// Add header with emoji based on status
	if config.IsTimeout {
		summary.WriteString("## ⏰ iFlow CLI Execution Summary - Timeout\n\n")
	} else if exitCode == 0 {
		summary.WriteString("## ✅ iFlow CLI Execution Summary\n\n")
	} else {
		summary.WriteString("## ❌ iFlow CLI Execution Summary\n\n")
	}

	// Add execution status with more detail
	summary.WriteString("### 📊 Status\n\n")
	if config.IsTimeout {
		summary.WriteString("⏰ **Execution**: Timed Out\n")
		summary.WriteString(fmt.Sprintf("🕒 **Timeout Duration**: %d seconds\n", config.Timeout))
		summary.WriteString(fmt.Sprintf("💥 **Exit Code**: %d\n\n", exitCode))
	} else if exitCode == 0 {
		summary.WriteString("🎉 **Execution**: Successful\n")
		summary.WriteString("🎯 **Exit Code**: 0\n\n")
	} else {
		summary.WriteString("⚠️ **Execution**: Failed\n")
		summary.WriteString(fmt.Sprintf("💥 **Exit Code**: %d\n\n", exitCode))
	}

	// Add configuration details in a table format
	summary.WriteString("### ⚙️ Configuration\n\n")
	summary.WriteString("| Setting | Value |\n")
	summary.WriteString("|---------|-------|\n")
	summary.WriteString(fmt.Sprintf("| Model | `%s` |\n", config.Model))
	summary.WriteString(fmt.Sprintf("| Base URL | `%s` |\n", config.BaseURL))
	summary.WriteString(fmt.Sprintf("| Timeout | %d seconds |\n", config.Timeout))
	summary.WriteString(fmt.Sprintf("| Working Directory | `%s` |\n", config.WorkingDir))
	if config.ExtraArgs != "" {
		summary.WriteString(fmt.Sprintf("| Extra Arguments | `%s` |\n", config.ExtraArgs))
	}
	summary.WriteString("\n")

	// Add prompt section
	summary.WriteString("### 📝 Input Prompt\n\n")
	prompt := config.Prompt
	if len(prompt) > 300 {
		prompt = prompt[:300] + "..."
	}
	// Escape any markdown characters in the prompt
	prompt = strings.ReplaceAll(prompt, "`", "\\`")
	summary.WriteString(fmt.Sprintf("> %s\n\n", prompt))

	// Add result section with better formatting
	summary.WriteString("### Output\n\n")
	if exitCode == 0 {
		displayResult := result
		if len(result) > 3000 {
			displayResult = result[:3000] + "\n\n... *(Output truncated. See full output in action logs)*"
		}

		// Check if result contains markdown or code blocks
		if strings.Contains(result, "```") {
			// Result already contains code blocks, display as-is
			summary.WriteString(fmt.Sprintf("%s\n\n", displayResult))
		} else if containsCode(result) {
			// Result looks like code, wrap in code block
			summary.WriteString(fmt.Sprintf("```\n%s\n```\n\n", displayResult))
		} else {
			// Regular text result, format as blockquote for readability
			lines := strings.Split(displayResult, "\n")
			for _, line := range lines {
				if strings.TrimSpace(line) != "" {
					summary.WriteString(fmt.Sprintf("> %s\n", line))
				} else {
					summary.WriteString(">\n")
				}
			}
			summary.WriteString("\n")
		}
	} else {
		// Error output, always in code block
		summary.WriteString("```\n")
		summary.WriteString(result)
		summary.WriteString("\n```\n\n")

		// Add troubleshooting hints for common errors
		if config.IsTimeout {
			summary.WriteString("#### ⏰ Timeout Information\n\n")
			summary.WriteString(fmt.Sprintf("- **Configured Timeout**: %d seconds\n", config.Timeout))
			summary.WriteString("- **Reason**: The iFlow CLI command did not complete within the specified timeout period\n")
			summary.WriteString("- **Exit Code**: 124 (timeout)\n\n")

			summary.WriteString("#### 🔧 Timeout Troubleshooting\n\n")
			summary.WriteString("- **Increase timeout**: Consider increasing the timeout value if the task legitimately needs more time\n")
			summary.WriteString("- **Optimize prompt**: Try breaking down complex prompts into smaller, more focused requests\n")
			summary.WriteString("- **Check model performance**: Some models may require longer processing time\n")
			summary.WriteString("- **Network issues**: Verify network connectivity and API response times\n")
			summary.WriteString("- **Resource constraints**: Check if the system has sufficient resources (CPU, memory)\n\n")
		} else if strings.Contains(result, "API Error") {
			summary.WriteString("#### 🔧 Troubleshooting Hints\n\n")
			summary.WriteString("- Check if your API key is valid and active\n")
			summary.WriteString("- Verify the base URL is accessible\n")
			summary.WriteString("- Ensure the selected model is available\n")
			summary.WriteString("- Try increasing the timeout value\n\n")
		}
	}

	// Add performance metrics if available
	summary.WriteString("### 📈 Metrics\n\n")
	summary.WriteString(fmt.Sprintf("- **Execution Time**: %s\n", time.Now().UTC().Format("2006-01-02 15:04:05 UTC")))
	summary.WriteString(fmt.Sprintf("- **Output Length**: %d characters\n", len(result)))
	if config.IsTimeout {
		summary.WriteString(fmt.Sprintf("- **Timeout Duration**: %d seconds\n", config.Timeout))
		summary.WriteString("- **Success Rate**: 0% (Timeout)\n\n")
	} else if exitCode == 0 {
		summary.WriteString("- **Success Rate**: 100%\n\n")
	} else {
		summary.WriteString("- **Success Rate**: 0%\n\n")
	}

	// Add footer
	summary.WriteString("---\n")
	summary.WriteString("*🤖 Generated by [iFlow CLI Action](https://github.com/iflow-ai/iflow-cli-action)*\n\n")

	return summary.String()
}

// Helper function to detect if text looks like code
func containsCode(text string) bool {
	codeIndicators := []string{
		"function", "class", "def ", "import ", "const ", "let ", "var ",
		"public ", "private ", "protected", "return ", "if (", "for (", "while (",
		"{", "}", ";", "//", "/*", "*/", "#include", "package ", "use ",
	}

	lowerText := strings.ToLower(text)
	for _, indicator := range codeIndicators {
		if strings.Contains(lowerText, indicator) {
			return true
		}
	}
	return false
}
