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
