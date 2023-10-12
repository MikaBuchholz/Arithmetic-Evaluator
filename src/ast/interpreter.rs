use std::io::{self, Write};

use super::{
    lexer::Lexer,
    parser::{ParseError, Parser},
};

pub struct Interpreter {
    lexer: Lexer,
    parser: Parser,
}

impl<'a> Interpreter {
    pub fn new() -> Self {
        return Self {
            lexer: Lexer::new(),
            parser: Parser::new(),
        };
    }

    pub fn console(&mut self) -> std::io::Result<()> {
        let mut buffer = String::new();

        loop {
            print!(">>> ");

            io::stdout().flush()?;

            std::io::stdin().read_line(&mut buffer)?;
            buffer = buffer.replace(&['\r', '\n'], "");

            match self.interpret(buffer.as_str()) {
                Ok(number) => {
                    buffer.clear();
                    println!(">>> {number}")
                }
                Err(err) => println!("Error: {}", err.message()),
            }
        }

        return Ok(());
    }

    pub fn interpret(&mut self, expression: &'a str) -> Result<f64, ParseError> {
        if expression.is_empty() {
            return Err(ParseError::ExpressionEmpty);
        }
        let token_stream = self.lexer.lex(expression);

        println!("{:?}", token_stream);
        let token_queue = self.parser.parse(token_stream)?;

        return Ok(self.parser.execute(token_queue))?;
    }
}
