use crate::traits::{DataUpdate, IODataMethods, LocalDataMethods};
use anyhow::Result;
use async_trait::async_trait;
use csv::ReaderBuilder;
use diesel::PgConnection;
use serde::{Deserialize, Serialize};
use serde_json::de::from_reader;
use std::collections::{BTreeMap, VecDeque};
use std::error::Error;
use std::ops::Bound::{Included, Unbounded};
use tokio::{fs::File, io::AsyncReadExt};

//TODO: port logic/types for Normalized Types, and based on todo's in features, write logic for
//candle creation and usage API

pub struct PlaceHolder;

//NOTE: Intial iteration, may rewrite for real-time bar builder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
    close_timestamp: i64,
}

#[derive(Debug, Clone)]
pub struct CandleDataSet {
    granularity: CandleGranularity,
    data: BTreeMap<i64, Candle>,
}

impl CandleDataSet {
    fn new(granularity: CandleGranularity) -> Self {
        CandleDataSet {
            granularity,
            data: BTreeMap::new(),
        }
    }
    fn single_insert(&mut self, timestamp: i64, candle: Candle) {
        self.data.insert(timestamp, candle);
    }
    fn get_range(&self, first_ts: i64, last_ts: i64) -> Result<Vec<&Candle>, Box<dyn Error>> {
        Ok(self
            .data
            .range(first_ts..=last_ts)
            .map(|(k, v)| v)
            .collect())
    }
}
impl DataUpdate for CandleDataSet {
    type NewData = Vec<Candle>;

    fn update(&mut self, data: Self::NewData) {
        data.iter()
            .map(|entry| self.data.insert(entry.close_timestamp, entry.clone()));
    }
}

impl LocalDataMethods for CandleDataSet {
    type Output = Vec<Candle>;

    fn get_timestamp_lookback(&self, lookback_ts: i64) -> Result<Self::Output, Box<dyn Error>> {
        Ok(self
            .data
            .range((Included(lookback_ts), Unbounded))
            .map(|(k, v)| v.clone())
            .collect())
    }

    fn get_timestamp_window(
        &self,
        first_ts: i64,
        last_ts: i64,
    ) -> Result<Self::Output, Box<dyn Error>> {
        Ok(self
            .data
            .range(first_ts..=last_ts)
            .map(|(k, v)| v.clone())
            .collect())
    }
}
#[async_trait]
impl IODataMethods for CandleDataSet {
    type Item = Candle;

    async fn from_file_full_dataset(&self, path: &str) -> Result<Vec<Self::Item>, std::io::Error> {
        todo!();
    }
    async fn from_file_by_ts_lookback(
        &self,
        path: &str,
        last_ts: i64,
    ) -> Result<Vec<Self::Item>, std::io::Error> {
        todo!();
    }
    async fn from_file_by_ts_window(
        &self,
        path: &str,
        first_ts: i64,
        last_ts: i64,
    ) -> Result<Vec<Self::Item>, std::io::Error> {
        todo!();
    }
    async fn from_db_all_entries(
        &self,
        conn: &PgConnection,
    ) -> Result<Vec<Self::Item>, diesel::result::Error> {
        todo!();
    }
    async fn db_ts_window(
        &self,
        conn: &PgConnection,
        first_ts: i64,
        last_entry: i64,
    ) -> Result<Vec<Self::Item>, diesel::result::Error> {
        todo!();
    }
}

// NOTE: choosing btree for historical book data, due to quick range fetching. Prioritizing ease of use for historical data
// NOTE: May add another type for real-time ob data where quick insertion/removal are higher priority

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quotes {
    pub level: f64,
    pub qty: f64,
    pub count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedBook {
    pub symbol: String,
    pub depth: u16,
    pub bids: Vec<Quotes>,
    pub asks: Vec<Quotes>,
    pub timestamp: i64,
}

#[derive(Debug, Clone)]
pub struct BookDataSet {
    data: BTreeMap<i64, NormalizedBook>,
}
impl BookDataSet {
    fn new(granularity: CandleGranularity) -> Self {
        BookDataSet {
            data: BTreeMap::new(),
        }
    }
    fn single_insert(&mut self, timestamp: i64, book: NormalizedBook) {
        self.data.insert(timestamp, book);
    }
}

impl DataUpdate for BookDataSet {
    type NewData = Vec<NormalizedBook>;

    fn update(&mut self, data: Self::NewData) {
        data.iter()
            .map(|entry| self.data.insert(entry.timestamp, entry.clone()));
    }
}

impl LocalDataMethods for BookDataSet {
    type Output = Vec<NormalizedBook>;

    fn get_timestamp_lookback(&self, lookback_ts: i64) -> Result<Self::Output, Box<dyn Error>> {
        Ok(self
            .data
            .range((Included(lookback_ts), Unbounded))
            .map(|(k, v)| v.clone())
            .collect())
    }
    fn get_timestamp_window(
        &self,
        first_ts: i64,
        last_ts: i64,
    ) -> Result<Self::Output, Box<dyn Error>> {
        Ok(self
            .data
            .range(first_ts..=last_ts)
            .map(|(k, v)| v.clone())
            .collect())
    }
}

#[async_trait]
impl IODataMethods for BookDataSet {
    type Item = NormalizedBook;

    async fn from_file_full_dataset(&self, path: &str) -> Result<Vec<Self::Item>, std::io::Error> {
        todo!();
    }
    async fn from_file_by_ts_lookback(
        &self,
        path: &str,
        last_ts: i64,
    ) -> Result<Vec<Self::Item>, std::io::Error> {
        // NOTE: Tmp implementation
        // TODO: Optimize
        // let file = File::open(path).await?;
        // let mut contents = String::new();
        // file.read_to_string(&mut contents).await?;
        // let reader = ReaderBuilder::new().from_reader(contents.as_bytes());
        //
        // let mut book = Vec::new();
        // for result in reader.deserialize::<NormalizedBook>() {
        //     let book = result?;
        //     if result.timestamp >= last_ts {
        //         book.push(result);
        //     }
        // }
        todo!();
        // Ok()
    }
    async fn from_file_by_ts_window(
        &self,
        path: &str,
        first_ts: i64,
        last_ts: i64,
    ) -> Result<Vec<Self::Item>, std::io::Error> {
        todo!();
    }
    async fn from_db_all_entries(
        &self,
        conn: &PgConnection,
    ) -> Result<Vec<Self::Item>, diesel::result::Error> {
        todo!();
    }
    async fn db_ts_window(
        &self,
        conn: &PgConnection,
        first_ts: i64,
        last_entry: i64,
    ) -> Result<Vec<Self::Item>, diesel::result::Error> {
        todo!();
    }
}

#[derive(Debug, Clone)]
pub struct TradesDataSet {
    data: VecDeque<NormalizedTrades>,
}
impl TradesDataSet {
    fn new() -> Self {
        TradesDataSet {
            data: VecDeque::new(),
        }
    }
    fn new_with_capacity(capacity: usize) -> Self {
        TradesDataSet {
            data: VecDeque::with_capacity(capacity),
        }
    }
    fn get(&self, index: usize) -> Option<NormalizedTrades> {
        self.data.get(index).cloned()
    }

    fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }

    fn len(&self) -> usize {
        self.data.len()
    }
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    fn truncate(&mut self, len: usize) {
        //NOTE: shortens keeping first len elements
        self.data.truncate(len);
    }
    fn sort_by_timestamp(&mut self) {
        self.data
            .make_contiguous()
            .sort_by(|a, b| a.transaction_timestamp.cmp(&b.transaction_timestamp));
    }

    fn back_timestamp(&self) -> Option<i64> {
        let back = self.data.back().and_then(|x| Some(x.transaction_timestamp));
        Some(back?)
    }
    fn binary_search_timestamp_by_return_range(
        &mut self,
        target_timestamp: i64,
    ) -> Result<Vec<&NormalizedTrades>, Box<dyn Error>> {
        self.sort_by_timestamp();

        match self
            .data
            .binary_search_by_key(&target_timestamp, |entry| entry.transaction_timestamp)
        {
            Ok(first) => Ok(self.data.range(0..first).collect::<Vec<_>>()),
            Err(_) => Err("Binary search TradesDataSet Error: Timestamp not found".into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NormalizedTrades {
    pub symbol: String,
    pub side: Side,
    pub price: f64,
    pub qty: f64,
    pub local_ids: u32,
    pub exch_id: i64,
    pub transaction_timestamp: i64,
}

impl LocalDataMethods for TradesDataSet {
    type Output = Vec<NormalizedTrades>;

    fn get_timestamp_lookback(&self, first_ts: i64) -> Result<Self::Output, Box<dyn Error>> {
        todo!();
    }
    fn get_timestamp_window(
        &self,
        first_ts: i64,
        last_ts: i64,
    ) -> Result<Self::Output, Box<dyn Error>> {
        todo!();
    }
}

#[async_trait]
impl IODataMethods for TradesDataSet {
    type Item = NormalizedTrades;

    async fn from_file_full_dataset(&self, path: &str) -> Result<Vec<Self::Item>, std::io::Error> {
        todo!();
    }
    async fn from_file_by_ts_lookback(
        &self,
        path: &str,
        last_ts: i64,
    ) -> Result<Vec<Self::Item>, std::io::Error> {
        todo!();
    }
    async fn from_file_by_ts_window(
        &self,
        path: &str,
        first_ts: i64,
        last_ts: i64,
    ) -> Result<Vec<Self::Item>, std::io::Error> {
        todo!();
    }
    async fn from_db_all_entries(
        &self,
        conn: &PgConnection,
    ) -> Result<Vec<Self::Item>, diesel::result::Error> {
        todo!();
    }
    async fn db_ts_window(
        &self,
        conn: &PgConnection,
        first_ts: i64,
        last_entry: i64,
    ) -> Result<Vec<Self::Item>, diesel::result::Error> {
        todo!();
    }
}

#[derive(Debug, Clone)]
pub enum NormalizedTypes {
    Candles(CandleDataSet),
    Orderbook(BookDataSet),
    Trades(TradesDataSet),
}

#[derive(Debug, Clone)]
pub enum Granularity {
    Candle(CandleGranularity),
    Trades(TradesGranularity),
    OB(OrderBookGranularity),
}

#[derive(Debug, Clone)]
pub enum CandleGranularity {
    OneMinute,
    FiveMinute,
    FifteenMinute,
    ThirtyMinute,
    OneHour,
    FourHour,
    Daily,
}

#[derive(Debug, Clone)]
pub struct TradesGranularity {
    data_length: u64,
}

#[derive(Debug, Clone)]
pub struct OrderBookGranularity {
    data_snapshot_length: u64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Side {
    Buy,
    Sell,
}
impl AsRef<str> for Side {
    fn as_ref(&self) -> &str {
        match self {
            Side::Buy => "buy",
            Side::Sell => "sell",
        }
    }
}
