# Kamino Integration Guide

## Overview

The Aegis Vault integrates with Kamino Finance for lending and borrowing operations. This document outlines the integration points and implementation details.

## Required Kamino Accounts

### For Supply Operations
- `kamino_reserve`: The USDC reserve account
- `kamino_reserve_liquidity`: Reserve's liquidity supply
- `kamino_collateral_mint`: Collateral token mint (kUSDC)
- `vault_collateral`: Vault's collateral token account
- `kamino_lending_market`: The lending market account

### For Borrow Operations
- `kamino_reserve`: The USDC reserve account
- `kamino_reserve_liquidity`: Reserve's liquidity supply
- `kamino_obligation`: Vault's obligation account
- `kamino_lending_market`: The lending market account

## Integration Steps

### 1. Add Kamino Program Dependency

Add to `programs/aegis-vault/Cargo.toml`:
```toml
[dependencies]
kamino-lending = { git = "https://github.com/Kamino-Finance/klend", tag = "v1.0.0" }
```

### 2. Import Kamino Types

In `src/lending/kamino.rs`:
```rust
use kamino_lending::{
    cpi::accounts::*,
    cpi::*,
    state::*,
};
```

### 3. Update Rebalance Instruction

Add Kamino accounts to the `Rebalance` struct:
```rust
#[derive(Accounts)]
pub struct Rebalance<'info> {
    // ... existing accounts ...
    
    /// CHECK: Kamino program
    pub kamino_program: AccountInfo<'info>,
    
    pub kamino_reserve: Account<'info, Reserve>,
    
    #[account(mut)]
    pub kamino_reserve_liquidity: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub kamino_obligation: Account<'info, Obligation>,
    
    pub kamino_lending_market: Account<'info, LendingMarket>,
    
    #[account(mut)]
    pub vault_collateral: Account<'info, TokenAccount>,
    
    pub kamino_collateral_mint: Account<'info, Mint>,
}
```

### 4. Implement CPI Calls

Replace the placeholder functions in `kamino.rs` with actual CPI calls using the Kamino SDK.

## Health Factor Calculation

Kamino's health factor is calculated as:
```
health_factor = total_collateral_value / total_borrowed_value
```

A health factor of 2.40 means the collateral is worth 2.40x the borrowed amount.

## Leverage Calculation

Target leverage is achieved by:
1. Supplying initial USDC
2. Borrowing against it
3. Re-supplying the borrowed USDC
4. Repeating until target leverage is reached

For 1.50x leverage:
- If we have $1000 equity
- We supply $1000, borrow $500
- Supply the $500, now have $1500 supplied
- Total leverage = $1500 / $1000 = 1.50x

## Error Handling

Kamino operations can fail for several reasons:
- Insufficient liquidity in reserve
- Health factor too low
- Reserve is paused
- Obligation is unhealthy

Always check Kamino's return values and handle errors appropriately.

## Testing

Before mainnet deployment:
1. Test on devnet with Kamino's devnet deployment
2. Verify all CPI calls work correctly
3. Test edge cases (low liquidity, high utilization)
4. Verify health factor calculations match Kamino's

## Kamino Program IDs

- **Devnet**: `KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD`
- **Mainnet**: `KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD`

## Resources

- [Kamino Docs](https://docs.kamino.finance/)
- [Kamino GitHub](https://github.com/Kamino-Finance/klend)
- [Kamino SDK](https://www.npmjs.com/package/@kamino-finance/klend-sdk)
