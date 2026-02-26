use anchor_lang::prelude::*;

/// Switchboard V2 AggregatorRound - partial layout for the fields we need.
/// The full AggregatorAccountData struct is large; we only read what we use.
///
/// Layout offsets (after the 8-byte discriminator):
///   name:                     32 bytes  (offset 8)
///   metadata:                 128 bytes (offset 40)
///   queue_pubkey:             32 bytes  (offset 168)
///   oracle_request_batch_size: 4 bytes  (offset 200)
///   min_oracle_results:       4 bytes   (offset 204)
///   min_job_results:          4 bytes   (offset 208)
///   min_update_delay_seconds: 4 bytes   (offset 212)
///   start_after:              8 bytes   (offset 216)
///   variance_threshold:       16 bytes  (offset 224)  — SwitchboardDecimal
///   force_report_period:      8 bytes   (offset 240)
///   expiration:               8 bytes   (offset 248)
///   consecutive_failure_count: 8 bytes  (offset 256)
///   next_allowed_update_time: 8 bytes   (offset 264)
///   is_locked:                1 byte    (offset 272)
///   _reserved1:               7 bytes   (offset 273)
///   crank_pubkey:             32 bytes  (offset 280)
///   latest_confirmed_round:   AggregatorRound starts at offset 312
///
/// AggregatorRound layout (176 bytes total):
///   num_success:              4 bytes   (+0)
///   num_error:                4 bytes   (+4)
///   is_closed:                1 byte    (+8)
///   _padding:                 7 bytes   (+9)
///   round_open_slot:          8 bytes   (+16)
///   round_open_timestamp:     8 bytes   (+24)
///   result (SwitchboardDecimal):
///     mantissa:               16 bytes  (+32)   — i128
///     scale:                  4 bytes   (+48)   — u32
///   std_deviation:            16+4 bytes (+52)
///   min_response:             16+4 bytes (+72)
///   max_response:             16+4 bytes (+92)
///   oracle_pubkeys_data:      32*16 bytes (+112) — only presence matters

const DISCRIMINATOR_LEN: usize = 8;
const LATEST_ROUND_OFFSET: usize = 312;

// Offsets within AggregatorRound
const ROUND_NUM_SUCCESS_OFFSET: usize = 0;
const ROUND_OPEN_SLOT_OFFSET: usize = 16;
const ROUND_RESULT_MANTISSA_OFFSET: usize = 32;
const ROUND_RESULT_SCALE_OFFSET: usize = 48;
const ROUND_STD_DEV_MANTISSA_OFFSET: usize = 52;

fn read_i128_le(data: &[u8], offset: usize) -> Option<i128> {
    data.get(offset..offset + 16)
        .map(|b| i128::from_le_bytes(b.try_into().unwrap()))
}

fn read_u32_le(data: &[u8], offset: usize) -> Option<u32> {
    data.get(offset..offset + 4)
        .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
}

fn read_u64_le(data: &[u8], offset: usize) -> Option<u64> {
    data.get(offset..offset + 8)
        .map(|b| u64::from_le_bytes(b.try_into().unwrap()))
}

/// Fetch the current USDC price from Switchboard oracle.
/// Normalizes to 6 decimal places (USDC standard).
pub fn get_switchboard_price(aggregator: &AccountInfo) -> Result<i64> {
    let data = aggregator.try_borrow_data()
        .map_err(|_| error!(crate::errors::ErrorCode::SwitchboardPriceUnavailable))?;

    let base = DISCRIMINATOR_LEN + LATEST_ROUND_OFFSET;

    let mantissa = read_i128_le(&data, base + ROUND_RESULT_MANTISSA_OFFSET)
        .ok_or(error!(crate::errors::ErrorCode::SwitchboardPriceUnavailable))?;
    let scale = read_u32_le(&data, base + ROUND_RESULT_SCALE_OFFSET)
        .ok_or(error!(crate::errors::ErrorCode::SwitchboardPriceUnavailable))?;

    // Normalize to 6 decimals (USDC standard)
    let normalized: i128 = if scale >= 6 {
        mantissa / 10i128.pow(scale - 6)
    } else {
        mantissa * 10i128.pow(6 - scale)
    };

    Ok(normalized as i64)
}

/// Check if Switchboard price feed is stale (based on round_open_slot).
pub fn is_switchboard_stale(
    aggregator: &AccountInfo,
    max_age_slots: u64,
    current_slot: u64,
) -> Result<bool> {
    let data = aggregator.try_borrow_data()
        .map_err(|_| error!(crate::errors::ErrorCode::SwitchboardPriceUnavailable))?;

    let base = DISCRIMINATOR_LEN + LATEST_ROUND_OFFSET;

    let round_open_slot = read_u64_le(&data, base + ROUND_OPEN_SLOT_OFFSET)
        .ok_or(error!(crate::errors::ErrorCode::SwitchboardPriceUnavailable))?;

    let slots_since = current_slot.saturating_sub(round_open_slot);
    Ok(slots_since > max_age_slots)
}

/// Get Switchboard price standard deviation (mantissa only, normalized to 6dp).
pub fn get_switchboard_std_dev(aggregator: &AccountInfo) -> Result<i128> {
    let data = aggregator.try_borrow_data()
        .map_err(|_| error!(crate::errors::ErrorCode::SwitchboardPriceUnavailable))?;

    let base = DISCRIMINATOR_LEN + LATEST_ROUND_OFFSET;

    let mantissa = read_i128_le(&data, base + ROUND_STD_DEV_MANTISSA_OFFSET)
        .ok_or(error!(crate::errors::ErrorCode::SwitchboardPriceUnavailable))?;

    Ok(mantissa)
}

/// Check if Switchboard feed has enough successful oracle responses.
pub fn has_sufficient_responses(aggregator: &AccountInfo, min_responses: u32) -> Result<bool> {
    let data = aggregator.try_borrow_data()
        .map_err(|_| error!(crate::errors::ErrorCode::SwitchboardPriceUnavailable))?;

    let base = DISCRIMINATOR_LEN + LATEST_ROUND_OFFSET;

    let num_success = read_u32_le(&data, base + ROUND_NUM_SUCCESS_OFFSET)
        .ok_or(error!(crate::errors::ErrorCode::SwitchboardPriceUnavailable))?;

    Ok(num_success >= min_responses)
}
