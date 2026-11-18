//! Basic fee_stats invariant checks.
use crate::types::FeeStats;

pub fn validate(stats: &FeeStats) -> Result<(), String> {
    if stats.p10_accepted_fee < 0
        || stats.p50_accepted_fee < 0
        || stats.p90_accepted_fee < 0
        || stats.p99_accepted_fee < 0
    {
        return Err("fees must be non-negative".into());
    }
    if !(stats.p10_accepted_fee <= stats.p50_accepted_fee
        && stats.p50_accepted_fee <= stats.p90_accepted_fee
        && stats.p90_accepted_fee <= stats.p99_accepted_fee)
    {
        return Err("percentiles must be non-decreasing".into());
    }
    Ok(())
}
