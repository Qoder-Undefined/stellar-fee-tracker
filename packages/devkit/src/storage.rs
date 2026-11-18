//! Storage trait: persistence abstraction for fee records.
use crate::types::FeeRecord;

#[derive(Default, Clone, Debug)]
pub struct QueryParams {
    pub from_ledger: Option<u32>,
    pub to_ledger: Option<u32>,
    pub limit: Option<usize>,
}

pub trait Storage {
    fn insert(&mut self, record: FeeRecord) -> Result<(), String>;
    fn query(&self, params: QueryParams) -> Vec<FeeRecord>;
}
