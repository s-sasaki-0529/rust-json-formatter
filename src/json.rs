use std::collections::HashMap;

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
    /**
     * JSON全体を整形した文字列を返す
     */
    pub fn format(&self, indent: usize) -> String {
        let mut formatted = String::new();
        self.format_value(indent, &mut formatted);
        return formatted;
    }

    /**
     * JSONに含まれる値を整形した文字列を返す
     * オブジェクトや配列の場合、再帰的に整形を繰り返す
     */
    fn format_value(&self, indent: usize, formatted: &mut String) {
        match self {
            JsonValue::Object(obj) => {}
            JsonValue::Array(array) => {}
            JsonValue::String(str) => {}
            JsonValue::Number(num) => {
                formatted.push_str(&num.to_string());
            }
            JsonValue::True => {
                formatted.push_str("true");
            }
            JsonValue::False => {
                formatted.push_str("false");
            }
            JsonValue::Null => {
                formatted.push_str("null");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_value_true() {
        let value = JsonValue::True;
        assert_eq!(value.format(2), "true");
    }

    #[test]
    fn test_format_value_false() {
        let value = JsonValue::False;
        assert_eq!(value.format(2), "false");
    }

    #[test]
    fn test_format_value_null() {
        let value = JsonValue::Null;
        assert_eq!(value.format(2), "null");
    }

    #[test]
    fn test_format_value_number() {
        let value = JsonValue::Number(123.456);
        assert_eq!(value.format(2), "123.456");
    }
}
