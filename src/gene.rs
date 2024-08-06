use crate::features::{ExecutionContext, Operation};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum GeneType {
    Terminal(TerminalData),
    Function(FunctionData),
}

#[derive(Debug, Clone)]
pub enum TerminalData {
    Constant(f64),
    Variable(String),
}
#[derive(Debug, Clone)]
pub enum FunctionData {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
    Sqrt,
    Root { degree: f64 },
}

#[derive(Debug, Clone)]
pub struct FunctionNode {
    operation: FunctionData,
    operands: Vec<Expression>,
}
pub struct Context {
    variables: HashMap<String, f64>,
}
impl Context {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
    pub fn with_variables(variables: HashMap<String, f64>) -> Self {
        Self { variables }
    }
    pub fn push_kv(&mut self, name: &str, value: f64) {
        self.variables.insert(name.to_string(), value);
    }

    pub fn try_get_variable_value(&self, name: &str) -> Result<f64, EvalError> {
        self.variables
            .get(name)
            .cloned()
            .ok_or_else(|| EvalError::UndefinedVariable(name.to_string()))
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Terminal(TerminalData),
    Operation(FunctionNode),
}

impl Expression {
    pub fn evaluate(&self, context: &Context) -> Result<f64, EvalError> {
        match self {
            Expression::Terminal(TerminalData::Constant(value)) => Ok(*value),
            Expression::Terminal(TerminalData::Variable(name)) => {
                Ok(context.try_get_variable_value(name)?)
            }
            Expression::Operation(function_node) => match function_node.operation {
                FunctionData::Add => {
                    let operand_result: Result<Vec<f64>, EvalError> = function_node
                        .operands
                        .iter()
                        .map(|op| op.evaluate(context))
                        .collect();
                    let result = operand_result?.iter().sum();

                    Ok(result)
                }
                FunctionData::Subtract => {
                    if function_node.operands.len() != 2 {
                        return Err(EvalError::IncorrectOperandCount);
                    }
                    let op_values: Vec<f64> = function_node
                        .operands
                        .iter()
                        .map(|op| op.evaluate(context))
                        .collect::<Result<Vec<f64>, EvalError>>()?;
                    Ok(op_values[0] - op_values[1])
                }
                FunctionData::Multiply => {
                    let product: f64 = function_node
                        .operands
                        .iter()
                        .map(|op| op.evaluate(context))
                        .collect::<Result<Vec<f64>, EvalError>>()?
                        .iter()
                        .product();
                    Ok(product)
                }

                FunctionData::Divide => {
                    if function_node.operands.len() != 2 {
                        return Err(EvalError::IncorrectOperandCount);
                    }
                    let op_values: Vec<f64> = function_node
                        .operands
                        .iter()
                        .map(|op| op.evaluate(context))
                        .collect::<Result<Vec<f64>, EvalError>>()?;

                    Ok(op_values[0] / op_values[1])
                }
                FunctionData::Exponent => {
                    if function_node.operands.len() != 2 {
                        return Err(EvalError::IncorrectOperandCount);
                    }
                    let op_values: Vec<f64> = function_node
                        .operands
                        .iter()
                        .map(|op| op.evaluate(context))
                        .collect::<Result<Vec<f64>, EvalError>>()?;

                    Ok(op_values[0].powf(op_values[1]))
                }

                FunctionData::Sqrt => {
                    if let [operand] = function_node.operands.as_slice() {
                        Ok(operand.evaluate(context)?.sqrt())
                    } else {
                        Err(EvalError::IncorrectOperandCount)
                    }
                }

                FunctionData::Root { degree } => {
                    if degree <= 0.0 {
                        return Err(EvalError::InvalidInput);
                    }

                    if function_node.operands.len() != 1 {
                        return Err(EvalError::IncorrectOperandCount);
                    }

                    let value = function_node.operands[0].evaluate(context);
                    Ok(Expression::custom_root(value?, degree))
                }
            },
        }
    }

    pub fn custom_root(x: f64, root: f64) -> f64 {
        x.powf(1.0 / root)
    }
}

#[derive(Debug, Clone)]
pub struct Gene {
    op: Operation,
    gene_type: GeneType,
    context: Option<usize>,
    children: Vec<Box<Gene>>,
}

impl Gene {
    fn new_terminal(op: Operation, value: f64) -> Gene {
        Gene {
            op,
            gene_type: GeneType::Terminal(TerminalData::Constant(value)),
            context: None,
            children: Vec::new(),
        }
    }
    fn new_function_gene(
        op: Operation,
        func: FunctionData,
        children: Vec<Box<Gene>>,
        context: Option<usize>,
    ) -> Gene {
        Gene {
            op,
            gene_type: GeneType::Function(func),
            context,
            children,
        }
    }
}

enum EvalError {
    UnsupportedOperation,
    IncorrectOperandCount,
    InvalidInput,
    UndefinedVariable(String),
    UninitializedContext,
}
