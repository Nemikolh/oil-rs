use super::*;

pub trait PackageVisitor: Sized {

    fn visit_import(&mut self,  item: &Import);

    fn visit_view(&mut self, item: &View) {
        walk_view(item, self);
    }
    fn visit_class(&mut self,  item: &Class) {
        walk_class(item, self);
    }
    fn visit_component(&mut self,  item: &Component) {
        walk_component(item, self);
    }
    fn visit_node(&mut self, item: &Node) {
        walk_node(item, self);
    }

    fn visit_ident(&mut self, _item: &Ident) {}
    fn visit_const(&mut self, _item: &ConstValue) {}

    fn visit_component_body(&mut self, item: &ComponentBody) {
        walk_component_body(item, self);
    }
}

/// Walk the AST and call visitor methods
/// where appropriate.
pub fn walk<T: PackageVisitor>(ast: &Package, visitor: &mut T) {
    // Visit imports first
    for import in &ast.imports {
        visitor.visit_import(import);
    }
    // Visit items in order
    for item in &ast.items {
        walk_item(item, visitor);
    }
}

pub fn walk_item<T: PackageVisitor>(item: &Item, visitor: &mut T) {
    match *item {
        Item::View(ref item) => visitor.visit_view(item),
        Item::Class(ref item) => visitor.visit_class(item),
        Item::Component(ref item) => visitor.visit_component(item),
    }
}

pub fn walk_view<T: PackageVisitor>(item: &View, visitor: &mut T) {
    walk_node(&item.node, visitor)
}

pub fn walk_class<T: PackageVisitor>(item: &Class, visitor: &mut T) {
    for ident in &item.arguments {
        visitor.visit_ident(ident);
    }
}

pub fn walk_component<T: PackageVisitor>(item: &Component, visitor: &mut T) {
    visitor.visit_component_body(&item.body);
}

pub fn walk_component_body<T: PackageVisitor>(item: &ComponentBody, visitor: &mut T) {
    match *item {
        ComponentBody::PreludeThenSingleNode(_, ref node) => visitor.visit_node(node),
        ComponentBody::SingleNode(ref node) => visitor.visit_node(node),
    }
}

pub fn walk_node<T: PackageVisitor>(item: &Node, visitor: &mut T) {
    for item in &item.children {
        visitor.visit_node(item);
    }
}

#[cfg(test)]
mod test {

    use ast::*;
    use grammar::parse_grammar;

    #[derive(Default)]
    struct TestVisitor {
        node_count: usize,
        import_count: usize,
        component_count: usize,
    }

    impl PackageVisitor for TestVisitor {
        fn visit_import(&mut self, item: &Import) {
            self.import_count += 1;
            assert_eq!(item.path, "some-import");
        }
        fn visit_view(&mut self, item: &View) {
            assert!(false);
        }
        fn visit_class(&mut self, item: &Class) {
            assert_eq!(item.name, "btn");
        }
        fn visit_datatype(&mut self, item: &DataType) {
            assert!(false);
        }
        fn visit_component(&mut self, item: &Component) {
            self.component_count += 1;
            assert_eq!(item.name, "el");
        }
        fn visit_node(&mut self, item: &Node) {
            match self.node_count {
                0 => if let NodeKind::Tag { ref name, .. } = item.kind {
                    assert_eq!(name, "a");
                } else { assert!(false); },
                1 => if let NodeKind::Text { ref content } = item.kind {
                    assert_eq!(content, "some text");
                } else { assert!(false); },
                2 => if let NodeKind::Tag { ref name, .. } = item.kind {
                    assert_eq!(name, "b");
                } else { assert!(false); },
                3 => if let NodeKind::Tag { ref name, .. } = item.kind {
                    assert_eq!(name, "c");
                } else { assert!(false); },
                4 => if let NodeKind::Tag { ref name, .. } = item.kind {
                    assert_eq!(name, "d");
                } else { assert!(false); },
                _ => assert!(false),
            }
            self.node_count += 1;
        }
    }

    #[test]
    fn test_walk_package() {
        let test = r#"
        import 'some-import';
        component el = <a>some text<b><c></c></b><d></d></a>;
        .btn {
            prop: 4 px;
            prop: 3 px if true;
        }
        "#;
        let package = parse_grammar(test).unwrap();
        let mut visitor = TestVisitor::default();
        walk(&package, &mut visitor);
        assert_eq!(visitor.node_count, 5);
        assert_eq!(visitor.import_count, 1);
        assert_eq!(visitor.component_count, 1);
    }
}
