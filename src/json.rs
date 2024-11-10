use indexmap::IndexMap;

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

pub type JsonObject = IndexMap<String, JsonValue>;
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
            JsonValue::Object(obj) => {
                self.push_str(formatted, "{\n");
                for (i, (key, value)) in obj.iter().enumerate() {
                    self.push_indent(formatted, indent + 2);
                    self.push_str(formatted, "\"");
                    self.push_str(formatted, key);
                    self.push_str(formatted, "\"");
                    self.push_str(formatted, ": ");
                    value.format_value(indent + 2, formatted);
                    if i < obj.len() - 1 {
                        self.push_str(formatted, ",\n");
                    } else {
                        self.push_str(formatted, "\n");
                    }
                }
                self.push_indent(formatted, indent);
                formatted.push_str("}")
            }
            JsonValue::Array(array) => {
                self.push_str(formatted, "[\n");
                for (i, value) in array.iter().enumerate() {
                    self.push_indent(formatted, indent + 2);
                    value.format_value(indent + 2, formatted);
                    if i < array.len() - 1 {
                        self.push_str(formatted, ",\n");
                    } else {
                        self.push_str(formatted, "\n");
                    }
                }
                self.push_indent(formatted, indent);
                formatted.push_str("]")
            }
            JsonValue::String(str) => {
                formatted.push('"');
                formatted.push_str(str);
                formatted.push('"');
            }
            JsonValue::Number(num) => {
                let value = &num.to_string();
                self.push_str(formatted, value);
            }
            JsonValue::True => {
                self.push_str(formatted, "true");
            }
            JsonValue::False => {
                self.push_str(formatted, "false");
            }
            JsonValue::Null => {
                self.push_str(formatted, "null");
            }
        }
    }

    fn push_str_with_indent(&self, formatted: &mut String, indent: usize, str: &str) {
        self.push_indent(formatted, indent);
        formatted.push_str(str);
    }

    fn push_str(&self, formatted: &mut String, str: &str) {
        formatted.push_str(str);
    }

    fn push_indent(&self, formatted: &mut String, indent: usize) {
        for _ in 0..indent {
            formatted.push_str(" ");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_value_true() {
        let value = JsonValue::True;
        assert_eq!(value.format(0), "true");
    }

    #[test]
    fn test_format_value_false() {
        let value = JsonValue::False;
        assert_eq!(value.format(0), "false");
    }

    #[test]
    fn test_format_value_null() {
        let value = JsonValue::Null;
        assert_eq!(value.format(0), "null");
    }

    #[test]
    fn test_format_value_number() {
        let value1 = JsonValue::Number(123.0);
        assert_eq!(value1.format(0), "123");

        let value2 = JsonValue::Number(123.456);
        assert_eq!(value2.format(0), "123.456");
    }

    #[test]
    fn test_format_value_string() {
        let value = JsonValue::String("hello, world".to_string());
        assert_eq!(value.format(0), "\"hello, world\"");
    }

    #[test]
    fn test_format_value_array() {
        let value = JsonValue::Array(vec![
            JsonValue::Number(1.0),
            JsonValue::Number(2.0),
            JsonValue::Number(3.0),
        ]);
        let expected = r#"[
  1,
  2,
  3
]"#;
        assert_eq!(value.format(0), expected);
    }

    #[test]
    fn test_format_value_array_nested() {
        let value = JsonValue::Array(vec![
            JsonValue::Number(1.1),
            JsonValue::Number(1.2),
            JsonValue::Array(vec![
                JsonValue::Number(2.1),
                JsonValue::Number(2.2),
                JsonValue::Array(vec![JsonValue::Number(3.1), JsonValue::Number(3.2)]),
            ]),
        ]);
        let expected = r#"[
  1.1,
  1.2,
  [
    2.1,
    2.2,
    [
      3.1,
      3.2
    ]
  ]
]"#;
        assert_eq!(value.format(0), expected);
    }

    #[test]
    fn test_format_value_object() {
        let mut object = IndexMap::new();
        object.insert("key1".to_string(), JsonValue::Number(123.0));
        object.insert("key2".to_string(), JsonValue::String("value".to_string()));
        let value = JsonValue::Object(object);
        let expected = r#"{
  "key1": 123,
  "key2": "value"
}"#;
        assert_eq!(value.format(0), expected);
    }

    #[test]
    fn test_format_value_object_nested() {
        let mut object = IndexMap::new();
        let mut nested_object = IndexMap::new();
        nested_object.insert("key3".to_string(), JsonValue::Number(456.0));
        nested_object.insert("key4".to_string(), JsonValue::Number(789.0));
        object.insert("key1".to_string(), JsonValue::Number(123.0));
        object.insert("key2".to_string(), JsonValue::Object(nested_object));

        let value = JsonValue::Object(object);
        let expected = r#"{
  "key1": 123,
  "key2": {
    "key3": 456,
    "key4": 789
  }
}"#;
        assert_eq!(value.format(0), expected);
    }

    fn test_format_value_mixed() {
        let mut object = IndexMap::new();
        let mut nested_object1 = IndexMap::new();
        nested_object1.insert("key2".to_string(), JsonValue::Number(2.0));
        nested_object1.insert("key3".to_string(), JsonValue::Number(3.0));

        let mut nested_object2 = IndexMap::new();
        nested_object2.insert(
            "key5".to_string(),
            JsonValue::Array(vec![JsonValue::Number(5.0), JsonValue::Number(6.0)]),
        );

        object.insert(
            "key1".to_string(),
            JsonValue::Array(vec![
                JsonValue::Number(1.0),
                JsonValue::Object(nested_object1),
                JsonValue::Array(vec![JsonValue::Number(4.0), JsonValue::Object(nested_object2)]),
            ]),
        );
        let value = JsonValue::Object(object);
        let expected = r#"{
  "key1": [
    1,
    {
      "key2": 2
      "key3": 3
    },
    [
      4,
      {
        "key5": [
          5,
          6
        ]
      }
    ]
  ],
}"#;
        assert_eq!(value.format(0), expected);
    }
}
