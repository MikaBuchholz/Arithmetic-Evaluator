#![allow(clippy::needless_return)]
use std::{collections::VecDeque, f32::consts::PI};

use super::lexer::{Token, TokenKind};

#[derive(Debug)]
pub enum ParseError {
    MissingExpression,
    ParensMismatch,
    ExpressionEmpty,
    DivisionByZero,
    UnexpectedOperator(TokenKind),
}

impl ParseError {
    pub fn message(&self) -> String {
        match self {
            ParseError::MissingExpression => String::from("Expression is incomplete!"),
            ParseError::ParensMismatch => String::from("Parenthesis not closed or never opened!"),
            ParseError::ExpressionEmpty => String::from("Expression can not be empty"),
            ParseError::DivisionByZero => String::from("Can not divide by 0"),
            Self::UnexpectedOperator(op) => format!("Unexpected operator: `{:?}`", op),
        }
    }
}

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        return Self;
    }

    fn balance_operators(&mut self, tokens: Vec<Token>) -> Result<Vec<Token>, ParseError> {
        let mut out = vec![];

        // - + => error
        // + - => -
        // - - => +

        let mut skip_next = false;
        for (index, token) in tokens.clone().into_iter().enumerate() {
            match token.kind() {
                TokenKind::Minus => {
                    let next = tokens
                        .get(index + 1)
                        .ok_or(ParseError::MissingExpression)?
                        .kind();

                    if matches!(next, TokenKind::Minus) {
                        skip_next = true;
                        out.push(Token::new(TokenKind::Plus));
                    }
                    if matches!(next, TokenKind::Plus) {
                        return Err(ParseError::UnexpectedOperator(TokenKind::Plus));
                    }

                    if !skip_next {
                        skip_next = false;
                        out.push(token)
                    }
                }
                TokenKind::Plus => {
                    let next = tokens
                        .get(index + 1)
                        .ok_or(ParseError::MissingExpression)?
                        .kind();
                    if matches!(next, TokenKind::Minus) {
                        skip_next = true;
                        out.push(Token::new(TokenKind::Minus));
                    }
                    if matches!(next, TokenKind::Plus) {
                        return Err(ParseError::UnexpectedOperator(TokenKind::Plus));
                    }

                    if !skip_next {
                        skip_next = false;
                        out.push(token)
                    }
                }
                _ => out.push(token),
            }
        }
        println!("{out:?}");
        return Ok(out);
    }

    fn insert_parenthesis(&mut self, tokens: Vec<Token>) -> Result<Vec<Token>, ParseError> {
        let mut out = Vec::new();
        let mut skip_next = false;
        for (index, token) in tokens.clone().into_iter().enumerate() {
            match token.kind() {
                TokenKind::Cos | TokenKind::Sin | TokenKind::Tan | TokenKind::Log => {
                    out.push(Token::new(TokenKind::OpenParen));
                    out.push(token);

                    let next = tokens.get(index + 1).ok_or(ParseError::MissingExpression)?;

                    out.push(next.to_owned());
                    out.push(Token::new(TokenKind::CloseParen));

                    skip_next = true;
                }
                TokenKind::Power => {
                    if index == 0 {
                        return Err(ParseError::MissingExpression);
                    }

                    out.remove(index - 1);

                    out.push(Token::new(TokenKind::OpenParen));

                    out.push(tokens[index - 1].clone());
                    out.push(token);

                    let next = tokens.get(index + 1).ok_or(ParseError::MissingExpression)?;

                    out.push(next.to_owned());
                    out.push(Token::new(TokenKind::CloseParen));

                    skip_next = true;
                }
                // - + => -
                // + - => -
                // - - => +
                _ => {
                    if skip_next {
                        skip_next = false;
                        continue;
                    }

                    out.push(token.clone());
                }
            }
        }
        //println!("{out:?}");
        return Ok(out);
    }

    //reverse polish notation
    pub fn execute(&self, mut tokens: VecDeque<Token>) -> Result<f32, ParseError> {
        let mut number_stack = vec![];

        while let Some(token) = tokens.pop_back() {
            if let TokenKind::Pi = token.kind().clone() {
                number_stack.push(PI);
                continue;
            }
            if token.kind().is_number() {
                if let TokenKind::Number(value) = token.kind().clone() {
                    number_stack.push(value)
                }
            } else {
                let mut is_div = false;
                let operation: Box<dyn Fn(f32, f32) -> f32> = match token.kind() {
                    super::lexer::TokenKind::Identifier(ident) => {
                        continue;
                    }
                    super::lexer::TokenKind::Plus => Box::new(|x, y| x + y),
                    super::lexer::TokenKind::Asterix => Box::new(|x, y| x * y),
                    super::lexer::TokenKind::Minus => Box::new(|x, y| y - x),
                    super::lexer::TokenKind::Power => Box::new(|x, y| y.powf(x)),
                    super::lexer::TokenKind::Slash => {
                        is_div = true;
                        Box::new(|x, y| y / x)
                    }
                    super::lexer::TokenKind::Sin => {
                        let rhs = number_stack.pop().unwrap();

                        number_stack.push(rhs.sin());
                        continue;
                    }

                    super::lexer::TokenKind::Cos => {
                        let rhs = number_stack.pop().unwrap();

                        number_stack.push(rhs.cos());
                        continue;
                    }

                    super::lexer::TokenKind::Tan => {
                        let rhs = number_stack.pop().unwrap();

                        number_stack.push(rhs.tan());
                        continue;
                    }

                    super::lexer::TokenKind::Log => {
                        let rhs = number_stack.pop().unwrap();

                        number_stack.push(rhs.log10());
                        continue;
                    }
                    unknown => unreachable!("Unhandled token: `{:?}`", unknown),
                };

                let lhs = number_stack.pop();

                let rhs = number_stack.pop();

                if lhs.is_none() || rhs.is_none() {
                    return Err(ParseError::MissingExpression);
                }

                if is_div && lhs.unwrap() == 0.0 {
                    return Err(ParseError::DivisionByZero);
                }

                number_stack.push(operation(lhs.unwrap(), rhs.unwrap()))
            }
        }

        return Ok(number_stack.pop().unwrap_or(0.0));
    }

    //shunting yard
    pub fn parse(&mut self, stream: Vec<Token>) -> Result<VecDeque<Token>, ParseError> {
        let with_parens = self.insert_parenthesis(stream)?;
        let with_ops_balanced = self.balance_operators(with_parens)?;

        for t in with_ops_balanced.clone() {
            print!("{}", t.kind());
        }
        println!();

        let token_stream = with_ops_balanced.into_iter().peekable();
        let mut output_queue = VecDeque::new();
        let mut operator_stack = vec![];

        for token in token_stream {
            if token.kind().is_number() {
                output_queue.push_front(token.clone())
            }

            if token.kind().is_operator() {
                if !operator_stack.is_empty() {
                    let mut top: &Token = &operator_stack[operator_stack.len() - 1];

                    let token_precedence = token.kind().precedence();
                    let top_token_precedence = top.kind().precedence();

                    while !top.kind().is_open_paren() && top_token_precedence >= token_precedence {
                        output_queue.push_front(operator_stack.pop().unwrap());

                        if let Some(tok) = &operator_stack.last() {
                            top = tok
                        } else {
                            break;
                        }
                    }
                }

                operator_stack.push(token.clone())
            }

            if token.kind().is_open_paren() {
                operator_stack.push(token.clone())
            }

            if token.kind().is_close_paren() {
                let mut top: &Token = &operator_stack[operator_stack.len() - 1]; //<- o2

                while !top.kind().is_open_paren() {
                    if operator_stack.is_empty() {
                        return Err(ParseError::ParensMismatch);
                    }
                    output_queue.push_front(operator_stack.pop().unwrap());
                    top = &operator_stack[operator_stack.len().saturating_sub(1)];
                }

                if !operator_stack[operator_stack.len() - 1]
                    .kind()
                    .is_open_paren()
                {
                    return Err(ParseError::ParensMismatch);
                }

                operator_stack.pop();
            }
        }

        while let Some(token) = operator_stack.pop() {
            output_queue.push_front(token)
        }

        return Ok(output_queue);
    }
}
