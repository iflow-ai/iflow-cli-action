package config

import (
	"os"
	"testing"
)

func TestNewConfig(t *testing.T) {
	cfg := NewConfig()
	
	if cfg.BaseURL != "https://apis.iflow.cn/v1" {
		t.Errorf("Expected default BaseURL to be https://apis.iflow.cn/v1, got %s", cfg.BaseURL)
	}
	
	if cfg.Model != "Qwen3-Coder" {
		t.Errorf("Expected default Model to be Qwen3-Coder, got %s", cfg.Model)
	}
	
	if cfg.WorkingDir != "." {
		t.Errorf("Expected default WorkingDir to be ., got %s", cfg.WorkingDir)
	}
	
	if cfg.Timeout != 3600 {
		t.Errorf("Expected default Timeout to be 3600, got %d", cfg.Timeout)
	}
}

func TestConfig_Validate(t *testing.T) {
	tests := []struct {
		name    string
		config  *Config
		wantErr bool
	}{
		{
			name: "valid config with API key",
			config: &Config{
				Prompt:  "test prompt",
				APIKey:  "test-key",
				Timeout: 3600,
			},
			wantErr: false,
		},
		{
			name: "valid config with settings JSON",
			config: &Config{
				Prompt:       "test prompt",
				SettingsJSON: `{"theme":"Default"}`,
				Timeout:      3600,
			},
			wantErr: false,
		},
		{
			name: "missing prompt",
			config: &Config{
				APIKey:  "test-key",
				Timeout: 3600,
			},
			wantErr: true,
		},
		{
			name: "missing API key and settings JSON",
			config: &Config{
				Prompt:  "test prompt",
				Timeout: 3600,
			},
			wantErr: true,
		},
		{
			name: "invalid timeout - too low",
			config: &Config{
				Prompt:  "test prompt",
				APIKey:  "test-key",
				Timeout: 0,
			},
			wantErr: true,
		},
		{
			name: "invalid timeout - too high",
			config: &Config{
				Prompt:  "test prompt",
				APIKey:  "test-key",
				Timeout: 90000,
			},
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := tt.config.Validate()
			if (err != nil) != tt.wantErr {
				t.Errorf("Validate() error = %v, wantErr %v", err, tt.wantErr)
			}
		})
	}
}

func TestConfig_LoadFromEnv(t *testing.T) {
	// Save original environment
	originalPrompt := os.Getenv("INPUT_PROMPT")
	originalAPIKey := os.Getenv("INPUT_API_KEY")
	originalTimeout := os.Getenv("INPUT_TIMEOUT")
	
	// Restore original environment after test
	defer func() {
		os.Setenv("INPUT_PROMPT", originalPrompt)
		os.Setenv("INPUT_API_KEY", originalAPIKey)
		os.Setenv("INPUT_TIMEOUT", originalTimeout)
	}()

	tests := []struct {
		name     string
		envVars  map[string]string
		expected Config
	}{
		{
			name: "load all fields",
			envVars: map[string]string{
				"INPUT_PROMPT":  "test prompt",
				"INPUT_API_KEY": "test-key",
				"INPUT_TIMEOUT": "1800",
				"INPUT_MODEL":   "test-model",
			},
			expected: Config{
				Prompt:  "test prompt",
				APIKey:  "test-key",
				Timeout: 1800,
				Model:   "test-model",
				BaseURL: "https://apis.iflow.cn/v1", // default value
				WorkingDir: ".", // default value
			},
		},
		{
			name: "load with dashes in names",
			envVars: map[string]string{
				"INPUT_WORKING_DIRECTORY": "/tmp",
				"INPUT_BASE_URL":          "https://custom.api.com",
			},
			expected: Config{
				BaseURL:    "https://custom.api.com",
				WorkingDir: "/tmp",
				Model:      "Qwen3-Coder", // default value
				Timeout:    3600,          // default value
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Clear environment first
			os.Clearenv()
			
			// Set test environment variables
			for key, value := range tt.envVars {
				os.Setenv(key, value)
			}
			
			cfg := NewConfig()
			err := cfg.LoadFromEnv()
			if err != nil {
				t.Fatalf("LoadFromEnv() error = %v", err)
			}
			
			// Compare relevant fields
			if cfg.Prompt != tt.expected.Prompt {
				t.Errorf("Prompt = %v, want %v", cfg.Prompt, tt.expected.Prompt)
			}
			if cfg.APIKey != tt.expected.APIKey {
				t.Errorf("APIKey = %v, want %v", cfg.APIKey, tt.expected.APIKey)
			}
			if cfg.Timeout != tt.expected.Timeout {
				t.Errorf("Timeout = %v, want %v", cfg.Timeout, tt.expected.Timeout)
			}
			if cfg.Model != tt.expected.Model {
				t.Errorf("Model = %v, want %v", cfg.Model, tt.expected.Model)
			}
			if cfg.BaseURL != tt.expected.BaseURL {
				t.Errorf("BaseURL = %v, want %v", cfg.BaseURL, tt.expected.BaseURL)
			}
			if cfg.WorkingDir != tt.expected.WorkingDir {
				t.Errorf("WorkingDir = %v, want %v", cfg.WorkingDir, tt.expected.WorkingDir)
			}
		})
	}
}

func TestIsGitHubActions(t *testing.T) {
	// Save original environment
	originalValue := os.Getenv("GITHUB_ACTIONS")
	defer os.Setenv("GITHUB_ACTIONS", originalValue)

	tests := []struct {
		name     string
		envValue string
		expected bool
	}{
		{
			name:     "GitHub Actions environment",
			envValue: "true",
			expected: true,
		},
		{
			name:     "Not GitHub Actions environment",
			envValue: "",
			expected: false,
		},
		{
			name:     "Different value",
			envValue: "false",
			expected: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			os.Setenv("GITHUB_ACTIONS", tt.envValue)
			result := IsGitHubActions()
			if result != tt.expected {
				t.Errorf("IsGitHubActions() = %v, want %v", result, tt.expected)
			}
		})
	}
}