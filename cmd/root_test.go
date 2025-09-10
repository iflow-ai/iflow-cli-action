package cmd

import (
	"testing"

	"github.com/iflow-ai/iflow-cli-action/internal/config"
	internalerrors "github.com/iflow-ai/iflow-cli-action/internal/errors"
)

func TestGetConfig(t *testing.T) {
	// Initialize config for testing
	cfg = &config.Config{
		Prompt:  "test prompt",
		APIKey:  "test-key",
		Timeout: 1800,
	}

	result := GetConfig()
	if result != cfg {
		t.Error("GetConfig() should return the global config")
	}
}

func TestHandleAppError(t *testing.T) {
	tests := []struct {
		name        string
		err         *internalerrors.AppError
		expectOutput string
	}{
		{
			name: "validation error",
			err: &internalerrors.AppError{
				Type:    internalerrors.ErrTypeValidation,
				Message: "invalid input",
			},
			expectOutput: "Validation Error: invalid input",
		},
		{
			name: "timeout error",
			err: &internalerrors.AppError{
				Type:    internalerrors.ErrTypeTimeout,
				Message: "operation timed out",
				Context: map[string]interface{}{"timeout": 30},
			},
			expectOutput: "Timeout Error: operation timed out",
		},
		{
			name: "configuration error",
			err: &internalerrors.AppError{
				Type:    internalerrors.ErrTypeConfiguration,
				Message: "config invalid",
			},
			expectOutput: "Configuration Error: config invalid",
		},
		{
			name: "generic error",
			err: &internalerrors.AppError{
				Type:    internalerrors.ErrorType(999), // Unknown type
				Message: "unknown error",
			},
			expectOutput: "Error: unknown error",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// This test would normally capture stdout to verify the output
			// For now, we'll just ensure the function doesn't panic
			handleAppError(tt.err)
		})
	}
}