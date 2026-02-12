#!/bin/bash

# GitHub Actions Deployment Setup Script
# This script helps you set up GitHub Actions for Solana deployment

set -e

echo "🚀 GitHub Actions Deployment Setup"
echo "===================================="
echo ""

# Check if we're in the right directory
if [ ! -f "Anchor.toml" ]; then
    echo "❌ Error: Not in aegis-vault directory"
    echo "Please run this from /Users/melvicsmith/Aegis-Growth/aegis-vault"
    exit 1
fi

echo "✅ In correct directory"
echo ""

# Check if deployer keypair exists
if [ ! -f "deployer-keypair.json" ]; then
    echo "❌ Error: deployer-keypair.json not found"
    echo "Run: solana-keygen new --outfile deployer-keypair.json"
    exit 1
fi

echo "✅ Deployer keypair found"
echo ""

# Get deployer address
DEPLOYER_ADDRESS=$(solana address -k deployer-keypair.json)
echo "📍 Deployer Address: $DEPLOYER_ADDRESS"
echo ""

# Check balance
echo "💰 Checking balance..."
BALANCE=$(solana balance $DEPLOYER_ADDRESS --url devnet 2>/dev/null || echo "0")
echo "   Balance: $BALANCE"
echo ""

# Check if funded
if [[ "$BALANCE" == "0 SOL" ]] || [[ "$BALANCE" == "0" ]]; then
    echo "⚠️  Wallet needs funding!"
    echo ""
    echo "Run this command to fund it:"
    echo "   solana airdrop 2 $DEPLOYER_ADDRESS --url devnet"
    echo ""
    read -p "Fund now? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        solana airdrop 2 $DEPLOYER_ADDRESS --url devnet
        echo "✅ Funded!"
    fi
fi

echo ""
echo "📋 Next Steps:"
echo "=============="
echo ""
echo "1. Copy the deployer keypair content:"
echo "   cat deployer-keypair.json | pbcopy"
echo ""
echo "2. Add GitHub Secret:"
echo "   - Go to: https://github.com/YOUR_USERNAME/YOUR_REPO/settings/secrets/actions"
echo "   - Click 'New repository secret'"
echo "   - Name: DEPLOYER_KEYPAIR"
echo "   - Value: Paste the keypair JSON"
echo ""
echo "3. Push the workflow to GitHub:"
echo "   git add .github/workflows/deploy-devnet.yml"
echo "   git add GITHUB_DEPLOYMENT.md"
echo "   git commit -m 'Add GitHub Actions deployment'"
echo "   git push origin main"
echo ""
echo "4. Run the deployment:"
echo "   - Go to GitHub Actions tab"
echo "   - Click 'Deploy to Devnet'"
echo "   - Click 'Run workflow'"
echo "   - Select 'devnet'"
echo "   - Click 'Run workflow'"
echo ""
echo "✅ Setup complete!"
