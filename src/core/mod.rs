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
    /// Registers a low-level handler
    fn register_core_handler(
        &mut self,
        handler_name: &str,
        handler_closure: Box<dyn FnMut(&str) -> Result<String, String>>,
    );
    /// Registers a function call
    fn register_func(
        &mut self,
        func_name: &str,
        mut handler_closure: Box<dyn FnMut(&Vec<ScriptValue>) -> Result<ScriptValue, ScriptError>>,
    ) {
        let core_handler_name = format!("func${}${}", func_name, uuid::Uuid::new_v4());
        self.register_core_handler(
            &core_handler_name,
            Box::new(move |data_str: &str| {
                let args: ScriptValue =
                    serde_json::from_str(data_str).map_err(|err| err.to_string())?;
                let args = args
                    .as_array()
                    .ok_or("Couldn't convert args to array of values to pass in rust")?;
                let res = handler_closure(&args).map_err(|err| err.to_string())?;
                return Ok(res.to_string());
            }),
        );
        let src = format!(
            "(ScriptIt.core.registerFunc('{}', '{}'), null)",
            func_name, core_handler_name
        );
        self.eval_expression(&src).unwrap();
    }
}
