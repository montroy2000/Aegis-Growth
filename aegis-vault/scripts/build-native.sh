#!/bin/bash

# Alternative build script that works without cargo-build-sbf
# Uses Anchor's legacy build system

set -e

echo "🔨 Building Aegis Vault (Alternative Method)..."

# Check for Anchor
if ! command -v anchor &> /dev/null; then
    # Try with full path
    if [ -f "$HOME/.cargo/bin/anchor" ]; then
        export PATH="$HOME/.cargo/bin:$PATH"
    else
        echo "❌ Anchor CLI not found"
        exit 1
    fi
fi

echo "✓ Anchor version: $(anchor --version | head -1)"

# Clean previous build
echo "🧹 Cleaning previous build..."
cargo clean 2>/dev/null || true
rm -rf target/deploy/*.so 2>/dev/null || true

# Build using cargo directly (without BPF)
echo "🔧 Building with cargo (Rust native)..."
cd programs/aegis-vault
cargo build --release

echo ""
echo "✅ Build complete!"
echo ""
echo "Note: This builds a native binary, not a Solana BPF program."
echo "To deploy to Solana, you need cargo-build-sbf."
echo ""
echo "Workaround options:"
echo "1. Build on a Linux machine with full Solana toolchain"
echo "2. Use Solana Playground (https://beta.solpg.io)"
echo "3. Use GitHub Actions CI to build"
