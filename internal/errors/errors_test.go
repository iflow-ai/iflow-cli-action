package errors

import (
	"errors"
	"testing"
)

func TestAppError_Error(t *testing.T) {
	tests := []struct {
		name     string
		err      *AppError
		expected string
	}{
		{
			name: "error with wrapped error",
			err: &AppError{
				Type:    ErrTypeValidation,
				Message: "validation failed",
				Err:     errors.New("invalid input"),
			},
			expected: "validation failed: invalid input",
		},
		{
			name: "error without wrapped error",
			err: &AppError{
				Type:    ErrTypeExecution,
				Message: "execution failed",
			},
			expected: "execution failed",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := tt.err.Error()
			if result != tt.expected {
				t.Errorf("Error() = %v, want %v", result, tt.expected)
			}
		})
	}
}

func TestAppError_Unwrap(t *testing.T) {
	wrappedErr := errors.New("wrapped error")
	err := &AppError{
		Type:    ErrTypeValidation,
		Message: "validation failed",
		Err:     wrappedErr,
	}

	unwrapped := err.Unwrap()
	if unwrapped != wrappedErr {
		t.Errorf("Unwrap() = %v, want %v", unwrapped, wrappedErr)
	}
}

func TestNewValidationError(t *testing.T) {
	wrappedErr := errors.New("invalid input")
	err := NewValidationError("validation failed", wrappedErr)

	if err.Type != ErrTypeValidation {
		t.Errorf("Type = %v, want %v", err.Type, ErrTypeValidation)
	}

	if err.Message != "validation failed" {
		t.Errorf("Message = %v, want %v", err.Message, "validation failed")
	}

	if err.Err != wrappedErr {
		t.Errorf("Err = %v, want %v", err.Err, wrappedErr)
	}
}

func TestNewExecutionError(t *testing.T) {
	wrappedErr := errors.New("command failed")
	context := map[string]interface{}{
		"command": "test",
		"exitCode": 1,
	}
	
	err := NewExecutionError("execution failed", wrappedErr, context)

	if err.Type != ErrTypeExecution {
		t.Errorf("Type = %v, want %v", err.Type, ErrTypeExecution)
	}

	if err.Message != "execution failed" {
		t.Errorf("Message = %v, want %v", err.Message, "execution failed")
	}

	if err.Err != wrappedErr {
		t.Errorf("Err = %v, want %v", err.Err, wrappedErr)
	}

	if len(err.Context) != 2 {
		t.Errorf("Context length = %v, want %v", len(err.Context), 2)
	}

	if err.Context["command"] != "test" {
		t.Errorf("Context[command] = %v, want %v", err.Context["command"], "test")
	}
}

func TestNewTimeoutError(t *testing.T) {
	err := NewTimeoutError("operation timed out", 30)

	if err.Type != ErrTypeTimeout {
		t.Errorf("Type = %v, want %v", err.Type, ErrTypeTimeout)
	}

	if err.Message != "operation timed out" {
		t.Errorf("Message = %v, want %v", err.Message, "operation timed out")
	}

	if err.Err != nil {
		t.Errorf("Err = %v, want %v", err.Err, nil)
	}

	if err.Context["timeout"] != 30 {
		t.Errorf("Context[timeout] = %v, want %v", err.Context["timeout"], 30)
	}
}

func TestNewConfigurationError(t *testing.T) {
	wrappedErr := errors.New("config file not found")
	err := NewConfigurationError("configuration failed", wrappedErr)

	if err.Type != ErrTypeConfiguration {
		t.Errorf("Type = %v, want %v", err.Type, ErrTypeConfiguration)
	}

	if err.Message != "configuration failed" {
		t.Errorf("Message = %v, want %v", err.Message, "configuration failed")
	}

	if err.Err != wrappedErr {
		t.Errorf("Err = %v, want %v", err.Err, wrappedErr)
	}
}