use super::parse_grammar;


#[test]
fn test_oil_view() {
    let view = r#"
    view my-view!(model, handlers) = {
        <app [name]={model.name} (start)={handlers.start}></app>
    }
    "#;
    parse_grammar(view).unwrap();
}

#[test]
fn test_oil_view2() {
    let view = r#"
    view my-view! (m, h) = {
        <app [name]={m.name} (start)={h.start}></app>
    }
    "#;
    parse_grammar(view).unwrap();
}

#[test]
#[should_panic]
fn test_oil_invalid_view_missing_one_arg() {
    let view = r#"
    view my-view! (model)= {
    }
    "#;
    parse_grammar(view).unwrap();
}

#[test]
#[should_panic]
fn test_oil_invalid_view_missing_args() {
    let view = r#"
    view my-view! = {
    }
    "#;
    parse_grammar(view).unwrap();
}

#[test]
#[should_panic]
fn test_oil_view_missing_bang() {
    let view = r#"
    view my-view (m, h) = {}
    "#;
    parse_grammar(view).unwrap();
}