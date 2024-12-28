use crate::Transform;
use mlua::{Lua, LuaSerdeExt, Value as LuaValue};
use serde_json::Value;
use std::fs;

pub struct Transformer {
    lua: Lua,
    scripts: std::collections::HashMap<String, String>,
}

impl Transformer {
    pub fn new() -> Self {
        let mut scripts = std::collections::HashMap::new();
        let lua = Lua::new();

        for transform in super::TRANSFORMS {
            let script = fs::read_to_string(format!("transformations/lua/{}.lua", transform))
                .expect("Failed to read Lua script");
            scripts.insert(transform.to_string(), script);
        }

        Self { lua, scripts }
    }
}

impl Transform for Transformer {
    fn transform(&self, transform: &str, value: &Value) -> Value {
        let script = self.scripts.get(transform).expect("Transform not found");

        // Convert JSON value to Lua value
        let lua_value: LuaValue = self
            .lua
            .to_value(value)
            .expect("Failed to convert to Lua value");

        // Create global value in Lua
        self.lua
            .globals()
            .set("value", lua_value)
            .expect("Failed to set global value");

        // Create json utilities
        let globals = self.lua.globals();
        let json_table = self
            .lua
            .create_table()
            .expect("Failed to create json table");
        json_table
            .set("null", mlua::Value::Nil)
            .expect("Failed to set null");
        globals
            .set("json", json_table)
            .expect("Failed to set json global");

        // Execute the script
        let result: LuaValue = self
            .lua
            .load(script)
            .eval()
            .expect("Failed to execute Lua script");
        match result {
            LuaValue::Nil => serde_json::Value::Null,
            r => {
                let a = self
                    .lua
                    .from_value::<Vec<serde_json::Value>>(r)
                    .expect("Failed to convert Lua result to JSON");
                serde_json::Value::Array(a)
            }
        }
    }
}
