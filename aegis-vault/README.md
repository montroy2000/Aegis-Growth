# Aegis Vault - Solana Smart Contract

## Overview

Immutable delta-neutral yield optimizer on Solana. This program implements a state machine that automatically manages leverage based on stablecoin peg health and oracle reliability.

## Architecture

```
aegis-vault/
├── programs/aegis-vault/src/
│   ├── lib.rs                    # Program entry point
│   ├── errors.rs                 # Error definitions
│   ├── state/
│   │   ├── vault.rs              # Vault account structure
│   │   └── user_position.rs      # User position tracking
│   ├── logic/
│   │   └── state_machine.rs      # State determination logic
│   ├── instructions/
│   │   ├── initialize_vault.rs   # Vault initialization
│   │   ├── deposit.rs            # User deposits
│   │   ├── withdraw.rs           # User withdrawals
│   │   └── rebalance.rs          # Keeper rebalancing
│   ├── oracles/
│   │   ├── pyth.rs               # Pyth price feeds (TODO)
│   │   └── switchboard.rs        # Switchboard feeds (TODO)
│   └── lending/
│       └── kamino.rs             # Kamino integration (TODO)
├── Anchor.toml                   # Anchor configuration
└── Cargo.toml                    # Workspace configuration
```

## State Machine

The vault operates in 5 states based on peg deviation and health factor:

| State | Trigger | Action |
|-------|---------|--------|
| **LOOP** | Peg < 10bps, HF ≥ 2.40 | Increase leverage to 1.50x |
| **CONTRACT** | Peg > 10bps OR HF < 2.40 | Reduce leverage by 10% |
| **EXIT** | Peg > 25bps | Emergency unwind all debt |
| **PANIC** | Peg > 50bps OR Oracle issues | Freeze (withdrawals only) |
| **IDLE** | Default | No action |

## Instructions

### Initialize Vault
```rust
initialize_vault(
    max_leverage_bps: u16,  // e.g., 15000 for 1.50x
    hf_floor_bps: u16,      // e.g., 24000 for 2.40
)
```

### Deposit
```rust
deposit(amount: u64)  // USDC amount in lamports
```

### Withdraw
```rust
withdraw(shares: u64)  // Vault shares to burn
```

### Rebalance
```rust
rebalance()  // Permissionless keeper action
```

## Building

```bash
# Install dependencies
anchor build

# Run tests
anchor test

# Deploy to devnet
anchor deploy --provider.cluster devnet
```

## Testing

```bash
# Unit tests
cargo test

# Integration tests with Anchor
anchor test
```

## Deployment Checklist

- [ ] Deploy to devnet
- [ ] Initialize vault with correct parameters
- [ ] Test all instructions
- [ ] Integrate with frontend
- [ ] Security audit
- [ ] Deploy to mainnet
- [ ] **Set upgrade authority to None** (immutability)

## Constants (from PRD)

```rust
MAX_LEVERAGE: 1.50x (15000 bps)
HF_FLOOR: 2.40 (24000 bps)
ORACLE_STALE_SLOTS: 150
PEG_WARN_BPS: 10
PEG_EXIT_BPS: 25
PEG_PANIC_BPS: 50
COOLDOWN_SLOTS: 30,000 (~3.3 hours)
REEXPANSION_DELAY: 30,000 seconds (~8.3 hours)
```

## TODO

- [ ] Implement Pyth oracle integration
- [ ] Implement Switchboard oracle integration
- [ ] Implement Kamino CPI calls (supply, borrow, repay, withdraw)
- [ ] Add health factor queries from Kamino
- [ ] Implement keeper fee distribution (0.15%)
- [ ] Add comprehensive tests
- [ ] Add program documentation

## Security

⚠️ **CRITICAL**: Once deployed to mainnet with upgrade authority set to `None`, the code becomes **permanently immutable**. All logic must be thoroughly tested and audited before this step.

## License

MIT
