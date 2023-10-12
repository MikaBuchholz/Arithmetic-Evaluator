mod ast;

#[cfg(test)]
mod tests {
    use crate::ast::interpreter::Interpreter;

    #[test]
    fn calculation_should_ok_1() {
        let mut inter = Interpreter::new();

        assert_eq!(1.0 + 1.0, inter.interpret("1 + 1").unwrap())
    }

    #[test]
    fn calculation_should_ok_2() {
        let mut inter = Interpreter::new();

        assert_eq!(10.0 * 10.0 - 1.0, inter.interpret("10 * 10 - 1").unwrap())
    }

    #[test]
    fn calculation_should_ok_3() {
        let mut inter = Interpreter::new();

        assert_eq!(
            10.0 * (10.0 + 1.0),
            inter.interpret("10 * (10 + 1)").unwrap()
        )
    }

    #[test]
    fn calculation_should_ok_4() {
        let mut inter = Interpreter::new();

        assert_eq!(
            33.0 * 10.0 - (10.0 / 2.0 / 2.0 + 1.0) * 3.0 - 1.0,
            inter
                .interpret("33 * 10 - (10 / 2 / 2 + 1) * 3 - 1")
                .unwrap()
        )
    }

    #[test]
    fn no_space_calculation_should_ok_1() {
        let mut inter = Interpreter::new();

        assert_eq!(1.0 + 1.0, inter.interpret("1+1").unwrap())
    }

    #[test]
    fn no_space_calculation_should_ok_2() {
        let mut inter = Interpreter::new();

        assert_eq!(10.0 * 10.0 - 1.0, inter.interpret("10*10-1").unwrap())
    }

    #[test]
    fn no_space_calculation_should_ok_3() {
        let mut inter = Interpreter::new();

        assert_eq!(10.0 * (10.0 + 1.0), inter.interpret("10*(10+1)").unwrap())
    }

    #[test]
    fn no_space_calculation_should_ok_4() {
        let mut inter = Interpreter::new();

        assert_eq!(
            33.0 * 10.0 - (10.0 / 2.0 / 2.0 + 1.0) * 3.0 - 1.0,
            inter.interpret("(33*10-(10/2/2+1)*3-1)").unwrap()
        )
    }

    #[test]
    #[should_panic]
    fn no_close_parn_should_panic() {
        let mut inter = Interpreter::new();

        inter.interpret("10*(10+1").unwrap();
    }

    #[test]
    #[should_panic]
    fn empty_should_panic() {
        let mut inter = Interpreter::new();

        inter.interpret("").unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_statement_should_panic_1() {
        let mut inter = Interpreter::new();

        inter.interpret("1 +").unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_statement_should_panic_2() {
        let mut inter = Interpreter::new();

        inter.interpret("+ 1").unwrap();
    }
}
