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
pub struct VariantSelector<'input> {
    pub tag_field: Option<&'input str>,
    pub tag_type: Box<Type<'input>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaseVariant<'input> {
    pub case_constants: Vec<Constant<'input>>,
    pub field_list: Option<RecordFieldList<'input>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordSection<'input> {
    pub identifier_list: Vec<&'input str>,
    pub type_denoter: Box<Type<'input>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordFixedPart<'input> {
    pub records: Vec<RecordSection<'input>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordVariantPart<'input> {
    pub variant_selector: VariantSelector<'input>,
    pub variants: Vec<CaseVariant<'input>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordFieldList<'input> {
    pub fixed_part: Option<RecordFixedPart<'input>>,
    pub variant_part: Option<RecordVariantPart<'input>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type<'input> {
    Identifier(&'input str),
    Simple(SimpleType),
    Enumerated(Vec<&'input str>),
    SubRange {
        start: Constant<'input>,
        end: Constant<'input>,
    },
    Array {
        index: Vec<Type<'input>>,
        component: Box<Type<'input>>,
        packed: bool,
    },
    Record {
        field_list: Option<RecordFieldList<'input>>,
        packed: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDef<'input> {
    pub ident: &'input str,
    pub value: Type<'input>,
}
