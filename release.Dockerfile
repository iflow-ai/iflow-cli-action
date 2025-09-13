# Use the base image that already has Go installed
FROM ghcr.io/iflow-ai/iflow-cli-action:main

# Set Go environment variables (ensure they're set correctly)
ENV PATH=$PATH:/usr/local/go/bin
ENV GOROOT=/usr/local/go
ENV GOPATH=/go
ENV PATH=$PATH:$GOPATH/bin

# Set working directory
WORKDIR /app

# Copy go mod files first for better layer caching
COPY go.mod go.sum ./
RUN go mod download

# Copy all source code
COPY main.go ./
COPY cmd/ ./cmd/
COPY internal/ ./internal/

# Build the application
RUN CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build \
    -ldflags='-w -s -extldflags "-static"' \
    -a \
    -o iflow-action .

# Install the binary and set permissions
RUN cp iflow-action /usr/local/bin/iflow-action \
    && chmod +x /usr/local/bin/iflow-action \
    && rm -rf /app/*

# Set working directory for runtime
WORKDIR /github/workspace

# Set entrypoint
ENTRYPOINT ["/usr/local/bin/iflow-action"]