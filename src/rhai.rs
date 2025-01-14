use crate::Transform;
use rhai::{serde::to_dynamic, Dynamic, Engine, Scope, AST};
use serde_json::Value;
use std::fs;

pub struct Transformer {
    engine: Engine,
    scripts: std::collections::HashMap<String, AST>,
}

impl Default for Transformer {
    fn default() -> Self {
        let mut scripts = std::collections::HashMap::new();
        let mut engine = Engine::new();
        //engine.set_strict_variables(true); //fails because variables from scope are not available at compile time
        engine.set_optimization_level(rhai::OptimizationLevel::Full);

        for transform in super::TRANSFORMS {
            let path = format!("transformations/rhai/{}.rhai", transform);
            if !std::fs::exists(&path).expect("Failed to access path") {
                continue;
            }

            let script = fs::read_to_string(path).expect("Failed to read Rhai script");
            scripts.insert(
                transform.to_string(),
                engine
                    .compile(&script)
                    .expect("Failed to compile Rhai script"),
            );
        }

        Self { engine, scripts }
    }
}

impl Transform for Transformer {
    fn transform(&self, transform: &str, value: &Value) -> Value {
        let script = self.scripts.get(transform).expect("Transform not found");
        let mut scope = Scope::new();
        scope.push(
            "value",
            to_dynamic(value).expect("Failed to convert JSON to Rhai value"),
        );

        let result = &self
            .engine
            .eval_ast_with_scope::<Dynamic>(&mut scope, script)
            .expect("Failed to execute Rhai script");
        serde_json::to_value(result).expect("Failed to convert Rhai result to JSON")
        // from_dynamic(result).expect("Failed to convert Rhai result to JSON")
    }

    fn name(&self) -> &str {
        "rhai"
    }

    fn accept(&self, transformation: &str) -> bool {
        self.scripts.contains_key(transformation)
    }
}
