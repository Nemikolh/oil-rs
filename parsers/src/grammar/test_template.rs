use super::parse_grammar;


#[test]
fn test_oil_basic_template_no_args() {
    let template = r#"
    template ui_el =
        <el></el>;
    "#;
    parse_grammar(template).unwrap();
}

#[test]
fn test_oil_basic_template_no_args1() {
    let template = r#"
    template ui_el [] =
        <el></el>;
    "#;
    parse_grammar(template).unwrap();
}

#[test]
fn test_oil_basic_template_unused_arg() {
    let template = r#"
    template ui_el [a] =
        <el></el>;
    "#;
    parse_grammar(template).unwrap();
}

#[test]
fn test_oil_basic_template_no_args_export() {
    let template = r#"
    export template ui_el =
        <el></el>;
    "#;
    parse_grammar(template).unwrap();
}

#[test]
fn test_oil_basic_template_with_args() {
    let template = r#"
    template ui_el [el_class] =
        <el class=el_class></el>
    ;"#;
    parse_grammar(template).unwrap();
}

#[test]
fn test_oil_basic_template_with_args_export() {
    let template = r#"
    export template ui_el [el_class] =
        <el class=el_class></el>;
    "#;
    parse_grammar(template).unwrap();
}

#[test]
fn test_oil_basic_template_with_args_return() {
    let template = r#"
    template ui_el [el_class] -> event =
        <el class=el_class (click)=event></el>;
    "#;
    parse_grammar(template).unwrap();
}

#[test]
fn test_oil_basic_template_with_many_return() {
    let template = r#"
    template ui_el [] -> (event, event2) =
        <el class=el_class (click)=event (event)=event2></el>;
    "#;
    parse_grammar(template).unwrap();
}

#[test]
fn test_oil_template_with_text_child() {
    let template = r#"
    export template ui_button [name] =
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
    export template ui_menu [game_name, btn_class] =
        <group>
            {game_name}
            <button class=btn_class>Play!</button>
            <button class={.btn_class}>Settings</button>
            <button class={.btn_class;}>Quit</button>
        </group>
    ;"#;
    parse_grammar(template).unwrap();
}

#[test]
fn test_oil_template_style_anonymous_class() {
    let template = r#"
    export template ui_menu [game, btn_class] =
        <group class={
            .dark_theme                 if game.dark_theme;
            width: 400px                if game.window.width > 300;
            width: game.window.width    if game.window.width <= 300;
            height: game.window.weight;
        }>
            {game.name}
            <button class={.btn_class}>Play!</button>
            <button class={.btn_class}>Settings</button>
            <button class={.btn_class}>Quit</button>
        </group>
    ;"#;
    parse_grammar(template).unwrap();
}

#[test]
fn test_oil_template_static_argument() {
    let template = r#"
    template ui_button =
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

#[test]
fn test_oil_template_no_keyword_in_template_or_view() {
    { let template = r#"export template a01 = export;"#; parse_grammar(template).unwrap(); }
    { let template = r#"export template a01 = import;"#; parse_grammar(template).unwrap(); }
    { let template = r#"export template a01 = template;"#; parse_grammar(template).unwrap(); }
    { let template = r#"export template a01 = view;"#; parse_grammar(template).unwrap(); }
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
