# Quick Start: Deploy via GitHub Actions Extension

## Prerequisites Checklist

- [x] GitHub Actions extension installed in VS Code
- [x] Deployer keypair generated (`deployer-keypair.json`)
- [x] Deployer keypair copied to clipboard
- [ ] Deployer wallet funded with 2 SOL
- [ ] GitHub secret `DEPLOYER_KEYPAIR` added
- [ ] Code pushed to GitHub

---

## Step 1: Fund Deployer Wallet

**Deployer Address:**
```
DDQNaUakLRMXP4MgQGEL3tpX1ST1CCW6EGEBZdUYazNH
```

**Option A: Web Faucet (Recommended)**
1. Visit: https://faucet.solana.com/
2. Paste address: `DDQNaUakLRMXP4MgQGEL3tpX1ST1CCW6EGEBZdUYazNH`
3. Click "Request Airdrop"
4. Wait for confirmation

**Option B: CLI**
```bash
solana airdrop 2 DDQNaUakLRMXP4MgQGEL3tpX1ST1CCW6EGEBZdUYazNH --url devnet
```

**Verify Balance:**
```bash
solana balance DDQNaUakLRMXP4MgQGEL3tpX1ST1CCW6EGEBZdUYazNH --url devnet
```

---

## Step 2: Add GitHub Secret

The deployer keypair is already in your clipboard!

### Using VS Code:

1. **Open Command Palette** (Cmd+Shift+P)
2. Type: `GitHub Actions: Add Secret`
3. Select your repository
4. **Name:** `DEPLOYER_KEYPAIR`
5. **Value:** Paste from clipboard (Cmd+V)
6. Save

### Using GitHub Web:

1. Go to: `https://github.com/YOUR_USERNAME/Aegis-Growth/settings/secrets/actions`
2. Click **"New repository secret"**
3. **Name:** `DEPLOYER_KEYPAIR`
4. **Value:** Paste from clipboard
5. Click **"Add secret"**

---

## Step 3: Push to GitHub

```bash
cd /Users/melvicsmith/Aegis-Growth/aegis-vault

# Stage files
git add .github/workflows/deploy-devnet.yml
git add GITHUB_DEPLOYMENT.md
git add setup-github-deploy.sh
git add .gitignore

# Commit
git commit -m "Add GitHub Actions deployment workflow for Solana"

# Push
git push origin main
```

---

## Step 4: Run Deployment via VS Code

### Using GitHub Actions Extension:

1. **Open GitHub Actions panel:**
   - Click the GitHub Actions icon in the sidebar
   - Or: View → Open View → GitHub Actions

2. **Find the workflow:**
   - Expand your repository
   - Look for **"Deploy to Devnet"**

3. **Run the workflow:**
   - Click on "Deploy to Devnet"
   - Click the **"Run workflow"** button
   - Select **network: devnet**
   - Click **"Run"**

4. **Monitor progress:**
   - Watch the workflow steps execute
   - Check for any errors
   - Wait for completion (~5-10 minutes)

### Alternative: Using Command Palette

1. **Cmd+Shift+P** → `GitHub Actions: Run Workflow`
2. Select: **Deploy to Devnet**
3. Choose: **devnet**
4. Confirm

---

## Step 5: Get Program ID

After successful deployment:

1. **Check workflow output:**
   - Look for the deployment summary
   - Find: `Program ID: <YOUR_PROGRAM_ID>`

2. **Copy the Program ID**

---

## Step 6: Update Frontend

Edit `/Users/melvicsmith/Aegis-Growth/aegis-app/.env.local`:

```env
NEXT_PUBLIC_SOLANA_NETWORK=devnet
NEXT_PUBLIC_RPC_ENDPOINT=https://api.devnet.solana.com
NEXT_PUBLIC_PROGRAM_ID=<PASTE_PROGRAM_ID_HERE>
```

Restart frontend:
```bash
cd /Users/melvicsmith/Aegis-Growth/aegis-app
npm run dev
```

---

## Step 7: Initialize Vault

1. **Open deployment UI:** http://localhost:3000/deploy
2. **Connect wallet** (ensure it's on devnet)
3. **Click "Initialize Vault"**
4. **Set parameters:**
   - Max Leverage: 300 (3x)
   - Health Factor Floor: 120 (1.2)
5. **Confirm transaction**
6. **Copy Vault Address**
7. **Update `.env.local`:**
   ```env
   NEXT_PUBLIC_VAULT_ADDRESS=<VAULT_ADDRESS>
   ```

---

## Step 8: Test!

1. Navigate to: http://localhost:3000
2. Test deposit
3. Test withdraw
4. Verify on Solana Explorer

---

## Troubleshooting

### Workflow fails with "Secret not found"
- Verify secret name is exactly: `DEPLOYER_KEYPAIR`
- Check secret value is the full JSON array
- Try re-adding the secret

### Workflow fails with "Insufficient balance"
- Check deployer balance
- Fund with at least 2 SOL
- Retry workflow

### Build errors
- Check the workflow logs
- Verify all Rust code compiles
- Review error messages

---

## Quick Commands

```bash
# Check if keypair is in clipboard
pbpaste | head -c 50

# Re-copy keypair to clipboard
cat deployer-keypair.json | pbcopy

# Check deployer balance
solana balance DDQNaUakLRMXP4MgQGEL3tpX1ST1CCW6EGEBZdUYazNH --url devnet

# View git status
git status

# Push to GitHub
git push origin main
```

---

## Timeline

- Fund wallet: 2 minutes
- Add secret: 2 minutes
- Push to GitHub: 1 minute
- Run workflow: 5-10 minutes
- Initialize vault: 2 minutes
- Test: 5 minutes

**Total: ~15-25 minutes**

---

## What's Next?

After successful deployment:

1. ✅ Smart contract deployed to devnet
2. ✅ Frontend connected to program
3. ✅ Vault initialized
4. ✅ Ready for testing

Then:
- Monitor vault performance
- Test all user flows
- Verify oracle integrations
- Prepare for mainnet (if applicable)

---

**Ready to deploy? Start with Step 1!** 🚀
