use super::pass_resolve_names;
use grammar::parse_grammar;
use ast::*;

#[test]
fn test_valid_local_name_resolution_for_classes() {
    let package_txt = r#"
    .some_class {}
    .other_class { .some_class; }
    "#;
    let mut ast = parse_grammar(package_txt).unwrap();
    pass_resolve_names(&mut ast).expect("Name should all have been resolved.");
}

#[test]
fn test_valid_local_name_resolution_for_templates() {
    let package_txt = r#"
    template a = <button></button>;
    template el = <a></a>;
    "#;
    let mut ast = parse_grammar(package_txt).unwrap();
    pass_resolve_names(&mut ast).expect("Name should all have been resolved.");
}

#[test]
fn test_valid_local_name_resolution_for_mixed() {
    let package_txt = r#"
    .some_class {}
    template el = <button class=some_class></button>;
    "#;
    let mut ast = parse_grammar(package_txt).unwrap();
    pass_resolve_names(&mut ast).expect("Name should all have been resolved.");
}

#[test]
#[should_panic]
fn test_invalid_local_name_resolution_for_templates() {
    let package_txt = r#"
    template a = <button></button>;
    template el = <b></b>;
    "#;
    let mut ast = parse_grammar(package_txt).unwrap();
    pass_resolve_names(&mut ast).expect("Name should all have been resolved.");
}

#[test]
#[should_panic]
fn test_invalid_local_name_resolution_for_classes() {
    let package_txt = r#"
    .some_class {}
    .other_class { .some_classa; }
    "#;
    let mut ast = parse_grammar(package_txt).unwrap();
    pass_resolve_names(&mut ast).expect("Name should all have been resolved.");
}
