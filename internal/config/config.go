package config

import (
	"os"
	"strconv"
	"strings"

	"github.com/iflow-ai/iflow-cli-action/internal/errors"
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

// NewConfig creates a new Config with default values
func NewConfig() *Config {
	return &Config{
		BaseURL:    "https://apis.iflow.cn/v1",
		Model:      "Qwen3-Coder",
		WorkingDir: ".",
		Timeout:    3600,
	}
}

// LoadFromEnv loads configuration from environment variables (GitHub Actions convention)
func (c *Config) LoadFromEnv() error {
	// Load configuration from environment variables (GitHub Actions convention)
	if prompt := getInput("prompt"); prompt != "" {
		c.Prompt = strings.TrimSpace(prompt)
	}
	if apiKey := getInput("api_key"); apiKey != "" {
		c.APIKey = apiKey
	}
	if settingsJSON := getInput("settings_json"); settingsJSON != "" {
		c.SettingsJSON = settingsJSON
	}
	if baseURL := getInput("base_url"); baseURL != "" {
		c.BaseURL = baseURL
	}
	if model := getInput("model"); model != "" {
		c.Model = model
	}
	if workingDir := getInput("working_directory"); workingDir != "" {
		c.WorkingDir = workingDir
	}
	if timeoutStr := getInput("timeout"); timeoutStr != "" {
		timeout, err := strconv.Atoi(timeoutStr)
		if err != nil {
			return errors.NewValidationError("invalid timeout value", err)
		}
		c.Timeout = timeout
	}

	if extraArgs := getInput("extra_args"); extraArgs != "" {
		c.ExtraArgs = strings.TrimSpace(extraArgs)
	}

	if preCmd := getInput("precmd"); preCmd != "" {
		c.PreCmd = strings.TrimSpace(preCmd)
	}

	if ghVersion := getInput("gh_version"); ghVersion != "" {
		c.GhVersion = strings.TrimSpace(ghVersion)
	}

	if iflowVersion := getInput("iflow_version"); iflowVersion != "" {
		c.IFlowVersion = strings.TrimSpace(iflowVersion)
	}

	return nil
}

// Validate checks if the configuration is valid
func (c *Config) Validate() error {
	// Validate required inputs
	if c.Prompt == "" {
		return errors.NewValidationError("prompt input is required and cannot be empty", nil)
	}

	if c.APIKey == "" && c.SettingsJSON == "" {
		return errors.NewValidationError("api_key input is required and cannot be empty", nil)
	}

	// Validate timeout range (1 second to 24 hours)
	if c.Timeout < 1 || c.Timeout > 86400 {
		return errors.NewValidationError("timeout value is out of range. Must be between 1 and 86400 seconds", nil)
	}

	return nil
}

// IsGitHubActions detects if running in GitHub Actions environment
func IsGitHubActions() bool {
	return os.Getenv("GITHUB_ACTIONS") == "true"
}

// getInput gets input from environment variables (GitHub Actions convention)
func getInput(name string) string {
	// GitHub Actions sets inputs as environment variables with INPUT_ prefix
	envName := "INPUT_" + strings.ToUpper(strings.ReplaceAll(name, "-", "_"))
	return os.Getenv(envName)
}