//! In-memory Storage implementation for tests and local runs.
use crate::storage::{QueryParams, Storage};
use crate::types::FeeRecord;

#[derive(Default)]
pub struct MemoryStorage {
    records: Vec<FeeRecord>,
}

impl Storage for MemoryStorage {
    fn insert(&mut self, record: FeeRecord) -> Result<(), String> {
        self.records.push(record);
        Ok(())
    }

    fn query(&self, params: QueryParams) -> Vec<FeeRecord> {
        self.records
            .iter()
            .filter(|r| params.from_ledger.map_or(true, |f| r.ledger >= f))
            .filter(|r| params.to_ledger.map_or(true, |t| r.ledger <= t))
            .cloned()
            .filter(|r| params.from_ledger.is_none_or(|f| r.ledger >= f))
            .filter(|r| params.to_ledger.is_none_or(|t| r.ledger <= t))
            .map(|r| FeeRecordLike { ledger: r.ledger, fee_charged: r.fee_charged })
            .take(params.limit.unwrap_or(usize::MAX))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_returns_complete_fee_records() {
        let mut storage = MemoryStorage::default();
        storage
            .insert(FeeRecord {
                ledger: 10,
                fee_charged: 100,
                max_fee: 500,
            })
            .unwrap();

        let records = storage.query(QueryParams::default());

        assert_eq!(records.len(), 1);
        assert_eq!(records[0].max_fee, 500);
    }
}
