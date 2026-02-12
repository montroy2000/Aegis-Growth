# Aegis Vault - Deployment Guide

## Prerequisites

- Rust 1.93.0+
- Solana CLI 1.18.26+
- Anchor CLI 0.32.1+
- Node.js 20+
- Sufficient SOL for deployment (~5 SOL recommended)

## Pre-Deployment Checklist

### Code Review
- [ ] All tests passing (`anchor test`)
- [ ] Code reviewed by team
- [ ] Security audit completed
- [ ] Oracle feeds verified (Pyth + Switchboard)
- [ ] Kamino integration tested on devnet

### Configuration
- [ ] Program ID generated and updated
- [ ] Oracle feed addresses configured
- [ ] Kamino program addresses configured
- [ ] Constants verified (leverage, health factor, etc.)

### Testing
- [ ] Unit tests: 100% pass rate
- [ ] Integration tests: All scenarios covered
- [ ] Devnet deployment tested
- [ ] Frontend integration tested
- [ ] Stress testing completed

## Deployment Steps

### 1. Generate Program Keypair

```bash
cd /Users/melvicsmith/Aegis-Growth/aegis-vault
solana-keygen new -o target/deploy/aegis_vault-keypair.json
```

### 2. Update Program ID

```bash
# Get the program ID
solana address -k target/deploy/aegis_vault-keypair.json

# Update in lib.rs
declare_id!("YOUR_PROGRAM_ID_HERE");

# Update in Anchor.toml
[programs.devnet]
aegis_vault = "YOUR_PROGRAM_ID_HERE"

[programs.mainnet]
aegis_vault = "YOUR_PROGRAM_ID_HERE"
```

### 3. Build the Program

```bash
anchor build
```

Verify the build:
```bash
ls -lh target/deploy/aegis_vault.so
# Should be ~100-200 KB
```

### 4. Deploy to Devnet

```bash
# Set cluster to devnet
solana config set --url devnet

# Airdrop SOL for deployment
solana airdrop 5

# Deploy
anchor deploy --provider.cluster devnet
```

### 5. Initialize Vault on Devnet

```bash
# Run initialization script
anchor run initialize-devnet
```

Or manually:
```typescript
const tx = await program.methods
  .initializeVault(
    15000, // 1.50x max leverage
    24000  // 2.40 health factor floor
  )
  .accounts({
    vault: vaultPda,
    authority: wallet.publicKey,
    usdcMint: DEVNET_USDC_MINT,
    vaultUsdc: vaultUsdcPda,
    pythUsdcFeed: DEVNET_PYTH_USDC_FEED,
    switchboardUsdcFeed: DEVNET_SWITCHBOARD_USDC_FEED,
    systemProgram: SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
  })
  .rpc();
```

### 6. Test on Devnet

```bash
# Run full test suite against devnet
anchor test --provider.cluster devnet

# Test with frontend
cd ../aegis-app
npm run dev
# Update .env to point to devnet program
```

### 7. Deploy to Mainnet

⚠️ **CRITICAL**: This is irreversible once upgrade authority is set to None.

```bash
# Set cluster to mainnet
solana config set --url mainnet-beta

# Ensure you have enough SOL
solana balance
# Need ~5 SOL for deployment

# Deploy
anchor deploy --provider.cluster mainnet-beta
```

### 8. Initialize Vault on Mainnet

```bash
# Run initialization script
anchor run initialize-mainnet
```

Use mainnet addresses:
- USDC Mint: `EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v`
- Pyth USDC/USD: `Gnt27xtC473ZT2Mw5u8wZ68Z3gULkSTb5DuxJy7eJotD`
- Switchboard USDC/USD: (get from Switchboard)

### 9. Verify Deployment

```bash
# Check program account
solana program show YOUR_PROGRAM_ID

# Verify upgrade authority
solana program show YOUR_PROGRAM_ID | grep "Authority"
```

### 10. Set Upgrade Authority to None (IMMUTABILITY)

⚠️ **POINT OF NO RETURN**: This makes the program permanently immutable.

```bash
# Final verification
anchor test --provider.cluster mainnet-beta

# Set authority to None
solana program set-upgrade-authority YOUR_PROGRAM_ID --final

# Verify
solana program show YOUR_PROGRAM_ID | grep "Authority"
# Should show: "Authority: none"
```

## Post-Deployment

### Verification
- [ ] Program is immutable (authority = none)
- [ ] Vault initialized correctly
- [ ] Oracle feeds working
- [ ] Frontend connected
- [ ] Monitoring set up

### Monitoring
- Set up alerts for:
  - Oracle staleness
  - Health factor drops
  - Large deposits/withdrawals
  - Rebalance failures

### Documentation
- [ ] Update README with program ID
- [ ] Document vault address
- [ ] Update frontend config
- [ ] Announce deployment

## Rollback Plan

If issues are discovered before setting authority to None:
1. Deploy new version with fixes
2. Migrate users to new program
3. Deprecate old program

After authority is set to None:
- **No rollback possible**
- Users can only withdraw
- New program must be deployed separately

## Emergency Procedures

If critical bug discovered after immutability:
1. Announce issue publicly
2. Guide users to withdraw funds
3. Deploy new fixed program
4. Migrate users manually

## Oracle Addresses

### Devnet
- Pyth USDC/USD: `5SSkXsEKQepHHAewytPVwdej4epN1nxgLVM84L4KXgy7`
- Switchboard USDC/USD: (configure)

### Mainnet
- Pyth USDC/USD: `Gnt27xtC473ZT2Mw5u8wZ68Z3gULkSTb5DuxJy7eJotD`
- Switchboard USDC/USD: (configure)

## Kamino Addresses

### Devnet
- Program: `KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD`
- USDC Reserve: (configure)

### Mainnet
- Program: `KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD`
- USDC Reserve: (configure)

## Cost Estimates

- Program deployment: ~2 SOL
- Vault initialization: ~0.5 SOL
- Testing transactions: ~0.5 SOL
- **Total**: ~3-5 SOL

## Timeline

1. **Week 1**: Devnet deployment and testing
2. **Week 2**: Security audit
3. **Week 3**: Mainnet deployment with upgrade authority
4. **Week 4**: Beta testing with TVL cap
5. **Week 5**: Set authority to None (immutability)

## Support

For deployment issues:
- Check Anchor docs: https://www.anchor-lang.com
- Solana Discord: https://discord.gg/solana
- Kamino Discord: https://discord.gg/kamino
