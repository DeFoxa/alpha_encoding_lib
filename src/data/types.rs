use crate::features::DataMethods;
//TODO: port logic/types for Normalized Types, and based on todo's in features, write logic for
//candle creation and usage API

#[derive(Debug, Clone)]
pub struct Candle;

impl DataMethods for Candle {
    fn get_by_window(&self, n: &str, granularity: Granularity) -> NormalizedTypes {
        todo!();
    }
}

#[derive(Debug, Clone)]
pub struct NormalizedBook;

impl DataMethods for NormalizedBook {
    fn get_by_window(&self, n: &str, granularity: Granularity) -> NormalizedTypes {
        todo!();
    }
}
#[derive(Debug, Clone)]
pub struct NormalizedTrades;

impl DataMethods for NormalizedTrades {
    fn get_by_window(&self, n: &str, granularity: Granularity) -> NormalizedTypes {
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
