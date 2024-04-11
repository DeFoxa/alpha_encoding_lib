use crate::data::types::*;

pub trait Executable {
    fn execute(&self, context: &ExecutionContext) -> f64;
}

#[derive(Debug)]
pub struct ExecutionContext {
    data: DataContext,
    parameters: ExecutionParameters,
    estimated_fees: Option<f64>,
    estimated_slippage: Option<f64>,
}

#[derive(Debug)]
pub enum DataSource {
    RealTime,
    Historical,
}

#[derive(Debug)]
pub struct DataContext {
    real_time_data: Option<RealTimeDataStructure>,
    historical_data: Option<HistoricalDataStructure>,
    active_source: DataSource,
}
impl DataContext {
    pub fn new() -> Self {
        todo!();
    }
    pub fn update_realtime_data(&mut self, data: RealTimeDataStructure) {
        self.real_time_data = Some(data);
        self.active_source = DataSource::RealTime;
    }
    pub fn update_historical_data(&mut self, data: HistoricalDataStructure) {
        self.historical_data = Some(data);
        self.active_source = DataSource::Historical;
    }
    pub fn get_data_window(&self, symbol: &str, n: usize, granularity: Option<CandleGranularity>) {
        match self.active_source {
            DataSource::RealTime => {
                todo!();
            }
            DataSource::Historical => {
                todo!();
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct RealTimeDataStructure;

#[derive(Debug, Clone)]
pub enum HistoricalDataStructure {
    Candle(Candle),
    Tick(NormalizedTrades),
    OB(NormalizedBook),
}

pub enum CandleGranularity {
    OneMinute,
    FiveMinute,
    FifteenMinute,
    ThirtyMinute,
    OneHour,
    FourHour,
    Daily,
}

#[derive(Debug)]
pub struct ExecutionParameters;

#[derive(Debug, Clone)]
pub enum Operation {
    MovingAverage(MA),
    CrossOver(CrossOverComponents),
    Momentum(MomentumTypes),
    Arbitrage(ArbTypes),
    Pairs,
    Custom,
}

#[derive(Debug, Clone)]
pub struct MA {
    period: usize,
}

#[derive(Debug, Clone)]
pub struct CrossOverComponents {
    components: Vec<MA>,
}

#[derive(Debug, Clone)]
pub enum MomentumTypes {
    TimeSeries,
    CrossSectional,
}

#[derive(Debug, Clone)]
pub enum ArbTypes {
    Basis,
    CrossExchange,
    Triangle,
    Funding,
    Statistical,
}
