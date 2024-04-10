pub trait Executable {
    fn execute(&self, context: &ExecutionContext) -> f64;
}
//TODO: specify context types
pub struct ExecutionContext;

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
