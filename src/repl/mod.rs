use std::io::stdin;
use crate::lexer::Lexer;
use crate::token::Token;

const PROMPT: &str = ">> ";

pub fn start() {
    loop {
        eprint!("{}", PROMPT);
        let mut scanned = String::new();
        stdin().read_line(&mut scanned).expect("TODO: panic message");
        let mut lexer = Lexer::new(scanned.as_str());
        let mut token: Token = lexer.next_token();
        while token != Token::EOF {
            println!("{}", token);
            token = lexer.next_token();
        }
    }
}