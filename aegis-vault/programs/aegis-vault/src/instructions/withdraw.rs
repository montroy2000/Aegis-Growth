use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, Transfer};
use crate::state::{Vault, UserPosition};
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    
    #[account(
        mut,
        seeds = [b"user-position", user.key().as_ref()],
        bump = user_position.bump
    )]
    pub user_position: Account<'info, UserPosition>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        constraint = user_usdc.mint == vault.usdc_mint,
        constraint = user_usdc.owner == user.key()
    )]
    pub user_usdc: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        seeds = [b"vault-usdc"],
        bump
    )]
    pub vault_usdc: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        seeds = [b"share-mint"],
        bump
    )]
    pub share_mint: Account<'info, Mint>,
    
    #[account(
        mut,
        constraint = user_shares.mint == share_mint.key(),
        constraint = user_shares.owner == user.key()
    )]
    pub user_shares: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<Withdraw>, shares: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let user_position = &mut ctx.accounts.user_position;
    
    require!(user_position.shares >= shares, ErrorCode::InsufficientEquity);
    
    // Calculate USDC to return
    let equity = vault.equity();
    let usdc_amount = (shares as u128)
        .checked_mul(equity as u128)
        .and_then(|v| v.checked_div(vault.total_shares as u128))
        .ok_or(ErrorCode::MathOverflow)? as u64;
    
    require!(vault_usdc.amount >= usdc_amount, ErrorCode::InsufficientEquity);
    
    // Burn shares
    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.share_mint.to_account_info(),
                from: ctx.accounts.user_shares.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        shares,
    )?;
    
    // Transfer USDC to user
    let vault_seeds = &[b"vault".as_ref(), &[vault.bump]];
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault_usdc.to_account_info(),
                to: ctx.accounts.user_usdc.to_account_info(),
                authority: vault.to_account_info(),
            },
            &[vault_seeds],
        ),
        usdc_amount,
    )?;
    
    // Update state
    user_position.shares -= shares;
    vault.total_shares -= shares;
    
    msg!("Withdrew {} shares for {} USDC", shares, usdc_amount);
    
    Ok(())
}
