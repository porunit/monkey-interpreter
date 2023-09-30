use std::fmt;
use std::fmt::{Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal,
    EOF,

    //Identifiers + literals
    Ident(String),
    Integer(i32),

    //Operators
    Assign, // =
    Plus, // +

    //Delimiters
    Comma, // ,
    Semicolon, // ;
    LParen, // (
    RParen, // )
    LBrace, // {
    RBrace, // }

    //Keywords
    Function,
    Let,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Token::Assign => write!(f, "="),
            Token::Ident(id) => write!(f, "{}", id),
            Token::Integer(i32) => write!(f, "{}", i32),
            Token::Plus => write!(f, "{}", "+"),
            Token::Comma => write!(f, "{}", ","),
            Token::Semicolon => write!(f, "{}", ";"),
            Token::LParen => write!(f, "{}", "("),
            Token::RParen => write!(f, "{}", ")"),
            Token::LBrace => write!(f, "{}", "{"),
            Token::RBrace => write!(f, "{}", "}"),
            Token::Function => write!(f, "{}", "fn"),
            Token::Let => write!(f, "{}", "let"),
            token => write!(f, "{:?}", token),
        }
    }
}