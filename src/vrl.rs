use super::{Transform, TRANSFORMS};
use serde_json::Value;
use std::fs;
use vrl::{
    compiler::{compile, state::RuntimeState, Context, Function, Program, TargetValue, TimeZone},
    value::{Secrets, Value as VrlValue},
};

pub struct Transformer {
    programs: std::collections::HashMap<String, Program>,
}

impl Transformer {
    pub fn new() -> Self {
        let mut programs = std::collections::HashMap::new();
        let functions: Vec<Box<dyn Function>> = vec![];

        for transform in TRANSFORMS {
            let source = fs::read_to_string(format!("transformations/vrl/{}.vrl", transform))
                .expect("Failed to read VRL script");

            let program = compile(&source, &functions)
                .expect("Failed to compile VRL program")
                .program;

            programs.insert(transform.to_string(), program);
        }

        Self { programs }
    }
}

impl Transform for Transformer {
    fn transform(&self, transform: &str, value: &Value) -> Value {
        let program = self.programs.get(transform).expect("Transform not found");

        let mut state = RuntimeState::default();
        let timezone = TimeZone::default();

        // This is the target that can be accessed / modified in the VRL program.
        // You can use any custom type that implements `Target`, but `TargetValue` is also provided for convenience.
        let mut target = TargetValue {
            value: value.clone().into(), // convert into VrlVvalue
            // the metadata is empty
            metadata: VrlValue::Object(std::collections::BTreeMap::new()),
            // and there are no secrets associated with the target
            secrets: Secrets::default(),
        };
        // A context bundles all the info necessary for the runtime to resolve a value.
        let mut ctx = Context::new(&mut target, &mut state, &timezone);

        // This executes the VRL program, making any modifications to the target, and returning a result.
        let result = program
            .resolve(&mut ctx)
            .expect("Failed to execute VRL program");

        result.try_into().expect("Failed to convert from VRL value")
    }
}