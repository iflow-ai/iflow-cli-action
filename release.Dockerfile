# Use the base image that already has Rust installed
FROM ghcr.io/iflow-ai/iflow-cli-action:main

# Copy the locally built binary
COPY target/release/iflow-cli-action /usr/local/bin/iflow-cli-action
RUN chmod +x /usr/local/bin/iflow-cli-action

# Set working directory for runtime
WORKDIR /github/workspace

# Set entrypoint
ENTRYPOINT ["/usr/local/bin/iflow-cli-action"]