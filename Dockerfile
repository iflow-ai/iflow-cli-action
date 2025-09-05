# Use Ubuntu 22.04 as base image
FROM ubuntu:22.04 AS runtime-base

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
        build-essential \
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
    # Clean up apt cache
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Set Go environment variables
ENV PATH=$PATH:/usr/local/go/bin
ENV GOROOT=/usr/local/go
ENV GOPATH=/go
ENV PATH=$PATH:$GOPATH/bin

# Create a non-root user with proper home directory
RUN groupadd -g 1001 iflow && \
    useradd -r -u 1001 -g iflow -m -d /home/iflow iflow

# Create .iflow directory for the non-root user and set permissions
RUN mkdir -p /home/iflow/.iflow && \
    chown -R iflow:iflow /home/iflow/.iflow

# Create npm global directory for the non-root user and set permissions
RUN mkdir -p /home/iflow/.npm-global && \
    chown -R iflow:iflow /home/iflow/.npm-global

# Ensure Go is in PATH for the runtime user
ENV PATH="/usr/local/go/bin:$PATH"

# Set npm global prefix for the runtime user
ENV npm_config_prefix=/home/iflow/.npm-global
ENV PATH="/home/iflow/.npm-global/bin:$PATH"

