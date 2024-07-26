use crate::{data::types::*, traits::*};
use diesel::prelude::*;
use diesel::PgConnection;
use std::error;

//TODO: determine if we are going to build bars locally, if so write logic. or only work with
//normalized data types. Could also write logic to handle both cases, make decision later.

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
}

#[derive(Debug)]
pub struct ExecutionParameters;
#[derive(Debug, Clone)]
pub enum Operation {
    MovingAverage(MA),
    CrossOver(CrossOverComponents),
    Momentum(MomentumTypes),
    Arb(ArbTypes),
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
