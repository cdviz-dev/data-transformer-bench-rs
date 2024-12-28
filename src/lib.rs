pub mod handlebars;
pub mod hardcoded_serde;
pub mod tera;

use serde_json::Value;

pub const DROP: &str = "drop";
pub const IDENTITY: &str = "identity";
pub const SKIP: &str = "skip";
pub const TRANSFORMS: [&str; 3] = [DROP, IDENTITY, SKIP];

pub trait Transform {
    fn transform(&self, transformation: &str, value: &Value) -> Value;
}
