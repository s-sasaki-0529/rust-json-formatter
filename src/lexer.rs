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
            Some(c) if c.is_digit(10) || c == '-' || c == '+' => {
                let string = self.read_number();
                if let Ok(number) = string.parse::<f64>() {
                    Some(Token::Number(number))
                } else {
                    None
                }
            }
            Some(c) if c.is_alphabetic() => {
                return self.read_literal();
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
                            // 今回は簡易的に4文字読み飛ばすだけにする
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
     * 数値リテラルを読み取る
     * 数値または "-" から始まる数値文字列を読み取る
     */
    fn read_number(&mut self) -> String {
        let mut result = String::new();
        while let Some(ch) = self.ch {
            if ch.is_digit(10) || ch == '.' || ch == '-' || ch == '+' || ch == 'e' || ch == 'E' {
                result.push(ch);
                self.read_char();
            } else {
                break;
            }
        }
        return result;
    }

    /**
     * リテラル (true, false, null) を読み取る
     */
    fn read_literal(&mut self) -> Option<Token> {
        let mut string = String::new();
        while let Some(ch) = self.ch {
            if ch.is_alphabetic() {
                string.push(ch);
                self.read_char();
            } else {
                break;
            }
        }
        return match string.as_str() {
            "true" => Some(Token::True),
            "false" => Some(Token::False),
            "null" => Some(Token::Null),
            _ => None, // 未知のリテラルは無視する
        };
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
        let lexer = Lexer::new(input);

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

        assert_eq!(lexer.next_token(), Some(Token::String("Hello, World!".to_string())));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn test_next_token_string_escape1() {
        let input = r#""Hello, \"World\"!""#;
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Some(Token::String("Hello, \"World\"!".to_string())));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn test_next_token_string_escape2() {
        let input = "\"\\b\\f\\n\\r\\t\"";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Some(Token::String("\x08\x0C\n\r\t".to_string())));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn test_next_token_number1() {
        let input = "12345";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Some(Token::Number(12345.0)));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn test_next_token_number2() {
        let input = "123.45";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Some(Token::Number(123.45)));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn test_next_token_number3() {
        let input = "-123.45";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Some(Token::Number(-123.45)));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn test_next_token_number4() {
        let input = "+123.45e6";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Some(Token::Number(123450000.0)));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn test_next_token_number5() {
        let input = "-123.45E-3";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Some(Token::Number(-0.12345)));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn test_next_token_literal_true() {
        let input = "true";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Some(Token::True));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn test_next_token_literal_false() {
        let input = "false";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Some(Token::False));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn test_next_token_literal_null() {
        let input = "null";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Some(Token::Null));
        assert_eq!(lexer.next_token(), None);
    }
}
