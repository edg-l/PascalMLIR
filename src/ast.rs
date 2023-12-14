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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimpleType {
    Integer,
    Real,
    Boolean,
    Char,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordField<'input> {
    Fixed {
        identifier_list: Vec<&'input str>,
        type_denoter: Type<'input>,
    },
    Case {
        // TODO: 6.4.3.3 Record-types
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type<'input> {
    Identifier(&'input str),
    Simple(SimpleType),
    Enumerated(Vec<&'input str>),
    Subrange {
        start: Constant<'input>,
        end: Constant<'input>,
    },
    Array {
        index: Vec<Type<'input>>,
        component: Box<Type<'input>>,
        packed: bool,
    },
    Record {
        fields: Vec<RecordField<'input>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDef<'input> {
    pub ident: &'input str,
    pub value: Type<'input>,
}
