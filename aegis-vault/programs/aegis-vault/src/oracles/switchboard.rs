use anchor_lang::prelude::*;
use switchboard_v2::AggregatorAccountData;

/// Fetch the current USDC price from Switchboard oracle
pub fn get_switchboard_price(aggregator: &AccountInfo) -> Result<i64> {
    let feed = AggregatorAccountData::new(aggregator)
        .map_err(|_| error!(crate::errors::ErrorCode::SwitchboardPriceUnavailable))?;
    
    // Get the current result
    let result = feed
        .get_result()
        .map_err(|_| error!(crate::errors::ErrorCode::SwitchboardPriceUnavailable))?;
    
    // Switchboard returns a mantissa and scale
    // For USDC/USD, we want to normalize to 6 decimals
    let price = result.mantissa;
    let scale = result.scale;
    
    // Normalize to 6 decimals (USDC standard)
    let normalized_price = if scale >= 6 {
        price / 10i128.pow((scale - 6) as u32)
    } else {
        price * 10i128.pow((6 - scale) as u32)
    };
    
    Ok(normalized_price as i64)
}

/// Check if Switchboard price feed is stale
pub fn is_switchboard_stale(
    aggregator: &AccountInfo,
    max_age_slots: u64,
    current_slot: u64,
) -> Result<bool> {
    let feed = AggregatorAccountData::new(aggregator)
        .map_err(|_| error!(crate::errors::ErrorCode::SwitchboardPriceUnavailable))?;
    
    // Get the latest update slot
    let latest_confirmed_round = feed.latest_confirmed_round;
    let round_open_slot = latest_confirmed_round.round_open_slot;
    
    let slots_since = current_slot.saturating_sub(round_open_slot);
    
    Ok(slots_since > max_age_slots)
}

/// Get Switchboard price standard deviation
pub fn get_switchboard_std_dev(aggregator: &AccountInfo) -> Result<i128> {
    let feed = AggregatorAccountData::new(aggregator)
        .map_err(|_| error!(crate::errors::ErrorCode::SwitchboardPriceUnavailable))?;
    
    let result = feed
        .get_result()
        .map_err(|_| error!(crate::errors::ErrorCode::SwitchboardPriceUnavailable))?;
    
    Ok(result.std_dev)
}

/// Check if Switchboard feed has enough successful responses
pub fn has_sufficient_responses(aggregator: &AccountInfo, min_responses: u32) -> Result<bool> {
    let feed = AggregatorAccountData::new(aggregator)
        .map_err(|_| error!(crate::errors::ErrorCode::SwitchboardPriceUnavailable))?;
    
    let latest_round = feed.latest_confirmed_round;
    
    Ok(latest_round.num_success >= min_responses)
}
