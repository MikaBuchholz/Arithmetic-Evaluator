use super::{
    lexer::Lexer,
    parser::{ParseError, Parser},
};
use colored::*;
use std::io::{self, Write};

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
            buffer = buffer.replace(['\r', '\n'], "");

            match self.interpret(buffer.as_str()) {
                Ok(number) => {
                    let colored = format!("{number:?}").green();
                    println!("=> {colored}")
                }
                Err(err) => println!(
                    "{}",
                    format!(
                        "{}: {}",
                        "Error".to_string().underline(),
                        format!("{}", err.message().red())
                    )
                    .red()
                ),
            }

            buffer.clear();
        }

        return Ok(());
    }

    pub fn interpret(&mut self, expression: &'a str) -> Result<f32, ParseError> {
        if expression.is_empty() {
            return Err(ParseError::ExpressionEmpty);
        }
        let token_stream = self.lexer.lex(expression);

        //println!("{:?}", token_stream);
        let token_queue = self.parser.parse(token_stream)?;

        return Ok(self.parser.execute(token_queue))?;
    }
}
