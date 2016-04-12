use std::collections::HashMap;

#[cfg(test)]
mod test;

/*
This is the tree generated by the parser generated by lalrpop.
*/

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

#[derive(Clone, Debug, PartialEq)]
pub struct Class {
    pub exported: bool,
    pub name: String,
    pub properties: HashMap<String, StyleProperty>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StyleProperty {
    Number(f32, Unit),
    Hex(String),
    Img {
        // Should always be '$img' for now.
        ident: String,
        view_x: Option<f32>,
        view_y: Option<f32>,
        view_w: Option<f32>,
        view_h: Option<f32>,
    },
    Font,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Unit {
    Px
}

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
