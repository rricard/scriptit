//! Core constructs available on all platforms

/// Contains the main error type
pub mod error;
/// Contains the main value type
pub mod value;

use error::ScriptError;
use value::ScriptValue;

pub trait ScriptingEnvironment {
    /// Evaluates a single JS expression
    fn eval_expression(&mut self, source: &str) -> Result<ScriptValue, ScriptError>;
    /// Runs JavaScript code
    fn run(&mut self, source: &str) -> Result<(), ScriptError>;
}
