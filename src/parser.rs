use crate::{
    json::{JsonArray, JsonObject, JsonValue},
    lexer::{Lexer, Token},
};
use std::collections::HashMap;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Option<Token>,
}

impl<'a> Parser<'a> {
    /**
     * 新しい Parser を生成する
     */
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer: lexer,
            current_token: None,
        };
        parser.next_token();
        return parser;
    }

    /**
     * JSON全体をパースする
     */
    pub fn parse(&mut self) -> Option<JsonValue> {
        self.parse_value()
    }

    /**
     * JSON値をパースする
     */
    fn parse_value(&mut self) -> Option<JsonValue> {
        match &self.current_token {
            Some(Token::LeftBrace) => self.parse_object(),  // { がオブジェクトの開始
            Some(Token::LeftBracket) => self.parse_array(), // [ が配列の開始
            Some(Token::String(string)) => {
                let cloned_string = string.clone();
                self.next_token();
                Some(JsonValue::String(cloned_string))
            }
            Some(Token::Number(number)) => {
                let copied_number = *number;
                self.next_token();
                Some(JsonValue::Number(copied_number))
            }
            Some(Token::True) => {
                self.next_token();
                Some(JsonValue::True)
            }
            Some(Token::False) => {
                self.next_token();
                Some(JsonValue::False)
            }
            Some(Token::Null) => {
                self.next_token();
                Some(JsonValue::Null)
            }
            _ => None,
        }
    }

    /**
     * オブジェクトをパースする
     * 現在のトークンが { であることが前提
     */
    fn parse_object(&mut self) -> Option<JsonValue> {
        let mut object: JsonObject = HashMap::new();

        // 先頭の { を読み飛ばす
        if !self.next_token_if_current_is(Token::LeftBrace) {
            return None;
        }

        // すぐに } が来る場合は空オブジェクトとして即終了
        if let Some(Token::RightBrace) = self.current_token {
            self.next_token();
            return Some(JsonValue::Object(object));
        }

        // キーバリューのペアの数だけ繰り返す
        loop {
            // key
            let key = if let Some(Token::String(s)) = &self.current_token {
                s.clone()
            } else {
                return None;
            };

            // :
            self.next_token();
            if !self.next_token_if_current_is(Token::Colon) {
                return None;
            }

            // value (値がオブジェクトや配列である場合のためにここで再帰する)
            if let Some(value) = self.parse_value() {
                object.insert(key, value);
            }

            // , なら次のキーバリューに続き } が来たらループ終了
            match &self.current_token {
                Some(Token::Comma) => {
                    self.next_token();
                }
                Some(Token::RightBrace) => {
                    self.next_token();
                    break;
                }
                _ => return None,
            }
        }
        return Some(JsonValue::Object(object));
    }

    /**
     * 配列をパースする
     */
    fn parse_array(&mut self) -> Option<JsonValue> {
        let mut array: JsonArray = Vec::new();

        // 先頭の [ を読み飛ばす
        if !self.next_token_if_current_is(Token::LeftBracket) {
            return None;
        }

        // すぐに ] が来る場合は空配列として即終了
        if let Some(Token::RightBracket) = self.current_token {
            self.next_token();
            return Some(JsonValue::Array(array));
        }

        // 配列の要素の数だけループする
        loop {
            // value (値がオブジェクトや配列である場合のためにここで再帰する)
            if let Some(value) = self.parse_value() {
                array.push(value);
            }

            // , なら次の要素に続き ] が来たらループ終了
            match &self.current_token {
                Some(Token::Comma) => {
                    self.next_token();
                }
                Some(Token::RightBracket) => {
                    self.next_token();
                    break;
                }
                _ => return None,
            }
        }

        return Some(JsonValue::Array(array));
    }

    /**
     * 次のトークンを取得する
     */
    fn next_token(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    /**
     * 現在のトークンが期待されるトークン化を確認してから次のトークンに進む
     * 期待されるトークンでない場合は何もしない
     */
    fn next_token_if_current_is(&mut self, expected_token: Token) -> bool {
        if self.current_token == Some(expected_token) {
            self.next_token();
            return true;
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_initialization() {
        let lexer = Lexer::new(r#"{"key": "value"}"#);
        let parser = Parser::new(lexer);

        assert_eq!(parser.current_token, Some(Token::LeftBrace));
    }

    #[test]
    fn test_parser_next_token() {
        let lexer = Lexer::new(r#"{"key": "value"}"#);
        let mut parser = Parser::new(lexer);

        assert_eq!(parser.current_token, Some(Token::LeftBrace));
        parser.next_token();
        assert_eq!(parser.current_token, Some(Token::String("key".to_string())));
        parser.next_token();
        assert_eq!(parser.current_token, Some(Token::Colon));
        parser.next_token();
        assert_eq!(parser.current_token, Some(Token::String("value".to_string())));
        parser.next_token();
        assert_eq!(parser.current_token, Some(Token::RightBrace));
        parser.next_token();
        assert_eq!(parser.current_token, None);
    }

    #[test]
    fn test_parse_string_simple() {
        let mut parser1 = Parser::new(Lexer::new(r#""Hello, World!""#));
        assert_eq!(parser1.parse(), Some(JsonValue::String("Hello, World!".to_string())));

        let mut parser2 = Parser::new(Lexer::new(r#"-123.1"#));
        assert_eq!(parser2.parse(), Some(JsonValue::Number(-123.1)));

        let mut parser3 = Parser::new(Lexer::new(r#"true"#));
        assert_eq!(parser3.parse(), Some(JsonValue::True));

        let mut parser4 = Parser::new(Lexer::new(r#"false"#));
        assert_eq!(parser4.parse(), Some(JsonValue::False));

        let mut parser5 = Parser::new(Lexer::new(r#"null"#));
        assert_eq!(parser5.parse(), Some(JsonValue::Null));
    }

    #[test]
    fn test_parse_object() {
        let mut parser = Parser::new(Lexer::new(r#"{"str": "hello", "num": -32.054, "array": [1, 2, 3]}"#));
        let object = parser.parse_value();

        let mut expected_object = HashMap::new();
        expected_object.insert("str".to_string(), JsonValue::String("hello".to_string()));
        expected_object.insert("num".to_string(), JsonValue::Number(-32.054));
        expected_object.insert(
            "array".to_string(),
            JsonValue::Array(vec![
                JsonValue::Number(1.0),
                JsonValue::Number(2.0),
                JsonValue::Number(3.0),
            ]),
        );

        assert_eq!(object, Some(JsonValue::Object(expected_object)));
    }

    #[test]
    fn test_parse_object_nested() {
        let mut parser = Parser::new(Lexer::new(r#"{"key": {"nested": "value"}}"#));
        let object = parser.parse_value();

        let mut nested_object = HashMap::new();
        nested_object.insert("nested".to_string(), JsonValue::String("value".to_string()));

        let mut expected_object = HashMap::new();
        expected_object.insert("key".to_string(), JsonValue::Object(nested_object));

        assert_eq!(object, Some(JsonValue::Object(expected_object)));
    }

    #[test]
    fn test_parse_array() {
        let mut parser = Parser::new(Lexer::new(r#"[1, -2, 0.03, true, false, null, { "key": "value" }]"#));
        let array = parser.parse_value();

        let expected_array = vec![
            JsonValue::Number(1.0),
            JsonValue::Number(-2.0),
            JsonValue::Number(0.03),
            JsonValue::True,
            JsonValue::False,
            JsonValue::Null,
            JsonValue::Object(HashMap::from([(
                "key".to_string(),
                JsonValue::String("value".to_string()),
            )])),
        ];

        assert_eq!(array, Some(JsonValue::Array(expected_array)));
    }

    #[test]
    fn test_parse_array_nested() {
        let mut parser = Parser::new(Lexer::new(r#"[1, [2, [3, [4]]]]"#));
        let array = parser.parse_value();

        let expected_array = vec![
            JsonValue::Number(1.0),
            JsonValue::Array(vec![
                JsonValue::Number(2.0),
                JsonValue::Array(vec![
                    JsonValue::Number(3.0),
                    JsonValue::Array(vec![JsonValue::Number(4.0)]),
                ]),
            ]),
        ];

        assert_eq!(array, Some(JsonValue::Array(expected_array)));
    }
}
