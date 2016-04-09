use super::parse_grammar;


#[test]
fn test_oil_basic_template_no_args() {
    let template = r#"
    template ui-el =
        <el></el>;
    "#;
    parse_grammar(template).unwrap();
}

#[test]
fn test_oil_basic_template_no_args1() {
    let template = r#"
    template ui-el [] =
        <el></el>;
    "#;
    parse_grammar(template).unwrap();
}

#[test]
fn test_oil_basic_template_unused_arg() {
    let template = r#"
    template ui-el [a] =
        <el></el>;
    "#;
    parse_grammar(template).unwrap();
}

#[test]
fn test_oil_basic_template_no_args_export() {
    let template = r#"
    export template ui-el =
        <el></el>;
    "#;
    parse_grammar(template).unwrap();
}

#[test]
fn test_oil_basic_template_with_args() {
    let template = r#"
    template ui-el [el-class] =
        <el class={el-class}></el>
    ;"#;
    parse_grammar(template).unwrap();
}

#[test]
fn test_oil_basic_template_with_args_export() {
    let template = r#"
    export template ui-el [el-class] =
        <el class={el-class}></el>;
    "#;
    parse_grammar(template).unwrap();
}

#[test]
fn test_oil_basic_template_with_args_return() {
    let template = r#"
    template ui-el [el-class] -> event =
        <el class={el-class} (click)={event}></el>;
    "#;
    parse_grammar(template).unwrap();
}

#[test]
fn test_oil_template_with_text_child() {
    let template = r#"
    export template ui-button [name] =
        <button>Hello {name}</button>
    ;"#;
    parse_grammar(template).unwrap();
}

#[test]
fn test_oil_template_with_binding_dot_dot() {
    let template = r#"
    export template hello [player] =
        Hello {player.name}!
    ;"#;
    parse_grammar(template).unwrap();
}

#[test]
fn test_oil_template_with_binding_dot_dot_dot() {
    let template = r#"
    export template window_info [settings] =
        Screen ratio {settings.window.ratio}
    ;"#;
    parse_grammar(template).unwrap();
}

#[test]
fn test_oil_template_with_many_child() {
    let template = r#"
    export template ui-menu [game-name, btn-class] =
        <group>
            {game-name}
            <button class={btn-class}>Play!</button>
            <button class={btn-class}>Settings</button>
            <button class={btn-class}>Quit</button>
        </group>
    ;"#;
    parse_grammar(template).unwrap();
}

#[test]
fn test_oil_template_static_attribute() {
    let template = r#"
    template ui-button =
        <button [gotoview]="foo"></button>;
    "#;
    parse_grammar(template).unwrap();
}

#[test]
#[should_panic]
fn test_oil_template_not_matching() {
    let template = r#"
    template john_doe = <button></btton>;
    "#;
    parse_grammar(template).unwrap();
}

#[test]
#[should_panic]
fn test_oil_template_not_matching2() {
    let template = r#"
    template john_doe = <button><button>;
    "#;
    parse_grammar(template).unwrap();
}

#[test]
#[should_panic]
fn test_oil_template_wrong_nesting() {
    let template = r#"
    template john_doe = <button><group></button></group>;
    "#;
    parse_grammar(template).unwrap();
}

// Weirdy cases
#[test]
fn test_oil_template_text_node_edge_cases() {
    { let template = r#"template a01 = +;"#; parse_grammar(template).unwrap(); }
    { let template = r#"template a02 = -;"#; parse_grammar(template).unwrap(); }
    { let template = r#"template a03 = &;"#; parse_grammar(template).unwrap(); }
    { let template = r#"template a04 = %;"#; parse_grammar(template).unwrap(); }
    { let template = r#"template a05 = ^;"#; parse_grammar(template).unwrap(); }
    { let template = r#"template a05 = !;"#; parse_grammar(template).unwrap(); }
    { let template = r#"template a05 = ?;"#; parse_grammar(template).unwrap(); }
    { let template = r#"template a05 = @;"#; parse_grammar(template).unwrap(); }
    { let template = r#"template a06 = *;"#; parse_grammar(template).unwrap(); }
    { let template = r#"template a07 = #;"#; parse_grammar(template).unwrap(); }
    { let template = r#"template a08 = ';"#; parse_grammar(template).unwrap(); }
    { let template = r#"template a09 = ";"#; parse_grammar(template).unwrap(); }
    { let template = r#"template a10 = $;"#; parse_grammar(template).unwrap(); }
    { let template = r#"template a11 = :;"#; parse_grammar(template).unwrap(); }
    { let template = r#"template a12 = .;"#; parse_grammar(template).unwrap(); }
    { let template = r#"template a13 = =;"#; parse_grammar(template).unwrap(); }
    { let template = r#"template a14 = [;"#; parse_grammar(template).unwrap(); }
    { let template = r#"template a15 = ];"#; parse_grammar(template).unwrap(); }
    { let template = r#"template a16 = (;"#; parse_grammar(template).unwrap(); }
    { let template = r#"template a17 = );"#; parse_grammar(template).unwrap(); }
}
