package github

import (
	"testing"
)

// MockActions is a mock implementation of the Actions interface for testing
type MockActions struct {
	Outputs         map[string]string
	Failed          bool
	FailureMessage  string
	SummaryContent  string
	InfoMessages    []string
	IsGHActions     bool
}

// NewMockActions creates a new mock Actions instance
func NewMockActions() *MockActions {
	return &MockActions{
		Outputs:      make(map[string]string),
		InfoMessages: []string{},
	}
}

// SetOutput sets a mock output
func (m *MockActions) SetOutput(name, value string) {
	m.Outputs[name] = value
}

// SetFailed sets the mock as failed
func (m *MockActions) SetFailed(message string) {
	m.Failed = true
	m.FailureMessage = message
}

// WriteStepSummary stores the summary content
func (m *MockActions) WriteStepSummary(content string) {
	m.SummaryContent = content
}

// Info stores info messages
func (m *MockActions) Info(message string) {
	m.InfoMessages = append(m.InfoMessages, message)
}

// IsGitHubActions returns the mock value
func (m *MockActions) IsGitHubActions() bool {
	return m.IsGHActions
}

// Reset clears all mock state
func (m *MockActions) Reset() {
	m.Outputs = make(map[string]string)
	m.Failed = false
	m.FailureMessage = ""
	m.SummaryContent = ""
	m.InfoMessages = []string{}
	m.IsGHActions = false
}

// AssertOutput checks if an output was set correctly
func (m *MockActions) AssertOutput(t *testing.T, name, expectedValue string) {
	t.Helper()
	if value, exists := m.Outputs[name]; !exists {
		t.Errorf("Output %s was not set", name)
	} else if value != expectedValue {
		t.Errorf("Output %s = %v, want %v", name, value, expectedValue)
	}
}

// AssertFailed checks if the action was set as failed
func (m *MockActions) AssertFailed(t *testing.T, expectedMessage string) {
	t.Helper()
	if !m.Failed {
		t.Error("Expected action to be failed, but it was not")
	} else if m.FailureMessage != expectedMessage {
		t.Errorf("Failure message = %v, want %v", m.FailureMessage, expectedMessage)
	}
}

// AssertNotFailed checks if the action was not set as failed
func (m *MockActions) AssertNotFailed(t *testing.T) {
	t.Helper()
	if m.Failed {
		t.Error("Expected action not to be failed, but it was")
	}
}

// AssertInfoMessage checks if an info message was logged
func (m *MockActions) AssertInfoMessage(t *testing.T, expectedMessage string) {
	t.Helper()
	found := false
	for _, msg := range m.InfoMessages {
		if msg == expectedMessage {
			found = true
			break
		}
	}
	if !found {
		t.Errorf("Info message %q was not found in logged messages: %v", expectedMessage, m.InfoMessages)
	}
}