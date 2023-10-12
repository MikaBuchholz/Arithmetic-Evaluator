use ast::interpreter::Interpreter;

mod ast;

//todo:
//parse floats i.e 1.233 and negative numbers i.e -6.3 and expressions like 1 - -5
//make sin,cos etc usable
//error report on 0 division
//clean up

fn main() -> std::io::Result<()> {
    let mut inter = Interpreter::new();

    //println!("{}", inter.interpret("17+114/32-89*13-51/52-12"))
    inter.console()?;

    return Ok(());
}
