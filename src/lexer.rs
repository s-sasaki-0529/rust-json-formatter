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

impl<'a> Lexer<'a> {
    /**
     * 新しい Lexer を生成する
     */
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };
        return lexer;
    }

    /**
     * 次の文字を読み込み、現在の位置を更新する
     */
    fn read_char(&mut self) {
        // 既に末尾の場合、終了する
        if self.read_position >= self.input.len() {
            self.ch = None;
        }
        // 次の文字があれば読み出す
        else {
            self.ch = self.input[self.read_position..].chars().next();
        }
        // 位置を進める(対象文字のバイト数分進める)
        self.position = self.read_position;
        self.read_position += self.ch.map_or(0, |c| c.len_utf8());
    }
}
