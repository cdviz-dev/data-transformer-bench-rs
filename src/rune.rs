use crate::Transform;
use rune::runtime::RuntimeContext;
use rune::termcolor::{ColorChoice, StandardStream};
use rune::{Context, Source, Sources, Unit};
use rune::{Diagnostics, Vm};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;

pub struct Transformer {
    sources: HashMap<String, Arc<Unit>>,
    _context: Context,
    runtime: Arc<RuntimeContext>,
}

impl Default for Transformer {
    fn default() -> Self {
        let context = Context::with_default_modules().expect("Failed to create Rune context");
        let runtime = Arc::new(context.runtime().expect("Failed to get Rune runtime"));

        let mut sources = HashMap::new();

        for transform in super::TRANSFORMS {
            let script = format!("transformations/rune/{}.rn", transform);
            let unit = load_script(&context, &script);
            sources.insert(transform.to_string(), Arc::new(unit));
        }

        Self {
            _context: context,
            runtime,
            sources,
        }
    }
}

fn load_script<P: AsRef<Path>>(context: &Context, path: P) -> Unit {
    let path = path.as_ref();
    let content = fs::read_to_string(path).expect("Failed to read rune script");
    let mut sources = Sources::new();
    sources
        .insert(Source::new(path.to_string_lossy(), content).expect("Failed to create rune source"))
        .expect("Failed to insert rune source");

    let mut diagnostics = Diagnostics::new();
    let res_unit = rune::prepare(&mut sources)
        .with_context(context)
        .with_diagnostics(&mut diagnostics)
        .build();
    if !diagnostics.is_empty() {
        let mut writer = StandardStream::stderr(ColorChoice::Always);
        diagnostics
            .emit(&mut writer, &sources)
            .expect("Failed to emit diagnostics");
    }
    res_unit.expect("Failed to build Rune program")
}

impl Transform for Transformer {
    fn name(&self) -> &str {
        "rune"
    }

    fn accept(&self, transformation: &str) -> bool {
        self.sources.contains_key(transformation)
    }

    fn transform(&self, transform: &str, value: &Value) -> Value {
        let mut vm = Vm::new(
            self.runtime.clone(),
            self.sources
                .get(transform)
                .expect("Transform not found")
                .clone(),
        );
        let input: rune::Value =
            serde_json::from_value(value.clone()).expect("Failed to convert JSON to Rune value");
        //let input = rune::runtime::Object::from_value(input).expect("Failed to convert JSON to Rune value");
        let output = vm
            //.call(["main"], (rune::to_value(value).expect("Failed to convert JSON to Rune value"),))
            .call(["main"], (input,))
            .expect("Failed to execute Rune program");
        let output: Value =
            serde_json::to_value(output).expect("Failed to convert Rune value to JSON");
        output
    }
}
