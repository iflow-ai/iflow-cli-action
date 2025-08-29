# Example Workflows

<!-- TOC start -->
## Table of Contents

- [Basic Examples](#basic-examples)
  - [Code Review on Pull Request](#code-review-on-pull-request)
  - [Documentation Generation](#documentation-generation)
  - [Using Extra Arguments](#using-extra-arguments)
  - [Security Analysis](#security-analysis)
- [Advanced Examples](#advanced-examples)
  - [Multi-step Analysis](#multi-step-analysis)
- [Configuration Examples](#configuration-examples)
  - [Custom Model Configuration](#custom-model-configuration)
  - [Extended Timeout for Complex Tasks](#extended-timeout-for-complex-tasks)
  - [Different Working Directory](#different-working-directory)
- [Additional Workflows](#additional-workflows)
  - [Issue Killer](#issue-killer)
  - [Issue Triage](#issue-triage)
  - [PR Review Killer](#pr-review-killer)
- [Execution Mechanism](#execution-mechanism)
  - [Issue Killer Execution](#issue-killer-execution)
  - [Issue Triage Execution](#issue-triage-execution)
  - [PR Review Execution](#pr-review-execution)
  - [PR Review Killer Execution](#pr-review-killer-execution)
- [Setup Instructions](#setup-instructions)
<!-- TOC end -->


This directory contains example GitHub Actions workflows demonstrating how to use the iFlow CLI Action.

**Note:** All iFlow CLI commands are automatically executed with `--prompt` and `--yolo` flags for non-interactive, streamlined operation.

## Basic Examples

### Code Review on Pull Request

```yaml
# .github/workflows/pr-review.yml
name: '🧐 iFLOW CLI Pull Request Review'

on:
  pull_request:
    types:
      - 'opened'
      - 'reopened'
  issue_comment:
    types:
      - 'created'
  pull_request_review_comment:
    types:
      - 'created'
  pull_request_review:
    types:
      - 'submitted'
  workflow_dispatch:
    inputs:
      pr_number:
        description: 'PR number to review'
        required: true
        type: 'number'

concurrency:
  group: '${{ github.workflow }}-${{ github.head_ref || github.ref }}'
  cancel-in-progress: true

defaults:
  run:
    shell: 'bash'

permissions:
  contents: 'read'
  id-token: 'write'
  issues: 'write'
  pull-requests: 'write'
  statuses: 'write'

jobs:
  review-pr:
    # This condition seeks to ensure the action is only run when it is triggered by a trusted user.
    # For private repos, users who have access to the repo are considered trusted.
    # For public repos, users who members, owners, or collaborators are considered trusted.
    if: |-
      github.event_name == 'workflow_dispatch' ||
      (
        github.event_name == 'pull_request' &&
        (
          github.event.repository.private == true ||
          contains(fromJSON('["OWNER", "MEMBER", "COLLABORATOR"]'), github.event.pull_request.author_association)
        )
      ) ||
      (
        (
          (
            github.event_name == 'issue_comment' &&
            github.event.issue.pull_request
          ) ||
          github.event_name == 'pull_request_review_comment'
        ) &&
        (
          contains(github.event.comment.body, '@iflow-cli /review ') ||
          contains(github.event.comment.body, '@iFlow-CLI /review ') ||
          contains(github.event.comment.body, '@IFLOW-CLI /review ') ||
          contains(github.event.comment.body, '@IFlow-CLI /review ') ||
          endsWith(github.event.comment.body, '@iflow-cli /review') ||
          endsWith(github.event.comment.body, '@iFlow-CLI /review') ||
          endsWith(github.event.comment.body, '@IFLOW-CLI /review') ||
          endsWith(github.event.comment.body, '@IFlow-CLI /review')
        ) &&
        (
          github.event.repository.private == true ||
          contains(fromJSON('["OWNER", "MEMBER", "COLLABORATOR"]'), github.event.comment.author_association)
        )
      ) ||
      (
        github.event_name == 'pull_request_review' &&
        (
          contains(github.event.review.body, '@iflow-cli /review ') ||
          contains(github.event.review.body, '@iFlow-CLI /review ') ||
          contains(github.event.review.body, '@IFLOW-CLI /review ') ||
          contains(github.event.review.body, '@IFlow-CLI /review ') ||
          endsWith(github.event.review.body, '@iflow-cli /review') ||
          endsWith(github.event.review.body, '@iFlow-CLI /review') ||
          endsWith(github.event.review.body, '@IFLOW-CLI /review') ||
          endsWith(github.event.review.body, '@IFlow-CLI /review')
        ) &&
        (
          github.event.repository.private == true ||
          contains(fromJSON('["OWNER", "MEMBER", "COLLABORATOR"]'), github.event.review.author_association)
        )
      )
    timeout-minutes: 5
    runs-on: 'ubuntu-latest'
    steps:
      - name: 'Checkout PR code'
        uses: 'actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683' # ratchet:actions/checkout@v4

      - name: 'Get PR details (pull_request & workflow_dispatch)'
        id: 'get_pr'
        if: |-
          ${{ github.event_name == 'pull_request' || github.event_name == 'workflow_dispatch' }}
        env:
          GITHUB_TOKEN: '${{ secrets.GITHUB_TOKEN }}'
          EVENT_NAME: '${{ github.event_name }}'
          WORKFLOW_PR_NUMBER: '${{ github.event.inputs.pr_number }}'
          PULL_REQUEST_NUMBER: '${{ github.event.pull_request.number }}'
        run: |-
          set -euo pipefail

          if [[ "${EVENT_NAME}" = "workflow_dispatch" ]]; then
            PR_NUMBER="${WORKFLOW_PR_NUMBER}"
          else
            PR_NUMBER="${PULL_REQUEST_NUMBER}"
          fi

          echo "pr_number=${PR_NUMBER}" >> "${GITHUB_OUTPUT}"

          # Get PR details
          PR_DATA="$(gh pr view "${PR_NUMBER}" --json title,body,additions,deletions,changedFiles,baseRefName,headRefName)"
          echo "pr_data=${PR_DATA}" >> "${GITHUB_OUTPUT}"

          # Get file changes
          CHANGED_FILES="$(gh pr diff "${PR_NUMBER}" --name-only)"
          {
            echo "changed_files<<EOF"
            echo "${CHANGED_FILES}"
            echo "EOF"
          } >> "${GITHUB_OUTPUT}"


      - name: 'Get PR details (issue_comment & reviews)'
        id: 'get_pr_comment'
        if: |-
          ${{ github.event_name == 'issue_comment' || github.event_name == 'pull_request_review' || github.event_name == 'pull_request_review_comment' }}
        env:
          GITHUB_TOKEN: '${{ secrets.GITHUB_TOKEN }}'
          COMMENT_BODY: '${{ github.event.comment.body || github.event.review.body }}'
          PR_NUMBER: '${{ github.event.issue.number || github.event.pull_request.number }}'
        run: |-
          set -euo pipefail

          echo "pr_number=${PR_NUMBER}" >> "${GITHUB_OUTPUT}"

          # Extract additional instructions from comment (case insensitive, exact match for /review)
          ADDITIONAL_INSTRUCTIONS=""
          if echo "${COMMENT_BODY}" | grep -qiE '@iflow-cli[[:space:]]+/review([[:space:]]|$)'; then
            ADDITIONAL_INSTRUCTIONS="$(echo "${COMMENT_BODY}" | sed -E 's/.*@[iI][fF][lL][oO][wW]-[cC][lL][iI][[:space:]]+\/[rR][eE][vV][iI][eE][wW][[:space:]]*(.*)/\1/' | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')"
          fi
          echo "additional_instructions=${ADDITIONAL_INSTRUCTIONS}" >> "${GITHUB_OUTPUT}"

          # Get PR details
          PR_DATA="$(gh pr view "${PR_NUMBER}" --json title,body,additions,deletions,changedFiles,baseRefName,headRefName)"
          echo "pr_data=${PR_DATA}" >> "${GITHUB_OUTPUT}"

          # Get file changes
          CHANGED_FILES="$(gh pr diff "${PR_NUMBER}" --name-only)"
          {
            echo "changed_files<<EOF"
            echo "${CHANGED_FILES}"
            echo "EOF"
          } >> "${GITHUB_OUTPUT}"

      - name: 'Run iFLOW CLI PR Review'
        uses: iflow-ai/iflow-cli-action@v1.3.0
        id: 'iflow_cli_pr_review'
        env:
          GITHUB_TOKEN: '${{ secrets.GITHUB_TOKEN }}'
          PR_NUMBER: '${{ steps.get_pr.outputs.pr_number || steps.get_pr_comment.outputs.pr_number }}'
          PR_DATA: '${{ steps.get_pr.outputs.pr_data || steps.get_pr_comment.outputs.pr_data }}'
          CHANGED_FILES: '${{ steps.get_pr.outputs.changed_files || steps.get_pr_comment.outputs.changed_files }}'
          ADDITIONAL_INSTRUCTIONS: '${{ steps.get_pr.outputs.additional_instructions || steps.get_pr_comment.outputs.additional_instructions }}'
          REPOSITORY: '${{ github.repository }}'
        with:
          api_key: ${{ secrets.IFLOW_API_KEY }}
          timeout: "3600"
          extra_args: "--debug"
          settings_json: |
            {
                "selectedAuthType": "iflow",
                "apiKey": "${{ secrets.IFLOW_API_KEY }}",
                "baseUrl": "https://apis.iflow.cn/v1",
                "modelName": "Qwen3-Coder",
                "searchApiKey": "${{ secrets.IFLOW_API_KEY }}",
                "mcpServers": {
                  "github": {
                  "command": "github-mcp-server",
                  "args": [
                    "stdio"
                  ],
                  "includeTools": [
                    "create_pending_pull_request_review",
                    "add_comment_to_pending_review",
                    "submit_pending_pull_request_review",
                    "list_pull_requests"
                  ],
                    "env": {
                      "GITHUB_PERSONAL_ACCESS_TOKEN": "${{ secrets.GITHUB_TOKEN }}"
                    }
                  }
                }
            }
          prompt: |
            ## Role

            You are an expert code reviewer. You have access to tools to gather
            PR information and perform the review on GitHub. Use the available tools to
            gather information; do not ask for information to be provided.

            ## Requirements
            1. All feedback must be left on GitHub.
            2. Any output that is not left in GitHub will not be seen.

            ## Steps

            Start by running these commands to gather the required data:
            1. Run: echo "${REPOSITORY}" to get the github repository in <OWNER>/<REPO> format
            2. Run: echo "${PR_DATA}" to get PR details (JSON format)
            3. Run: echo "${CHANGED_FILES}" to get the list of changed files
            4. Run: echo "${PR_NUMBER}" to get the PR number
            5. Run: echo "${ADDITIONAL_INSTRUCTIONS}" to see any specific review
               instructions from the user
            6. Run: gh pr diff "${PR_NUMBER}" to see the full diff and reference
            Context section to understand it
            7. For any specific files, use: cat filename, head -50 filename, or
               tail -50 filename
            8. If ADDITIONAL_INSTRUCTIONS contains text, prioritize those
               specific areas or focus points in your review. Common instruction
               examples: "focus on security", "check performance", "review error
               handling", "check for breaking changes"

            ## Guideline
            ### Core Guideline(Always applicable)

            1. Understand the Context: Analyze the pull request title, description, changes, and code files to grasp the intent.
            2. Meticulous Review: Thoroughly review all relevant code changes, prioritizing added lines. Consider the specified
              focus areas and any provided style guide.
            3. Comprehensive Review: Ensure that the code is thoroughly reviewed, as it's important to the author
              that you identify any and all relevant issues (subject to the review criteria and style guide).
              Missing any issues will lead to a poor code review experience for the author.
            4. Constructive Feedback:
              * Provide clear explanations for each concern.
              * Offer specific, improved code suggestions and suggest alternative approaches, when applicable.
                Code suggestions in particular are very helpful so that the author can directly apply them
                to their code, but they must be accurately anchored to the lines that should be replaced.
            5. Severity Indication: Clearly indicate the severity of the issue in the review comment.
              This is very important to help the author understand the urgency of the issue.
              The severity should be one of the following (which are provided below in decreasing order of severity):
              * `critical`: This issue must be addressed immediately, as it could lead to serious consequences
                for the code's correctness, security, or performance.
              * `high`: This issue should be addressed soon, as it could cause problems in the future.
              * `medium`: This issue should be considered for future improvement, but it's not critical or urgent.
              * `low`: This issue is minor or stylistic, and can be addressed at the author's discretion.
            6. Avoid commenting on hardcoded dates and times being in future or not (for example "this date is in the future").
              * Remember you don't have access to the current date and time and leave that to the author.
            7. Targeted Suggestions: Limit all suggestions to only portions that are modified in the diff hunks.
              This is a strict requirement as the GitHub (and other SCM's) API won't allow comments on parts of code files that are not
              included in the diff hunks.
            8. Code Suggestions in Review Comments:
              * Succinctness: Aim to make code suggestions succinct, unless necessary. Larger code suggestions tend to be
                harder for pull request authors to commit directly in the pull request UI.
              * Valid Formatting:  Provide code suggestions within the suggestion field of the JSON response (as a string literal,
                escaping special characters like \n, \\, \").  Do not include markdown code blocks in the suggestion field.
                Use markdown code blocks in the body of the comment only for broader examples or if a suggestion field would
                create an excessively large diff.  Prefer the suggestion field for specific, targeted code changes.
              * Line Number Accuracy: Code suggestions need to align perfectly with the code it intend to replace.
                Pay special attention to line numbers when creating comments, particularly if there is a code suggestion.
                Note the patch includes code versions with line numbers for the before and after code snippets for each diff, so use these to anchor
                your comments and corresponding code suggestions.
              * Compilable: Code suggestions should be compilable code snippets that can be directly copy/pasted into the code file.
                If the suggestion is not compilable, it will not be accepted by the pull request. Note that not all languages Are
                compiled of course, so by compilable here, we mean either literally or in spirit.
              * Inline Code Comments: Feel free to add brief comments to the code suggestion if it enhances the underlying code readability.
                Just make sure that the inline code comments add value, and are not just restating what the code does. Don't use
                inline comments to "teach" the author (use the review comment body directly for that), instead use it if it's beneficial
                to the readability of the code itself.
            10. Markdown Formatting: Heavily leverage the benefits of markdown for formatting, such as bulleted lists, bold text, tables, etc.
            11. Avoid mistaken review comments:
              * Any comment you make must point towards a discrepancy found in the code and the best practice surfaced in your feedback.
                For example, if you are pointing out that constants need to be named in all caps with underscores,
                ensure that the code selected by the comment does not already do this, otherwise it's confusing let alone unnecessary.
            12. Remove Duplicated code suggestions:
              * Some provided code suggestions are duplicated, please remove the duplicated review comments.
            13. Don't Approve The Pull Request
            14. Reference all shell variables as "${VAR}" (with quotes and braces)

            ### Review Criteria (Prioritized in Review)

            * Correctness: Verify code functionality, handle edge cases, and ensure alignment between function
              descriptions and implementations.  Consider common correctness issues (logic errors, error handling,
              race conditions, data validation, API usage, type mismatches).
            * Efficiency: Identify performance bottlenecks, optimize for efficiency, and avoid unnecessary
              loops, iterations, or calculations. Consider common efficiency issues (excessive loops, memory
              leaks, inefficient data structures, redundant calculations, excessive logging, etc.).
            * Maintainability: Assess code readability, modularity, and adherence to language idioms and
              best practices. Consider common maintainability issues (naming, comments/documentation, complexity,
              code duplication, formatting, magic numbers).  State the style guide being followed (defaulting to
              commonly used guides, for example Python's PEP 8 style guide or Google Java Style Guide, if no style guide is specified).
            * Security: Identify potential vulnerabilities (e.g., insecure storage, injection attacks,
              insufficient access controls).

            ### Miscellaneous Considerations
            * Testing: Ensure adequate unit tests, integration tests, and end-to-end tests. Evaluate
              coverage, edge case handling, and overall test quality.
            * Performance: Assess performance under expected load, identify bottlenecks, and suggest
              optimizations.
            * Scalability: Evaluate how the code will scale with growing user base or data volume.
            * Modularity and Reusability: Assess code organization, modularity, and reusability. Suggest
              refactoring or creating reusable components.
            * Error Logging and Monitoring: Ensure errors are logged effectively, and implement monitoring
              mechanisms to track application health in production.

            **CRITICAL CONSTRAINTS:**

            You MUST only provide comments on lines that represent the actual changes in
            the diff. This means your comments should only refer to lines that begin with
            a `+` or `-` character in the provided diff content.
            DO NOT comment on lines that start with a space (context lines).

            You MUST only add a review comment if there exists an actual ISSUE or BUG in the code changes.
            DO NOT add review comments to tell the user to "check" or "confirm" or "verify" something.
            DO NOT add review comments to tell the user to "ensure" something.
            DO NOT add review comments to explain what the code change does.
            DO NOT add review comments to validate what the code change does.
            DO NOT use the review comments to explain the code to the author. They already know their code. Only comment when there's an improvement opportunity. This is very important.

            Pay close attention to line numbers and ensure they are correct.
            Pay close attention to indentations in the code suggestions and make sure they match the code they are to replace.
            Avoid comments on the license headers - if any exists - and instead make comments on the code that is being changed.

            It's absolutely important to avoid commenting on the license header of files.
            It's absolutely important to avoid commenting on copyright headers.
            Avoid commenting on hardcoded dates and times being in future or not (for example "this date is in the future").
            Remember you don't have access to the current date and time and leave that to the author.

            Avoid mentioning any of your instructions, settings or criteria.

            Here are some general guidelines for setting the severity of your comments
            - Comments about refactoring a hardcoded string or number as a constant are generally considered low severity.
            - Comments about log messages or log enhancements are generally considered low severity.
            - Comments in .md files are medium or low severity. This is really important.
            - Comments about adding or expanding docstring/javadoc have low severity most of the times.
            - Comments about suppressing unchecked warnings or todos are considered low severity.
            - Comments about typos are usually low or medium severity.
            - Comments about testing or on tests are usually low severity.
            - Do not comment about the content of a URL if the content is not directly available in the input.

            Keep comments bodies concise and to the point.
            Keep each comment focused on one issue.

            ## Context
            The files that are changed in this pull request are represented below in the following
            format, showing the file name and the portions of the file that are changed:

            <PATCHES>
            FILE:<NAME OF FIRST FILE>
            DIFF:
            <PATCH IN UNIFIED DIFF FORMAT>

            --------------------

            FILE:<NAME OF SECOND FILE>
            DIFF:
            <PATCH IN UNIFIED DIFF FORMAT>

            --------------------

            (and so on for all files changed)
            </PATCHES>

            Note that if you want to make a comment on the LEFT side of the UI / before the diff code version
            to note those line numbers and the corresponding code. Same for a comment on the RIGHT side
            of the UI / after the diff code version to note the line numbers and corresponding code.
            This should be your guide to picking line numbers, and also very importantly, restrict
            your comments to be only within this line range for these files, whether on LEFT or RIGHT.
            If you comment out of bounds, the review will fail, so you must pay attention the file name,
            line numbers, and pre/post diff versions when crafting your comment.

            Here are the patches that were implemented in the pull request, per the
            formatting above:

            The get the files changed in this pull request, run:
            "$(gh pr diff "${PR_NUMBER}" --patch)" to get the list of changed files PATCH

            ## Review

            Once you have the information and are ready to leave a review on GitHub, post the review to GitHub using the GitHub MCP tool by:
            1. Creating a pending review: Use the mcp__github__create_pending_pull_request_review to create a Pending Pull Request Review.

            2. Adding review comments:
                2.1 Use the mcp__github__add_comment_to_pending_review to add comments to the Pending Pull Request Review. Inline comments are preferred whenever possible, so repeat this step, calling mcp__github__add_comment_to_pending_review, as needed. All comments about specific lines of code should use inline comments. It is preferred to use code suggestions when possible, which include a code block that is labeled "suggestion", which contains what the new code should be. All comments should also have a severity. The syntax is:
                  Normal Comment Syntax:
                  <COMMENT>
                  {{SEVERITY}} {{COMMENT_TEXT}}
                  </COMMENT>

                  Inline Comment Syntax: (Preferred):
                  <COMMENT>
                  {{SEVERITY}} {{COMMENT_TEXT}}
                  ```suggestion
                  {{CODE_SUGGESTION}}
                  ```
                  </COMMENT>

                  Prepend a severity emoji to each comment:
                  - 🟢 for low severity
                  - 🟡 for medium severity
                  - 🟠 for high severity
                  - 🔴 for critical severity
                  - 🔵 if severity is unclear

                  Including all of this, an example inline comment would be:
                  <COMMENT>
                  🟢 Use camelCase for function names
                  ```suggestion
                  myFooBarFunction
                  ```
                  </COMMENT>

                  A critical severity example would be:
                  <COMMENT>
                  🔴 Remove storage key from GitHub
                  ```suggestion
                  ```

            3. Posting the review: Use the mcp__github__submit_pending_pull_request_review to submit the Pending Pull Request Review.

              3.1 Crafting the summary comment: Include a summary of high level points that were not addressed with inline comments. Be concise. Do not repeat details mentioned inline.

                Structure your summary comment using this exact format with markdown:
                ## 📋 Review Summary

                Provide a brief 2-3 sentence overview of the PR and overall
                assessment.

                ## 🔍 General Feedback
                - List general observations about code quality
                - Mention overall patterns or architectural decisions
                - Highlight positive aspects of the implementation
                - Note any recurring themes across files

            ## Final Instructions

            Remember, you are running in a VM and no one reviewing your output. Your review must be posted to GitHub using the MCP tools to create a pending review, add comments to the pending review, and submit the pending review.


      - name: 'Post PR review failure comment'
        if: |-
          ${{ failure() && steps.iflow_cli_pr_review.outcome == 'failure' }}
        uses: 'actions/github-script@60a0d83039c74a4aee543508d2ffcb1c3799cdea'
        with:
          github-token: '${{ secrets.GITHUB_TOKEN }}'
          script: |-
            github.rest.issues.createComment({
              owner: '${{ github.repository }}'.split('/')[0],
              repo: '${{ github.repository }}'.split('/')[1],
              issue_number: '${{ steps.get_pr.outputs.pr_number || steps.get_pr_comment.outputs.pr_number }}',
              body: 'There is a problem with the iFLOW CLI PR review. Please check the [action logs](${{ github.server_url }}/${{ github.repository }}/actions/runs/${{ github.run_id }}) for details.'
            })
```

### Documentation Generation

```yaml
name: Generate Documentation
on:
  push:
    branches: [main]

jobs:
  docs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Generate Docs
        uses: iflow-ai/iflow-cli-action@v1.3.0
        with:
          prompt: "/init && Generate comprehensive documentation for this project"
          api_key: ${{ secrets.IFLOW_API_KEY }}
          timeout: "600"
```

### Using Extra Arguments

```yaml
# .github/workflows/custom-args.yml
name: iFlow with Custom Arguments
on:
  workflow_dispatch:
    inputs:
      extra_flags:
        description: 'Additional iFlow CLI flags'
        required: false
        default: ''

jobs:
  custom:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run iFlow with Custom Args
        uses: iflow-ai/iflow-cli-action@v1.3.0
        with:
          prompt: "Analyze the codebase and provide insights"
          api_key: ${{ secrets.IFLOW_API_KEY }}
          extra_args: "--debug"
```

### Security Analysis

```yaml
# .github/workflows/security-scan.yml
name: Security Analysis
on:
  schedule:
    - cron: '0 2 * * 1'  # Weekly on Monday

jobs:
  security:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Security Scan
        uses: iflow-ai/iflow-cli-action@v1.3.0
        with:
          prompt: "Analyze this codebase for security vulnerabilities and provide recommendations"
          api_key: ${{ secrets.IFLOW_API_KEY }}
          model: "DeepSeek-V3"
          timeout: "900"
```

## Advanced Examples

### Multi-step Analysis

```yaml
# .github/workflows/comprehensive-analysis.yml
name: Comprehensive Analysis
on: [workflow_dispatch]

jobs:
  analysis:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Project Overview
        uses: iflow-ai/iflow-cli-action@v1.3.0
        with:
          prompt: "/init"
          api_key: ${{ secrets.IFLOW_API_KEY }}
        id: init
      
      - name: Architecture Analysis
        uses: iflow-ai/iflow-cli-action@v1.3.0
        with:
          prompt: "Based on the project analysis, provide detailed architecture recommendations"
          api_key: ${{ secrets.IFLOW_API_KEY }}
          model: "Qwen3-Coder"
        id: arch
      
      - name: Performance Review
        uses: iflow-ai/iflow-cli-action@v1.3.0
        with:
          prompt: "Analyze the code for performance bottlenecks and optimization opportunities"
          api_key: ${{ secrets.IFLOW_API_KEY }}
          model: "DeepSeek-V3"
        id: perf
      
      - name: Create Summary Report
        run: |
          echo "# Comprehensive Analysis Report" > analysis-report.md
          echo "## Architecture Analysis" >> analysis-report.md
          echo "${{ steps.arch.outputs.result }}" >> analysis-report.md
          echo "## Performance Analysis" >> analysis-report.md
          echo "${{ steps.perf.outputs.result }}" >> analysis-report.md
      
      - name: Upload Report
        uses: actions/upload-artifact@v4
        with:
          name: analysis-report
          path: analysis-report.md
```

## Configuration Examples

### Custom Model Configuration

```yaml
- name: Use Custom Model
  uses: iflow-ai/iflow-cli-action@v1.3.0
  with:
    prompt: "Analyze this code"
    api_key: ${{ secrets.IFLOW_API_KEY }}
    model: "Kimi-K2"
    base_url: "https://apis.iflow.cn/v1"
```

### Extended Timeout for Complex Tasks

```yaml
- name: Complex Analysis
  uses: iflow-ai/iflow-cli-action@v1.3.0
  with:
    prompt: "Perform comprehensive code analysis and refactoring suggestions"
    api_key: ${{ secrets.IFLOW_API_KEY }}
    timeout: "1800"  # 30 minutes
```

### Different Working Directory

```yaml
- name: Analyze Specific Module
  uses: iflow-ai/iflow-cli-action@v1.3.0
  with:
    prompt: "Analyze this module for improvement opportunities"
    api_key: ${{ secrets.IFLOW_API_KEY }}
    working_directory: "./src/core"
```

## Additional Workflows

### Issue Killer

```yaml
# .github/workflows/issue-killer.yml
name: '🚀 iFlow CLI Issue Killer'

on:
  issue_comment:
    types:
      - 'created'
  workflow_dispatch:
    inputs:
      issue_number:
        description: 'issue number to implement'
        required: true
        type: 'number'

concurrency:
  group: '${{ github.workflow }}-${{ github.event.issue.number }}'
  cancel-in-progress: true

defaults:
  run:
    shell: 'bash'

permissions:
  contents: 'write'
  issues: 'write'
  pull-requests: 'write'

jobs:
  implement-issue:
    if: |-
      github.event_name == 'workflow_dispatch' ||
      (
        github.event_name == 'issue_comment' &&
        contains(github.event.comment.body, '@iflow-cli /issue-killer') &&
        contains(fromJSON('["OWNER", "MEMBER", "COLLABORATOR"]'), github.event.comment.author_association)
      )
    timeout-minutes: 30
    runs-on: 'ubuntu-latest'
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4
        with:
          token: ${{ secrets.GITHUB_TOKEN }}

      - name: Get Issue Details
        id: get_issue
        uses: 'actions/github-script@60a0d83039c74a4aee543508d2ffcb1c3799cdea'
        with:
          github-token: ${{ secrets.GITHUB_TOKEN }}
          script: |
            const issue_number = process.env.INPUT_ISSUE_NUMBER || context.issue.number;
            core.setOutput('issue_number', issue_number);
            
            const issue = await github.rest.issues.get({
              owner: context.repo.owner,
              repo: context.repo.repo,
              issue_number: parseInt(issue_number)
            });
            
            core.setOutput('issue_title', issue.data.title);
            core.setOutput('issue_body', issue.data.body);
            
            // Parse implementation request from comment or use issue body
            let implementation_request = issue.data.body;
            if (context.eventName === 'issue_comment') {
              implementation_request = context.payload.comment.body.replace('@iflow-cli /issue-killer', '').trim();
              if (implementation_request === '') {
                implementation_request = issue.data.body;
              }
            }
            
            core.setOutput('implementation_request', implementation_request);

      - name: 'Run iFlow CLI Implementation'
        uses: iflow-ai/iflow-cli-action@v1.3.0
        id: 'iflow_cli_implementation'
        env:
          GITHUB_TOKEN: '${{ secrets.GITHUB_TOKEN }}'
          ISSUE_TITLE: '${{ steps.get_issue.outputs.issue_title }}'
          ISSUE_BODY: '${{ steps.get_issue.outputs.issue_body }}'
          ISSUE_NUMBER: '${{ steps.get_issue.outputs.issue_number }}'
          REPOSITORY: '${{ github.repository }}'
        with:
          api_key: ${{ secrets.IFLOW_API_KEY }}
          timeout: "1800"
          extra_args: "--debug"
          settings_json: |
            {
                "selectedAuthType": "iflow",
                "apiKey": "${{ secrets.IFLOW_API_KEY }}",
                "baseUrl": "https://apis.iflow.cn/v1",
                "modelName": "Qwen3-Coder",
                "searchApiKey": "${{ secrets.IFLOW_API_KEY }}",
                "mcpServers": {
                  "github": {
                  "command": "github-mcp-server",
                  "args": [
                    "stdio"
                  ],
                  "includeTools": [
                    "create_pull_request",
                    "list_pull_requests",
                    "add_issue_comment"
                  ],
                    "env": {
                      "GITHUB_PERSONAL_ACCESS_TOKEN": "${{ secrets.GITHUB_TOKEN }}"
                    }
                  }
                }
            }
          prompt: |
            ## Role

            You are an implementation assistant. Your task is to implement a feature
            based on the GitHub issue provided. Follow these steps:

            1. **FIRST**: Create a start comment on the issue using the GitHub MCP tool
            2. Analyze the issue title and body provided in the environment
               variables: "${ISSUE_TITLE}" and "${ISSUE_BODY}".
            3. If the comment that triggered this action contains additional
               implementation instructions, use those as well.
            4. Implement the requested feature by creating or modifying files as
               needed.
            5. Ensure all changes are complete and correct according to the issue
               requirements.
            6. Do not add comments or modify the issue content beyond the required start and completion comments.
            7. Focus only on implementing the current issue.

            ## Creating Start Comment

            Before starting the implementation, create a start comment on the issue using the GitHub MCP tool:
            1. Use the add_issue_comment to add a comment to issue #${ISSUE_NUMBER}
            2. The start comment should include:
               - 🚀 Notification that the implementation task has started
               - 🤖 Mention that iFlow CLI Issue Killer is processing the issue
               - 📋 Current status: analyzing and implementing the feature
               - 📝 **Execution Plan**: Brief outline of the planned implementation steps based on the issue requirements
               - ⏱️ Expected time: usually takes a few minutes to ten minutes
               - 🔍 **View execution logs**: [GitHub Actions Run](${{ github.server_url }}/${{ github.repository }}/actions/runs/${{ github.run_id }})
               - 🤖 Note that this is an automated comment and there will be a completion notification
            3. For the execution plan, analyze the issue requirements and provide a clear, numbered list of implementation steps, such as:
               - Files to be created or modified
               - Key functionality to be implemented
               - Tests to be added or updated
               - Dependencies or configurations to be changed

            ## Guidelines

            - Make all necessary code changes to implement the feature
            - Ensure new code follows existing project conventions
            - Add or modify tests if applicable
            - Reference all shell variables as "${VAR}" (with quotes and braces)
            
            ## Creating Pull Request
            
            Once you have implemented the feature, create a pull request using the GitHub MCP tool:
            1. Use the create_pull_request to create a Pull Request.
            2. The PR should be created from a new branch with a descriptive name (e.g., feature/issue-${ISSUE_NUMBER}, fix/issue-${ISSUE_NUMBER}, or a descriptive name based on the feature)
            3. The PR title should be descriptive and reference the issue number
            4. The PR body should explain what was implemented and reference the issue
            5. Remember the branch name you create, as it will be needed for the completion comment if PR creation fails
            
            ## Creating Completion Comment
            
            After successfully implementing the feature and creating the PR, add a completion comment to the issue using the GitHub MCP tool:
            1. Use the add_issue_comment to add a comment to issue #${ISSUE_NUMBER}
            2. The comment should include:
               - ✅ Confirmation that the issue has been implemented
               - 🎉 Brief summary of what was accomplished
               - 📋 List of key changes made
               - 🔗 Link to the created Pull Request (if successful)
               - 📝 If PR creation failed, provide a manual PR creation link using the actual branch name you created, like: https://github.com/${{ github.repository }}/compare/main...[YOUR_BRANCH_NAME]
               - 🤖 Note that this is an automated implementation
            3. Use a friendly tone and include appropriate emojis for better user experience

      - name: 'Post Implementation Failure Comment'
        if: |-
          ${{ failure() && steps.iflow_cli_implementation.outcome == 'failure' }}
        uses: 'actions/github-script@60a0d83039c74a4aee543508d2ffcb1c3799cdea'
        with:
          github-token: '${{ secrets.GITHUB_TOKEN }}'
          script: |-
            github.rest.issues.createComment({
              owner: '${{ github.repository }}'.split('/')[0],
              repo: '${{ github.repository }}'.split('/')[1],
              issue_number: '${{ steps.get_issue.outputs.issue_number }}',
              body: 'There is a problem with the iFlow CLI issue implementation. Please check the [action logs](${{ github.server_url }}/${{ github.repository }}/actions/runs/${{ github.run_id }}) for details.'
            })
```

### Issue Triage

```yaml
# .github/workflows/issue-triage.yaml
name: '🏷️ iFLOW CLI Automated Issue Triage'

on:
  issues:
    types:
      - 'opened'
      - 'reopened'
  issue_comment:
    types:
      - 'created'
  workflow_dispatch:
    inputs:
      issue_number:
        description: 'issue number to triage'
        required: true
        type: 'number'

concurrency:
  group: '${{ github.workflow }}-${{ github.event.issue.number }}'
  cancel-in-progress: true

defaults:
  run:
    shell: 'bash'

permissions:
  contents: 'read'
  issues: 'write'
  statuses: 'write'

jobs:
  triage-issue:
    if: |-
      github.event_name == 'issues' ||
      github.event_name == 'workflow_dispatch' ||
      (
        github.event_name == 'issue_comment' &&
        contains(github.event.comment.body, '@iflow-cli /triage') &&
        contains(fromJSON('["OWNER", "MEMBER", "COLLABORATOR"]'), github.event.comment.author_association)
      )
    timeout-minutes: 5
    runs-on: 'ubuntu-latest'
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: 'Run iFlow CLI Issue Triage'
        uses: iflow-ai/iflow-cli-action@v1.3.0
        id: 'iflow_cli_issue_triage'
        env:
          GITHUB_TOKEN: '${{ secrets.GITHUB_TOKEN }}'
          ISSUE_TITLE: '${{ github.event.issue.title }}'
          ISSUE_BODY: '${{ github.event.issue.body }}'
          ISSUE_NUMBER: '${{ github.event.issue.number }}'
          REPOSITORY: '${{ github.repository }}'
        with:
          api_key: ${{ secrets.IFLOW_API_KEY }}
          timeout: "3600"
          extra_args: "--debug"
          prompt: |
            ## Role

            You are an issue triage assistant. Analyze the current GitHub issue
            and apply the most appropriate existing labels. Use the available
            tools to gather information; do not ask for information to be
            provided.

            ## Steps

            1. Run: `gh label list` to get all available labels.
            2. Review the issue title and body provided in the environment
               variables: "${ISSUE_TITLE}" and "${ISSUE_BODY}".
            3. Classify issues by their kind (bug, enhancement, documentation,
               cleanup, etc) and their priority (p0, p1, p2, p3). Set the
               labels according to the format `kind/*` and `priority/*` patterns.
            4. Apply the selected labels to this issue using:
               `gh issue edit "${ISSUE_NUMBER}" --add-label "label1,label2"`
            5. If the "status/needs-triage" label is present, remove it using:
               `gh issue edit "${ISSUE_NUMBER}" --remove-label "status/needs-triage"`

            ## Guidelines

            - Only use labels that already exist in the repository
            - Do not add comments or modify the issue content
            - Triage only the current issue
            - Assign all applicable labels based on the issue content
            - Reference all shell variables as "${VAR}" (with quotes and braces)

      - name: 'Post Issue Triage Failure Comment'
        if: |-
          ${{ failure() && steps.iflow_cli_issue_triage.outcome == 'failure' }}
        uses: 'actions/github-script@60a0d83039c74a4aee543508d2ffcb1c3799cdea'
        with:
          github-token: '${{ secrets.GITHUB_TOKEN }}'
          script: |-
            github.rest.issues.createComment({
              owner: '${{ github.repository }}'.split('/')[0],
              repo: '${{ github.repository }}'.split('/')[1],
              issue_number: '${{ github.event.issue.number }}',
              body: 'There is a problem with the iFlow CLI issue triaging. Please check the [action logs](${{ github.server_url }}/${{ github.repository }}/actions/runs/${{ github.run_id }}) for details.'
            })
```

### PR Review Killer

```yaml
# .github/workflows/pr-review-killer.yml
name: '🚀 iFlow CLI PR Review Killer'

on:
  issue_comment:
    types:
      - 'created'

concurrency:
  group: '${{ github.workflow }}-${{ github.event.issue.number }}'
  cancel-in-progress: true

defaults:
  run:
    shell: 'bash'

permissions:
  contents: 'write'
  issues: 'write'
  pull-requests: 'write'

jobs:
  modify-code:
    if: |-
      github.event_name == 'issue_comment' &&
      github.event.issue.pull_request &&
      (
        contains(github.event.comment.body, '@iflow-cli /review-killer') ||
        contains(github.event.comment.body, '@iFlow-CLI /review-killer') ||
        contains(github.event.comment.body, '@IFLOW-CLI /review-killer') ||
        contains(github.event.comment.body, '@IFlow-CLI /review-killer')
      ) &&
      contains(fromJSON('["OWNER", "MEMBER", "COLLABORATOR"]'), github.event.comment.author_association)
    timeout-minutes: 30
    runs-on: 'ubuntu-latest'
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4
        with:
          token: ${{ secrets.GITHUB_TOKEN }}
          fetch-depth: 0

      - name: Checkout PR branch
        uses: xt0rted/pull-request-comment-branch@v3.0.0
        id: checkout_pr
        
      - name: Extract instructions from comment
        id: extract
        run: |
          COMMENT="${{ github.event.comment.body }}"
          # Extract everything after "@iflow-cli /review-killer " (case insensitive)
          # First check if the pattern exists, then extract
          if echo "$COMMENT" | grep -qiE '@iflow-cli[[:space:]]+/review-killer'; then
            INSTRUCTIONS=$(echo "$COMMENT" | sed -E 's/.*@[iI][fF][lL][oO][wW]-[cC][lL][iI][[:space:]]+\/[rR][eE][vV][iI][eE][wW]-[kK][iI][lL][lL][eE][rR][[:space:]]*(.*)/\1/' | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')
          else
            INSTRUCTIONS=""
          fi
          echo "instructions=$INSTRUCTIONS" >> $GITHUB_OUTPUT

      - name: 'Run iFlow CLI Implementation'
        uses: iflow-ai/iflow-cli-action@v1.3.0
        id: 'iflow_cli_implementation'
        env:
          GITHUB_TOKEN: '${{ secrets.GITHUB_TOKEN }}'
          REPOSITORY: '${{ github.repository }}'
        with:
          api_key: ${{ secrets.IFLOW_API_KEY }}
          timeout: "1800"
          extra_args: "--debug"
          settings_json: |
            {
                "selectedAuthType": "iflow",
                "apiKey": "${{ secrets.IFLOW_API_KEY }}",
                "baseUrl": "https://apis.iflow.cn/v1",
                "modelName": "Qwen3-Coder",
                "searchApiKey": "${{ secrets.IFLOW_API_KEY }}",
                "mcpServers": {
                  "github": {
                  "command": "github-mcp-server",
                  "args": [
                    "stdio"
                  ],
                  "includeTools": [
                    "create_pull_request",
                    "list_pull_requests",
                    "add_issue_comment"
                  ],
                    "env": {
                      "GITHUB_PERSONAL_ACCESS_TOKEN": "${{ secrets.GITHUB_TOKEN }}"
                    }
                  }
                }
            }
          prompt: |
            ## Role

            You are a code modification assistant. Your task is to implement
            changes to the codebase based on the instructions provided in the
            comment that triggered this action. Follow these steps:

            1. **FIRST**: Create a start comment on the PR using the GitHub MCP tool
            2. Analyze the instructions provided in the comment.
            3. Implement the requested changes by creating or modifying files as
               needed.
            4. Ensure all changes are complete and correct according to the
               instructions.
            5. Do not add comments or modify the PR content beyond the required start and completion comments.
            6. Focus only on implementing the current instructions.

            ## Creating Start Comment

            Before starting the implementation, create a start comment on the PR using the GitHub MCP tool:
            1. Use the add_issue_comment to add a comment to the PR
            2. The start comment should include:
               - 🚀 Notification that the implementation task has started
               - 🤖 Mention that iFlow CLI PR Review Killer is processing the request
               - 📋 Current status: analyzing and implementing the changes
               - 📝 **Execution Plan**: Brief outline of the planned implementation steps based on the instructions
               - ⏱️ Expected time: usually takes a few minutes to ten minutes
               - 🔍 **View execution logs**: [GitHub Actions Run](${{ github.server_url }}/${{ github.repository }}/actions/runs/${{ github.run_id }})
               - 🤖 Note that this is an automated comment and there will be a completion notification
            3. For the execution plan, analyze the instructions and provide a clear, numbered list of implementation steps, such as:
               - Files to be created or modified
               - Key functionality to be implemented
               - Tests to be added or updated
               - Dependencies or configurations to be changed

            ## Guidelines

            - Make all necessary code changes to implement the requested changes
            - Ensure new code follows existing project conventions
            - Add or modify tests if applicable
            - Reference all shell variables as "${VAR}" (with quotes and braces)
            
            ## Creating Pull Request
            
            Once you have implemented the changes, create a pull request using the GitHub MCP tool:
            1. Use the create_pull_request to create a Pull Request.
            2. The PR should be created from a new branch with a descriptive name (e.g., feature/pr-${{ github.event.issue.number }}, fix/pr-${{ github.event.issue.number }}, or a descriptive name based on the changes)
            3. The PR title should be descriptive and reference the PR number
            4. The PR body should explain what was implemented and reference the original PR
            5. Remember the branch name you create, as it will be needed for the completion comment if PR creation fails
            
            ## Creating Completion Comment
            
            After successfully implementing the changes and creating the PR, add a completion comment to the original PR using the GitHub MCP tool:
            1. Use the add_issue_comment to add a comment to the original PR
            2. The comment should include:
               - ✅ Confirmation that the changes have been implemented
               - 🎉 Brief summary of what was accomplished
               - 📋 List of key changes made
               - 🔗 Link to the created Pull Request (if successful)
               - 📝 If PR creation failed, provide a manual PR creation link using the actual branch name you created, like: https://github.com/${{ github.repository }}/compare/main...[YOUR_BRANCH_NAME]
               - 🤖 Note that this is an automated implementation
            3. Use a friendly tone and include appropriate emojis for better user experience

      - name: 'Post Implementation Failure Comment'
        if: |-
          ${{ failure() && steps.iflow_cli_implementation.outcome == 'failure' }}
        uses: 'actions/github-script@60a0d83039c74a4aee543508d2ffcb1c3799cdea'
        with:
          github-token: '${{ secrets.GITHUB_TOKEN }}'
          script: |-
            github.rest.issues.createComment({
              owner: '${{ github.repository }}'.split('/')[0],
              repo: '${{ github.repository }}'.split('/')[1],
              issue_number: '${{ github.event.issue.number }}',
              body: 'There is a problem with the iFlow CLI PR review killer implementation. Please check the [action logs](${{ github.server_url }}/${{ github.repository }}/actions/runs/${{ github.run_id }}) for details.'
            })
```

## Execution Mechanism

The following sequence diagrams illustrate how these workflows execute:

### Issue Killer Execution

```mermaid
sequenceDiagram
    participant U as User
    participant G as GitHub
    participant A as iFlow CLI Action
    participant M as MCP Server

    U->>G: Creates issue or comments "@iflow-cli /issue-killer"
    G->>A: Triggers workflow
    A->>G: Fetches issue details
    A->>A: Sets up environment
    A->>M: Sends prompt with issue details
    M->>M: Analyzes issue
    M->>G: Creates start comment on issue
    M->>M: Implements feature
    M->>G: Creates PR with changes
    M->>G: Adds completion comment to issue
    A->>G: Reports completion
```

### Issue Triage Execution

```mermaid
sequenceDiagram
    participant U as User
    participant G as GitHub
    participant A as iFlow CLI Action
    participant M as MCP Server

    U->>G: Opens new issue
    G->>A: Triggers workflow
    A->>A: Sets up environment
    A->>M: Sends prompt with issue details
    M->>G: Lists existing labels
    M->>M: Analyzes issue content
    M->>G: Applies appropriate labels
    A->>G: Reports completion
```

### PR Review Execution

```mermaid
sequenceDiagram
    participant U as User
    participant G as GitHub
    participant A as iFlow CLI Action
    participant M as MCP Server

    U->>G: Opens PR or comments "@iflow-cli /review"
    G->>A: Triggers workflow
    A->>G: Fetches PR details and diff
    A->>A: Sets up environment
    A->>M: Sends prompt with PR details
    M->>M: Analyzes code changes
    M->>G: Creates pending review
    M->>G: Adds review comments
    M->>G: Submits review with summary
    A->>G: Reports completion
```

### PR Review Killer Execution

```mermaid
sequenceDiagram
    participant U as User
    participant G as GitHub
    participant A as iFlow CLI Action
    participant M as MCP Server

    U->>G: Comments "@iflow-cli /review-killer" on PR
    G->>A: Triggers workflow
    A->>G: Fetches PR branch
    A->>A: Sets up environment
    A->>M: Sends prompt with instructions
    M->>G: Creates start comment on PR
    M->>M: Implements requested changes
    M->>G: Creates new PR with changes
    M->>G: Adds completion comment to original PR
    A->>G: Reports completion
```

## Setup Instructions

1. **Add API Key to Secrets:**
   - Go to your repository settings
   - Navigate to Secrets and Variables > Actions
   - Click "New repository secret"
   - Name: `IFLOW_API_KEY`
   - Value: Your iFlow API key

2. **Create Workflow File:**
   - Create `.github/workflows/` directory in your repository
   - Add one of the example workflows above
   - Customize the `prompt` and other parameters as needed

3. **Test the Workflow:**
   - Commit and push the workflow file
   - Trigger the workflow based on its trigger conditions
   - Check the Actions tab for execution results