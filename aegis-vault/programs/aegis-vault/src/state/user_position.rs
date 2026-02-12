use anchor_lang::prelude::*;

#[account]
pub struct UserPosition {
    /// Owner of this position
    pub owner: Pubkey,
    
    /// Number of vault shares owned
    pub shares: u64,
    
    /// Timestamp when first deposited
    pub deposited_at: i64,
    
    pub bump: u8,
}

impl UserPosition {
    pub const LEN: usize = 8 + // discriminator
        32 + // owner
        8 + // shares
        8 + // deposited_at
        1; // bump
}
