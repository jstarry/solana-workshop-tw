FROM gitpod/workspace-full

# Install custom tools, runtime, etc.
RUN sudo apt-get update \
    && sudo apt-get install -y \
        libssl-dev libudev-dev pkg-config zlib1g-dev llvm clang make \
    && sudo rm -rf /var/lib/apt/lists/*

RUN sh -c "$(curl -sSfL https://release.solana.com/v1.5.8/install)"
RUN export PATH=~/.local/share/solana/install/active_release/bin:$PATH
