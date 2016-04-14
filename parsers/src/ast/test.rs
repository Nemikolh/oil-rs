use grammar::parse_grammar;
use super::*;

#[test]
fn test_ast_import_should_contains_components() {
    let import = r#"import * from 'material-oil';"#;
    let package = parse_grammar(import).unwrap();
    assert_eq!(package.imports[0].components, Components::All);
}

#[test]
fn test_ast_import_should_contain_path() {
    let import = r#"import * from 'material-oil';"#;
    let package = parse_grammar(import).unwrap();
    assert_eq!(package.imports[0].path, "material-oil");
}

#[test]
fn test_ast_import_should_contain_path2() {
    let import = r#"import $img from './logo.png';"#;
    let package = parse_grammar(import).unwrap();
    assert_eq!(package.imports[0].path, "./logo.png");
}

#[test]
fn test_ast_package_should_collect_all_import() {
    let many_imports = r#"import * from 'a'; import 'b';
     import $font from './somewhere/to/font.otf';"#;
    let package = parse_grammar(many_imports).unwrap();
    assert_eq!(package.imports.len(), 3);
    assert_eq!(package.imports[0].components, Components::All);
    assert_eq!(package.imports[1].components, Components::All);
    assert_eq!(package.imports[2].components, Components::Font);
}

#[test]
fn test_ast_template_with_text_child() {
    let template = r#"template test = Hello!;"#;
    let package = parse_grammar(template).unwrap();
    assert_eq!(package.items.len(), 1);
    if let Item::Template(ref template) = package.items[0] {
        assert_eq!(template.exported, false);
        assert_eq!(template.name, "test");
        assert_eq!(template.arguments.len(), 0);
        assert_eq!(template.events.len(), 0);
        if let NodeKind::Text { ref content } = template.nodes[0].kind {
            assert_eq!(content, "Hello!");
        } else {
            assert!(false);
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_ast_style_should_have_name_without_dot() {
    let class = r#".some_class {}"#;
    let package = parse_grammar(class).unwrap();
    assert_eq!(package.items.len(), 1);
    if let Item::Class(ref class) = package.items[0] {
        assert_eq!(class.name, "some_class");
    } else {
        assert!(false);
    }
}
