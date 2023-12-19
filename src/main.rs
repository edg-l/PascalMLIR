use lalrpop_util::lalrpop_mod;

use crate::lexer::Lexer;

lalrpop_mod!(pub grammar);

pub mod ast;
pub mod lexer;
pub mod tokens;

fn main() {
    println!("Hello, world!");

    let input = r#"
record
    year : 0..2000;
    month : 1..12;
    day : 1..31
end
    "#;

    let lexer = Lexer::new(input);
    let parser = grammar::TypeParser::new();
    let parsed = parser.parse("", lexer).unwrap();
    println!("{:#?}", parsed)
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

        check(r#""hello world""#, ast::Constant::String("\"hello world\""));
        check(
            r#""\"hell\"o world""#,
            ast::Constant::String("\"\\\"hell\\\"o world\""),
        );
    }

    #[test]
    fn parse_constant_definition() {
        #[track_caller]
        fn check(input: &str, value: ast::ConstantDef) {
            let lexer = Lexer::new(input);
            let parser = grammar::ConstantDefParser::new();
            assert_eq!(parser.parse("", lexer).unwrap(), value)
        }

        check(
            "MYIDENTIFIER = 2.2",
            ast::ConstantDef {
                ident: "MYIDENTIFIER",
                value: ast::Constant::Number(ast::Number::Real("2.2")),
            },
        );
    }

    #[test]
    fn parse_simple_type() {
        #[track_caller]
        fn check(input: &str, value: ast::SimpleType) {
            let lexer = Lexer::new(input);
            let parser = grammar::SimpleTypeParser::new();
            assert_eq!(parser.parse("", lexer).unwrap(), value)
        }

        check("Boolean", ast::SimpleType::Boolean);
        check("char", ast::SimpleType::Char);
        check("integer", ast::SimpleType::Integer);
        check("real", ast::SimpleType::Real);
    }

    #[test]
    fn parse_enumerated_type() {
        #[track_caller]
        fn check(input: &str, value: Vec<&str>) {
            let lexer = Lexer::new(input);
            let parser = grammar::EnumeratedTypeParser::new();
            assert_eq!(parser.parse("", lexer).unwrap(), value)
        }

        check("(hello, world)", vec!["hello", "world"]);
        check("(hello,world)", vec!["hello", "world"]);
        check("(hello)", vec!["hello"]);
    }

    #[test]
    #[should_panic]
    fn parse_enumerated_type_panic() {
        #[track_caller]
        fn check(input: &str, value: Vec<&str>) {
            let lexer = Lexer::new(input);
            let parser = grammar::EnumeratedTypeParser::new();
            assert_eq!(parser.parse("", lexer).unwrap(), value)
        }

        check("(hello,world,)", vec!["hello", "world"]);
    }

    #[test]
    fn parse_type() {
        #[track_caller]
        fn check(input: &str, value: ast::Type) {
            let lexer = Lexer::new(input);
            let parser = grammar::TypeParser::new();
            assert_eq!(parser.parse("", lexer).unwrap(), value)
        }

        check("Boolean", ast::Type::Simple(ast::SimpleType::Boolean));
        check("mytype", ast::Type::Identifier("mytype"));
        check(
            "1..100",
            ast::Type::SubRange {
                start: ast::Constant::Number(ast::Number::Integer("1")),
                end: ast::Constant::Number(ast::Number::Integer("100")),
            },
        );
        check(
            "array [1..100] of real",
            ast::Type::Array {
                index: vec![ast::Type::SubRange {
                    start: ast::Constant::Number(ast::Number::Integer("1")),
                    end: ast::Constant::Number(ast::Number::Integer("100")),
                }],
                component: Box::new(ast::Type::Simple(ast::SimpleType::Real)),
                packed: false,
            },
        );
        check(
            "array [1..100, Boolean] of real",
            ast::Type::Array {
                index: vec![
                    ast::Type::SubRange {
                        start: ast::Constant::Number(ast::Number::Integer("1")),
                        end: ast::Constant::Number(ast::Number::Integer("100")),
                    },
                    ast::Type::Simple(ast::SimpleType::Boolean),
                ],
                component: Box::new(ast::Type::Simple(ast::SimpleType::Real)),
                packed: false,
            },
        );

        check(
            "packed array [Boolean] of packed array [0..10] of real",
            ast::Type::Array {
                index: vec![ast::Type::Simple(ast::SimpleType::Boolean)],
                component: Box::new(ast::Type::Array {
                    index: vec![ast::Type::SubRange {
                        start: ast::Constant::Number(ast::Number::Integer("0")),
                        end: ast::Constant::Number(ast::Number::Integer("10")),
                    }],
                    component: Box::new(ast::Type::Simple(ast::SimpleType::Real)),
                    packed: true,
                }),
                packed: true,
            },
        );
    }

    #[test]
    fn parse_type_def() {
        #[track_caller]
        fn check(input: &str, value: ast::TypeDef) {
            let lexer = Lexer::new(input);
            let parser = grammar::TypeDefParser::new();
            assert_eq!(parser.parse("", lexer).unwrap(), value)
        }

        check(
            "mytype = Boolean",
            ast::TypeDef {
                ident: "mytype",
                value: ast::Type::Simple(ast::SimpleType::Boolean),
            },
        );

        check(
            "mytype = myident2",
            ast::TypeDef {
                ident: "mytype",
                value: ast::Type::Identifier("myident2"),
            },
        );
    }
}
