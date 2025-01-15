use serde_json::Value;

use super::Transform;

/// Hardcoded serde transformer used as reference for speed comparison
pub struct Transformer;

impl Default for Transformer {
    fn default() -> Self {
        Self
    }
}

impl Transform for Transformer {
    fn transform(&self, transformation: &str, value: &Value) -> Value {
        match transformation {
            "drop" => serde_json::from_str("[]"), //Value::Array(vec![])
            "identity" => serde_json::from_str(&format!(
                "[{}]",
                serde_json::to_string(value).expect("Failed to serialize value")
            )),
            "skip" => serde_json::from_str("null"), //Value::Null,
            "gh_01" => serde_json::from_str(&format!(
                "[{}]",
                serde_json::to_string(&serde_json::json!({
                "context": {
                    "version": "0.5.0-draft",
                    "id": "0",
                    "source": "/api.github.com/repos/octocat/Hello-World/releases",
                    "type": "dev.cdevents.artifact.published.0.3.0-draft",
                    "timestamp": "2013-02-27T19:35:32Z",
                },
                "subject": {
                    "id": "pkg:github/octocat/Hello-World@v1.0.0",
                    "type": "artifact",
                    "content": {
                        "user": "octocat"
                    }
                }
                }))
                .expect("Failed to serialize value")
            )),
            _ => panic!("Unknown transform: {}", transformation),
        }
        .expect("Failed to deserialize value")
    }

    fn name(&self) -> &str {
        "hardcoded_serde"
    }

    fn accept(&self, _transformation: &str) -> bool {
        true
    }
}
