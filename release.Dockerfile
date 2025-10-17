# Use the base image that already has Go installed
FROM ghcr.io/iflow-ai/iflow-cli-action:main

# # Install Rust using rustup
# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# ENV PATH="/root/.cargo/bin:${PATH}"

ENV IFLOW_CLI_ACTION_VERSION=v2.0.0-beta.4
RUN wget https://github.com/iflow-ai/iflow-cli-action/releases/download/${IFLOW_CLI_ACTION_VERSION}/iflow-cli-action -O /usr/local/bin/iflow-cli-action \
    && chmod +x /usr/local/bin/iflow-cli-action

# Set working directory for runtime
WORKDIR /github/workspace

# Set entrypoint
ENTRYPOINT ["/usr/local/bin/iflow-cli-action"]