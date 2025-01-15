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
pub const GH_01: &str = "gh_01";
pub const TRANSFORMS: [&str; 4] = [DROP, SKIP, IDENTITY, GH_01];

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
                let custom_value_path = format!("assets/values/{}.json", transform);
                let value = if std::fs::exists(&custom_value_path).expect("Failed to access path") {
                    let txt =
                        std::fs::read_to_string(custom_value_path).expect("Failed to read file");
                    serde_json::from_str(&txt).expect("Failed to deserialize value")
                } else {
                    test_value.clone()
                };
                assert_eq!(
                    expected,
                    approach.transform(transform, &value),
                    "{}/{}",
                    approach.name(),
                    transform,
                );
            }
        }
    }
}
