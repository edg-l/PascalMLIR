

pub enum Number<'a> {
    Integer(&'a str),
    Real(&'a str)
}

pub enum Constant<'a> {
    Identifier {
        is_negative: bool,
        ident: &'a str
    },
    Number(Number<'a>),
    String(&'a str),
}

pub struct ConstantDef<'a> {
    pub ident: &'a str,
    pub value: Constant<'a>
}
