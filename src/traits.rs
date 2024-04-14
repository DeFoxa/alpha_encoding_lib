use crate::{
    data::types::{Granularity, NormalizedTypes},
    features::ExecutionContext,
};
use diesel::PgConnection;
use std::{error::Error, fs};

pub trait Executable {
    fn execute(&self, context: &ExecutionContext) -> f64;
}
pub trait LocalDataMethods {
    fn get_by_lookback_window(
        &self,
        n: &str,
        granularity: Granularity,
    ) -> Result<NormalizedTypes, Box<dyn Error>>;
    fn get_timestamp_lookback(&self, first_ts: i64) -> Result<NormalizedTypes, Box<dyn Error>>;
    fn get_timestamp_window(
        &self,
        first_ts: i64,
        last_ts: i64,
    ) -> Result<NormalizedTypes, Box<dyn Error>>;
}

pub trait IODataMethods {
    type Item; // Target queryable type for DB reads/writes

    fn from_file_full_dataset(&self, path: &str) -> Result<NormalizedTypes, std::io::Error>;
    fn from_file_by_ts_lookback(
        &self,
        path: &str,
        last_ts: i64,
    ) -> Result<NormalizedTypes, std::io::Error>;
    fn from_file_by_ts_window(
        &self,
        path: &str,
        first_ts: i64,
        last_ts: i64,
    ) -> Result<NormalizedTypes, std::io::Error>;
    fn from_db_all_entries(
        &self,
        conn: &PgConnection,
    ) -> Result<Vec<Self::Item>, diesel::result::Error>;
    fn db_entry_range_by_ts(
        &self,
        conn: &PgConnection,
        first_ts: i64,
        last_entry: i64,
    ) -> Result<Vec<Self::Item>, diesel::result::Error>;
}
