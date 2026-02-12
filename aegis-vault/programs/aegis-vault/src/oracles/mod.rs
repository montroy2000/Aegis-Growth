// Oracle integration module
use anchor_lang::prelude::*;

pub mod pyth;
pub mod switchboard;

pub use pyth::*;
pub use switchboard::*;

/// Combined oracle data from both Pyth and Switchboard
#[derive(Debug, Clone)]
pub struct OracleData {
    pub pyth_price: i64,
    pub switchboard_price: i64,
    pub avg_price: i64,
    pub is_stale: bool,
    pub has_conflict: bool,
    pub peg_deviation_bps: u16,
}

/// Fetch prices from both oracles and calculate metrics
pub fn fetch_oracle_data(
    pyth_feed: &AccountInfo,
    switchboard_feed: &AccountInfo,
    max_stale_slots: u64,
    max_deviation_bps: u16,
) -> Result<OracleData> {
    let clock = Clock::get()?;
    
    // Fetch prices
    let pyth_price = pyth::get_pyth_price(pyth_feed)?;
    let switchboard_price = switchboard::get_switchboard_price(switchboard_feed)?;
    
    // Calculate average price
    let avg_price = (pyth_price + switchboard_price) / 2;
    
    // Check staleness
    let pyth_stale = pyth::is_pyth_stale(pyth_feed, max_stale_slots, clock.slot)?;
    let switchboard_stale = switchboard::is_switchboard_stale(
        switchboard_feed,
        max_stale_slots,
        clock.slot,
    )?;
    let is_stale = pyth_stale || switchboard_stale;
    
    // Check for price conflict (deviation between oracles)
    let price_diff = (pyth_price - switchboard_price).abs();
    let deviation_bps = ((price_diff * 10000) / avg_price) as u16;
    let has_conflict = deviation_bps > max_deviation_bps;
    
    // Calculate peg deviation (from $1.00)
    let target_price = 1_000_000i64; // $1.00 in 6 decimals
    let peg_diff = (target_price - avg_price).abs();
    let peg_deviation_bps = ((peg_diff * 10000) / target_price) as u16;
    
    Ok(OracleData {
        pyth_price,
        switchboard_price,
        avg_price,
        is_stale,
        has_conflict,
        peg_deviation_bps,
    })
}

/// Validate oracle data quality
pub fn validate_oracle_quality(
    pyth_feed: &AccountInfo,
    switchboard_feed: &AccountInfo,
) -> Result<bool> {
    // Check Pyth confidence
    let pyth_conf = pyth::get_pyth_confidence(pyth_feed)?;
    let pyth_price = pyth::get_pyth_price(pyth_feed)?;
    let pyth_conf_bps = ((pyth_conf * 10000) / pyth_price as u64) as u16;
    
    // Pyth confidence should be < 1% of price
    if pyth_conf_bps > 100 {
        return Ok(false);
    }
    
    // Check Switchboard has sufficient responses (at least 3)
    let has_responses = switchboard::has_sufficient_responses(switchboard_feed, 3)?;
    if !has_responses {
        return Ok(false);
    }
    
    Ok(true)
}
