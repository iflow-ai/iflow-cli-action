# Use Ubuntu 22.04 as base image for builder stage
FROM ubuntu:22.04 AS builder

# Set noninteractive installation mode to avoid prompts
ENV DEBIAN_FRONTEND=noninteractive

# Install build dependencies and clean up in the same layer
RUN apt-get update -y && apt-get -y upgrade \
    && apt-get install -y \
    wget \
    curl \
    git \
    build-essential \
    libssl-dev \
    pkg-config \
    cmake \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Install Rust for building the application
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Copy the Rust source code
COPY . /workspace
WORKDIR /workspace

# Build the Rust binary
RUN cargo build --release

# Use Ubuntu 22.04 as base image for runtime stage
FROM ubuntu:22.04 AS runtime

# Set noninteractive installation mode to avoid prompts
ENV DEBIAN_FRONTEND=noninteractive

# Install runtime dependencies including Node.js, GitHub CLI, Go, and other tools
RUN apt-get update -y && apt-get -y upgrade \
    && apt-get install -y \
    wget \
    bash \
    curl \
    git \
    procps \
    ca-certificates \
    software-properties-common \
    libssl-dev \
    pkg-config \
    libtool \
    autoconf \
    libreadline-dev \
    cmake \
    libev-dev \
    python3 \
    unzip \
    lsb-core \
    iproute2 \
    iputils-ping \
    netcat-traditional \
    apt-transport-https \
    gnupg \
    lsb-release \
    file \
    vim \
    zlib1g-dev \
    ripgrep \
    && add-apt-repository ppa:xmake-io/xmake \
    && apt-get update -y \
    && apt install xmake linux-tools-generic google-perftools libgoogle-perftools-dev -y \
    # Install Node.js (newer LTS) so npm is available for later steps
    && curl -fsSL https://deb.nodesource.com/setup_24.x | bash - \
    && apt-get install -y nodejs \
    # Install GitHub CLI (gh)
    && curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg \
    && chmod go+r /usr/share/keyrings/githubcli-archive-keyring.gpg \
    && echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | tee /etc/apt/sources.list.d/github-cli.list > /dev/null \
    && apt-get update \
    && apt-get install -y gh \
    # Install Go for github-mcp-server
    && wget https://go.dev/dl/go1.24.7.linux-amd64.tar.gz \
    && tar -C /usr/local -xzf go1.24.7.linux-amd64.tar.gz \
    && rm go1.24.7.linux-amd64.tar.gz \
    # Pre-install iFlow CLI using npm package
    && npm install -g @iflow-ai/iflow-cli@latest \
    # Install uv - ultra-fast Python package manager
    && curl -LsSf https://astral.sh/uv/install.sh | sh \
    # Install github-mcp-server CLI tool
    && /usr/local/go/bin/go install github.com/github/github-mcp-server/cmd/github-mcp-server@latest \
    # Install Rust (runtime components only)
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain none \
    # Clean up apt cache and temporary files in the same layer
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Create .iflow directory
RUN mkdir -p /root/.iflow

ENV GOROOT=/usr/local/go
ENV GOPATH=/go
ENV PATH=$PATH:$GOPATH/bin
# Ensure Go is in PATH
# Set Rust environment variables
ENV PATH="/root/.cargo/bin:/usr/local/go/bin:${PATH}"

# Copy the compiled binary from builder stage
COPY --from=builder /workspace/target/release/iflow-cli-action /usr/local/bin/iflow-cli-action

# Set working directory for runtime
WORKDIR /github/workspace

# Set entrypoint
ENTRYPOINT ["/usr/local/bin/iflow-cli-action"]
