pub mod handlebars;
pub mod hardcoded_serde;
pub mod lua;
pub mod rhai;
pub mod rune;
pub mod tera;
pub mod vrl;

use serde_json::Value;

pub const DROP: &str = "drop";
pub const IDENTITY: &str = "identity";
pub const SKIP: &str = "skip";
pub const TRANSFORMS: [&str; 3] = [DROP, IDENTITY, SKIP];

pub trait Transform {
    fn name(&self) -> &str;
    fn accept(&self, transformation: &str) -> bool;
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

        let approach1 = hardcoded_serde::Transformer;
        let approaches: Vec<Box<dyn Transform>> = vec![
            Box::new(handlebars::Transformer::default()),
            Box::new(tera::Transformer::default()),
            Box::new(vrl::Transformer::default()),
            Box::new(rhai::Transformer::default()),
            Box::new(lua::Transformer::default()),
            Box::new(rune::Transformer::default()),
        ];
        for transform in TRANSFORMS {
            let expected = approach1.transform(transform, &test_value);
            for approach in approaches.iter() {
                if !approach.accept(transform) {
                    continue;
                }
                assert_eq!(
                    expected,
                    approach.transform(transform, &test_value),
                    "{}/{}",
                    approach.name(),
                    transform,
                );
            }
        }
    }
}
