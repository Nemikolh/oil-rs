use super::pass_resolve_symbols;
use grammar::parse_grammar;

#[test]
fn test_valid_local_name_resolution_for_classes() {
    let package_txt = r#"
    .some_class {}
    .other_class { .some_class; }
    "#;
    let mut ast = parse_grammar(package_txt).unwrap();
    pass_resolve_symbols(&mut ast).expect("Name should all have been resolved.");
}

#[test]
fn test_valid_local_name_resolution_for_components() {
    let package_txt = r#"
    component a = <button></button>;
    component el = <a></a>;
    "#;
    let mut ast = parse_grammar(package_txt).unwrap();
    pass_resolve_symbols(&mut ast).expect("Name should all have been resolved.");
}

#[test]
fn test_valid_local_name_resolution_for_mixed() {
    let package_txt = r#"
    .some_class {}
    component el = <button class=some_class></button>;
    "#;
    let mut ast = parse_grammar(package_txt).unwrap();
    pass_resolve_symbols(&mut ast).expect("Name should all have been resolved.");
}

#[test]
#[should_panic]
#[ignore]
fn test_invalid_local_name_resolution_for_components() {
    let package_txt = r#"
    component a = <button></button>;
    component el = <b></b>;
    "#;
    let mut ast = parse_grammar(package_txt).unwrap();
    pass_resolve_symbols(&mut ast).unwrap();
}

#[test]
#[should_panic]
#[ignore]
fn test_invalid_local_name_resolution_for_classes() {
    let package_txt = r#"
    .some_class {}
    .other_class { .some_classa; }
    "#;
    let mut ast = parse_grammar(package_txt).unwrap();
    pass_resolve_symbols(&mut ast).unwrap();
}
