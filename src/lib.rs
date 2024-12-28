pub mod handlebars;
pub mod hardcoded_serde;
pub mod tera;
pub mod vrl;

use serde_json::Value;

pub const DROP: &str = "drop";
pub const IDENTITY: &str = "identity";
pub const SKIP: &str = "skip";
pub const TRANSFORMS: [&str; 3] = [DROP, IDENTITY, SKIP];

pub trait Transform {
    fn transform(&self, transformation: &str, value: &Value) -> Value;
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn compare_transformers_output() {
        let test_value = json!({
            "name": "test",
            "value": 42,
            "nested": {
                "array": [1, 2, 3]
            }
        });

        let approach1 = hardcoded_serde::Transformer::new();
        let approach2 = handlebars::Transformer::new();
        let approach3 = tera::Transformer::new();
        let approach4 = vrl::Transformer::new();
        for transform in TRANSFORMS {
            let expected = approach1.transform(transform, &test_value);
            assert_eq!(
                expected,
                approach2.transform(transform, &test_value),
                "{}_handlebars",
                transform
            );
            assert_eq!(
                expected,
                approach3.transform(transform, &test_value),
                "{}_tera",
                transform
            );
            assert_eq!(
                expected,
                approach4.transform(transform, &test_value),
                "{}_vrl",
                transform
            );
        }
    }
}
