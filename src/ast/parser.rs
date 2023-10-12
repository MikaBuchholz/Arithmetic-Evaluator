#![allow(clippy::needless_return)]
use std::{collections::VecDeque, f32::consts::PI};

use super::lexer::{Token, TokenKind};

#[derive(Debug)]
pub enum ParseError {
    MissingExpression,
    ParensMismatch,
    ExpressionEmpty,
}

impl ParseError {
    pub fn message(&self) -> String {
        match self {
            ParseError::MissingExpression => String::from("Expression is incomplete!"),
            ParseError::ParensMismatch => String::from("Parenthesis not closed or never opened!"),
            ParseError::ExpressionEmpty => String::from("Expression can not be empty"),
        }
    }
}

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        return Self;
    }

    //reverse polish notation
    pub fn execute(&self, mut tokens: VecDeque<Token>) -> Result<f64, ParseError> {
        let mut number_stack = vec![];

        while let Some(token) = tokens.pop_back() {
            if token.kind().is_number() {
                if let TokenKind::Pi = token.kind().clone() {
                    number_stack.push(PI as f64)
                }

                if let TokenKind::Number(value) = token.kind().clone() {
                    number_stack.push(value)
                }
            } else {
                let operation: Box<dyn Fn(f64, f64) -> f64> = match token.kind() {
                    super::lexer::TokenKind::Plus => Box::new(|x, y| x + y),
                    super::lexer::TokenKind::Asterix => Box::new(|x, y| x * y),
                    super::lexer::TokenKind::Minus => Box::new(|x, y| y - x),
                    super::lexer::TokenKind::Slash => Box::new(|x, y| y / x),
                    unknown => unreachable!("Unkown token: `{:?}`", unknown),
                };

                let lhs = number_stack.pop();

                let rhs = number_stack.pop();

                if lhs.is_none() || rhs.is_none() {
                    return Err(ParseError::MissingExpression);
                }

                number_stack.push(operation(lhs.unwrap(), rhs.unwrap()))
            }
        }

        return Ok(number_stack[0]);
    }

    //shunting yard
    pub fn parse(&mut self, stream: Vec<Token>) -> Result<VecDeque<Token>, ParseError> {
        let token_stream = stream.into_iter().peekable();
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
