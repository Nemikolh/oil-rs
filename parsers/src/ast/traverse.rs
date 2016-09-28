use super::*;

pub trait PackageVisitor {

    fn visit_import(&mut self,  item: &mut Import);
    fn visit_view(&mut self, item: &mut View);
    fn visit_class(&mut self,  item: &mut Class);
    fn visit_datatype(&mut self,  item: &mut DataType);
    fn visit_component(&mut self,  item: &mut Component);
    fn visit_node(&mut self, item: &mut Node);
}

/// Traverse the AST and call visitor methods
/// where appropriate.
pub fn traverse<T: PackageVisitor>(ast: &mut Package, visitor: &mut T) {
    // Visit imports first
    for import in ast.imports.iter_mut() {
        visitor.visit_import(import);
    }
    // Visit items in order
    for item in ast.items.iter_mut() {
        traverse_item(item, visitor);
    }
}

fn traverse_item<T: PackageVisitor>(item: &mut Item, visitor: &mut T) {
    match item {
        &mut Item::View(ref mut item) => traverse_view(item, visitor),
        &mut Item::Class(ref mut item) => traverse_class(item, visitor),
        &mut Item::DataType(ref mut item) => traverse_datatype(item, visitor),
        &mut Item::Component(ref mut item) => traverse_component(item, visitor),
    }
}

fn traverse_view<T: PackageVisitor>(item: &mut View, visitor: &mut T) {
    visitor.visit_view(item);
    traverse_node(&mut item.node, visitor)
}

fn traverse_class<T: PackageVisitor>(item: &mut Class, visitor: &mut T) {
    visitor.visit_class(item);
}

fn traverse_datatype<T: PackageVisitor>(item: &mut DataType, visitor: &mut T) {
    visitor.visit_datatype(item);
}

fn traverse_component<T: PackageVisitor>(item: &mut Component, visitor: &mut T) {
    visitor.visit_component(item);
    for item in item.nodes.iter_mut() {
        traverse_node(item, visitor);
    }
}

fn traverse_node<T: PackageVisitor>(item: &mut Node, visitor: &mut T) {
    visitor.visit_node(item);
    for item in item.children.iter_mut() {
        traverse_node(item, visitor);
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
        fn visit_import(&mut self, item: &mut Import) {
            self.import_count += 1;
            assert_eq!(item.path, "some-import");
        }
        fn visit_view(&mut self, item: &mut View) {
            assert!(false);
        }
        fn visit_class(&mut self, item: &mut Class) {
            assert_eq!(item.name, "btn");
        }
        fn visit_datatype(&mut self, item: &mut DataType) {
            assert!(false);
        }
        fn visit_component(&mut self, item: &mut Component) {
            self.component_count += 1;
            assert_eq!(item.name, "el");
        }
        fn visit_node(&mut self, item: &mut Node) {
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
    fn test_traverse_package() {
        let test = r#"
        import 'some-import';
        component el = <a>some text<b><c></c></b><d></d></a>;
        .btn {
            prop: 4 px;
            prop: 3 px if true;
        }
        "#;
        let mut package = parse_grammar(test).unwrap();
        let mut visitor = TestVisitor::default();
        traverse(&mut package, &mut visitor);
        assert_eq!(visitor.node_count, 5);
        assert_eq!(visitor.import_count, 1);
        assert_eq!(visitor.component_count, 1);
    }
}
