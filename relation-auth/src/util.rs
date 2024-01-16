use chrono::{DateTime, TimeZone, Utc};

pub fn current_timestamp() -> u64 {
    // NOTE: confusing time api output
    // ic_cdk::api::time(), e.g. output: 1638348267_014414_000_u64
    ic_cdk::api::time() / 1_000_000_u64
}

/// Gets current utc time
pub fn utc_time_now() -> DateTime<Utc> {
    let epoch_secs = ic_cdk::api::time() / 1_000_000_000_u64;
    Utc.timestamp(epoch_secs as i64, 0)
}
