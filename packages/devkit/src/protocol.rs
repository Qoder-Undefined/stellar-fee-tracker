//! Parses raw Horizon fee_stats JSON into FeeStats.
use serde::Deserialize;
use std::fmt;

use crate::types::FeeStats;

pub fn parse_fee_stats(raw_json: &str) -> Result<FeeStats, serde_json::Error> {
    let raw: RawFeeStats = serde_json::from_str(raw_json)?;

    Ok(FeeStats {
        last_ledger: raw.last_ledger.to_u32("last_ledger")?,
        last_ledger_base_fee: raw.last_ledger_base_fee.to_i64("last_ledger_base_fee")?,
        min_accepted_fee: raw.fee_charged.min.to_i64("fee_charged.min")?,
        mode_accepted_fee: raw.fee_charged.mode.to_i64("fee_charged.mode")?,
        p10_accepted_fee: raw.fee_charged.p10.to_i64("fee_charged.p10")?,
        p50_accepted_fee: raw.fee_charged.p50.to_i64("fee_charged.p50")?,
        p90_accepted_fee: raw.fee_charged.p90.to_i64("fee_charged.p90")?,
        p99_accepted_fee: raw.fee_charged.p99.to_i64("fee_charged.p99")?,
    })
}

#[derive(Deserialize)]
struct RawFeeStats {
    last_ledger: Number,
    last_ledger_base_fee: Number,
    fee_charged: RawFeeCharged,
}

#[derive(Deserialize)]
struct RawFeeCharged {
    min: Number,
    mode: Number,
    p10: Number,
    p50: Number,
    p90: Number,
    p99: Number,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Number {
    String(String),
    Integer(i64),
    Unsigned(u64),
}

impl Number {
    fn to_i64(&self, field: &str) -> Result<i64, serde_json::Error> {
        match self {
            Self::String(value) => value.parse().map_err(|_| invalid_number(field, value)),
            Self::Integer(value) => Ok(*value),
            Self::Unsigned(value) => {
                i64::try_from(*value).map_err(|_| invalid_number(field, value))
            }
        }
    }

    fn to_u32(&self, field: &str) -> Result<u32, serde_json::Error> {
        let value = self.to_i64(field)?;
        u32::try_from(value).map_err(|_| invalid_number(field, value))
    }
}

fn invalid_number(field: &str, value: impl fmt::Display) -> serde_json::Error {
    <serde_json::Error as serde::de::Error>::custom(format!(
        "{field} must be an integer, got {value}"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_horizon_fee_stats_into_shared_type() {
        let stats = parse_fee_stats(
            r#"{
                "last_ledger": "50000001",
                "last_ledger_base_fee": "100",
                "fee_charged": {
                    "min": "100",
                    "mode": "213",
                    "p10": "100",
                    "p50": "150",
                    "p90": "500",
                    "p99": "1200"
                }
            }"#,
        )
        .unwrap();

        assert_eq!(stats.last_ledger, 50_000_001);
        assert_eq!(stats.last_ledger_base_fee, 100);
        assert_eq!(stats.min_accepted_fee, 100);
        assert_eq!(stats.mode_accepted_fee, 213);
        assert_eq!(stats.p10_accepted_fee, 100);
        assert_eq!(stats.p50_accepted_fee, 150);
        assert_eq!(stats.p90_accepted_fee, 500);
        assert_eq!(stats.p99_accepted_fee, 1200);
    }

    #[test]
    fn rejects_non_integer_fee_stats_values() {
        let error = parse_fee_stats(
            r#"{
                "last_ledger": "50000001",
                "last_ledger_base_fee": "100",
                "fee_charged": {
                    "min": "100",
                    "mode": "not-an-integer",
                    "p10": "100",
                    "p50": "150",
                    "p90": "500",
                    "p99": "1200"
                }
            }"#,
        )
        .unwrap_err();

        assert!(error.to_string().contains("fee_charged.mode"));
    }
}
}
