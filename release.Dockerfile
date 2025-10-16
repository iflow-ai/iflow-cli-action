# Use the base image that already has Go installed
FROM ghcr.io/iflow-ai/iflow-cli-action:main

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Set working directory
WORKDIR /app
# Copy all source code
COPY . .

# Build dependencies only (for better caching)
RUN cargo build --release \
    && rm -rf target/release/iflow-cli-action*

# Build the application
RUN cargo build --release

# Install the binary and set permissions
RUN cp target/release/iflow-cli-action /usr/local/bin/iflow-cli-action \
    && chmod +x /usr/local/bin/iflow-cli-action \
    && rm -rf /app/*

# Set working directory for runtime
WORKDIR /github/workspace

# Set entrypoint
ENTRYPOINT ["/usr/local/bin/iflow-cli-action"]