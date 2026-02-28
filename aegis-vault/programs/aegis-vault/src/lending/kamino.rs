// V2 placeholder: Kamino Finance lending protocol integration.
// CPI calls will be implemented when the Kamino SDK is compatible
// with Anchor 0.30.1 + Solana 1.18+.
use anchor_lang::prelude::*;

/// Returns the health factor in basis points.
/// V1: returns a hardcoded floor value (3.12x = 31200 bps).
/// V2: will parse the on-chain Kamino obligation account.
pub fn get_health_factor(_kamino_obligation: &AccountInfo) -> Result<u16> {
    Ok(31_200u16) // 3.12 × 10000
}

/// Returns the target borrow amount for a desired leverage level.
pub fn calculate_target_borrow(total_supplied: u64, target_leverage_bps: u16) -> u64 {
    let leverage = target_leverage_bps as u128;
    let supplied = total_supplied as u128;
    ((supplied * leverage.saturating_sub(10_000)) / leverage) as u64
}

/// Returns the amount to repay to reduce leverage by a given percentage.
pub fn calculate_repay_amount(total_borrowed: u64, reduction_pct: u8) -> u64 {
    ((total_borrowed as u128 * reduction_pct as u128) / 100) as u64
}
