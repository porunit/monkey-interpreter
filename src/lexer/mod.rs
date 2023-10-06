use crate::token::Token;

#[derive(Default)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect::<Vec<char>>(),
            ..Default::default()
        };
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            '=' => {
                if self.peak_char() == '=' {
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            '!' => {
                if self.peak_char() == '=' {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            }
            '-' => Token::Minus,
            '/' => Token::Slash,
            '*' => Token::Asterisk,
            '<' => Token::LT,
            '>' => Token::GT,
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            ':' => Token::Colon,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '\0' => Token::EOF,
            c => {
                if is_letter(c) {
                    let id = self.read_identifier();
                    return match id.as_str() {
                        "let" => Token::Let,
                        "fn" => Token::Fn,
                        "true" => Token::Bool(true),
                        "false" => Token::Bool(false),
                        "if" => Token::If,
                        "else" => Token::Else,
                        "return" => Token::Ret,
                        _ => Token::Ident(id)
                    };
                } else if c.is_digit(10) {
                    let n = self.read_numeric();
                    Token::Integer(n)
                } else {
                    Token::Illegal
                }
            }
        };
        self.read_char();
        token
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }
    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].iter().collect()
    }
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peak_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '0'
        } else {
            self.input[self.read_position]
        }
    }
    fn read_numeric(&mut self) -> i32 {
        let pos = self.position;
        while self.ch.is_digit(10) {
            self.read_char();
        }
        let end = self.position;
        self.read_position -= 1; // ???
        self.input[pos..end]
            .iter()
            .collect::<String>().parse::<i32>().expect("Unexpected char in number seq")
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_delimiters() {
        let input = "(){}[],;:";
        let mut tokens = Lexer::new(input);

        let expected = vec![
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::LBracket,
            Token::RBracket,
            Token::Comma,
            Token::Semicolon,
            Token::Colon,
            Token::EOF,
        ];

        for token_expected in expected.iter() {
            let token = tokens.next_token();
            assert_eq!(&token, token_expected);
        }
    }

    #[test]
    fn test_tokenize_operators() {
        let input = "+ = == ! != - / * < >";
        let mut tokens = Lexer::new(input);

        let expected = vec![
            Token::Plus,
            Token::Assign,
            Token::Eq,
            Token::Bang,
            Token::NotEq,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::LT,
            Token::GT,
            Token::EOF,
        ];

        for token_expected in expected.iter() {
            let token = tokens.next_token();
            assert_eq!(&token, token_expected);
        }
    }

    #[test]
    fn test_tokenize_keywords() {
        let input = "if else true false return";
        let mut tokens = Lexer::new(input);

        let expected = vec![
            Token::If,
            Token::Else,
            Token::Bool(true),
            Token::Bool(false),
            Token::Ret,
            Token::EOF,
        ];

        for token_expected in expected.iter() {
            let token = tokens.next_token();
            assert_eq!(&token, token_expected);
        }
    }

    #[test]
    fn test_tokenize_program() {
        let input = "let five = 5; \
                     let ten = 10; \
                     let add = fn(x, y) { x + y;}; \
                     let result = add(five, ten);";

        let mut tokens = Lexer::new(input);

        let expected = vec![
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Integer(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".to_string()),
            Token::Assign,
            Token::Integer(10),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".to_string()),
            Token::Assign,
            Token::Fn,
            Token::LParen,
            Token::Ident("x".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Ident("y".to_string()),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".to_string()),
            Token::Assign,
            Token::Ident("add".to_string()),
            Token::LParen,
            Token::Ident("five".to_string()),
            Token::Comma,
            Token::Ident("ten".to_string()),
            Token::RParen,
            Token::Semicolon,
            Token::EOF,
        ];

        for token_expected in expected.iter() {
            let token = tokens.next_token();
            assert_eq!(&token, token_expected);
        }
    }

    #[test]
    fn test_tokenize_string() {
        let input = "\"foobar\"";
        let mut tokens = Lexer::new(input);

        let expected = vec![Token::String("foobar".to_string())];

        for token_expected in expected.iter() {
            let token = tokens.next_token();
            assert_eq!(&token, token_expected);
        }
    }
}