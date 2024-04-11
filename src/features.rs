pub trait Executable {
    fn execute(&self, context: &ExecutionContext) -> f64;
}
//TODO: specify context types
#[derive(Debug)]
pub struct ExecutionContext {
    data: DataContext,
    parameters: ExecutionParameters,
    estimated_fees: Option<f64>,
    estimated_slippage: Option<f64>,
}

#[derive(Debug)]
pub struct DataContext;

#[derive(Debug)]
pub struct ExecutionParameters;

#[derive(Debug, Clone)]
pub enum Operation {
    MovingAverage { period: usize },
    CrossOver,
    Momentum(MomentumTypes),
    Arbitrage(ArbTypes),
    Pairs,
    Custom,
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
