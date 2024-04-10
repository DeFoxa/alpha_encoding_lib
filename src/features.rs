pub trait Executable {
    fn execute(&self, context: &ExecutionContext) -> f64;
}
//TODO: specify context types
pub struct ExecutionContext {
    data: DataContext,
    parameters: ExecutionParameters,
    estimated_fees: Option<f64>,
    estimated_slippage: Option<f64>,
}

pub struct DataContext;
pub struct ExecutionParameters;

pub enum Operation {
    MovingAverage { period: usize },
    CrossOver,
    Momentum(MomentumTypes),
    Arbitrage(ArbTypes),
    Pairs,
    Custom,
}

pub enum MomentumTypes {
    TimeSeries,
    CrossSectional,
}

pub enum ArbTypes {
    Basis,
    CrossExchange,
    Triangle,
    Funding,
    Statistical,
}
