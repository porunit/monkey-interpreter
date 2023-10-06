use std::fmt;
use std::fmt::{Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal,
    EOF,

    //Identifiers + literals
    Ident(String),
    Integer(i32),
    Bool(bool),
    String(String),

    //Operators
    Assign,
    // =
    Plus,
    // +
    Minus,
    // -
    Bang,
    // !
    Asterisk,
    // *
    Slash,
    // /
    Eq,
    // ==
    NotEq, // !=

    LT,
    // <
    GT, // >

    //Delimiters
    Comma,
    // ,
    Semicolon,
    // ;
    Colon,
    // :
    LParen,
    // (
    RParen,
    // )
    LBrace,
    // {
    RBrace,
    // }
    LBracket,
    // [
    RBracket, // ]

    //Keywords
    Fn,
    Let,
    If,
    Else,
    Ret,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Token::Ident(id) => write!(f, "{}", id),
            Token::Integer(i) => write!(f, "{}", i),
            Token::Bool(b) => write!(f, "{}", b),
            Token::String(s) => write!(f, "{}", s),
            Token::Assign => write!(f, "="),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Bang => write!(f, "!"),
            Token::Asterisk => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::LT => write!(f, "<"),
            Token::GT => write!(f, ">"),
            Token::Eq => write!(f, "=="),
            Token::NotEq => write!(f, "!="),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::Colon => write!(f, ":"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::LBracket => write!(f, "["),
            Token::RBracket => write!(f, "]"),
            Token::Fn => write!(f, "fn"),
            Token::Let => write!(f, "let"),
            Token::Ret => write!(f, "return"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            token => write!(f, "{:?}", token)
        }
    }
}