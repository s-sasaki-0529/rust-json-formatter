#[derive(Debug, PartialEq)]
pub enum Token {
    LeftBrace,      // {
    RightBrace,     // }
    LeftBracket,    // [
    RightBracket,   // ]
    Colon,          // :
    Comma,          // ,
    String(String), // "string"
    Number(f64),    // 123, 45.67
    True,           // true
    False,          // false
    Null,           // null
}

pub struct Lexer<'a> {
    input: &'a str,       // 字句解析対象の文字列全体
    position: usize,      // 解析中の現在の文字位置
    read_position: usize, // 解析中の次の文字位置
    ch: Option<char>,     // 現在解析中の文字 (None は EOF)
}
