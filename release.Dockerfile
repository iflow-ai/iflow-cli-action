# Use official Go 1.24.4 image for building
FROM golang:1.24.4-bullseye AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y git ca-certificates curl

# Set working directory
WORKDIR /app

# Copy go mod files first for better layer caching
COPY go.mod go.sum ./

# Download dependencies
RUN go mod download

# Copy source code
COPY main.go ./
COPY cmd/ ./cmd/

# Build the application
RUN CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -o iflow-action .

# Final stage - copy Go binary to Ubuntu runtime
FROM ghcr.io/iflow-ai/iflow-cli-action:main

USER root
# Set working directory
WORKDIR /github/workspace

# Copy the binary from builder stage
COPY --from=builder /app/iflow-action /usr/local/bin/iflow-action

# Make sure binary is executable
RUN chmod +x /usr/local/bin/iflow-action

# Switch to non-root user
USER iflow

RUN sudo apt-get update \
    && sudo apt-get install -y gh \
    # Install Go for github-mcp-server
    && wget https://go.dev/dl/go1.23.2.linux-amd64.tar.gz \
    && tar -C /usr/local -xzf go1.23.2.linux-amd64.tar.gz \
    && rm go1.23.2.linux-amd64.tar.gz \
    # Pre-install iFlow CLI using npm package
    && npm install -g @iflow-ai/iflow-cli@latest \
    # Install uv - ultra-fast Python package manager
    && curl -LsSf https://astral.sh/uv/install.sh | sh \
    # Install github-mcp-server CLI tool
    && /usr/local/go/bin/go install github.com/github/github-mcp-server/cmd/github-mcp-server@latest \
    # Clean up apt cache
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Set entrypoint
ENTRYPOINT ["/usr/local/bin/iflow-action"]

