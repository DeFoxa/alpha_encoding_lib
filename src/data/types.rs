use crate::traits::{DataUpdate, IODataMethods, LocalDataMethods};
use async_trait::async_trait;
use csv::ReaderBuilder;
use diesel::sql_types::Timestamp;
use diesel::PgConnection;
use eyre::Result;
use serde::{Deserialize, Serialize};
use serde_json::de::from_reader;
use std::{
    collections::{BTreeMap, VecDeque},
    error::Error,
    io::{BufReader, ErrorKind},
    ops::Bound::{Included, Unbounded},
};
use tokio::{fs::File, io::AsyncReadExt};

//TODO: Throughouly test all search/get methods for expected behavior, especially TickDataSet

// Timestamp
type TS = i64;

//NOTE: Intial iteration, may rewrite for real-time bar builder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bar {
    o: f64,
    h: f64,
    l: f64,
    c: f64,
    v: f64,
    ts: TS,
}

#[derive(Debug, Clone)]
pub struct BarDataSet {
    pub granularity: BarGranularity,
    pub data: BTreeMap<TS, Bar>,
}

impl BarDataSet {
    fn new(granularity: BarGranularity) -> Self {
        BarDataSet {
            granularity,
            data: BTreeMap::new(),
        }
    }
    fn single_insert(&mut self, timestamp: i64, candle: Bar) {
        self.data.insert(timestamp, candle);
    }
    fn get_range(&self, first_ts: TS, last_ts: TS) -> Result<Vec<&Bar>, Box<dyn Error>> {
        Ok(self
            .data
            .range(first_ts..=last_ts)
            .map(|(k, v)| v)
            .collect())
    }
}

impl DataUpdate for BarDataSet {
    type NewData = Vec<Bar>;

    fn update(&mut self, data: Self::NewData) {
        data.iter()
            .map(|entry| self.data.insert(entry.ts, entry.clone()));
    }
}

impl LocalDataMethods for BarDataSet {
    type Output = Vec<Bar>;

    fn get_timestamp_lookback(&self, lookback_ts: TS) -> Result<Self::Output, Box<dyn Error>> {
        Ok(self
            .data
            .range((Included(lookback_ts), Unbounded))
            .map(|(k, v)| v.clone())
            .collect())
    }

    fn get_timestamp_window(
        &self,
        first_ts: TS,
        last_ts: TS,
    ) -> Result<Self::Output, Box<dyn Error>> {
        Ok(self
            .data
            .range(first_ts..=last_ts)
            .map(|(k, v)| v.clone())
            .collect())
    }
}

#[async_trait]
impl IODataMethods for BarDataSet {
    type Item = Bar;

    async fn from_file_full_dataset(&self, path: &str) -> Result<Vec<Self::Item>, std::io::Error> {
        let mut file = File::open(path).await?;
        let mut contents = String::new();

        file.read_to_string(&mut contents).await?;

        let mut csv_reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(contents.as_bytes());

        let mut bars = Vec::new();
        for result in csv_reader.deserialize() {
            let record: Bar = result.map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))?;
        }

        Ok(bars)
    }
    async fn from_file_by_ts_lookback(
        &self,
        path: &str,
        last_ts: TS,
    ) -> Result<Vec<Self::Item>, std::io::Error> {
        let mut file = File::open(path).await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;

        let mut csv_reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(contents.as_bytes());

        let mut bars = Vec::new();

        for result in csv_reader.deserialize() {
            let record: Bar = result.map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))?;
            if record.ts >= last_ts {
                bars.push(record);
            }
        }
        Ok(bars)
    }
    async fn from_file_by_ts_window(
        &self,
        path: &str,
        first_ts: TS,
        last_ts: TS,
    ) -> Result<Vec<Self::Item>, std::io::Error> {
        let mut file = File::open(path).await?;
        let mut contents = String::new();

        file.read_to_string(&mut contents).await?;

        let mut csv_reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(contents.as_bytes());

        let mut bars = Vec::new();

        for result in csv_reader.deserialize() {
            let record: Bar = result.map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))?;
            if record.ts >= first_ts && record.ts <= last_ts {
                bars.push(record);
            }
        }

        Ok(bars)
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
        first_ts: TS,
        last_entry: TS,
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
    pub ts: TS,
}

#[derive(Debug, Clone)]
pub struct BookDataSet {
    data: BTreeMap<TS, NormalizedBook>,
}
impl BookDataSet {
    fn new(granularity: BarGranularity) -> Self {
        BookDataSet {
            data: BTreeMap::new(),
        }
    }
    fn single_insert(&mut self, timestamp: TS, book: NormalizedBook) {
        self.data.insert(timestamp, book);
    }
}

impl DataUpdate for BookDataSet {
    type NewData = Vec<NormalizedBook>;

    fn update(&mut self, data: Self::NewData) {
        data.iter()
            .map(|entry| self.data.insert(entry.ts, entry.clone()));
    }
}

impl LocalDataMethods for BookDataSet {
    type Output = Vec<NormalizedBook>;

    fn get_timestamp_lookback(&self, lookback_ts: TS) -> Result<Self::Output, Box<dyn Error>> {
        Ok(self
            .data
            .range((Included(lookback_ts), Unbounded))
            .map(|(k, v)| v.clone())
            .collect())
    }
    fn get_timestamp_window(
        &self,
        first_ts: TS,
        last_ts: TS,
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
        let mut file = File::open(path).await?;
        let mut contents = String::new();

        file.read_to_string(&mut contents).await?;

        let mut csv_reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(contents.as_bytes());

        let mut book = Vec::new();
        for result in csv_reader.deserialize() {
            let record: NormalizedBook =
                result.map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))?;
        }

        Ok(book)
    }
    async fn from_file_by_ts_lookback(
        &self,
        path: &str,
        last_ts: TS,
    ) -> Result<Vec<Self::Item>, std::io::Error> {
        let mut file = File::open(path).await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;

        let mut reader = ReaderBuilder::new().from_reader(contents.as_bytes());

        let mut book = Vec::new();
        for result in reader.deserialize() {
            let record: NormalizedBook =
                result.map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))?;

            if record.ts >= last_ts {
                book.push(record);
            }
        }

        Ok(book)
    }
    async fn from_file_by_ts_window(
        &self,
        path: &str,
        first_ts: TS,
        last_ts: TS,
    ) -> Result<Vec<Self::Item>, std::io::Error> {
        let mut file = File::open(path).await?;
        let mut contents = String::new();

        file.read_to_string(&mut contents).await?;

        let mut csv_reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(contents.as_bytes());

        let mut book = Vec::new();

        for result in csv_reader.deserialize() {
            let record: NormalizedBook =
                result.map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))?;
            if record.ts >= first_ts && record.ts <= last_ts {
                book.push(record);
            }
        }

        Ok(book)
    }
    async fn from_db_all_entries(
        &self,
        conn: &PgConnection,
    ) -> Result<Vec<Self::Item>, diesel::result::Error> {
        unimplemented!();
    }
    async fn db_ts_window(
        &self,
        conn: &PgConnection,
        first_ts: TS,
        last_entry: TS,
    ) -> Result<Vec<Self::Item>, diesel::result::Error> {
        unimplemented!();
    }
}

//NOTE:
// Tick data is read-in from file into a local VecDeque, each VecDeque of data is stored in a
// Vec<VecDeque NormalizedTick>
//

#[derive(Debug, Clone)]
pub struct TickDataSet {
    identifier: String,
    data: VecDeque<NormalizedTicks>,
}

impl TickDataSet {
    pub fn new(identifier: String) -> Self {
        TickDataSet {
            identifier,
            data: VecDeque::new(),
        }
    }
    pub fn new_with_capacity(identifier: String, capacity: usize) -> Self {
        TickDataSet {
            identifier,
            data: VecDeque::with_capacity(capacity),
        }
    }
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    pub fn set_identifier(&mut self, new_identifier: String) {
        self.identifier = new_identifier;
    }

    pub fn get(&self, index: usize) -> Option<NormalizedTicks> {
        self.data.get(index).cloned()
    }

    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn truncate(&mut self, len: usize) {
        //NOTE: shortens keeping first len elements
        self.data.truncate(len);
    }
    pub fn sort_by_timestamp(&mut self) {
        self.data
            .make_contiguous()
            .sort_by(|a, b| a.tx_ts.cmp(&b.tx_ts));
    }

    pub fn back_timestamp(&self) -> Option<TS> {
        let back = self.data.back().and_then(|x| Some(x.tx_ts));
        Some(back?)
    }

    pub fn binary_search_timestamp_index(
        &mut self,
        target_timestamp: TS,
    ) -> Result<usize, Box<dyn Error>> {
        self.sort_by_timestamp();

        match self
            .data
            .binary_search_by_key(&target_timestamp, |entry| entry.tx_ts)
        {
            Ok(index) => Ok(index),
            Err(_) => Err("Binary search TickDataSet Error: TS not found".into()),
        }
    }

    // find nearest index, next greater element, if exact match not required
    pub fn find_nearest_ts_index(&mut self, target_timestamp: TS) -> usize {
        self.sort_by_timestamp();

        match self
            .data
            .binary_search_by_key(&target_timestamp, |entry| entry.tx_ts)
        {
            Ok(index) => index,
            Err(index) => index,
        }
    }

    pub fn get_data_by_timestamp_lookback(&mut self, first_ts: TS) -> Result<Vec<NormalizedTicks>> {
        let start_index = self.find_nearest_ts_index(first_ts);
        let end_index = self.data.len();

        Ok(self.data.range(start_index..end_index).cloned().collect())
    }

    pub fn get_data_by_timestamp_window(
        &self,
        first_ts: TS,
        last_ts: TS,
    ) -> Result<Vec<NormalizedTicks>> {
        Ok(self
            .data
            .iter()
            .filter(|tick| tick.tx_ts >= first_ts && tick.tx_ts <= last_ts)
            .cloned()
            .collect())
    }
}

#[async_trait]
impl IODataMethods for TickDataSet {
    type Item = NormalizedTicks;

    async fn from_file_full_dataset(&self, path: &str) -> Result<Vec<Self::Item>, std::io::Error> {
        unimplemented!();
    }
    async fn from_file_by_ts_lookback(
        &self,
        path: &str,
        last_ts: TS,
    ) -> Result<Vec<Self::Item>, std::io::Error> {
        unimplemented!();
    }
    async fn from_file_by_ts_window(
        &self,
        path: &str,
        first_ts: TS,
        last_ts: TS,
    ) -> Result<Vec<Self::Item>, std::io::Error> {
        unimplemented!();
    }
    async fn from_db_all_entries(
        &self,
        conn: &PgConnection,
    ) -> Result<Vec<Self::Item>, diesel::result::Error> {
        unimplemented!();
    }
    async fn db_ts_window(
        &self,
        conn: &PgConnection,
        first_ts: TS,
        last_entry: TS,
    ) -> Result<Vec<Self::Item>, diesel::result::Error> {
        unimplemented!();
    }
}

#[derive(Debug, Clone)]
pub struct NormalizedTicks {
    pub symbol: String,
    pub side: Side,
    pub px: f64,
    pub qty: f64,
    pub local_ids: u32,
    pub server_id: i64,
    pub tx_ts: TS,
}

#[derive(Debug, Clone)]
pub enum NormalizedTypes {
    Bar(BarDataSet),
    Orderbook(BookDataSet),
    Ticks(TickDataSet),
}

#[derive(Debug, Clone)]
pub enum Granularity {
    Bar(BarGranularity),
    Ticks(TickGranularity),
    OB(OBGranularity),
}

#[derive(Debug, Clone)]
pub enum BarGranularity {
    OneMinute,
    FiveMinute,
    FifteenMinute,
    ThirtyMinute,
    OneHour,
    FourHour,
    Daily,
}

#[derive(Debug, Clone)]
pub struct TickGranularity {
    data_length: u64,
}

#[derive(Debug, Clone)]
pub struct OBGranularity {
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
