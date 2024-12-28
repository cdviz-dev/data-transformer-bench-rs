use handlebars::Handlebars;
use serde_json::Value;
use std::fs;

pub const DROP: &str = "drop";
pub const IDENTITY: &str = "identity";
pub const SKIP: &str = "skip";
pub const TRANSFORMS: [&str; 3] = [DROP, IDENTITY, SKIP];

pub struct CloneTransformer;

impl CloneTransformer {
    pub fn new() -> Self {
        Self
    }

    pub fn transform(&self, _transform: &str, value: &Value) -> Value {
        // Placeholder implementation
        value.clone()
    }
}

pub struct HandlebarsTransformer {
    handlebars: Handlebars<'static>,
}

impl HandlebarsTransformer {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();
        // Register the json helper
        handlebars_misc_helpers::register(&mut handlebars);

        for transform in TRANSFORMS {
            let template = fs::read_to_string(&format!("transform/handlebars/{}.hbs", transform))
                .expect("Failed to read template file");
            handlebars
                .register_template_string(transform, &template)
                .expect("Failed to register template");
        }
        Self { handlebars }
    }

    pub fn transform(&self, transform: &str, value: &Value) -> Value {
        let rendered = self.handlebars
            .render(transform, value)
            .expect("Failed to render template");
        serde_json::from_str(&rendered)
            .expect("Failed to parse rendered JSON")
    }
}
