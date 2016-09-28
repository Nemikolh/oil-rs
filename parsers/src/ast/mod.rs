use std::collections::HashMap;


pub use self::traverse::traverse;
pub use self::traverse::PackageVisitor;

mod traverse;
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
    DataType(DataType),
    Component(Component),
    Class(Class),
}

// =================================
//          AST: View
//

#[derive(Clone, Debug, PartialEq)]
pub struct View {
    /// Name of the view.
    pub name: String,
    /// Name of the model parameter
    pub model_name: String,
    /// Name of the handlers parameter
    pub handlers_name: String,
    /// Single root node that start the view.
    pub node: Node,
}

// =================================
//          AST: DataType
//

#[derive(Clone, Debug, PartialEq)]
pub struct DataType {
    /// True if the type is visible outside of this `Package`.
    pub exported: bool,
    /// Name of the data type
    pub name: String,
    /// Properties of for each value of that type
    /// and their default value (can use arguments)
    pub properties: ObjectValue,
    /// Constructor argument to the data type.
    pub arguments: Vec<String>,
}

// =================================
//          AST: Style
//

#[derive(Clone, Debug, PartialEq)]
pub struct Class {
    /// Is the class visible outside of this Package?
    pub exported: bool,
    /// Name of the class
    pub name: String,
    /// Arguments accepted by this class
    pub arguments: Vec<String>,
    /// List of properties of this class
    pub properties: Vec<RawPropertyOrInclude>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AnonymousClass {
    /// Properties of this anonymous class
    pub properties: Vec<RawPropertyOrInclude>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RawPropertyOrInclude {
    /// Other class included
    Include(IncludeCond),
    /// Property key value
    RawProperty((String, StyleValueCond)),
}

#[derive(Clone, Debug, PartialEq)]
pub struct IncludeCond {
    /// Included class parameters
    pub incl: Include,
    /// Condition that must hold to include all the property
    /// of the `incl` class.
    pub cond: Option<Box<Expr>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Include {
    /// Name of the class that will be included in that one.
    pub name: String,
    /// Arguments to the style inclusion
    pub arguments: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StyleValueCond {
    /// Property value
    pub prop: StyleValue,
    /// Condition that must hold to apply this value to the property.
    pub cond: Option<Box<Expr>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StyleValue {
    /// A length value
    Length(Box<Expr>, Unit),
    /// A property keyword, represented as string literals
    Keyword(String),
    /// Unspecified type (will depend on the property)
    Unspecified(Box<Expr>),
    /// Hexadecimal value (useful for colors)
    Hex(String),
    /// Image variant can accept argument
    /// to restrict the image to be displayed.
    Img {
        /// Ident that should represent an image
        ident: String,
        /// X offset of the view
        view_x: Option<f32>,
        /// Y offset of the view
        view_y: Option<f32>,
        /// Width of the view
        view_w: Option<f32>,
        /// Height of the view
        view_h: Option<f32>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Unit {
    /// Pixel unit. The only one for now.
    Px
}

// =================================
//          AST: Component
//

#[derive(Clone, Debug, PartialEq)]
pub struct Component {
    /// Is this component visible outside of this Package?
    pub exported: bool,
    /// Name of the component
    pub name: String,
    /// Arguments name accepted by the component
    pub arguments: Vec<String>,
    /// Events accepted by the component
    pub events: Vec<String>,
    /// Children of this component
    pub nodes: Vec<Node>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    /// Span for location (used for error messages)
    pub span: Span,
    /// Node type.
    pub kind: NodeKind,
    /// Children
    pub children: Vec<Node>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NodeKind {
    /// A text node, no processing require apart
    /// from preserving the text.
    Text {
        /// The actual text.
        content: String
    },
    /// This node is a text binding.
    Binding {
        /// The binding is done on the following
        /// property.
        property: String,
    },
    /// A generic tag use `<tag></tag>`
    Tag {
        /// The name of the tag.
        name: String,
        /// The class argument attaching style properties
        /// to that node.
        class: Option<AnonymousClassOrIdent>,
        /// The arguments that are passed to the component
        /// used to instantiate that node.
        arguments: Vec<(String, ObjectValue)>,
        /// The events passed to the component used to
        /// instantiate that node.
        events: Vec<(String, String)>,
    },
    /// Query node `<select:query />`
    Query {
        /// Query type
        kind: QueryKind,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QueryKind {
    /// Select all children.
    Children
}

#[derive(Clone, Debug, PartialEq)]
pub enum AnonymousClassOrIdent {
    /// Simple identifier
    Ident(String),
    /// Anonymouse class case
    AnCls(AnonymousClass),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ObjectValue {
    /// A string literal
    StrLit(String),
    /// An expression
    Expr(Box<Expr>),
    /// A list of properties
    Props(HashMap<String, ObjectValue>),
}

// =================================
//          AST: Expressions
//

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    /// Boolean,
    Boolean(bool),
    /// A raw number
    Number(f32),
    /// A var access such as `a.b.c`
    VarAccess(String),
    /// A new model instance: `new ModelName(a, b)`
    New(String, Vec<ObjectValue>),
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
