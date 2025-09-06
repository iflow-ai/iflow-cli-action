# Use the base image that already has Go installed
FROM ghcr.io/iflow-ai/iflow-cli-action:main

# Set working directory
WORKDIR /app

# Copy go mod files first for better layer caching
COPY go.mod go.sum ./
# Copy source code
COPY main.go ./
COPY cmd/ ./cmd/

# Build the application
RUN go install github.com/github/github-mcp-server/cmd/github-mcp-server@latest \
    && go mod download \
    && CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -o iflow-action . \
    && cp iflow-action /usr/local/bin/iflow-action \
    && chmod +x /usr/local/bin/iflow-action

# Set working directory
WORKDIR /github/workspace
# Set entrypoint
ENTRYPOINT ["/usr/local/bin/iflow-action"]