#!/bin/bash
# Build script for Aegis Vault smart contract

set -e

echo "🔨 Building Aegis Vault Smart Contract..."

# Check if Anchor is installed
if ! command -v anchor &> /dev/null; then
    echo "❌ Anchor CLI not found. Please install it first:"
    echo "   cargo install --git https://github.com/coral-xyz/anchor avm --locked --force"
    echo "   avm install latest"
    echo "   avm use latest"
    exit 1
fi

# Check Anchor version
ANCHOR_VERSION=$(anchor --version | cut -d' ' -f2)
echo "✓ Anchor version: $ANCHOR_VERSION"

# Check if cargo-build-sbf is available
if ! command -v cargo-build-sbf &> /dev/null; then
    echo "⚠️  cargo-build-sbf not found. Attempting to use Solana's build tools..."
    
    # Check if solana is installed
    if ! command -v solana &> /dev/null; then
        echo "❌ Solana CLI not found. Please install it:"
        echo "   sh -c \"\$(curl -sSfL https://release.solana.com/stable/install)\""
        exit 1
    fi
    
    # Add Solana bin to PATH
    export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
fi

# Clean previous build
echo "🧹 Cleaning previous build..."
rm -rf target/deploy/*.so target/idl/*.json 2>/dev/null || true

# Build the program
echo "🔧 Building program..."
anchor build

# Check if build was successful
if [ -f "target/deploy/aegis_vault.so" ]; then
    SIZE=$(ls -lh target/deploy/aegis_vault.so | awk '{print $5}')
    echo "✅ Build successful! Program size: $SIZE"
    echo ""
    echo "📦 Artifacts:"
    echo "   Program: target/deploy/aegis_vault.so"
    echo "   IDL: target/idl/aegis_vault.json"
    echo ""
    echo "Next steps:"
    echo "   1. Run tests: ./scripts/test.sh"
    echo "   2. Deploy to devnet: ./scripts/deploy-devnet.sh"
else
    echo "❌ Build failed. Check errors above."
    exit 1
fi
