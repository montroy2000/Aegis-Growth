use anchor_lang::prelude::*;
use crate::state::Vault;
use crate::logic::{determine_vault_state, VaultState};
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct Rebalance<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub keeper: Signer<'info>,

    /// CHECK: Pyth USDC/USD price feed — validated inside fetch_oracle_data().
    pub pyth_feed: AccountInfo<'info>,

    // V2: Add Kamino lending accounts here when CPI is implemented.
}

pub fn handler(ctx: Context<Rebalance>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let clock = Clock::get()?;

    // --- Cooldown check ---
    require!(
        clock.slot >= vault.last_rebalance_slot + vault.cooldown_slots,
        ErrorCode::RebalanceCooldown
    );

    // --- Oracle read (Pyth only in v1) ---
    let oracle_data = crate::oracles::fetch_oracle_data(
        &ctx.accounts.pyth_feed,
        vault.oracle_stale_slots,
    )?;

    let oracle_quality_ok = crate::oracles::validate_oracle_quality(&ctx.accounts.pyth_feed)?;
    if !oracle_quality_ok {
        msg!("WARNING: Pyth confidence interval is wide — proceeding conservatively");
    }

    // --- Health factor (v1: placeholder 3.12×; v2: read from Kamino obligation) ---
    let health_factor_bps = crate::lending::kamino::get_health_factor(
        &ctx.accounts.pyth_feed, // placeholder account — unused in v1
    )?;

    // --- State determination ---
    let state = determine_vault_state(
        oracle_data.peg_deviation_bps,
        oracle_data.is_stale,
        oracle_data.has_conflict,
        health_factor_bps,
        vault,
    );

    // --- Execute state-specific logic ---
    match state {
        VaultState::Loop => {
            require!(
                clock.unix_timestamp >= vault.reexpansion_unlocked_at,
                ErrorCode::ReexpansionCooldown
            );
            // V2: invoke Kamino CPI to increase leverage toward max_leverage_bps.
            msg!("LOOP: Conditions met for leverage expansion (v2: Kamino CPI pending)");
        }
        VaultState::Contract => {
            vault.reexpansion_unlocked_at =
                clock.unix_timestamp + vault.reexpansion_delay_sec;
            // V2: invoke Kamino CPI to reduce leverage.
            msg!("CONTRACT: Peg deviation detected — marking re-expansion delay");
        }
        VaultState::Exit => {
            // V2: invoke Kamino CPI to unwind all leverage.
            msg!("EXIT: Emergency unwind triggered (v2: Kamino CPI pending)");
        }
        VaultState::Panic => {
            return Err(ErrorCode::VaultInPanicState.into());
        }
        VaultState::Idle => {
            msg!("IDLE: No action required");
        }
    }

    vault.last_rebalance_slot = clock.slot;

    // V2: Pay keeper fee (0.15 % of rebalanced amount).

    Ok(())
}
