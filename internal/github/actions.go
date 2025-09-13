package github

import (
	"fmt"
	"os"
	"strings"
)

// Actions interface defines GitHub Actions operations
type Actions interface {
	SetOutput(name, value string)
	SetFailed(message string)
	WriteStepSummary(content string)
	Info(message string)
	IsGitHubActions() bool
}

// GitHubActions implements the Actions interface
type GitHubActions struct{}

// NewGitHubActions creates a new GitHubActions instance
func NewGitHubActions() *GitHubActions {
	return &GitHubActions{}
}

// SetOutput sets a GitHub Actions output
func (g *GitHubActions) SetOutput(name, value string) {
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

// SetFailed sets the action as failed and exits
func (g *GitHubActions) SetFailed(message string) {
	fmt.Printf("::error::%s\n", message)
	os.Exit(1)
}

// WriteStepSummary writes content to the GitHub Actions step summary
func (g *GitHubActions) WriteStepSummary(content string) {
	summaryFile := os.Getenv("GITHUB_STEP_SUMMARY")
	if summaryFile == "" {
		// Not in GitHub Actions environment or summary not supported
		return
	}

	f, err := os.OpenFile(summaryFile, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		// Log error but don't fail
		fmt.Printf("::error::Failed to open step summary file: %v\n", err)
		return
	}
	defer f.Close()

	_, err = f.WriteString(content)
	if err != nil {
		// Log error but don't fail
		fmt.Printf("::error::Failed to write to step summary: %v\n", err)
	}
}

// Info logs an informational message
func (g *GitHubActions) Info(message string) {
	if g.IsGitHubActions() {
		fmt.Printf("::notice::%s\n", message)
	} else {
		fmt.Printf("INFO: %s\n", message)
	}
}

// IsGitHubActions detects if running in GitHub Actions environment
func (g *GitHubActions) IsGitHubActions() bool {
	return os.Getenv("GITHUB_ACTIONS") == "true"
}

// GenerateSummaryMarkdown generates a comprehensive summary markdown
func GenerateSummaryMarkdown(result string, exitCode int, config map[string]interface{}) string {
	var summary strings.Builder

	isTimeout := false
	if timeoutVal, ok := config["isTimeout"].(bool); ok {
		isTimeout = timeoutVal
	}

	timeoutVal := 3600
	if t, ok := config["timeout"].(int); ok {
		timeoutVal = t
	}

	modelVal := "Qwen3-Coder"
	if m, ok := config["model"].(string); ok {
		modelVal = m
	}

	baseURLVal := "https://apis.iflow.cn/v1"
	if b, ok := config["baseURL"].(string); ok {
		baseURLVal = b
	}

	workingDirVal := "."
	if w, ok := config["workingDir"].(string); ok {
		workingDirVal = w
	}

	extraArgsVal := ""
	if e, ok := config["extraArgs"].(string); ok {
		extraArgsVal = e
	}

	promptVal := ""
	if p, ok := config["prompt"].(string); ok {
		promptVal = p
	}

	// Add header with emoji based on status
	if isTimeout {
		summary.WriteString("## ⏰ iFlow CLI Execution Summary - Timeout\n\n")
	} else if exitCode == 0 {
		summary.WriteString("## ✅ iFlow CLI Execution Summary\n\n")
	} else {
		summary.WriteString("## ❌ iFlow CLI Execution Summary\n\n")
	}

	// Add execution status with more detail
	summary.WriteString("### 📊 Status\n\n")
	if isTimeout {
		summary.WriteString("⏰ **Execution**: Timed Out\n")
		summary.WriteString(fmt.Sprintf("🕒 **Timeout Duration**: %d seconds\n", timeoutVal))
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
	summary.WriteString(fmt.Sprintf("| Model | `%s` |\n", modelVal))
	summary.WriteString(fmt.Sprintf("| Base URL | `%s` |\n", baseURLVal))
	summary.WriteString(fmt.Sprintf("| Timeout | %d seconds |\n", timeoutVal))
	summary.WriteString(fmt.Sprintf("| Working Directory | `%s` |\n", workingDirVal))
	if extraArgsVal != "" {
		summary.WriteString(fmt.Sprintf("| Extra Arguments | `%s` |\n", extraArgsVal))
	}
	summary.WriteString("\n")

	// Add prompt section
	summary.WriteString("### 📝 Input Prompt\n\n")
	prompt := promptVal
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
		if isTimeout {
			summary.WriteString("#### ⏰ Timeout Information\n\n")
			summary.WriteString(fmt.Sprintf("- **Configured Timeout**: %d seconds\n", timeoutVal))
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

	// Add footer
	summary.WriteString("---\n")
	summary.WriteString("*🤖 Generated by [iFlow CLI Action](https://github.com/iflow-ai/iflow-cli-action)*\n\n")

	return summary.String()
}

// containsCode detects if text looks like code
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