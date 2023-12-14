use logos::Logos;
use std::convert::Infallible;

//  https://github.com/maciejhirsz/logos/issues/133

#[derive(Debug, PartialEq, Clone, Default)]
pub enum LexingError {
    NumberParseError,
    #[default]
    Other,
}

impl From<std::num::ParseIntError> for LexingError {
    fn from(_: std::num::ParseIntError) -> Self {
        LexingError::NumberParseError
    }
}

impl From<Infallible> for LexingError {
    fn from(_: Infallible) -> Self {
        LexingError::Other
    }
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error = LexingError, skip r"[ \t\n\f]+", skip r"//.*\n?", skip r"\{[^}]*\}" skip r"\(\*(.|[\r\n])*?\*\)")]
pub enum Token<'input> {
    #[regex(r"[a-zA-Z][a-zA-Z\d]*")]
    Identifier(&'input str), // also directive
    #[regex(r"[+-]?[0-9][0-9]*")]
    Integer(&'input str),
    #[regex(r"[+-]?[0-9][0-9]*\.[0-9][0-9]*([eE][+-]?[0-9][0-9]*)?")]
    #[regex(r"[+-]?[0-9][0-9]*[eE][+-]?[0-9][0-9]*")]
    Real(&'input str),
    #[regex(r#""(?:[^"]|\\")*""#)]
    String(&'input str),

    // special symbols
    #[token("+")]
    SpecialPlus,
    #[token("-")]
    SpecialMinus,
    #[token("*")]
    SpecialMul,
    #[token("/")]
    SpecialDiv,
    #[token("=")]
    SpecialEqual,
    #[token("<")]
    SpecialLower,
    #[token(">")]
    SpecialGreater,
    #[token("[")]
    SpecialOpenBracket,
    #[token("]")]
    SpecialCloseBracket,
    #[token(".")]
    SpecialDot,
    #[token(",")]
    SpecialComma,
    #[token(";")]
    SpecialDotComma,
    #[token("\"")]
    SpecialQuotation,
    #[token("(")]
    SpecialOpenParen,
    #[token(")")]
    SpecialCloseParen,
    #[token("<>")]
    SpecialSpaceship,
    #[token("<=")]
    SpecialLessEqual,
    #[token(">=")]
    SpecialGreaterEqual,
    #[token(":=")]
    SpecialAssign,
    #[token("..")]
    SpecialRange,

    // special symbols - word symbols
    #[token("and")]
    WordAnd,
    #[token("array")]
    WordArray,
    #[token("begin")]
    WordBegin,
    #[token("case")]
    WordCase,
    #[token("const")]
    WordConst,
    #[token("div")]
    WordDiv,
    #[token("do")]
    WordDo,
    #[token("downto")]
    WordDownto,
    #[token("else")]
    WordElse,
    #[token("end")]
    WordEnd,
    #[token("file")]
    WordFile,
    #[token("for")]
    WordFor,
    #[token("function")]
    WordFunction,
    #[token("goto")]
    WordGoto,
    #[token("if")]
    WordIf,
    #[token("in")]
    WordIn,
    #[token("label")]
    WordLabel,
    #[token("mod")]
    WordMod,
    #[token("nil")]
    WordNil,
    #[token("not")]
    WordNot,
    #[token("of")]
    WordOf,
    #[token("or")]
    WordOr,
    #[token("packed")]
    WordPacked,
    #[token("procedure")]
    WordProcedure,
    #[token("program")]
    WordProgram,
    #[token("record")]
    WordRecord,
    #[token("repeat")]
    WordRepeat,
    #[token("set")]
    WordSet,
    #[token("then")]
    WordThen,
    #[token("to")]
    WordTo,
    #[token("type")]
    WordType,
    #[token("until")]
    WordUntil,
    #[token("var")]
    WordVar,
    #[token("while")]
    WordWhile,
    #[token("with")]
    WordWith,
}
