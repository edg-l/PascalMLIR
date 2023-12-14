#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Number<'a> {
    Integer(&'a str),
    Real(&'a str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constant<'a> {
    Identifier { is_negative: bool, ident: &'a str },
    Number(Number<'a>),
    String(&'a str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstantDef<'a> {
    pub ident: &'a str,
    pub value: Constant<'a>,
}
