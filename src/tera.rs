use crate::Transform;
use serde_json::Value;
use tera::Tera;

pub struct Transformer {
    pub(crate) tera: Tera,
}

impl Default for Transformer {
    fn default() -> Self {
        let mut tera = Tera::default();
        // Register the templates for each transformation
        for transform in super::TRANSFORMS {
            let template =
                std::fs::read_to_string(format!("transformations/tera/{}.tera", transform))
                    .expect("Failed to read template file");
            tera.add_raw_template(transform, &template)
                .expect("Failed to register template");
        }
        Self { tera }
    }
}

impl Transform for Transformer {
    fn transform(&self, transform: &str, value: &Value) -> Value {
        let mut context = tera::Context::new();
        context.insert("value", value);

        let rendered = self
            .tera
            .render(transform, &context)
            .expect("Failed to render template");
        serde_json::from_str(&rendered).expect("Failed to parse rendered JSON")
    }

    fn name(&self) -> &str {
        "tera"
    }

    fn accept(&self, transformation: &str) -> bool {
        self.tera.get_template(transformation).is_ok()
    }
}
