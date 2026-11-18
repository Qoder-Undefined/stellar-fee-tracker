//! SQLite-backed storage for FeeRecord rows.
use rusqlite::{params, Connection, Result as SqlResult};

pub struct SqliteStorage {
    conn: Connection,
}

impl SqliteStorage {
    pub fn open(path: &str) -> SqlResult<Self> {
        let conn = Connection::open(path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS fee_record (
                ledger INTEGER PRIMARY KEY,
                fee_charged INTEGER NOT NULL,
                max_fee INTEGER NOT NULL
            )",
            [],
        )?;
        Ok(Self { conn })
    }

    pub fn insert(&self, ledger: u32, fee_charged: i64, max_fee: i64) -> SqlResult<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO fee_record (ledger, fee_charged, max_fee) VALUES (?1, ?2, ?3)",
            params![ledger, fee_charged, max_fee],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inserting_existing_ledger_replaces_its_fee_record() {
        let storage = SqliteStorage::open(":memory:").unwrap();

        storage.insert(10, 100, 500).unwrap();
        storage.insert(10, 200, 600).unwrap();

        let (count, fee_charged, max_fee): (u32, i64, i64) = storage
            .conn
            .query_row(
                "SELECT COUNT(*), fee_charged, max_fee FROM fee_record WHERE ledger = 10",
                [],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .unwrap();

        assert_eq!(count, 1);
        assert_eq!(fee_charged, 200);
        assert_eq!(max_fee, 600);
    }
}
