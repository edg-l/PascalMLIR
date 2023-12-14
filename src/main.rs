use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

pub mod ast;
pub mod lexer;
pub mod tokens;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use crate::{ast, grammar, lexer::Lexer};

    #[test]
    fn parse_number() {
        #[track_caller]
        fn check(input: &str, value: ast::Number) {
            let lexer = Lexer::new(input);
            let parser = grammar::NumberParser::new();
            assert_eq!(parser.parse("", lexer).unwrap(), value)
        }

        check("2", ast::Number::Integer("2"));
        check("+100", ast::Number::Integer("+100"));
        check("-100", ast::Number::Integer("-100"));
        check("-0.1", ast::Number::Real("-0.1"));
        check("2.2", ast::Number::Real("2.2"));
        check("1e10", ast::Number::Real("1e10"));
        check("-1e10", ast::Number::Real("-1e10"));
        check("87.35E+8", ast::Number::Real("87.35E+8"));
        check("-87.35E+8", ast::Number::Real("-87.35E+8"));
    }

    #[test]
    fn parse_constant() {
        #[track_caller]
        fn check(input: &str, value: ast::Constant) {
            let lexer = Lexer::new(input);
            let parser = grammar::ConstantParser::new();
            assert_eq!(parser.parse("", lexer).unwrap(), value)
        }

        check(
            "MYIDENTIFIER",
            ast::Constant::Identifier {
                is_negative: false,
                ident: "MYIDENTIFIER",
            },
        );
        check(
            "-MYIDENTIFIER",
            ast::Constant::Identifier {
                is_negative: true,
                ident: "MYIDENTIFIER",
            },
        );

        check("2", ast::Constant::Number(ast::Number::Integer("2")));
        check("+100", ast::Constant::Number(ast::Number::Integer("+100")));
        check("-100", ast::Constant::Number(ast::Number::Integer("-100")));

        check("-0.1", ast::Constant::Number(ast::Number::Real("-0.1")));
        check("2.2", ast::Constant::Number(ast::Number::Real("2.2")));
        check("1e10", ast::Constant::Number(ast::Number::Real("1e10")));
    }
}
