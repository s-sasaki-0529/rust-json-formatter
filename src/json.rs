use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Object(JsonObject), // {"key": "value"}
    Array(JsonArray),   // [1, 2, 3]
    String(String),     // "hello, world"
    Number(f64),        // 123.456
    True,               // true
    False,              // false
    Null,               // null
}

pub type JsonObject = HashMap<String, JsonValue>;
pub type JsonArray = Vec<JsonValue>;

impl JsonValue {
    pub fn pretty(&self, indent: usize) -> String {
        let indent_str = " ".repeat(indent);

        match self {
            JsonValue::Object(obj) => String::from(""),
            JsonValue::Array(array) => String::from(""),
            JsonValue::String(str) => String::from(""),
            JsonValue::Number(num) => String::from(""),
            JsonValue::True => String::from(""),
            JsonValue::False => String::from(""),
            JsonValue::Null => String::from(""),
        }
    }
}
