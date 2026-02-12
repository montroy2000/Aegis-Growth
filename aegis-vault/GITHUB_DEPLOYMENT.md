# GitHub Actions Deployment Guide

## Overview

This guide explains how to deploy the Aegis Vault smart contract using GitHub Actions, which provides a clean Linux build environment and avoids local macOS tooling issues.

---

## Prerequisites

1. **GitHub Repository**
   - Push your code to GitHub
   - Ensure the repository is accessible

2. **Solana Keypair**
   - A funded Solana wallet for deployment
   - At least 2 SOL for devnet deployment
   - At least 5 SOL for mainnet deployment

---

## Setup Instructions

### Step 1: Generate Deployer Keypair

If you don't have a deployer keypair yet:

```bash
# Generate new keypair
solana-keygen new --outfile deployer-keypair.json

# Get the public address
solana address -k deployer-keypair.json

# Fund it (devnet)
solana airdrop 2 $(solana address -k deployer-keypair.json) --url devnet
```

**For mainnet:** Transfer SOL from your wallet to the deployer address.

---

### Step 2: Add GitHub Secret

1. **Get the keypair content:**
   ```bash
   cat deployer-keypair.json
   ```
   
   Copy the entire JSON array (e.g., `[123,45,67,...]`)

2. **Add to GitHub:**
   - Go to your repository on GitHub
   - Click **Settings** → **Secrets and variables** → **Actions**
   - Click **New repository secret**
   - Name: `DEPLOYER_KEYPAIR`
   - Value: Paste the JSON array
   - Click **Add secret**

> [!WARNING]
> **Never commit `deployer-keypair.json` to Git!**
> 
> Add it to `.gitignore`:
> ```bash
> echo "deployer-keypair.json" >> .gitignore
> ```

---

### Step 3: Push Workflow to GitHub

```bash
cd /Users/melvicsmith/Aegis-Growth/aegis-vault

# Add the workflow file
git add .github/workflows/deploy-devnet.yml

# Commit
git commit -m "Add GitHub Actions deployment workflow"

# Push to GitHub
git push origin main
```

---

### Step 4: Run the Deployment

1. **Go to GitHub Actions:**
   - Navigate to your repository
   - Click the **Actions** tab

2. **Run the workflow:**
   - Click **Deploy to Devnet** in the left sidebar
   - Click **Run workflow** button
   - Select network: `devnet` or `mainnet-beta`
   - Click **Run workflow**

3. **Monitor progress:**
   - Click on the running workflow
   - Watch the build and deployment steps
   - Check for any errors

---

## Deployment Output

After successful deployment, you'll see:

```
🚀 Deployment Successful!

Network: devnet
Program ID: 5R3w89qTN5S1hF6cep8mAnRJT79M5s7RCyokwd7NcDKW

Next Steps:
1. Update frontend .env.local with the Program ID
2. Initialize the vault using the frontend deployment UI
3. Test deposit/withdraw functionality
```

---

## Update Frontend Configuration

After deployment, update your frontend:

```bash
cd /Users/melvicsmith/Aegis-Growth/aegis-app
```

Edit `.env.local`:

```env
NEXT_PUBLIC_SOLANA_NETWORK=devnet
NEXT_PUBLIC_RPC_ENDPOINT=https://api.devnet.solana.com
NEXT_PUBLIC_PROGRAM_ID=<PROGRAM_ID_FROM_GITHUB_ACTIONS>
```

Restart the frontend:

```bash
npm run dev
```

---

## Troubleshooting

### Error: "Low balance"

**Solution:** Fund your deployer address:

```bash
# Get deployer address
solana address -k deployer-keypair.json

# Airdrop (devnet only)
solana airdrop 2 <DEPLOYER_ADDRESS> --url devnet
```

### Error: "Program already deployed"

**Solution:** This is normal if redeploying. The workflow will upgrade the existing program.

### Error: "Anchor build failed"

**Solution:** Check the build logs for specific compilation errors. Common issues:
- Missing dependencies
- Syntax errors in Rust code
- Version mismatches

### Error: "Secret not found"

**Solution:** Verify you added `DEPLOYER_KEYPAIR` secret correctly in GitHub Settings.

---

## Security Best Practices

1. **Separate Keypairs:**
   - Use different keypairs for devnet and mainnet
   - Never reuse mainnet keypairs for testing

2. **Rotate Secrets:**
   - Periodically generate new deployer keypairs
   - Update GitHub secrets accordingly

3. **Limit Permissions:**
   - Only fund deployer with necessary SOL
   - Transfer excess SOL back to main wallet

4. **Audit Logs:**
   - Review GitHub Actions logs regularly
   - Monitor deployment transactions on Solana Explorer

---

## Mainnet Deployment Checklist

Before deploying to mainnet:

- [ ] Thoroughly test on devnet
- [ ] Complete security audit
- [ ] Verify build reproducibility
- [ ] Review all oracle configurations
- [ ] Test with real USDC on devnet
- [ ] Prepare multi-sig for upgrade authority
- [ ] Document emergency procedures
- [ ] Set up monitoring and alerts

---

## Next Steps

After successful deployment:

1. **Initialize Vault:**
   - Use the frontend deployment UI at `http://localhost:3000/deploy`
   - Set initial parameters (max leverage, health factor floor)

2. **Test Functionality:**
   - Connect wallet on devnet
   - Test deposit
   - Test withdraw
   - Verify oracle price feeds

3. **Monitor:**
   - Check Solana Explorer for transactions
   - Monitor vault metrics
   - Watch for any errors

---

## Useful Commands

```bash
# Check deployment status
solana program show <PROGRAM_ID> --url devnet

# View program logs
solana logs <PROGRAM_ID> --url devnet

# Get program account info
solana account <PROGRAM_ID> --url devnet

# Check deployer balance
solana balance -k deployer-keypair.json --url devnet
```

---

## Support

If you encounter issues:

1. Check GitHub Actions logs for detailed error messages
2. Review Solana Explorer for transaction details
3. Verify all prerequisites are met
4. Ensure sufficient SOL balance for deployment

For persistent issues, review the build logs and error messages carefully.
