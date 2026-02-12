# Aegis Vault - Testing Guide

## Overview

This guide covers testing strategies for the Aegis Vault smart contract.

## Test Categories

### 1. Unit Tests

Test individual functions in isolation.

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_state_determination() {
        // Test LOOP state
        let state = determine_vault_state(5, false, false, 300, &mock_vault());
        assert_eq!(state, VaultState::Loop);
        
        // Test CONTRACT state
        let state = determine_vault_state(15, false, false, 300, &mock_vault());
        assert_eq!(state, VaultState::Contract);
        
        // Test EXIT state
        let state = determine_vault_state(30, false, false, 300, &mock_vault());
        assert_eq!(state, VaultState::Exit);
        
        // Test PANIC state
        let state = determine_vault_state(60, false, false, 300, &mock_vault());
        assert_eq!(state, VaultState::Panic);
    }
    
    #[test]
    fn test_share_calculation() {
        // First deposit: 1:1
        let shares = calculate_shares(1000, 0, 0);
        assert_eq!(shares, 1000);
        
        // Second deposit with existing shares
        let shares = calculate_shares(1000, 1000, 1000);
        assert_eq!(shares, 1000);
        
        // Deposit after profit
        let shares = calculate_shares(1000, 1000, 1100);
        assert_eq!(shares, 909); // 1000 * 1000 / 1100
    }
}
```

### 2. Integration Tests

Test full instruction flows.

```typescript
// tests/aegis-vault.ts
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AegisVault } from "../target/types/aegis_vault";

describe("aegis-vault", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  
  const program = anchor.workspace.AegisVault as Program<AegisVault>;
  
  it("Initializes vault", async () => {
    const tx = await program.methods
      .initializeVault(15000, 24000)
      .accounts({
        // ... accounts
      })
      .rpc();
    
    const vault = await program.account.vault.fetch(vaultPda);
    assert.equal(vault.maxLeverageBps, 15000);
    assert.equal(vault.hfFloorBps, 24000);
  });
  
  it("Handles deposit", async () => {
    const depositAmount = 1000_000_000; // 1000 USDC
    
    const tx = await program.methods
      .deposit(new anchor.BN(depositAmount))
      .accounts({
        // ... accounts
      })
      .rpc();
    
    const userPosition = await program.account.userPosition.fetch(positionPda);
    assert.equal(userPosition.shares.toNumber(), depositAmount);
  });
  
  it("Handles withdrawal", async () => {
    const withdrawShares = 500_000_000; // 500 shares
    
    const tx = await program.methods
      .withdraw(new anchor.BN(withdrawShares))
      .accounts({
        // ... accounts
      })
      .rpc();
    
    const userPosition = await program.account.userPosition.fetch(positionPda);
    assert.equal(userPosition.shares.toNumber(), 500_000_000);
  });
});
```

### 3. State Machine Tests

Test all state transitions.

```typescript
describe("State Machine", () => {
  it("Transitions to LOOP when healthy", async () => {
    // Mock oracle prices at $1.00
    // Mock health factor at 3.12
    // Expect LOOP state
  });
  
  it("Transitions to CONTRACT on depeg", async () => {
    // Mock oracle prices at $0.989 (11 bps)
    // Expect CONTRACT state
  });
  
  it("Transitions to EXIT on major depeg", async () => {
    // Mock oracle prices at $0.974 (26 bps)
    // Expect EXIT state
  });
  
  it("Transitions to PANIC on oracle failure", async () => {
    // Mock stale oracle
    // Expect PANIC state
  });
});
```

### 4. Edge Case Tests

Test boundary conditions and error cases.

```typescript
describe("Edge Cases", () => {
  it("Rejects rebalance during cooldown", async () => {
    await program.methods.rebalance().rpc();
    
    // Try to rebalance again immediately
    try {
      await program.methods.rebalance().rpc();
      assert.fail("Should have thrown");
    } catch (err) {
      assert.include(err.message, "RebalanceCooldown");
    }
  });
  
  it("Rejects withdrawal exceeding shares", async () => {
    const userPosition = await program.account.userPosition.fetch(positionPda);
    const excessiveShares = userPosition.shares.add(new anchor.BN(1));
    
    try {
      await program.methods.withdraw(excessiveShares).rpc();
      assert.fail("Should have thrown");
    } catch (err) {
      assert.include(err.message, "InsufficientEquity");
    }
  });
  
  it("Handles zero deposits correctly", async () => {
    try {
      await program.methods.deposit(new anchor.BN(0)).rpc();
      assert.fail("Should have thrown");
    } catch (err) {
      // Should reject zero deposits
    }
  });
});
```

### 5. Oracle Tests

Test oracle integration and failure modes.

```typescript
describe("Oracle Integration", () => {
  it("Fetches prices from both oracles", async () => {
    // Verify Pyth price is fetched
    // Verify Switchboard price is fetched
    // Verify average is calculated correctly
  });
  
  it("Detects oracle staleness", async () => {
    // Mock stale Pyth feed
    // Expect PANIC state
  });
  
  it("Detects oracle conflict", async () => {
    // Mock Pyth at $1.00
    // Mock Switchboard at $0.98
    // Expect PANIC state (>15 bps deviation)
  });
});
```

## Running Tests

```bash
# Run all tests
anchor test

# Run specific test file
anchor test --skip-deploy -- --test aegis-vault

# Run with logs
anchor test -- --nocapture

# Run on devnet
anchor test --provider.cluster devnet
```

## Test Coverage

Aim for:
- **Unit tests**: 100% of logic functions
- **Integration tests**: All instruction paths
- **Edge cases**: All error conditions
- **State machine**: All transitions

## Continuous Integration

Add to `.github/workflows/test.yml`:
```yaml
name: Test
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: anchor test
```

## Pre-Deployment Checklist

Before deploying to mainnet:
- [ ] All unit tests pass
- [ ] All integration tests pass
- [ ] All edge cases covered
- [ ] State machine tested thoroughly
- [ ] Oracle integration tested
- [ ] Kamino integration tested
- [ ] Fuzz testing completed
- [ ] Security audit completed
