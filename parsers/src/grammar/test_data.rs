use super::parse_grammar;

#[test]
fn test_model_decl_as_strlit() {
    let model = r#"datatype ConstantModel = "test";"#;
    parse_grammar(model).unwrap();
}

#[test]
fn test_model_decl_as_alias() {
    let model = r#"datatype AliasModel = OtherModel;"#;
    parse_grammar(model).unwrap();
}

#[test]
fn test_model_decl_as_object() {
    let model = r#"datatype MapModel = { a : { b: "test", c: 23 + 3 ^ 2}};"#;
    parse_grammar(model).unwrap();
}

#[test]
fn test_model_decl_as_alias_2() {
    // In practice this is equivalent to the alias case.
    // It should be rejected by later pass as it does not make
    // a lot of sense to accept it.
    let model = r#"datatype SubModel = new OtherModel;"#;
    parse_grammar(model).unwrap();
}

#[test]
fn test_model_decl_with_submodel() {
    let model = r#"datatype Test = { a: new OtherModel };"#;
    parse_grammar(model).unwrap();
}

#[test]
fn test_model_use_in_template() {
    let model = r#"
    datatype Test(a) = { a: a };
    template test = <el [arg1]={new Test("test")}></el>;
    "#;
    parse_grammar(model).unwrap();
}
