use crate::lexer::{Lexer, Token};
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
     */
    fn parse_object(&mut self) -> Option<JsonValue> {
        None
    }

    /**
     * 配列をパースする
     */
    fn parse_array(&mut self) -> Option<JsonValue> {
        None
    }

    /**
     * 次のトークンを取得する
     */
    fn next_token(&mut self) {
        self.current_token = self.lexer.next_token();
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
}
