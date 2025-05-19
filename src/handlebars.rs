use handlebars::Handlebars;
use serde_json::Value;
use std::fs;

use super::{TRANSFORMS, Transform};

pub struct Transformer {
    pub(crate) handlebars: Handlebars<'static>,
}

impl Default for Transformer {
    fn default() -> Self {
        let mut handlebars = Handlebars::new();
        // Register the json helper
        handlebars_misc_helpers::register(&mut handlebars);

        for transform in TRANSFORMS {
            let path = format!("transformations/handlebars/{}.hbs", transform);
            if !std::fs::exists(&path).expect("Failed to access path") {
                continue;
            }

            let template = fs::read_to_string(path).expect("Failed to read template file");
            handlebars
                .register_template_string(transform, &template)
                .expect("Failed to register template");
        }
        Self { handlebars }
    }
}

impl Transform for Transformer {
    fn transform(&self, transform: &str, value: &Value) -> Value {
        let rendered = self
            .handlebars
            .render(transform, value)
            .expect("Failed to render template");
        serde_json::from_str(&rendered).expect("Failed to parse rendered JSON")
    }

    fn name(&self) -> &str {
        "handlebars"
    }

    fn accept(&self, transformation: &str) -> bool {
        self.handlebars.get_template(transformation).is_some()
    }
}
