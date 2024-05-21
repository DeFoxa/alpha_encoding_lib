use crate::{data::types::*, traits::*};
use diesel::prelude::*;
use diesel::PgConnection;
use std::error;

//TODO: determine if we are going to locally build candles, if so write logic. or only work with
//normalized data types. Could also write logic to handle both cases, make decision.

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::types::{Candle, NormalizedTypes};
    use std::path::PathBuf;

    fn setup_test_env() -> (DataContext, PathBuf) {
        let data = NormalizedTypes::Candles(vec![]);
        let source = DataSource::Historical(data.clone());
        let ctx = DataContext::new(data, source);
        let path = PathBuf::from("test_data.csv");

        (ctx, path)
    }

    #[test]
    fn test_from_file_full_dataset() {
        // Old: changed methods
        // let (context, path) = setup_test_env();
        // assert!(context
        //     .from_file_full_dataset(path.to_str().unwrap())
        //     .is_ok());
    }
    #[test]
    fn test_timestamp_lookback() {
        let (ctx, _) = setup_test_env();

        // NOTE: old
        // let result = ctx.get_timestamp_lookback(1622548800);
        // assert!(result.is_ok());

        //TODO: Fix testing to match new DataSet types, replace old version implemented for
        // Vec<Type>
        // match result {
        //     Ok(NormalizedTypes::Candles(candles)) if !candles.is_empty() => {
        //         assert!(true, "candles data retrieved successfully")
        //     }
        //     Ok(_) => assert!(
        //         false,
        //         "data retrieved, but it was not candle data or was empty"
        //     ),
        //     Err(e) => assert!(false, "Error: {:?}", e),
        // }
    }
}
