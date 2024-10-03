use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<_>>(); 

    if args.len() > 2 {
        println!("Usage: rox [script]");
        process::exit(64); 
    } else if args.len() == 2 {
        run_file(args[1].clone()); 
    } else {
        todo!(); 
    }
}

fn run_file(path: String) {
    let source = std::fs::read_to_string(&path).unwrap(); 
    run(source); 
}

struct Scanner {
    source: String,
}

impl Scanner {
    fn scan_tokens(self) -> Vec<Token> {
        let mut iter = self.source.char_indices().peekable();
        let mut tokens = Vec::new(); 

        while let Some((byte, char)) = iter.next() {
            match char {
                '(' => tokens.push(Token::LeftParen), 
                ')' => tokens.push(Token::RightParen), 
                _ => {}
            }
        }
        tokens
    }
}

fn run(source: String) {
    let scanner = Scanner {source}; 
    let tokens = scanner.scan_tokens(); 
    for token in tokens {
        println!("{token:?}"); 
    }
}

#[derive(Debug)]
#[allow(unused)]
enum Token {
    // single character token 
    LeftParen, 
    RightParen, 
    LeftBrace,
    RightBrace, 
    Comma, 
    Dot, 
    Minus, 
    Plus,
    Semicolon,
    Slash, 
    Star, 

    // one or two character tokens
    Bang, 
    BangEqual, 
    Equal,
    EqualEqual, 
    Greater, 
    GreaterEqual, 
    Less, 
    LessEqual, 

    // literal
    Identifier(String), 
    String(String), 
    Number(f64), 

    // keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,   
}