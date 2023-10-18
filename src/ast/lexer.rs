#![allow(clippy::needless_return)]

use core::fmt;
use std::{f32::consts::PI, iter::Peekable, str::Chars};

use colored::Colorize;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Number(f32),
    Plus,
    Asterix,
    Minus,
    Slash,
    Power,
    Sin,
    Cos,
    Tan,
    Pi,
    Log,
    Identifier(String),

    OpenParen,
    CloseParen,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TokenKind::Number(num) => format!("{num:?}"),
            TokenKind::Plus => " + ".to_string(),
            TokenKind::Asterix => " * ".to_string(),
            TokenKind::Minus => " - ".to_string(),
            TokenKind::Slash => "/".to_string(),
            TokenKind::Power => "^".to_string(),
            TokenKind::Sin => "Sin ".to_string(),
            TokenKind::Cos => "Cos ".to_string(),
            TokenKind::Tan => "Tan ".to_string(),
            TokenKind::Pi => PI.to_string(),
            TokenKind::Identifier(ident) => ident.to_string(),
            TokenKind::OpenParen => "(".to_string(),
            TokenKind::CloseParen => ")".to_string(),
            TokenKind::Log => "Log ".to_string(),
        };

        write!(f, "{}", s)
    }
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
                | TokenKind::Tan
                | TokenKind::Power
                | TokenKind::Log
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
            TokenKind::Plus | TokenKind::Minus => 1,
            TokenKind::Slash | TokenKind::Asterix => 2,
            TokenKind::OpenParen | TokenKind::CloseParen | TokenKind::Power => 3,
            TokenKind::Sin | TokenKind::Cos | TokenKind::Tan | TokenKind::Log => 4,
            TokenKind::Pi => 5,
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

    fn skip_while(&mut self, input: &mut Peekable<Chars<'a>>, target: char) -> Option<()> {
        if self.peek(input)? == target {
            let mut next = self.next_char(input)?;
            while next == target {
                if self.peek(input)? != target {
                    break;
                }
                next = self.next_char(input)?;
            }
        }

        return Some(());
    }

    fn next_token(&mut self, input: &mut Peekable<Chars<'a>>) -> Option<Token> {
        if self.current_pos >= self.len {
            return None;
        }

        self.skip_while(input, ' ');

        let current_char = self.next_char(input)?;

        if current_char.is_numeric() {
            let number = String::from(current_char);

            if let Some(peeked_chr) = self.peek(input) {
                if peeked_chr != '.' && !peeked_chr.is_numeric() {
                    return Some(Token::new(TokenKind::Number(
                        number.parse::<f32>().unwrap(),
                    )));
                }
            }

            let number = self.collect_until(current_char, |x| x != '.' && !x.is_numeric(), input);

            return Some(Token::new(TokenKind::Number(
                number.parse::<f32>().unwrap(),
            )));
        }

        if current_char.is_alphabetic() {
            let alpha = self.collect_until(current_char, |x| !x.is_alphabetic(), input);

            let math_fn = match alpha.to_lowercase().as_str() {
                "sin" => TokenKind::Sin,
                "cos" => TokenKind::Cos,
                "tan" => TokenKind::Tan,
                "pi" => TokenKind::Pi,
                "log" => TokenKind::Log,
                identifier => TokenKind::Identifier(identifier.to_string()),
            };

            return Some(Token::new(math_fn));
        }

        let mut operator: Option<TokenKind> = None;

        match current_char {
            '*' => operator = Some(TokenKind::Asterix),
            '-' => operator = Some(TokenKind::Minus),
            '+' => operator = Some(TokenKind::Plus),
            '/' => operator = Some(TokenKind::Slash),
            '^' => operator = Some(TokenKind::Power),
            '(' => operator = Some(TokenKind::OpenParen),
            ')' => operator = Some(TokenKind::CloseParen),
            unknown => {
                if !unknown.is_alphabetic() {
                    println!(
                        "{}",
                        format!(
                            "{}: Unknown symbol: `{}`",
                            "Warning".to_string().underline(),
                            format!("{unknown}").bold()
                        )
                        .yellow()
                    )
                }
            }
        };

        if let Some(op) = operator {
            return Some(Token::new(op));
        }

        //Variable area

        return None;
    }

    fn peek(&mut self, input: &mut Peekable<Chars<'a>>) -> Option<char> {
        return input.peek().copied();
    }

    fn next_char(&mut self, input: &mut Peekable<Chars<'a>>) -> Option<char> {
        let next = input.next();

        if self.current_pos < self.len {
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
