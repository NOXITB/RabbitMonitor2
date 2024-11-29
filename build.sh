#!/bin/bash

set -e

# Install dependencies
apt-get update
apt-get install -y \
    build-essential \
    musl-tools \
    curl

# Install Rust if needed
if ! command -v rustup &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    . "$HOME/.cargo/env"
fi

# Add targets
rustup target add x86_64-unknown-linux-musl
rustup target add aarch64-unknown-linux-musl

# Configure cargo
mkdir -p .cargo
cat > .cargo/config.toml << EOF
[target.x86_64-unknown-linux-musl]
linker = "musl-gcc"

[target.aarch64-unknown-linux-musl]
linker = "aarch64-linux-gnu-gcc"
EOF

# Build AMD64
RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/Blstmomonitor ./blstmomonitor-amd64

# Build ARM64
RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target aarch64-unknown-linux-musl
cp target/aarch64-unknown-linux-musl/release/Blstmomonitor ./blstmomonitor-arm64

echo "Build complete!"
ls -lh blstmomonitor-*