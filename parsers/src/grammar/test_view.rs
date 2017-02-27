use super::parse_grammar;


#[test]
fn test_oil_view() {
    let view = r#"
    view my_view =
        <app [name]={store.name} (start)={reducers.start}></app>
    ;
    "#;
    parse_grammar(view).unwrap();
}

#[test]
fn test_oil_view2() {
    let view = r#"
    view my_view =
        <app [name]={store.name} (start)={reducers.start}></app>
    ;
    "#;
    parse_grammar(view).unwrap();
}

#[test]
#[should_panic]
fn test_oil_invalid_view_missing_body() {
    let view = r#"
    view my_view = ;
    "#;
    parse_grammar(view).unwrap();
}