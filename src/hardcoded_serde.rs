use serde_json::Value;

use super::Transform;

/// Hardcoded serde transformer used as reference for speed comparison
pub struct Transformer;

impl Transformer {
    pub fn new() -> Self {
        Self
    }
}

impl Transform for Transformer {
    fn transform(&self, transformation: &str, value: &Value) -> Value {
        match transformation {
            "drop" => serde_json::from_str("[]"), //Value::Array(vec![])
            "identity" => serde_json::from_str(&serde_json::to_string(value).expect("Failed to serialize value")), //value.clone()
            "skip" => serde_json::from_str("null"), //Value::Null,
            _ => panic!("Unknown transform: {}", transformation),
        }.expect("Failed to deserialize value")
    }
}
