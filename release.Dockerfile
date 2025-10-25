# Use the base image that already has Go、Cargo installed
FROM ghcr.io/iflow-ai/iflow-cli-action:main

ENV IFLOW_CLI_ACTION_VERSION=v2.1.0
RUN wget https://github.com/iflow-ai/iflow-cli-action/releases/download/${IFLOW_CLI_ACTION_VERSION}/iflow-cli-action -O /usr/local/bin/iflow-cli-action \
    && chmod +x /usr/local/bin/iflow-cli-action

# Set working directory for runtime
WORKDIR /github/workspace

# Set entrypoint
ENTRYPOINT ["/usr/local/bin/iflow-cli-action"]
