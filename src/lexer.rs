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
     * 次のトークンを取得する
     */
    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        let token: Option<Token> = match self.ch {
            Some('{') => {
                self.read_char();
                Some(Token::LeftBrace)
            }
            Some('}') => {
                self.read_char();
                Some(Token::RightBrace)
            }
            Some('[') => {
                self.read_char();
                Some(Token::LeftBracket)
            }
            Some(']') => {
                self.read_char();
                Some(Token::RightBracket)
            }
            Some(':') => {
                self.read_char();
                Some(Token::Colon)
            }
            Some(',') => {
                self.read_char();
                Some(Token::Comma)
            }
            Some('"') => {
                // ここで後ほど文字列の処理を実装します
                None
            }
            Some(c) if c.is_digit(10) || c == '-' => {
                // TODO: 数値の処理を実装する
                None
            }
            Some(c) if c.is_alphabetic() => {
                // TODO: true, false, null の処理を実装する
                None
            }
            None => return None,
            _ => {
                // 未知の文字
                self.read_char();
                None
            }
        };
        return token;
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

    /**
     * 次の文字を覗き見する(読み進めはしない)
     */
    fn peek_char(&self) -> Option<char> {
        if self.read_position >= self.input.len() {
            return None;
        } else {
            self.input[self.read_position..].chars().next()
        }
    }

    /**
     * ホワイトスペースの間は読み飛ばす
     */
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.ch {
            if ch.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }
}
