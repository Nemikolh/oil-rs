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
    View,
    Template(Template),
    Class(Class),
}

// =================================
//          AST: Style
//

#[derive(Clone, Debug, PartialEq)]
pub struct Class {
    pub exported: bool,
    pub name: String,
    pub arguments: Vec<String>,
    pub includes: Vec<IncludeCond>,
    pub properties: HashMap<String, StyleValueCond>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AnonymousClass {
    pub includes: Vec<IncludeCond>,
    pub properties: HashMap<String, StyleValueCond>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Include {
    pub name: String,
    pub arguments: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IncludeCond {
    pub incl: Include,
    pub cond: Option<Box<Expr>>,
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
    Font {
        ident: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Unit {
    Px
}

// =================================
//          AST: Template
//

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Template {
    pub exported: bool,
    pub name: String,
    pub arguments: Vec<String>,
    pub events: Vec<String>,
    pub nodes: Vec<Node>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    pub span: Span,
    pub kind: NodeKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeKind {
    Text {
        content: String
    },
    Binding {
        property: String,
    },
    Tag {
        class: Option<()>,
        arguments: Vec<()>,
        events: Vec<()>,
    },
    Query {
        kind: QueryKind,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QueryKind {
    Children
}

// =================================
//          AST: Expressions
//

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Number(f32),
    VarAccess(String),
    BinaryOp(Box<Expr>, OpCode, Box<Expr>),
    Not(Box<Expr>),
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
// (not really part of the ast)

#[derive(Clone, Debug, PartialEq)]
pub enum PropertyOrInclude {
    Include(IncludeCond),
    Property((String, StyleValueCond)),
}

pub fn is_include(prop_or_include: PropertyOrInclude) -> Option<IncludeCond> {
    if let PropertyOrInclude::Include(s) = prop_or_include {
        Some(s)
    } else {
        None
    }
}

pub fn is_props(prop_or_include: PropertyOrInclude) -> Option<(String, StyleValueCond)> {
    if let PropertyOrInclude::Property(p) = prop_or_include {
        Some(p)
    } else {
        None
    }
}

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
