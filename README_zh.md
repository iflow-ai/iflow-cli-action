# 🤖 iFlow CLI GitHub Action

<!-- TOC start -->
## Table of Contents

- [功能特性](#功能特性)
- [使用方法](#使用方法)
  - [基础示例](#基础示例)
- [输入参数](#输入参数)
- [输出参数](#输出参数)
- [认证](#认证)
  - [获取 iFlow API 密钥](#获取-iflow-api-密钥)
  - [可用模型](#可用模型)
- [自定义配置](#自定义配置)
  - [使用附加参数](#使用附加参数)
    - [附加参数示例](#附加参数示例)
  - [使用预执行命令](#使用预执行命令)
    - [多行命令](#多行命令)
    - [带引号的参数](#带引号的参数)
  - [使用自定义设置](#使用自定义设置)
- [使用 MCP 服务器](#使用-mcp-服务器)
  - [示例：使用 DeepWiki MCP 服务器](#示例使用-deepwiki-mcp-服务器)
  - [何时使用 MCP 服务器](#何时使用-mcp-服务器)
- [常见用例](#常见用例)
  - [代码分析和审查](#代码分析和审查)
  - [文档生成](#文档生成)
  - [自动化测试建议](#自动化测试建议)
  - [问题管理](#问题管理)
  - [架构分析](#架构分析)
- [故障排除](#故障排除)
  - [常见问题](#常见问题)
  - [调试模式](#调试模式)
- [贡献](#贡献)
- [许可证](#许可证)
- [相关链接](#相关链接)

[Back to Table of Contents](README.md#table-of-contents)
<!-- TOC end -->


一个 GitHub Action，使您能够在 GitHub 工作流中运行 [iFlow CLI](https://github.com/iflow-ai/iflow-cli) 命令。这个基于 Docker 的操作预装了 Node.js 22、npm 和 uv（超快 Python 包管理器）以实现最佳性能，并使用 iFlow CLI 执行您指定的命令。

- [English Docs](README.md)

> 文档站点（使用 iFlow CLI GitHub Action 生成）：[https://iflow-ai.github.io/iflow-cli-action/](https://iflow-ai.github.io/iflow-cli-action/)

## 功能特性

- ✅ 基于 Docker 的操作，预装 Node.js 22、npm 和 uv
- ✅ 可配置的 iFlow API 认证
- ✅ 支持自定义模型和 API 端点
- ✅ 灵活的命令执行和超时控制
- ✅ 可在任何工作目录中运行
- ✅ 使用 Go 构建，快速可靠
- ✅ **GitHub Actions 摘要集成**：在 PR 摘要中提供丰富的执行报告
- ✅ PR/问题集成：与 GitHub 评论和 PR 审查无缝协作

## 使用方法

### 基础示例

使用 iFLOW CLI 进行问题分类：

```yaml
name: '🏷️ iFLOW CLI 自动化问题分类'

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
        description: '要分类的问题编号'
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
    runs-on: 'ubuntu-latest'
    steps:
      - name: 检出仓库
        uses: actions/checkout@v4

      - name: '运行 iFlow CLI 问题分类'
        uses: iflow-ai/iflow-cli-action@v2.0.0
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
          debug: "true"
          prompt: |
            ## 角色

            您是一个问题分类助手。分析当前的 GitHub 问题
            并应用最合适的现有标签。使用可用的
            工具收集信息；不要要求提供信息。

            ## 步骤

            1. 运行：`gh label list` 获取所有可用标签。
            2. 审查环境变量中提供的问题标题和正文：
               "${ISSUE_TITLE}" 和 "${ISSUE_BODY}"。
            3. 按类型（错误、增强、文档、
               清理等）和优先级（p0、p1、p2、p3）对问题进行分类。设置
               标签按照 `kind/*` 和 `priority/*` 模式。
            4. 使用以下命令将选定的标签应用到此问题：
               `gh issue edit "${ISSUE_NUMBER}" --add-label "label1,label2"`
            5. 如果存在 "status/needs-triage" 标签，使用以下命令移除它：
               `gh issue edit "${ISSUE_NUMBER}" --remove-label "status/needs-triage"`

            ## 指南

            - 仅使用仓库中已存在的标签
            - 不要添加评论或修改问题内容
            - 仅分类当前问题
            - 根据问题内容分配所有适用的标签
            - 引用所有 shell 变量为 "${VAR}"（带引号和大括号）

      - name: '发布问题分类失败评论'
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
              body: 'iFlow CLI 问题分类存在问题。请检查[操作日志](${{ github.server_url }}/${{ github.repository }}/actions/runs/${{ github.run_id }})了解详情。'
            })
```

## 输入参数

| 输入 | 描述 | 必需 | 默认值 |
|-------|-------------|----------|---------|
| `prompt` | 要使用 iFlow CLI 执行的提示 | ✅ 是 | - |
| `api_key` | 用于认证的 iFlow API 密钥 | ✅ 是 | - |
| `settings_json` | 完整的 `~/.iflow/settings.json` 内容（JSON 字符串）。如果提供，将覆盖其他配置选项。 | ❌ 否 | - |
| `base_url` | iFlow API 的自定义基础 URL | ❌ 否 | `https://apis.iflow.cn/v1` |
| `model` | 要使用的模型名称 | ❌ 否 | `Qwen3-Coder` |
| `working_directory` | 运行 iFlow CLI 的工作目录 | ❌ 否 | `.` |
| `timeout` | iFlow CLI 执行超时时间（秒）（1-86400） | ❌ 否 | `86400` |
| `precmd` | 在运行 iFlow CLI 之前执行的 Shell 命令（例如 "npm install", "git fetch"） | ❌ 否 | `` |

## 输出参数

| 输出 | 描述 |
|--------|-------------|
| `result` | iFlow CLI 执行的输出 |
| `exit_code` | iFlow CLI 执行的退出代码 |

## 认证

### 获取 iFlow API 密钥

1. 在 [iflow.cn](https://iflow.cn) 注册 iFlow 账户
2. 转到您的个人资料设置或[点击这里](https://iflow.cn/?open=setting)
3. 在弹出对话框中点击"重置"以生成新的 API 密钥
4. 将 API 密钥添加到您的 GitHub 仓库 secrets 中，命名为 `IFLOW_API_KEY`

### 可用模型

[Models](https://platform.iflow.cn/docs/api-mode)

- `Qwen3-Coder`（默认）- 适用于代码分析和生成
- `Kimi-K2` - 适用于通用 AI 任务和更长的上下文
- `DeepSeek-V3` - 高级推理和问题解决
- 支持通过 OpenAI 兼容 API 的自定义模型

## 自定义配置

### 使用预执行命令

`precmd` 输入允许您在执行 iFlow CLI 之前运行 Shell 命令。这对于设置环境或安装 iFlow 命令所需的依赖项非常有用。

```yaml
- name: 带预执行命令的 iFlow
  uses: iflow-ai/iflow-cli-action@v2.0.0
  with:
    prompt: "在安装依赖项后分析此代码库"
    api_key: ${{ secrets.IFLOW_API_KEY }}
    precmd: |
      npm install
      git fetch origin main
```

#### 多行命令

您可以通过用换行符分隔来指定多个命令：

```yaml
precmd: |
  npm ci
  npm run build
```

### 使用自定义设置

对于需要完全控制 iFlow 配置的高级用户，您可以直接提供自定义的 `settings.json`：

```yaml
- name: 自定义 iFlow 配置
  uses: iflow-ai/iflow-cli-action@v2.0.0
  with:
    prompt: "使用自定义配置分析此代码库"
    api_key: ${{ secrets.IFLOW_API_KEY }}  # 仍需要用于基本验证
    settings_json: |
      {
        "theme": "Dark",
        "selectedAuthType": "iflow",
        "apiKey": "${{ secrets.IFLOW_API_KEY }}",
        "baseUrl": "https://custom-api.example.com/v1",
        "modelName": "custom-model",
        "searchApiKey": "${{ secrets.SEARCH_API_KEY }}",
        "customField": "customValue"
      }
```

当提供 `settings_json` 时，它优先于单个配置输入（`base_url`、`model` 等）。这允许您：

- 使用自定义认证类型
- 配置输入中不可用的附加字段
- 在多个工作流运行中维护复杂配置
- 支持自定义 API 端点和模型

**注意：** 仍需要 `api_key` 输入进行验证，但实际使用的 API 密钥将是您在 `settings_json` 中指定的密钥。

## 使用 MCP 服务器

[MCP (Model Context Protocol)](https://modelcontextprotocol.io) 允许 iFlow CLI 连接到外部工具和服务，扩展其超越 AI 模型交互的能力。您可以在工作流中配置 MCP 服务器，以启用代码搜索、数据库查询或自定义工具集成等功能。

### 示例：使用 DeepWiki MCP 服务器

以下示例演示了如何配置和使用 DeepWiki MCP 服务器以增强代码搜索功能：

```yaml
- name: 带 MCP 服务器的 iFlow CLI
  uses: iflow-ai/iflow-cli-action@v2.0.0
  with:
    prompt: "使用 @deepwiki 搜索如何使用 Skynet 构建游戏"
    api_key: ${{ secrets.IFLOW_API_KEY }}
    settings_json: |
      {
        "selectedAuthType": "iflow",
        "apiKey": "${{ secrets.IFLOW_API_KEY }}",
        "baseUrl": "https://apis.iflow.cn/v1",
        "modelName": "Qwen3-Coder",
        "searchApiKey": "${{ secrets.IFLOW_API_KEY }}",
        "mcpServers": {
          "deepwiki": {
            "command": "npx",
            "args": ["-y", "mcp-deepwiki@latest"]
          }
        }
      }
    model: "Qwen3-Coder"
    timeout: "1800"
    debug: "true"
```

在此示例中：

- `mcpServers` 配置定义了一个名为 `deepwiki` 的服务器
- 服务器通过 `npx -y mcp-deepwiki@latest` 执行
- 提示中使用 `@deepwiki` 引用服务器以利用其功能
- `searchApiKey` 用于 DeepWiki 服务的认证

### 何时使用 MCP 服务器

当您需要以下功能时，MCP 服务器特别有用：

- 增强的代码搜索和文档查找功能
- 与外部工具和服务的集成
- 访问专业知识库或数据库
- 扩展 iFlow CLI 功能的自定义工具

## 常见用例

有关这些用例的详细示例，请参阅 [examples](examples) 目录中的工作流文件。

### 代码分析和审查

代码审查工作流，请参见：
- [拉取请求代码审查](examples/code-review.yml) - 在 PR 打开/重新打开或通过评论触发时自动审查 PR
- [PR Review Killer](examples/pr-review-killer.yml) - 根据评论对 PR 实施更改

### 文档生成

文档生成工作流，请参见：
- [文档生成](examples/documentation.yml) - 自动从代码库生成技术文档

### 自动化测试建议

自动化测试建议可以类似代码审查模式实施。请参阅 [代码审查示例](examples/code-review.yml) 了解可通过修改提示适应测试建议的模板。

### 问题管理

问题管理工作流，请参见：
- [Issue Killer](examples/issue-killer.yml) - 基于 GitHub 问题自动实现功能
- [问题分类](examples/issue-triage.yaml) - 自动为新的 GitHub 问题添加标签
- [Issue Killer](examples/issue-killer.yml) - 基于 GitHub 问题实现功能

### 架构分析

可以使用带有自定义提示的 iFlow CLI Action 执行架构分析。请参阅 [文档示例](examples/documentation.yml) 了解可通过修改提示适应架构分析的模板。

## 故障排除

### 常见问题

**命令超时：** 为复杂操作增加 `timeout` 值

```yaml
timeout: "900"  # 15 分钟
```

**API 认证失败：** 验证您的 API 密钥是否正确设置在仓库 secrets 中

**工作目录未找到：** 确保路径存在且使用了 checkout 操作

```yaml
- uses: actions/checkout@v4  # 使用 iFlow 操作前必需
```

### 调试模式

通过设置环境变量启用详细日志记录：

```yaml
env:
  ACTIONS_STEP_DEBUG: true
```

## 贡献

欢迎贡献！请随时提交问题和拉取请求。

## GitHub 应用集成

本项目设计用于与 iFlow CLI GitHub 应用集成，以实现移动和桌面端的使用。但请注意，由于 API 速率限制的约束，iFlow CLI GitHub 应用目前暂不对外开放。我们正在努力在未来使其对公众开放。

目前，您可以通过以下方式触发 iFlow CLI 工作流：
- GitHub Actions（如示例中所示）
- GitHub CLI 命令
- 直接调用 iFlow 平台的 API

一旦 GitHub 应用对外开放，我们将更新此部分。

## 许可证

该项目根据 MIT 许可证授权 - 有关详细信息，请参见 [LICENSE](LICENSE) 文件。

## 相关链接

- [iFlow CLI](https://github.com/iflow-ai/iflow-cli) - 底层 CLI 工具
- [iFlow 平台](https://docs.iflow.cn/en/docs) - 官方文档
- [GitHub Actions 文档](https://docs.github.com/en/actions)
- [Gemini CLI GitHub Action](https://github.com/google-github-actions/run-gemini-cli)