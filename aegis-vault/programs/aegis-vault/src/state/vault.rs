use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    /// Admin authority (set to None after deployment for immutability)
    pub authority: Pubkey,
    
    /// USDC token mint
    pub usdc_mint: Pubkey,
    
    /// Vault's USDC token account
    pub vault_usdc: Pubkey,
    
    /// Vault share token mint
    pub share_mint: Pubkey,
    
    /// Oracle configuration
    pub pyth_usdc_feed: Pubkey,
    pub switchboard_usdc_feed: Pubkey,
    
    /// Vault metrics
    pub total_supplied: u64,      // Total USDC supplied to lending
    pub total_borrowed: u64,      // Total USDC borrowed
    pub total_shares: u64,        // Total vault shares minted
    
    /// State tracking
    pub last_rebalance_slot: u64,
    pub reexpansion_unlocked_at: i64,
    
    /// Constants (hard-coded per PRD)
    pub max_leverage_bps: u16,    // 15000 (1.50x)
    pub hf_floor_bps: u16,        // 24000 (2.40)
    pub oracle_stale_slots: u64,  // 150 slots
    pub peg_warn_bps: u16,        // 10 bps
    pub peg_exit_bps: u16,        // 25 bps
    pub peg_panic_bps: u16,       // 50 bps
    pub cooldown_slots: u64,      // 30,000 slots
    pub reexpansion_delay_sec: i64, // 30,000 seconds
    
    pub bump: u8,
}

impl Vault {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        32 + // usdc_mint
        32 + // vault_usdc
        32 + // share_mint
        32 + // pyth_usdc_feed
        32 + // switchboard_usdc_feed
        8 + // total_supplied
        8 + // total_borrowed
        8 + // total_shares
        8 + // last_rebalance_slot
        8 + // reexpansion_unlocked_at
        2 + // max_leverage_bps
        2 + // hf_floor_bps
        8 + // oracle_stale_slots
        2 + // peg_warn_bps
        2 + // peg_exit_bps
        2 + // peg_panic_bps
        8 + // cooldown_slots
        8 + // reexpansion_delay_sec
        1; // bump
    
    /// Calculate current equity (supplied - borrowed)
    pub fn equity(&self) -> u64 {
        self.total_supplied.saturating_sub(self.total_borrowed)
    }
    
    /// Calculate current leverage in basis points
    pub fn leverage_bps(&self) -> u16 {
        if self.equity() == 0 {
            return 0;
        }
        ((self.total_supplied as u128 * 10000) / self.equity() as u128) as u16
    }
}
