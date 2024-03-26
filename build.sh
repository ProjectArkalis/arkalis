#!/bin/bash
echo "Building for $TARGETPLATFORM"
if [ "$TARGETPLATFORM" = "linux/arm64" ]; then 
    echo "Building for arm64"
    apt update && apt install -y gcc-aarch64-linux-gnu
    rustup target add aarch64-unknown-linux-gnu
    cargo build --release --target aarch64-unknown-linux-gnu
    echo "Compiled for arm64" >> /app/arch.txt
    cp /app/target/aarch64-unknown-linux-gnu/release/arkalis /app/arkalis
else 
    echo "Building for x86_64"
    cargo build --release
    echo "Compiled for x86_64" >> /app/arch.txt
    cp /app/target/release/arkalis /app/arkalis
fi