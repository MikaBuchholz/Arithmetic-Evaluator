mod ast;

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use crate::ast::interpreter::Interpreter;

    #[test]
    fn test_calculation_1() {
        let mut inter = Interpreter::new();

        assert_eq!(1.0 + 1.0, inter.interpret("1 + 1").unwrap())
    }

    #[test]
    fn test_calculation_2() {
        let mut inter = Interpreter::new();

        assert_eq!(10.0 * 10.0 - 1.0, inter.interpret("10 * 10 - 1").unwrap())
    }

    #[test]
    fn test_calculation_3() {
        let mut inter = Interpreter::new();

        assert_eq!(
            10.0 * (10.0 + 1.0),
            inter.interpret("10 * (10 + 1)").unwrap()
        )
    }

    #[test]
    fn test_calculation_4() {
        let mut inter = Interpreter::new();

        assert_eq!(
            33.0 * 10.0 - (10.0 / 2.0 / 2.0 + 1.0) * 3.0 - 1.0,
            inter
                .interpret("33.0 * 10.0 - (10.0 / 2.0 / 2.0 + 1.0) * 3.0 - 1.0")
                .unwrap()
        )
    }

    #[test]
    fn no_space_test_calculation_1() {
        let mut inter = Interpreter::new();

        assert_eq!(1.0 + 1.0, inter.interpret("1+1").unwrap())
    }

    #[test]
    fn no_space_test_calculation_2() {
        let mut inter = Interpreter::new();

        assert_eq!(10.0 * 10.0 - 1.0, inter.interpret("10*10-1").unwrap())
    }

    #[test]
    fn no_space_test_calculation_3() {
        let mut inter = Interpreter::new();

        assert_eq!(10.0 * (10.0 + 1.0), inter.interpret("10*(10+1)").unwrap())
    }

    #[test]
    fn no_space_test_calculation_4() {
        let mut inter = Interpreter::new();

        assert_eq!(
            33.0 * 10.0 - (10.0 / 2.0 / 2.0 + 1.0) * 3.0 - 1.0,
            inter.interpret("(33*10-(10/2/2+1)*3-1)").unwrap()
        )
    }

    #[test]
    #[should_panic]
    fn no_close_paren_should_panic() {
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

    #[test]
    fn test_power_1() {
        let mut inter = Interpreter::new();

        assert_eq!(2.0f32.powf(2.0), inter.interpret("2^2").unwrap())
    }

    #[test]
    fn test_power_2() {
        let mut inter = Interpreter::new();

        assert_eq!(2.0f32.powf(2.0) * 3.0, inter.interpret("2^2 * 3").unwrap())
    }

    #[test]
    fn test_pi_1() {
        let mut inter = Interpreter::new();

        assert_eq!(PI, inter.interpret("pi").unwrap())
    }

    #[test]
    fn test_pi_2() {
        let mut inter = Interpreter::new();

        assert_eq!(PI, inter.interpret("PI").unwrap())
    }

    #[test]
    fn test_pi_caluclation_1() {
        let mut inter = Interpreter::new();

        assert_eq!((2.0 * PI), inter.interpret("2 * pi").unwrap())
    }

    #[test]
    fn test_pi_caluclation_2() {
        let mut inter = Interpreter::new();

        // This cast is needed because with f64 there is an inaccuracy at the seventh digit after the .
        assert_eq!(
            (PI - 2.0 * (10.0 * PI)),
            inter.interpret("pi - 2 * (10*pi)").unwrap()
        )
    }

    #[test]
    fn test_sin_calcultion_1() {
        let mut inter = Interpreter::new();

        // This cast is needed because with f64 there is an inaccuracy at the seventh digit after the .
        assert_eq!(1.0_f32.sin(), inter.interpret("sin 1").unwrap())
    }

    #[test]
    fn test_sin_calcultion_2() {
        let mut inter = Interpreter::new();

        // This cast is needed because with f64 there is an inaccuracy at the seventh digit after the .
        assert_eq!(
            1.0_f32.sin() * 2.0_f32.sin(),
            inter.interpret("sin 1 * sin 2").unwrap()
        )
    }

    #[test]
    fn test_cos_calcultion_1() {
        let mut inter = Interpreter::new();

        // This cast is needed because with f64 there is an inaccuracy at the seventh digit after the .
        assert_eq!(1.0_f32.cos(), inter.interpret("cos 1").unwrap())
    }

    #[test]
    fn test_cos_calcultion_2() {
        let mut inter = Interpreter::new();

        // This cast is needed because with f64 there is an inaccuracy at the seventh digit after the .
        assert_eq!(
            1.0_f32.cos() * 2.0_f32.cos(),
            inter.interpret("cos 1 * cos 2").unwrap()
        )
    }

    #[test]
    fn test_tan_calcultion_1() {
        let mut inter = Interpreter::new();

        // This cast is needed because with f64 there is an inaccuracy at the seventh digit after the .
        assert_eq!(1.0_f32.tan(), inter.interpret("tan 1").unwrap())
    }

    #[test]
    fn test_tan_calcultion_2() {
        let mut inter = Interpreter::new();

        // This cast is needed because with f64 there is an inaccuracy at the seventh digit after the .
        assert_eq!(
            1.0_f32.tan() * 2.0_f32.tan(),
            inter.interpret("tan 1 * tan 2").unwrap()
        )
    }

    #[test]
    fn test_log_calculation_1() {
        let mut inter = Interpreter::new();

        assert_eq!(2.0_f32.log10(), inter.interpret("log 2").unwrap())
    }

    #[test]
    fn test_log_calculation_2() {
        let mut inter = Interpreter::new();

        assert_eq!(
            2.0_f32.log10() * 2.0_f32.log10(),
            inter.interpret("log 2 * log 2").unwrap()
        )
    }

    #[test]
    fn test_mixed_calculation_1() {
        let mut inter = Interpreter::new();

        assert_eq!(
            2.0 * 3.0 - 4.0_f32.sin() * (10.0 / 20.0 * 1.0_f32.sin()),
            inter
                .interpret("2 * 3 - (sin 4) * (10 / 20 * (sin 1))")
                .unwrap()
        )
    }

    #[test]
    fn test_mixed_calculation_2() {
        let mut inter = Interpreter::new();

        assert_eq!(
            2.0 * 3.0 - 4.0_f32.sin() * (10.0 / 20.0 * 1.0_f32.cos()),
            inter
                .interpret("2 * 3 - (sin 4) * (10 / 20 * (cos 1))")
                .unwrap()
        )
    }

    #[test]
    fn test_mixed_calculation_3() {
        let mut inter = Interpreter::new();

        assert_eq!(
            2.0 * 3.0 - 4.0_f32.sin() * (10.0 / 20.0 * 1.0_f32.cos()) + 2.0_f32.tan(),
            inter
                .interpret("2 * 3 - (sin 4) * (10 / 20 * (cos 1)) + tan 2")
                .unwrap()
        )
    }
}
