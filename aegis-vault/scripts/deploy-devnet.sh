#!/bin/bash
# Deploy script for devnet

set -e

echo "🚀 Deploying Aegis Vault to Devnet..."

# Set cluster to devnet
solana config set --url devnet

# Check SOL balance
BALANCE=$(solana balance | awk '{print $1}')
echo "💰 Current balance: $BALANCE SOL"

if (( $(echo "$BALANCE < 5" | bc -l) )); then
    echo "⚠️  Low balance. Requesting airdrop..."
    solana airdrop 5
fi

# Deploy
echo "📤 Deploying program..."
anchor deploy --provider.cluster devnet

# Get program ID
PROGRAM_ID=$(solana address -k target/deploy/aegis_vault-keypair.json)
echo ""
echo "✅ Deployment successful!"
echo ""
echo "📋 Program Details:"
echo "   Program ID: $PROGRAM_ID"
echo "   Network: Devnet"
echo ""
echo "Next steps:"
echo "   1. Update Anchor.toml with program ID"
echo "   2. Update lib.rs with declare_id!(\"$PROGRAM_ID\")"
echo "   3. Rebuild: anchor build"
echo "   4. Initialize vault: anchor run initialize-devnet"
