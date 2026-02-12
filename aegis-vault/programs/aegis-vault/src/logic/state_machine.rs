use anchor_lang::prelude::*;
use crate::state::Vault;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum VaultState {
    Idle,
    Loop,
    Contract,
    Exit,
    Panic,
}

/// Determine the vault's current state based on conditions
pub fn determine_vault_state(
    peg_bps: u16,
    oracle_stale: bool,
    oracle_conflict: bool,
    health_factor_bps: u16,
    vault: &Vault,
) -> VaultState {
    // PANIC: Oracle issues or severe depeg
    if oracle_stale || oracle_conflict || peg_bps > vault.peg_panic_bps {
        return VaultState::Panic;
    }
    
    // EXIT: Major depeg
    if peg_bps > vault.peg_exit_bps {
        return VaultState::Exit;
    }
    
    // CONTRACT: Moderate depeg or low health factor
    if peg_bps > vault.peg_warn_bps || health_factor_bps < vault.hf_floor_bps {
        return VaultState::Contract;
    }
    
    // LOOP: Healthy conditions
    if peg_bps < vault.peg_warn_bps && health_factor_bps >= vault.hf_floor_bps {
        return VaultState::Loop;
    }
    
    // Default: IDLE
    VaultState::Idle
}

/// Calculate peg deviation in basis points
pub fn calculate_peg_bps(price: i64) -> u16 {
    let target = 1_000_000i64; // $1.00 in 6 decimals
    let deviation = (target - price).abs();
    ((deviation * 10000) / target) as u16
}
