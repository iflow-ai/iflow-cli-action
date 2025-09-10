package errors

import (
	"fmt"
)

// ErrorType represents the type of error
type ErrorType int

const (
	ErrTypeValidation ErrorType = iota
	ErrTypeExecution
	ErrTypeTimeout
	ErrTypeConfiguration
)

// AppError represents a structured application error
type AppError struct {
	Type    ErrorType
	Message string
	Err     error
	Context map[string]interface{}
}

// Error implements the error interface
func (e *AppError) Error() string {
	if e.Err != nil {
		return fmt.Sprintf("%s: %v", e.Message, e.Err)
	}
	return e.Message
}

// Unwrap implements the error unwrapping interface
func (e *AppError) Unwrap() error {
	return e.Err
}

// Helper functions for creating specific error types

func NewValidationError(message string, err error) *AppError {
	return &AppError{
		Type:    ErrTypeValidation,
		Message: message,
		Err:     err,
	}
}

func NewExecutionError(message string, err error, context map[string]interface{}) *AppError {
	return &AppError{
		Type:    ErrTypeExecution,
		Message: message,
		Err:     err,
		Context: context,
	}
}

func NewTimeoutError(message string, timeout int) *AppError {
	return &AppError{
		Type:    ErrTypeTimeout,
		Message: message,
		Context: map[string]interface{}{
			"timeout": timeout,
		},
	}
}

func NewConfigurationError(message string, err error) *AppError {
	return &AppError{
		Type:    ErrTypeConfiguration,
		Message: message,
		Err:     err,
	}
}