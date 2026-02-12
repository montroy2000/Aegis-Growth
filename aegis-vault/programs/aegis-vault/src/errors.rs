use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Vault is in PANIC state - only withdrawals allowed")]
    VaultInPanicState,
    
    #[msg("Rebalance cooldown period not elapsed")]
    RebalanceCooldown,
    
    #[msg("Reexpansion delay not elapsed")]
    ReexpansionCooldown,
    
    #[msg("Oracle price is stale")]
    OraclePriceStale,
    
    #[msg("Oracle price conflict detected")]
    OraclePriceConflict,
    
    #[msg("Pyth price unavailable")]
    PythPriceUnavailable,
    
    #[msg("Switchboard price unavailable")]
    SwitchboardPriceUnavailable,
    
    #[msg("Insufficient vault equity for withdrawal")]
    InsufficientEquity,
    
    #[msg("Math overflow")]
    MathOverflow,
    
    #[msg("Invalid leverage parameter")]
    InvalidLeverage,
    
    #[msg("Invalid health factor parameter")]
    InvalidHealthFactor,
}
