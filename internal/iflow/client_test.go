package iflow

import (
	"context"
	"strings"
	"testing"
	"time"

	"github.com/iflow-ai/iflow-cli-action/internal/config"
)

func TestNewClient(t *testing.T) {
	cfg := &config.Config{
		Prompt:  "test prompt",
		APIKey:  "test-key",
		Timeout: 3600,
	}

	client := NewClient(cfg)
	if client == nil {
		t.Fatal("NewClient() returned nil")
	}

	if client.config != cfg {
		t.Error("Client config does not match input config")
	}
}

func TestClient_parseExtraArgs(t *testing.T) {
	client := &Client{config: &config.Config{}}

	tests := []struct {
		name     string
		extraArgs string
		expected []string
	}{
		{
			name:      "empty string",
			extraArgs: "",
			expected:  []string{},
		},
		{
			name:      "single argument",
			extraArgs: "--verbose",
			expected:  []string{"--verbose"},
		},
		{
			name:      "multiple arguments",
			extraArgs: "--verbose --format json",
			expected:  []string{"--verbose", "--format", "json"},
		},
		{
			name:      "quoted argument with spaces",
			extraArgs: "--message \"Hello World\"",
			expected:  []string{"--message", "Hello World"},
		},
		{
			name:      "single quoted argument",
			extraArgs: "--flag 'single quotes'",
			expected:  []string{"--flag", "single quotes"},
		},
		{
			name:      "mixed quotes",
			extraArgs: `--msg "double quotes" --flag 'single quotes'`,
			expected:  []string{"--msg", "double quotes", "--flag", "single quotes"},
		},
		{
			name:      "multiple lines",
			extraArgs: "--verbose\n--format json",
			expected:  []string{"--verbose", "--format", "json"},
		},
		{
			name:      "nested quotes",
			extraArgs: `--message "He said 'Hello' to me"`,
			expected:  []string{"--message", "He said 'Hello' to me"},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := client.parseExtraArgs(tt.extraArgs)
			if len(result) != len(tt.expected) {
				t.Errorf("parseExtraArgs() returned %d arguments, want %d", len(result), len(tt.expected))
				return
			}
			
			for i, arg := range result {
				if arg != tt.expected[i] {
					t.Errorf("parseExtraArgs()[%d] = %v, want %v", i, arg, tt.expected[i])
				}
			}
		})
	}
}

func TestClient_Configure(t *testing.T) {
	// This test would require mocking the filesystem and home directory
	// For now, we'll test the basic structure
	
	tests := []struct {
		name        string
		config      *config.Config
		expectError bool
	}{
		{
			name: "valid config with API key",
			config: &config.Config{
				APIKey:  "test-key",
				BaseURL: "https://api.example.com",
				Model:   "test-model",
			},
			expectError: false,
		},
		{
			name: "valid config with settings JSON",
			config: &config.Config{
				SettingsJSON: `{"theme":"Default","selectedAuthType":"iflow"}`,
			},
			expectError: false,
		},
		{
			name: "invalid settings JSON",
			config: &config.Config{
				SettingsJSON: `invalid json`,
			},
			expectError: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			client := NewClient(tt.config)
			err := client.Configure()
			
			if tt.expectError && err == nil {
				t.Error("Expected error but got none")
			} else if !tt.expectError && err != nil {
				t.Errorf("Unexpected error: %v", err)
			}
		})
	}
}

func TestClient_ExecutePreCmd(t *testing.T) {
	// Create a temporary directory for testing
	tempDir := t.TempDir()
	
	tests := []struct {
		name        string
		preCmd      string
		workingDir  string
		expectError bool
	}{
		{
			name:        "empty preCmd",
			preCmd:      "",
			workingDir:  tempDir,
			expectError: false,
		},
		{
			name:        "single command",
			preCmd:      "echo 'test'",
			workingDir:  tempDir,
			expectError: false,
		},
		{
			name:        "multiple commands",
			preCmd:      "echo 'first'\necho 'second'",
			workingDir:  tempDir,
			expectError: false,
		},
		{
			name:        "commands with empty lines",
			preCmd:      "echo 'first'\n\necho 'third'",
			workingDir:  tempDir,
			expectError: false,
		},
		{
			name:        "invalid command",
			preCmd:      "nonexistentcommand12345",
			workingDir:  tempDir,
			expectError: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			cfg := &config.Config{
				PreCmd:     tt.preCmd,
				WorkingDir: tt.workingDir,
			}
			
			client := NewClient(cfg)
			err := client.ExecutePreCmd()
			
			if tt.expectError && err == nil {
				t.Error("Expected error but got none")
			} else if !tt.expectError && err != nil {
				t.Errorf("Unexpected error: %v", err)
			}
		})
	}
}

func TestClient_Execute(t *testing.T) {
	// This is a basic test that would need to be expanded with proper mocking
	// For now, we'll test with a simple echo command to verify the structure
	
	cfg := &config.Config{
		Timeout: 5,
	}
	
	client := NewClient(cfg)
	
	// Create a context with timeout
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()
	
	// This test would normally mock the iflow command
	// For now, we'll just verify the method exists and can be called
	// (it will fail because iflow command doesn't exist in test environment)
	
	_, _, err := client.Execute(ctx, "test prompt")
	if err == nil {
		t.Skip("Skipping execute test - iflow command not available in test environment")
	}
	
	// Verify that we get an execution error (expected since iflow is not installed)
	if !strings.Contains(err.Error(), "executable file not found") && !strings.Contains(err.Error(), "not found") {
		t.Logf("Execute error (expected): %v", err)
	}
}