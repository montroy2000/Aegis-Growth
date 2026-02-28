use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::state::Vault;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(
        init,
        payer = authority,
        space = Vault::LEN,
        seeds = [b"vault"],
        bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub usdc_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        seeds = [b"vault-usdc"],
        bump,
        token::mint = usdc_mint,
        token::authority = vault
    )]
    pub vault_usdc: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = authority,
        seeds = [b"share-mint"],
        bump,
        mint::decimals = 6,
        mint::authority = vault
    )]
    pub share_mint: Account<'info, Mint>,

    /// CHECK: Pyth USDC/USD price feed; validated by the oracle module at rebalance time.
    pub pyth_usdc_feed: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<InitializeVault>,
    max_leverage_bps: u16,
    hf_floor_bps: u16,
) -> Result<()> {
    require!(max_leverage_bps <= 20_000, ErrorCode::InvalidLeverage);    // Max 2.0x
    require!(hf_floor_bps >= 10_000, ErrorCode::InvalidHealthFactor);   // Min 1.0

    let vault = &mut ctx.accounts.vault;

    vault.authority = ctx.accounts.authority.key();
    vault.usdc_mint = ctx.accounts.usdc_mint.key();
    vault.vault_usdc = ctx.accounts.vault_usdc.key();
    vault.share_mint = ctx.accounts.share_mint.key();
    vault.pyth_usdc_feed = ctx.accounts.pyth_usdc_feed.key();

    vault.total_supplied = 0;
    vault.total_borrowed = 0;
    vault.total_shares = 0;
    vault.last_rebalance_slot = 0;
    vault.reexpansion_unlocked_at = 0;

    // Hard-coded constants from PRD §4.1
    vault.max_leverage_bps = max_leverage_bps;
    vault.hf_floor_bps = hf_floor_bps;
    vault.oracle_stale_slots = 150;
    vault.peg_warn_bps = 10;
    vault.peg_exit_bps = 25;
    vault.peg_panic_bps = 50;
    vault.cooldown_slots = 30_000;
    vault.reexpansion_delay_sec = 30_000;

    vault.bump = ctx.bumps.vault;

    msg!(
        "Vault initialized — max_leverage: {}bps, hf_floor: {}bps",
        max_leverage_bps,
        hf_floor_bps
    );

    Ok(())
}
