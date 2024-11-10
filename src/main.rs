mod json;
mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;
use std::io::{self, Read};

fn main() {
    // 標準入力からJSON文字列を読み込む
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("テキストの読み込みに失敗しました");

    // 字句解析+構文解析
    let lexer = Lexer::new(&input);
    let mut parser = Parser::new(lexer);
    let json = parser.parse().expect("JSONのパースに失敗しました");

    // パース結果を標準出力
    println!("{}", json.format(0));
}
