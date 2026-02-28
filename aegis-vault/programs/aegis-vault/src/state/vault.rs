use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    /// Admin authority (set to None after deployment for immutability)
    pub authority: Pubkey,

    /// USDC token mint
    pub usdc_mint: Pubkey,

    /// Vault's USDC token account (PDA)
    pub vault_usdc: Pubkey,

    /// Vault share token mint (PDA)
    pub share_mint: Pubkey,

    /// Pyth USDC/USD price feed account
    pub pyth_usdc_feed: Pubkey,

    /// Vault metrics
    pub total_supplied: u64,   // Total USDC supplied to lending (v2: Kamino)
    pub total_borrowed: u64,   // Total USDC borrowed
    pub total_shares: u64,     // Total vault shares minted

    /// State tracking
    pub last_rebalance_slot: u64,
    pub reexpansion_unlocked_at: i64,

    /// Constants (hard-coded per PRD, set at init)
    pub max_leverage_bps: u16,      // 15000 (1.50x)
    pub hf_floor_bps: u16,          // 24000 (2.40)
    pub oracle_stale_slots: u64,    // 150 slots (~60 seconds)
    pub peg_warn_bps: u16,          // 10 bps → CONTRACT
    pub peg_exit_bps: u16,          // 25 bps → EXIT
    pub peg_panic_bps: u16,         // 50 bps → PANIC
    pub cooldown_slots: u64,        // 30,000 slots (~4 hours)
    pub reexpansion_delay_sec: i64, // 30,000 seconds after depeg recovery

    pub bump: u8,
}

impl Vault {
    pub const LEN: usize = 8    // discriminator
        + 32  // authority
        + 32  // usdc_mint
        + 32  // vault_usdc
        + 32  // share_mint
        + 32  // pyth_usdc_feed
        + 8   // total_supplied
        + 8   // total_borrowed
        + 8   // total_shares
        + 8   // last_rebalance_slot
        + 8   // reexpansion_unlocked_at
        + 2   // max_leverage_bps
        + 2   // hf_floor_bps
        + 8   // oracle_stale_slots
        + 2   // peg_warn_bps
        + 2   // peg_exit_bps
        + 2   // peg_panic_bps
        + 8   // cooldown_slots
        + 8   // reexpansion_delay_sec
        + 1;  // bump

    /// Current equity = supplied − borrowed (saturating to prevent underflow)
    pub fn equity(&self) -> u64 {
        self.total_supplied.saturating_sub(self.total_borrowed)
    }

    /// Current leverage in basis points (10000 = 1.00x)
    pub fn leverage_bps(&self) -> u16 {
        let eq = self.equity();
        if eq == 0 {
            return 0;
        }
        ((self.total_supplied as u128 * 10_000) / eq as u128) as u16
    }
}
