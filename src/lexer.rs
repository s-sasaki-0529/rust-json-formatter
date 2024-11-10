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
        lexer.read_char();
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
                let string = self.read_string();
                Some(Token::String(string))
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
     * 文字列リテラルを読み取る
     * `"` から `"` までの文字列を読み取る
     */
    fn read_string(&mut self) -> String {
        let mut result = String::new();
        self.read_char(); // 現在地が先頭の `"` なので読み飛ばす

        while let Some(ch) = self.ch {
            // 文字列の終端の場合そこで終了
            if ch == '"' {
                self.read_char();
                break;
            }
            // エスケープシーケンスの場合は処理
            if ch == '\\' {
                // 次の文字がシーケンスになるので、対応する文字コードに変換する
                self.read_char();
                if let Some(esc) = self.ch {
                    match esc {
                        '"' => result.push('"'),
                        '\\' => result.push('\\'),
                        '/' => result.push('/'),
                        'b' => result.push('\x08'), // Backspace
                        'f' => result.push('\x0C'), // Form feed
                        'n' => result.push('\n'),   // Line feed
                        'r' => result.push('\r'),   // Carriage return
                        't' => result.push('\t'),   // Horizontal tab
                        'u' => {
                            // Unicode エスケープシーケンスの場合
                            // TODO: 細かい実装はあとで。今は簡易的に次の4文字を読み飛ばす。
                            for _ in 0..4 {
                                self.read_char();
                            }
                        }
                        _ => {} // 未知のエスケープシーケンスは無視する
                    }
                }
            } else {
                // 通常の文字の場合はそのまま追加
                result.push(ch);
            }
            self.read_char(); // 次の文字へ
        }
        return result;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_initialization() {
        let input = r#"{ "[Test]" }"#;
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.input, input);
        assert_eq!(lexer.position, 0);
        assert_eq!(lexer.read_position, 1);
        assert_eq!(lexer.ch, Some('{'));
    }

    #[test]
    fn test_next_token_simple() {
        let input = r#"{ }"#;
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Some(Token::LeftBrace)); // {
        assert_eq!(lexer.next_token(), Some(Token::RightBrace)); // }
    }

    #[test]
    fn test_next_token_string() {
        let input = r#""Hello, World!"#;
        let mut lexer = Lexer::new(input);

        assert_eq!(
            lexer.next_token(),
            Some(Token::String("Hello, World!".to_string()))
        );
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn test_next_token_string_escape1() {
        let input = r#""Hello, \"World\"!""#;
        let mut lexer = Lexer::new(input);

        assert_eq!(
            lexer.next_token(),
            Some(Token::String("Hello, \"World\"!".to_string()))
        );
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn test_next_token_string_escape2() {
        let input = "\"\\b\\f\\n\\r\\t\"";
        let mut lexer = Lexer::new(input);

        assert_eq!(
            lexer.next_token(),
            Some(Token::String("\x08\x0C\n\r\t".to_string()))
        );
        assert_eq!(lexer.next_token(), None);
    }
}
