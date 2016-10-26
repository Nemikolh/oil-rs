use super::*;

pub trait PackageVisitor {

    fn visit_import(&mut self,  item: &Import) {
        walk_import(item, self);
    }
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
    fn visit_ident(&mut self, item: &Ident) {}

    fn visit_const(&mut self, item: &ConstValue) {}
}

/// Walk the AST and call visitor methods
/// where appropriate.
pub fn walk<T: PackageVisitor>(ast: &Package, visitor: &mut T) {
    // Visit imports first
    for import in ast.imports.iter_mut() {
        visitor.visit_import(import);
    }
    // Visit items in order
    for item in ast.items.iter_mut() {
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
    walk_node(&mut item.node, visitor)
}

pub fn walk_import<T: PackageVisitor>(item: &Import, visitor: &mut T) {
    // FIXME
}

pub fn walk_class<T: PackageVisitor>(item: &Class, visitor: &mut T) {
    for ident in &item.arguments {
        visitor.visit_ident(ident);
    }
}

pub fn walk_component<T: PackageVisitor>(item: &Component, visitor: &mut T) {
    for item in item.nodes.iter_mut() {
        visitor.visit_component(item);
    }
}

pub fn walk_node<T: PackageVisitor>(item: &Node, visitor: &mut T) {
    for item in item.children.iter_mut() {
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
