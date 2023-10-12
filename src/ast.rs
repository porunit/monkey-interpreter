use crate::token::Token;
use std::fmt;

pub enum Node {
    Program(Vec<Statement>),
    Stmt(Statement),
    Expr(Expression),
}

pub enum Statement {
    Let(String, Expression),
    Return(Expression),
    Expr(Expression),
}

pub type BlockStatement = Vec<Statement>;

pub enum Expression {
    Ident(String),
    Lit(Literal),
    Prefix(Token, Box<Expression>),
    Infix(Token, Box<Expression>, Box<Expression>),
    If(Box<Expression>, BlockStatement, Option<BlockStatement>),
    Fn(Vec<String>, BlockStatement),
    FnCall(Box<Expression>, Vec<Expression>),
    Index(Box<Expression>, Box<Expression>),
}

pub enum Literal {
    Integer(i32),
    Boolean(bool),
    String(String),
    Array(Vec<Expression>),
    Hash(Vec<(Expression, Expression)>),
}
