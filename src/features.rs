use crate::data::types::*;

//TODO: determine if we are going to locally build candles, if so write logic. or only work with
//normalized data types. Could also write logic to handle both cases, make decision.

pub trait Executable {
    fn execute(&self, context: &ExecutionContext) -> f64;
}

pub trait DataMethods {
    fn get_by_window(&self, n: &str, granularity: Granularity) -> NormalizedTypes;
}

#[derive(Debug)]
pub struct ExecutionContext {
    data: DataContext,
    parameters: ExecutionParameters,
    estimated_fees: Option<f64>,
    estimated_slippage: Option<f64>,
}

impl ExecutionContext {
    pub fn new(data_context: DataContext, parameters: ExecutionParameters) -> Self {
        ExecutionContext {
            data: data_context,
            parameters,
            estimated_fees: None,
            estimated_slippage: None,
        }
    }
    pub fn update_fees(&mut self, fees: f64) {
        self.estimated_fees = Some(fees);
    }
    pub fn update_slippage(&mut self, slippage: f64) {
        self.estimated_slippage = Some(slippage);
    }
}

#[derive(Debug)]
pub enum DataSource {
    RealTime(NormalizedTypes),
    Historical(NormalizedTypes),
}

#[derive(Debug)]
pub struct DataContext {
    pub data: NormalizedTypes,
    pub active_source: DataSource,
}
impl DataContext {
    pub fn new(data: NormalizedTypes, source: DataSource) -> Self {
        DataContext {
            data,
            active_source: source,
        }
    }

    pub fn update_realtime_data(&mut self, data: NormalizedTypes) {
        todo!();
    }
    pub fn update_historical_data(&mut self, data: NormalizedTypes) {
        todo!();
    }
}

impl DataMethods for DataContext {
    fn get_by_window(&self, n: &str, window: Granularity) -> NormalizedTypes {
        todo!();
    }
}

// #[derive(Debug, Clone)]
// pub struct RealTimeDataStructure;
//
// #[derive(Debug, Clone)]
// pub enum HistoricalDataStructure {
//     Candles(Candle),
//     Ticks(NormalizedTrades),
//     OB(NormalizedBook),
// }

// impl HistoricalDataStructure {
//     fn
// }

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
