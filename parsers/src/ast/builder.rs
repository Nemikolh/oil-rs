use lalrpop_intern::InternedString;
use std::str::FromStr;
use super::*;

/// Produces expressions. Used by the parser.
pub trait ASTBuilder {

    fn import(span: Span, symbols: ImportSymbols, path: InternedString) -> Import;
    fn ident(span: Span, name: InternedString) -> Ident;
    fn node_txt(span: Span, content: InternedString) -> Node;
    fn node_agg(span: Span, children: Vec<Node>) -> Node;
    fn node_binding(span: Span, property: PathExpr) -> Node;
    fn package(imports: Vec<Import>, items: Vec<Item>) -> Package {
        Package {
            imports: imports,
            items: items,
        }
    }
    fn cpt(exported: bool, name: Ident, args: Vec<Ident>, events: Vec<Ident>, body: ComponentBody) -> Component {
        Component {
            exported: exported,
            name: name,
            arguments: args,
            events: events,
            body: body,
        }
    }
    fn const_value(exported: bool, name: Ident, object: ObjectValue) -> ConstValue {
        ConstValue {
            exported: exported,
            name: name,
            object: object,
        }
    }
    fn view(name: Ident, node: Node) -> View {
        View {
            name: name,
            node: node,
        }
    }
    fn style_value(expr: Expr) -> StyleValue {
        StyleValue::Unspecified(Box::new(expr))
    }
    fn constant<'input>(value: &'input str) -> Constant {
        if let Ok(value) = i32::from_str_radix(value, 10) {
            Constant::Integer(value)
        } else {
            Constant::Float(f64::from_str(value).unwrap())
        }
    }
    fn const_to_expr(constant: Constant) -> Expr {
        Expr::Constant(constant)
    }
    fn not(expr: Expr) -> Expr {
        Expr::Not(Box::new(expr))
    }
    fn variable(name: Ident) -> PathExpr {
        PathExpr::Variable(name)
    }
    fn option(opt: PathExpr) -> PathExpr {
        PathExpr::Option {
            opt: Box::new(opt),
        }
    }
    fn propaccess(map: PathExpr, property: Ident) -> PathExpr {
        PathExpr::PropertyAccess {
            map: Box::new(map),
            property: property,
        }
    }
    fn index(map: PathExpr, index: Constant) -> PathExpr {
        PathExpr::Index {
            map: Box::new(map),
            index: index,
        }
    }
    fn path(path: PathExpr) -> Expr {
        Expr::PathExpr(path)
    }
    fn binaryop(left: Expr, op: OpCode, right: Expr) -> Expr {
        Expr::BinaryOp(Box::new(left), op, Box::new(right))
    }
    fn signed(sign: Sign, expr: Expr) -> Expr {
        Expr::Signed(sign, Box::new(expr))
    }
    fn strlit(s: InternedString) -> Expr {
        Expr::Constant(Constant::StrLit(s))
    }
    fn boolean(b: bool) -> Expr {
        Expr::Constant(Constant::Boolean(b))
    }
}

/// Generate AST with proper span information
pub struct ASTFullSpan;


impl ASTBuilder for ASTFullSpan {

    fn import(span: Span, symbols: ImportSymbols, path: InternedString) -> Import {
        Import {
            span: span,
            symbols: symbols,
            package: SubPackage::UnresolvedPath(path)
        }
    }

    fn ident(span: Span, name: InternedString) -> Ident {
        Ident {
            span: span,
            name: name,
        }
    }

    fn node_txt(span: Span, content: InternedString) -> Node {
        Node {
            span: span,
            children: vec![],
            kind: NodeKind::Text { content: content },
        }
    }

    fn node_agg(span: Span, mut children: Vec<Node>) -> Node {
        if children.len() == 1 {
            children.pop().unwrap()
        } else {
            Node {
                span: span,
                children: children,
                kind: NodeKind::NoType,
            }
        }
    }

    fn node_binding(span: Span, property: PathExpr) -> Node {
        Node {
            span: span,
            children: vec![],
            kind: NodeKind::Binding { property: property },
        }
    }
}