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
fn test_ast_component_with_text_child() {
    let component = r#"component test = Hello!;"#;
    let package = parse_grammar(component).unwrap();
    assert_eq!(package.items.len(), 1);
    if let Item::Component(ref component) = package.items[0] {
        assert_eq!(component.exported, false);
        assert_eq!(component.name, "test");
        assert_eq!(component.arguments.len(), 0);
        assert_eq!(component.events.len(), 0);
        if let NodeKind::Text { ref content } = component.nodes[0].kind {
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

#[test]
fn test_ast_style_should_be_unspecified() {
    let class = r#".some_class {
        prop1: a.b if c;
        prop2: a.b px;
        prop3: a.b px if c > 3;
        prop4: 0.3;
        .inclusion_of_another_class;
        prop5: "auto";
        prop6: #FFaa4422;
        prop7: a.b.c > 1;
        prop8: some_img; // but not recognised as such
        prop9: some_img[1, 2, 3, 4];
        prop10: some_img[];
    }"#;
    let package = parse_grammar(class).unwrap();
    if let Item::Class(ref class) = package.items[0] {
        let mut iter = class.properties.iter();
        if let &RawPropertyOrInclude::RawProperty(ref prop1) = iter.next().unwrap() {
            assert_variant!(prop1.1.prop, StyleValue::Unspecified);
            assert!(prop1.1.cond.is_some());
        } else { assert!(false, "Wrong prop1"); }
        if let &RawPropertyOrInclude::RawProperty(ref prop2) = iter.next().unwrap() {
            assert_variant!(prop2.1.prop, StyleValue::Length);
        } else { assert!(false, "Wrong prop2"); }
        if let &RawPropertyOrInclude::RawProperty(ref prop3) = iter.next().unwrap() {
            assert_variant!(prop3.1.prop, StyleValue::Length);
            assert!(prop3.1.cond.is_some());
        } else { assert!(false, "Wrong prop3"); }
        if let &RawPropertyOrInclude::RawProperty(ref prop4) = iter.next().unwrap() {
            assert_variant!(prop4.1.prop, StyleValue::Unspecified);
        } else { assert!(false, "Wrong prop4"); }
        if let &RawPropertyOrInclude::Include(ref inclu) = iter.next().unwrap() {
            assert_eq!(inclu.incl.name, "inclusion_of_another_class");
        } else { assert!(false, "Wrong inclu"); }
        if let &RawPropertyOrInclude::RawProperty(ref prop5) = iter.next().unwrap() {
            assert_variant!(prop5.1.prop, StyleValue::Keyword);
        } else { assert!(false, "Wrong prop5"); }
        if let &RawPropertyOrInclude::RawProperty(ref prop6) = iter.next().unwrap() {
            assert_variant!(prop6.1.prop, StyleValue::Hex);
        } else { assert!(false, "Wrong prop6"); }
        if let &RawPropertyOrInclude::RawProperty(ref prop7) = iter.next().unwrap() {
            assert_variant!(prop7.1.prop, StyleValue::Unspecified);
        } else { assert!(false, "Wrong prop7"); }
        if let &RawPropertyOrInclude::RawProperty(ref prop8) = iter.next().unwrap() {
            assert_variant!(prop8.1.prop, StyleValue::Unspecified);
        } else { assert!(false, "Wrong prop8"); }
        if let &RawPropertyOrInclude::RawProperty(ref prop9) = iter.next().unwrap() {
            assert_variant!(prop9.1.prop, brace StyleValue::Img);
        } else { assert!(false, "Wrong prop9"); }
        if let &RawPropertyOrInclude::RawProperty(ref prop10) = iter.next().unwrap() {
            assert_variant!(prop10.1.prop, brace StyleValue::Img);
        } else { assert!(false, "Wrong prop10"); }
    } else {
        assert!(false, "Class couldn't be found!");
    }
}
