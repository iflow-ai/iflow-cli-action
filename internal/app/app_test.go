package app

import (
	"testing"

	"github.com/iflow-ai/iflow-cli-action/internal/config"
	"github.com/iflow-ai/iflow-cli-action/internal/github"
)

func TestNewApp(t *testing.T) {
	cfg := &config.Config{
		Prompt:  "test prompt",
		APIKey:  "test-key",
		Timeout: 3600,
	}

	mockActions := github.NewMockActions()
	app := NewApp(cfg, mockActions)

	if app == nil {
		t.Fatal("NewApp() returned nil")
	}

	if app.config != cfg {
		t.Error("App config does not match input config")
	}

	if app.ghActions != mockActions {
		t.Error("App ghActions does not match input ghActions")
	}

	if app.iflowClient == nil {
		t.Error("App iflowClient is nil")
	}
}

func TestApp_setupWorkingDirectory(t *testing.T) {
	tests := []struct {
		name        string
		workingDir  string
		expectError bool
	}{
		{
			name:        "current directory",
			workingDir:  ".",
			expectError: false,
		},
		{
			name:        "empty working directory",
			workingDir:  "",
			expectError: false,
		},
		{
			name:        "invalid directory",
			workingDir:  "/nonexistent/directory",
			expectError: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			cfg := &config.Config{
				WorkingDir: tt.workingDir,
			}

			mockActions := github.NewMockActions()
			app := NewApp(cfg, mockActions)

			err := app.setupWorkingDirectory()

			if tt.expectError && err == nil {
				t.Error("Expected error but got none")
			} else if !tt.expectError && err != nil {
				t.Errorf("Unexpected error: %v", err)
			}
		})
	}
}

func TestApp_getCommandVersion(t *testing.T) {
	app := &App{}

	tests := []struct {
		name      string
		command   string
		wantError bool
	}{
		{
			name:      "existing command",
			command:   "echo",
			wantError: false,
		},
		{
			name:      "non-existing command",
			command:   "nonexistentcommand12345",
			wantError: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			version, err := app.getCommandVersion(tt.command)

			if tt.wantError && err == nil {
				t.Error("Expected error but got none")
			} else if !tt.wantError && err != nil {
				t.Errorf("Unexpected error: %v", err)
			}

			if !tt.wantError && version == "" {
				t.Error("Expected version to be non-empty")
			}
		})
	}
}

func TestApp_handleOutputs(t *testing.T) {
	tests := []struct {
		name           string
		config         *config.Config
		result         string
		exitCode       int
		expectFailed   bool
		expectedOutput map[string]string
	}{
		{
			name: "successful execution in GitHub Actions mode",
			config: &config.Config{
				UseEnvVars: true,
			},
			result:       "test output",
			exitCode:     0,
			expectFailed: false,
			expectedOutput: map[string]string{
				"result":    "test output",
				"exit_code": "0",
			},
		},
		{
			name: "failed execution in GitHub Actions mode",
			config: &config.Config{
				UseEnvVars: true,
			},
			result:       "error output",
			exitCode:     1,
			expectFailed: true,
			expectedOutput: map[string]string{
				"result":    "error output",
				"exit_code": "1",
			},
		},
		{
			name: "successful execution in CLI mode",
			config: &config.Config{
				UseEnvVars: false,
			},
			result:       "test output",
			exitCode:     0,
			expectFailed: false,
			expectedOutput: map[string]string{},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			mockActions := github.NewMockActions()
			mockActions.IsGHActions = tt.config.UseEnvVars || config.IsGitHubActions()

			app := NewApp(tt.config, mockActions)

			err := app.handleOutputs(tt.result, tt.exitCode)

			if err != nil {
				t.Fatalf("handleOutputs() error = %v", err)
			}

			// Check outputs
			for key, expectedValue := range tt.expectedOutput {
				mockActions.AssertOutput(t, key, expectedValue)
			}

			// Check if failed
			if tt.expectFailed {
				mockActions.AssertFailed(t, "iFlow CLI exited with code 1")
			} else {
				mockActions.AssertNotFailed(t)
			}
		})
	}
}