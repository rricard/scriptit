#[derive(Debug, PartialEq)]
pub enum ScriptValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    Undefined,
}
