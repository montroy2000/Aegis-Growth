#!/bin/bash
# Test script for Aegis Vault smart contract

set -e

echo "🧪 Testing Aegis Vault Smart Contract..."

# Check if dependencies are installed
if [ ! -d "node_modules" ]; then
    echo "📦 Installing test dependencies..."
    npm install
fi

# Run Anchor tests
echo "🔬 Running Anchor tests..."
anchor test

echo "✅ All tests passed!"
