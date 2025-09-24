package iflow

import (
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"strings"

	"github.com/iflow-ai/iflow-cli-action/internal/errors"
)

// PrintRecentSessionFile prints the most recently modified session .jsonl file
func (c *Client) PrintRecentSessionFile() error {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return errors.NewConfigurationError("failed to get home directory", err)
	}

	// direct to search projects directory
	projectsDir := filepath.Join(homeDir, ".iflow", "projects")

	if _, err := os.Stat(projectsDir); os.IsNotExist(err) {
		return errors.NewConfigurationError("projects directory not found", err)
	}

	// search all .jsonl files in projects directory and its subdirectories
	var jsonlFiles []struct {
		path string
		info os.FileInfo
	}
	err = filepath.Walk(projectsDir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return nil
		}
		if !info.IsDir() && strings.HasSuffix(info.Name(), ".jsonl") {
			jsonlFiles = append(jsonlFiles, struct {
				path string
				info os.FileInfo
			}{
				path: path,
				info: info,
			})
		}
		return nil
	})

	if err != nil {
		return errors.NewConfigurationError("failed to walk projects directory", err)
	}

	if len(jsonlFiles) == 0 {
		fmt.Println("No session jsonl files found in projects directory")
		return nil
	}

	// sort files by modification time, latest first
	sort.Slice(jsonlFiles, func(i, j int) bool {
		return jsonlFiles[i].info.ModTime().After(jsonlFiles[j].info.ModTime())
	})

	// get latest jsonl file
	latestFile := jsonlFiles[0]
	filePath := latestFile.path

	fmt.Printf("\n=== Latest Session File: %s (Modified: %s) ===\n\n",
		filepath.Base(filePath), latestFile.info.ModTime().Format("2006-01-02 15:04:05"))

	file, err := os.Open(filePath)
	if err != nil {
		return errors.NewExecutionError("failed to open session file", err, nil)
	}
	defer file.Close()

	lineCount := 0
	scanner := bufio.NewScanner(file)
	// Increase buffer size to handle long lines
	buf := make([]byte, 0, 64*1024) // 64KB initial buffer size
	scanner.Buffer(buf, 1024*1024)  // max buffer size
	for scanner.Scan() {
		line := scanner.Text()
		if strings.TrimSpace(line) != "" {
			fmt.Println(line)
			lineCount++
		}
	}

	if err := scanner.Err(); err != nil {
		return errors.NewExecutionError("failed to read session file", err, nil)
	}

	fmt.Printf("\n=== End of Session File (%d lines) ===\n", lineCount)
	return nil
}
