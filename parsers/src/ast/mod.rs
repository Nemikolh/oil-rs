use std::collections::HashMap;

#[cfg(test)]
mod test;

// =================================
//          AST: Package
//

#[derive(Clone, Debug, PartialEq)]
pub struct Package {
    pub imports: Vec<Import>,
    pub items: Vec<Item>,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span(pub usize, pub usize);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Import {
    pub components: Components,
    pub path: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Components {
    All,
    Img,
    Font,
    Only(Vec<String>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Item {
    View(View),
    Model(Model),
    Template(Template),
    Class(Class),
}

// =================================
//          AST: View
//

#[derive(Clone, Debug, PartialEq)]
pub struct View {
    pub name: String,
    pub model_name: String,
    pub handlers_name: String,
    pub node: Node,
}

// =================================
//          AST: Model
//

#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    pub exported: bool,
    pub name: String,
    pub properties: Box<ObjectValue>,
}

// =================================
//          AST: Style
//

#[derive(Clone, Debug, PartialEq)]
pub struct Class {
    pub exported: bool,
    pub name: String,
    pub arguments: Vec<String>,
    pub properties: Vec<RawPropertyOrInclude>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AnonymousClass {
    pub properties: Vec<RawPropertyOrInclude>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RawPropertyOrInclude {
    Include(IncludeCond),
    RawProperty((String, StyleValueCond)),
}

#[derive(Clone, Debug, PartialEq)]
pub struct IncludeCond {
    pub incl: Include,
    pub cond: Option<Box<Expr>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Include {
    pub name: String,
    pub arguments: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StyleValueCond {
    pub prop: StyleValue,
    pub cond: Option<Box<Expr>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StyleValue {
    Length(Box<Expr>, Unit),
    Keyword(String),
    Unspecified(Box<Expr>),
    Hex(String),
    Img {
        ident: String,
        view_x: Option<f32>,
        view_y: Option<f32>,
        view_w: Option<f32>,
        view_h: Option<f32>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Unit {
    Px
}

// =================================
//          AST: Template
//

#[derive(Clone, Debug, PartialEq)]
pub struct Template {
    pub exported: bool,
    pub name: String,
    pub arguments: Vec<String>,
    pub events: Vec<String>,
    pub nodes: Vec<Node>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub span: Span,
    pub kind: NodeKind,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NodeKind {
    Text {
        content: String
    },
    Binding {
        property: String,
    },
    Tag {
        class: Option<AnonymousClassOrIdent>,
        arguments: Vec<(String, Box<ObjectValue>)>,
        events: Vec<(String, String)>,
    },
    Query {
        kind: QueryKind,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QueryKind {
    Children
}

#[derive(Clone, Debug, PartialEq)]
pub enum AnonymousClassOrIdent {
    Ident(String),
    AnCls(AnonymousClass),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ObjectValue {
    StrLit(String),
    Expr(Box<Expr>),
    Props(HashMap<String, Box<ObjectValue>>),
}

// =================================
//          AST: Expressions
//

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    /// A raw number
    Number(f32),
    /// A var access such as `a.b.c`
    VarAccess(String),
    /// A new model instance: `new ModelName`
    New(String),
    /// A binary operation between two things such as `a + b`
    BinaryOp(Box<Expr>, OpCode, Box<Expr>),
    /// Negation of an expression
    Not(Box<Expr>),
    /// A signed expression.
    Signed(Sign, Box<Expr>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Sign {
    Plus,
    Minus,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OpCode {
    Add,
    Sub,
    Or,
    Mul,
    Div,
    Pow,
    Mod,
    And,
    EqEq,
    NotEq,
    LessThan,
    GreaterThan,
    LessThanOrEq,
    GreaterThanOrEq,
}

// =================================
//          Utilities
//

#[derive(Clone, Debug, PartialEq)]
pub enum ImgViewOrUnit<'input> {
    ImgView(Vec<(Option<&'input str>, f32)>),
    Unit(Unit),
}

pub fn is_valid_img_range<'input>(img_range: &Vec<(Option<&'input str>, f32)>) -> bool {
    img_range.len() <= 4 && (
        img_range.iter().all(|&x| x.0.is_none()) ||
        img_range.iter().all(|&x| x.0.is_some())
    )
}
