#![allow(clippy::needless_return)]

use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Number(f64),
    Plus,
    Asterix,
    Minus,
    Slash,
    Sin,
    Cos,
    Tan,
    Pi,

    OpenParen,
    CloseParen,
}

impl TokenKind {
    pub fn is_number(&self) -> bool {
        matches!(self, TokenKind::Number(_))
    }

    pub fn is_operator(&self) -> bool {
        matches!(
            self,
            TokenKind::Asterix
                | TokenKind::Minus
                | TokenKind::Plus
                | TokenKind::Slash
                | TokenKind::Sin
                | TokenKind::Cos
                | TokenKind::Pi
        )
    }

    pub fn is_open_paren(&self) -> bool {
        matches!(self, TokenKind::OpenParen)
    }

    pub fn is_close_paren(&self) -> bool {
        matches!(self, TokenKind::CloseParen)
    }

    pub fn precedence(&self) -> usize {
        // ( / ) > * > / > + / -
        match self {
            TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Sin
            | TokenKind::Cos
            | TokenKind::Pi => 1,
            TokenKind::Slash | TokenKind::Asterix => 2,
            TokenKind::OpenParen | TokenKind::CloseParen => 3,
            _ => unreachable!("Unknown operator"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    kind: TokenKind,
}

impl Token {
    pub fn new(kind: TokenKind) -> Self {
        return Self { kind };
    }

    pub fn kind(&self) -> &TokenKind {
        return &self.kind;
    }
}

#[derive(Debug)]
pub struct Lexer {
    current_pos: usize,
    len: usize,
}

impl<'a> Lexer {
    pub fn new() -> Self {
        Self {
            current_pos: 0,
            len: 0,
        }
    }

    fn collect_until<F>(
        &mut self,
        current_char: char,
        f: F,
        input: &mut Peekable<Chars<'a>>,
    ) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut buffer = String::from(current_char);

        let mut next = self.next_char(input);

        while let Some(chr) = next {
            if f(chr) {
                break;
            }

            buffer.push(chr);

            if let Some(peeked_chr) = self.peek(input) {
                if f(peeked_chr) {
                    break;
                }
            }

            next = self.next_char(input);
        }

        return buffer;
    }

    fn next_token(&mut self, input: &mut Peekable<Chars<'a>>) -> Option<Token> {
        if self.current_pos >= self.len {
            return None;
        }

        if self.peek(input)? == ' ' {
            let mut next = self.next_char(input)?;
            while next == ' ' {
                if self.peek(input)? != ' ' {
                    break;
                }
                next = self.next_char(input)?;
            }
        }

        let current_char = self.next_char(input)?;

        if current_char.is_numeric() {
            let number = String::from(current_char);

            if let Some(peeked_chr) = self.peek(input) {
                if !peeked_chr.is_numeric() {
                    return Some(Token::new(TokenKind::Number(
                        number.parse::<f64>().unwrap(),
                    )));
                }
            }

            let number = self.collect_until(current_char, |x| !x.is_numeric(), input);

            return Some(Token::new(TokenKind::Number(
                number.parse::<f64>().unwrap(),
            )));
        }

        let mut operator: Option<TokenKind> = None;

        match current_char {
            '*' => operator = Some(TokenKind::Asterix),
            '-' => operator = Some(TokenKind::Minus),
            '+' => operator = Some(TokenKind::Plus),
            '/' => operator = Some(TokenKind::Slash),
            '(' => operator = Some(TokenKind::OpenParen),
            ')' => operator = Some(TokenKind::CloseParen),
            _ => {}
        };

        if let Some(op) = operator {
            return Some(Token::new(op));
        }

        //Variable area

        if current_char.is_alphabetic() {
            let alpha = self.collect_until(current_char, |x| !x.is_alphabetic(), input);

            let mut math_fn = None;

            match alpha.to_lowercase().as_str() {
                "sin" => math_fn = Some(TokenKind::Sin),
                "cos" => math_fn = Some(TokenKind::Cos),
                "tan" => math_fn = Some(TokenKind::Tan),
                "pi" => math_fn = Some(TokenKind::Pi),
                &_ => {}
            }

            if let Some(math_fn) = math_fn {
                return Some(Token::new(math_fn));
            }
        }

        return None;
    }

    fn peek(&mut self, input: &mut Peekable<Chars<'a>>) -> Option<char> {
        return input.peek().copied();
    }

    fn next_char(&mut self, input: &mut Peekable<Chars<'a>>) -> Option<char> {
        let next = input.next();

        if self.current_pos + 1 <= self.len {
            self.current_pos += 1;
        } else {
            return None;
        }

        return next;
    }

    pub fn lex(&mut self, input: &'a str) -> Vec<Token> {
        let mut iter = input.chars().peekable();
        let mut token_stream = vec![];
        self.len = input.len();

        while let Some(token) = self.next_token(&mut iter) {
            token_stream.push(token)
        }

        self.current_pos = 0;

        return token_stream;
    }
    /*
    pub fn collect(&mut self) -> Vec<Token> {
        let mut token_stream = vec![];
        while let Some(token) = self.next_token() {
            token_stream.push(token)
        }

        return token_stream;
    }
    */
}
