// Error handling for Julia the Viper
use thiserror::Error;

pub type Result<T> = std::result::Result<T, JtvError>;

#[derive(Error, Debug, Clone)]
pub enum JtvError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Type error: {0}")]
    TypeError(String),

    #[error("Runtime error: {0}")]
    RuntimeError(String),

    #[error("Purity violation: {0}")]
    PurityViolation(String),

    #[error("Totality violation: {0}")]
    TotalityViolation(String),

    #[error("Undefined variable: {0}")]
    UndefinedVariable(String),

    #[error("Undefined function: {0}")]
    UndefinedFunction(String),

    #[error("Arity mismatch: expected {expected}, got {got}")]
    ArityMismatch { expected: usize, got: usize },

    #[error("Division by zero")]
    DivisionByZero,

    #[error("Integer overflow")]
    IntegerOverflow,

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Code injection attempt detected: {0}")]
    InjectionAttempt(String),

    #[error("Maximum iteration count exceeded (possible infinite loop)")]
    MaxIterationsExceeded,

    #[error("IO error: {0}")]
    IoError(String),
}

impl From<std::io::Error> for JtvError {
    fn from(err: std::io::Error) -> Self {
        JtvError::IoError(err.to_string())
    }
}

impl From<std::num::ParseIntError> for JtvError {
    fn from(err: std::num::ParseIntError) -> Self {
        JtvError::ParseError(format!("Failed to parse integer: {}", err))
    }
}

impl From<std::num::ParseFloatError> for JtvError {
    fn from(err: std::num::ParseFloatError) -> Self {
        JtvError::ParseError(format!("Failed to parse float: {}", err))
    }
}
