use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

pub mod lexer;
pub mod tokens;
pub mod ast;

fn main() {
    println!("Hello, world!");
}
