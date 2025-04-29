use serde_json::Value;

pub trait Executable {
    fn execute(&self, input: Value) -> Result<Value, String>;
}
