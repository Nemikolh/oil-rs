use super::parse_grammar;

#[test]
fn test_variable_decl_as_strlit() {
    let var = r#"let v = "test";"#;
    parse_grammar(var).unwrap();
}

#[test]
fn test_variable_decl_as_alias() {
    let var = r#"let v = foobar;"#;
    parse_grammar(var).unwrap();
}

#[test]
fn test_variable_decl_as_object() {
    let var = r#"let v = { a : { b: "test", c: 23 + 3 ^ 2}};"#;
    parse_grammar(var).unwrap();
}
