use std::fs;
use serde_json::Value;
use handlebars::Handlebars;

use super::{TRANSFORMS, Transform};

pub struct Transformer {
    pub(crate) handlebars: Handlebars<'static>,
}

impl Transformer {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();
        // Register the json helper
        handlebars_misc_helpers::register(&mut handlebars);

        for transform in TRANSFORMS {
            let template = fs::read_to_string(&format!("transformations/handlebars/{}.hbs", transform))
                .expect("Failed to read template file");
            handlebars
                .register_template_string(transform, &template)
                .expect("Failed to register template");
        }
        Self { handlebars }
    }
}

impl Transform for Transformer {
    fn transform(&self, transform: &str, value: &Value) -> Value {
        let rendered = self.handlebars
            .render(transform, value)
            .expect("Failed to render template");
        serde_json::from_str(&rendered)
            .expect("Failed to parse rendered JSON")
    }
}