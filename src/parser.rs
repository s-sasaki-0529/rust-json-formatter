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
}
