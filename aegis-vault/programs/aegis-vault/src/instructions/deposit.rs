use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Transfer};
use crate::state::{Vault, UserPosition};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    
    #[account(
        init_if_needed,
        payer = user,
        space = UserPosition::LEN,
        seeds = [b"user-position", user.key().as_ref()],
        bump
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
        init_if_needed,
        payer = user,
        associated_token::mint = share_mint,
        associated_token::authority = user
    )]
    pub user_shares: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let user_position = &mut ctx.accounts.user_position;
    let clock = Clock::get()?;
    
    // Calculate shares to mint
    let shares = if vault.total_shares == 0 {
        amount // 1:1 for first deposit
    } else {
        let equity = vault.equity();
        (amount as u128)
            .checked_mul(vault.total_shares as u128)
            .and_then(|v| v.checked_div(equity as u128))
            .ok_or(crate::errors::ErrorCode::MathOverflow)? as u64
    };
    
    // Transfer USDC from user to vault
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_usdc.to_account_info(),
                to: ctx.accounts.vault_usdc.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        amount,
    )?;
    
    // Mint shares to user
    let vault_seeds = &[b"vault".as_ref(), &[vault.bump]];
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.share_mint.to_account_info(),
                to: ctx.accounts.user_shares.to_account_info(),
                authority: vault.to_account_info(),
            },
            &[vault_seeds],
        ),
        shares,
    )?;
    
    // Update state
    if user_position.owner == Pubkey::default() {
        user_position.owner = ctx.accounts.user.key();
        user_position.deposited_at = clock.unix_timestamp;
        user_position.bump = ctx.bumps.user_position;
    }
    user_position.shares += shares;
    vault.total_shares += shares;
    
    msg!("Deposited {} USDC, minted {} shares", amount, shares);
    
    Ok(())
}
