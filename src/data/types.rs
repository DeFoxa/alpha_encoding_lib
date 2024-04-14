use crate::traits::{IODataMethods, LocalDataMethods};
use std::error::Error;
//TODO: port logic/types for Normalized Types, and based on todo's in features, write logic for
//candle creation and usage API

#[derive(Debug, Clone)]
pub struct Candle;

impl LocalDataMethods for Candle {
    fn get_by_lookback_window(
        &self,
        n: &str,
        window: Granularity,
    ) -> Result<NormalizedTypes, Box<dyn Error>> {
        todo!();
    }
    fn get_timestamp_lookback(&self, first_ts: i64) -> Result<NormalizedTypes, Box<dyn Error>> {
        todo!();
    }
    fn get_timestamp_window(
        &self,
        first_ts: i64,
        last_ts: i64,
    ) -> Result<NormalizedTypes, Box<dyn Error>> {
        todo!();
    }
}

#[derive(Debug, Clone)]
pub struct NormalizedBook;

impl LocalDataMethods for NormalizedBook {
    fn get_by_lookback_window(
        &self,
        n: &str,
        window: Granularity,
    ) -> Result<NormalizedTypes, Box<dyn Error>> {
        todo!();
    }
    fn get_timestamp_lookback(&self, first_ts: i64) -> Result<NormalizedTypes, Box<dyn Error>> {
        todo!();
    }
    fn get_timestamp_window(
        &self,
        first_ts: i64,
        last_ts: i64,
    ) -> Result<NormalizedTypes, Box<dyn Error>> {
        todo!();
    }
}

#[derive(Debug, Clone)]
pub struct NormalizedTrades;

impl LocalDataMethods for NormalizedTrades {
    fn get_by_lookback_window(
        &self,
        n: &str,
        window: Granularity,
    ) -> Result<NormalizedTypes, Box<dyn Error>> {
        todo!();
    }
    fn get_timestamp_lookback(&self, first_ts: i64) -> Result<NormalizedTypes, Box<dyn Error>> {
        todo!();
    }
    fn get_timestamp_window(
        &self,
        first_ts: i64,
        last_ts: i64,
    ) -> Result<NormalizedTypes, Box<dyn Error>> {
        todo!();
    }
}

#[derive(Debug, Clone)]
pub enum NormalizedTypes {
    Candles(Vec<Candle>),
    Orderbook(Vec<NormalizedBook>),
    Trades(Vec<NormalizedTrades>),
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
