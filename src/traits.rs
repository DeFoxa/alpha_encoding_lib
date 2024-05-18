use crate::{
    data::types::{Granularity, NormalizedTypes},
    features::ExecutionContext,
};
use async_trait::async_trait;
use diesel::PgConnection;
use eyre::Result;
use std::{error::Error, fs};

pub trait Executable {
    fn execute(&self, context: &ExecutionContext) -> f64;
}
pub trait LocalDataMethods {
    type Output;

    fn get_timestamp_lookback(&self, first_ts: i64) -> Result<Self::Output, Box<dyn Error>>;
    fn get_timestamp_window(
        &self,
        first_ts: i64,
        last_ts: i64,
    ) -> Result<Self::Output, Box<dyn Error>>;
}

#[async_trait]
pub trait IODataMethods {
    type Item;

    async fn from_file_full_dataset(&self, path: &str) -> Result<Vec<Self::Item>, std::io::Error>;
    async fn from_file_by_ts_lookback(
        &self,
        path: &str,
        last_ts: i64,
    ) -> Result<Vec<Self::Item>, std::io::Error>;
    async fn from_file_by_ts_window(
        &self,
        path: &str,
        first_ts: i64,
        last_ts: i64,
    ) -> Result<Vec<Self::Item>, std::io::Error>;
    async fn from_db_all_entries(
        &self,
        conn: &PgConnection,
    ) -> Result<Vec<Self::Item>, diesel::result::Error>;
    async fn db_ts_window(
        &self,
        conn: &PgConnection,
        first_ts: i64,
        last_ts: i64,
    ) -> Result<Vec<Self::Item>, diesel::result::Error>;
}

pub trait DataUpdate {
    type NewData;

    fn update(&mut self, data: Self::NewData);
}
