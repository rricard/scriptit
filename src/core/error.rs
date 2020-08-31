#[derive(Debug)]
pub enum ScriptError {
    CastError {
        type_from: &'static str,
        type_to: &'static str,
    },
    CompileError(String),
    RuntimeError(String),
}

impl std::fmt::Display for ScriptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScriptError::CastError { type_from, type_to } => write!(
                f,
                "ScriptError::CastError: Casting from `{}` to `{}` failed!",
                type_from, type_to
            ),
            ScriptError::CompileError(msg) => write!(f, "ScriptError::CompileError: {}", msg),
            ScriptError::RuntimeError(msg) => write!(f, "ScriptError::RuntimeError: {}", msg),
        }
    }
}

impl std::error::Error for ScriptError {}
