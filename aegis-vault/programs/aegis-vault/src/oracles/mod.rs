// Oracle integration module — Pyth only for v1.
// Switchboard dual-feed validation is a v2 feature (requires compatible SDK).
use anchor_lang::prelude::*;

pub mod pyth;

pub use pyth::*;

/// Oracle data fetched from Pyth
#[derive(Debug, Clone)]
pub struct OracleData {
    pub pyth_price: i64,
    /// Always false in v1 (single oracle). Will be re-enabled in v2 with dual-feed.
    pub has_conflict: bool,
    pub is_stale: bool,
    pub peg_deviation_bps: u16,
}

/// Fetch price from Pyth and calculate peg metrics.
/// Price is normalised to 6 decimal places ($1.000000 = 1_000_000).
pub fn fetch_oracle_data(
    pyth_feed: &AccountInfo,
    max_stale_slots: u64,
) -> Result<OracleData> {
    let clock = Clock::get()?;

    let pyth_price = pyth::get_pyth_price(pyth_feed)?;
    let is_stale = pyth::is_pyth_stale(pyth_feed, max_stale_slots, clock.slot)?;

    // Peg deviation from $1.00 (= 1_000_000 in 6dp)
    let target_price = 1_000_000i64;
    let peg_diff = (target_price - pyth_price).unsigned_abs();
    let peg_deviation_bps = ((peg_diff as u128 * 10_000) / target_price as u128) as u16;

    Ok(OracleData {
        pyth_price,
        has_conflict: false, // v1: single feed, no conflict possible
        is_stale,
        peg_deviation_bps,
    })
}

/// Validate Pyth price quality — confidence interval must be < 1% of price.
pub fn validate_oracle_quality(pyth_feed: &AccountInfo) -> Result<bool> {
    let pyth_conf = pyth::get_pyth_confidence(pyth_feed)?;
    let pyth_price = pyth::get_pyth_price(pyth_feed)?;

    if pyth_price == 0 {
        return Ok(false);
    }

    let conf_bps = ((pyth_conf as u128 * 10_000) / pyth_price.unsigned_abs() as u128) as u16;
    Ok(conf_bps <= 100) // confidence must be < 1%
}
