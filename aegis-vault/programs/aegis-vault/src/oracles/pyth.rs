use anchor_lang::prelude::*;
use pyth_sdk_solana::load_price_feed_from_account_info;

/// Fetch the current USDC price from Pyth oracle
pub fn get_pyth_price(price_feed: &AccountInfo) -> Result<i64> {
    let price_feed_data = load_price_feed_from_account_info(price_feed)
        .map_err(|_| error!(crate::errors::ErrorCode::PythPriceUnavailable))?;
    
    let price = price_feed_data
        .get_current_price()
        .ok_or(crate::errors::ErrorCode::PythPriceUnavailable)?;
    
    // Pyth returns price with exponent, normalize to 6 decimals (USDC standard)
    // For USDC/USD, exponent is typically -8, price is in format: price * 10^exponent
    let normalized_price = if price.expo >= 0 {
        price.price * 10i64.pow(price.expo as u32) * 1_000_000
    } else {
        let divisor = 10i64.pow((-price.expo) as u32);
        (price.price * 1_000_000) / divisor
    };
    
    Ok(normalized_price)
}

/// Check if Pyth price feed is stale
pub fn is_pyth_stale(price_feed: &AccountInfo, max_age_slots: u64, current_slot: u64) -> Result<bool> {
    let price_feed_data = load_price_feed_from_account_info(price_feed)
        .map_err(|_| error!(crate::errors::ErrorCode::PythPriceUnavailable))?;
    
    let price = price_feed_data
        .get_current_price()
        .ok_or(crate::errors::ErrorCode::PythPriceUnavailable)?;
    
    // Pyth publish_time is in Unix timestamp, need to convert to slots
    // Approximate: 1 slot = 400ms, so slots_since = (current_time - publish_time) / 0.4
    let clock = Clock::get()?;
    let time_diff = clock.unix_timestamp - price.publish_time;
    let slots_since = (time_diff as u64 * 10) / 4; // Convert seconds to slots
    
    Ok(slots_since > max_age_slots)
}

/// Get Pyth price confidence interval
pub fn get_pyth_confidence(price_feed: &AccountInfo) -> Result<u64> {
    let price_feed_data = load_price_feed_from_account_info(price_feed)
        .map_err(|_| error!(crate::errors::ErrorCode::PythPriceUnavailable))?;
    
    let price = price_feed_data
        .get_current_price()
        .ok_or(crate::errors::ErrorCode::PythPriceUnavailable)?;
    
    Ok(price.conf)
}
