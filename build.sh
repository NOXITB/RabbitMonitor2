#!/bin/bash

set -e  # Exit on any error

if ! command -v rustup &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

. "$HOME/.cargo/env"

brew install FiloSottile/musl-cross/musl-cross


echo "Adding targets..."
rustup target remove x86_64-unknown-linux-musl
rustup target add x86_64-unknown-linux-musl
rustup target remove aarch64-unknown-linux-musl
rustup target add aarch64-unknown-linux-musl

# Build for AMD64
echo "Building for AMD64..."
cargo build --release --target x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/Blstmomonitor ./blstmomonitor-amd64

# Build for ARM64
echo "Building for ARM64..."
cargo build --release --target aarch64-unknown-linux-musl
cp target/aarch64-unknown-linux-musl/release/Blstmomonitor ./blstmomonitor-arm64

echo "Build complete!"
ls -lh blstmomonitor-*