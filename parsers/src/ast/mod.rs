use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use lalrpop_intern::InternedString;

pub mod visit;
pub mod builder;
pub mod folder;
pub mod pathexpr;

#[cfg(test)]
mod test;

// =================================
//          AST: Package
//

#[derive(Clone, Debug, PartialEq)]
pub struct Package {
    /// List of imports made by this package.
    pub imports: Vec<Import>,
    /// List of items defined by this package.
    pub items: Vec<Item>,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span(pub usize, pub usize);

#[derive(Clone, Debug, PartialEq)]
pub struct Import {
    /// Span for this import
    pub span: Span,
    /// List of symbols imported.
    pub symbols: ImportSymbols,
    /// Resolved tree of the imported package
    /// Might also be
    pub package: SubPackage,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SubPackage {
    /// Another package that the main one depend on.
    Package(Rc<RefCell<Package>>),
    /// ResourcePath is a path for an image or a font.
    /// Might be extended to other things such as shaders.
    ResourcePath(InternedString),
    /// An unresolved path.
    UnresolvedPath(InternedString),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImportSymbols {
    /// All exported symbols are imported.
    All,
    /// All symbols are imported under Ident.
    AllAsIdent(Ident),
    /// Import only the following symbols from the package.
    Only(Vec<Ident>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Item {
    View(View),
    Component(Component),
    Class(Class),
}

// =================================
//          AST: View
//

#[derive(Clone, Debug, PartialEq)]
pub struct View {
    /// Name of the view.
    pub name: Ident,
    /// Single root node that start the view.
    pub node: Node,
}

// =================================
//          AST: Constants
//

#[derive(Clone, Debug, PartialEq)]
pub struct ConstValue {
    /// True if the type is visible outside of this `Package`.
    pub exported: bool,
    /// Name of the data type
    pub name: Ident,
    /// Properties of for each value of that type
    /// and their default value (can use arguments)
    pub object: ObjectValue,
}

// =================================
//          AST: Style
//

#[derive(Clone, Debug, PartialEq)]
pub struct Class {
    /// Is the class visible outside of this Package?
    pub exported: bool,
    /// Name of the class
    pub name: Ident,
    /// Arguments accepted by this class
    pub arguments: Vec<Ident>,
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
    RawProperty((InternedString, StyleValueCond)),
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
    pub name: Ident,
    /// Arguments to the style inclusion
    pub arguments: Vec<Ident>,
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
    Keyword(InternedString),
    /// Unspecified type (will depend on the property)
    Unspecified(Box<Expr>),
    /// Hexadecimal value (useful for colors)
    Hex(InternedString),
    /// Image variant can accept argument
    /// to restrict the image to be displayed.
    Img {
        /// Ident that should represent an image
        ident: InternedString,
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
    pub name: Ident,
    /// Arguments name accepted by the component
    pub arguments: Vec<Ident>,
    /// Events accepted by the component
    pub events: Vec<Ident>,
    /// Component body
    pub body: ComponentBody,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ComponentBody {
    SingleNode(Node),
    PreludeThenSingleNode(Option<()>, Node),
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
        content: InternedString
    },
    /// This node is a text binding.
    Binding {
        /// The binding is done on the following
        /// property.
        property: PathExpr,
    },
    /// A node aggregating other nodes with no particular semantics.
    NoType,
    /// A generic tag use `<tag></tag>`
    Tag {
        /// The name of the tag.
        name: InternedString,
        /// The class argument attaching style properties
        /// to that node.
        class: Option<AnonymousClassOrIdent>,
        /// The arguments that are passed to the component
        /// used to instantiate that node.
        arguments: Vec<(Ident, ObjectValue)>,
        /// The events passed to the component used to
        /// instantiate that node.
        /// FIXME: add event chain.
        events: Vec<(Ident, ())>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum AnonymousClassOrIdent {
    /// Simple identifier
    Ident(InternedString),
    /// Anonymouse class case
    AnCls(AnonymousClass),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ObjectValue {
    /// An expression
    Expr(Box<Expr>),
    /// A list of properties
    Props(HashMap<InternedString, ObjectValue>),
}

// =================================
//          AST: Expressions
//

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    /// A constant known at compile time.
    Constant(Constant),
    /// A path expression such as `a.b.c?`
    PathExpr(PathExpr),
    /// A binary operation between two things such as `a + b`
    BinaryOp(Box<Expr>, OpCode, Box<Expr>),
    /// Bit negation of an expression
    Not(Box<Expr>),
    /// A signed expression.
    Signed(Sign, Box<Expr>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Constant {
    /// Boolean,
    Boolean(bool),
    /// Floating point value
    Float(f64),
    /// Integer value
    Integer(i32),
    /// String literal
    StrLit(InternedString),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PathExpr {
    /// A variable
    Variable(Ident),
    /// A check for an optional value.
    Option {
        opt: Box<PathExpr>,
    },
    /// A property being accessed.
    PropertyAccess {
        map: Box<PathExpr>,
        property: Ident,
    },
    /// An index being accessed.
    Index {
        map: Box<PathExpr>,
        // TODO: Replace this with an expression.
        //       We could have anything here.
        index: Constant,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sign {
    Plus,
    Minus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

/// An ident conveniently store the string
/// representation as well as the span info.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ident {
    /// Span for this ident.
    pub span: Span,
    /// Ident
    pub name: InternedString,
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
