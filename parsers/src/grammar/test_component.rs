use super::parse_grammar;


#[test]
fn test_oil_basic_component_no_args() {
    let component = r#"
    component ui_el =
        <el></el>;
    "#;
    parse_grammar(component).unwrap();
}

#[test]
fn test_oil_basic_component_no_args1() {
    let component = r#"
    component ui_el [] =
        <el></el>;
    "#;
    parse_grammar(component).unwrap();
}

#[test]
fn test_oil_basic_component_unused_arg() {
    let component = r#"
    component ui_el [a] =
        <el></el>;
    "#;
    parse_grammar(component).unwrap();
}

#[test]
fn test_oil_basic_component_no_args_export() {
    let component = r#"
    export component ui_el =
        <el></el>;
    "#;
    parse_grammar(component).unwrap();
}

#[test]
fn test_oil_basic_component_with_args() {
    let component = r#"
    component ui_el [el_class] =
        <el class=el_class></el>
    ;"#;
    parse_grammar(component).unwrap();
}

#[test]
fn test_oil_basic_component_with_args_export() {
    let component = r#"
    export component ui_el [el_class] =
        <el class={.el_class;}></el>;
    "#;
    parse_grammar(component).unwrap();
}

#[test]
fn test_oil_basic_component_with_args_return() {
    let component = r#"
    component ui_el [el_class] -> event =
        <el class={.el_class;} (click)={event}></el>;
    "#;
    parse_grammar(component).unwrap();
}

#[test]
fn test_oil_basic_component_with_many_return() {
    let component = r#"
    component ui_el [] -> (event, event2) =
        <el class={.el_class;} (click)={event} (event)={event2}></el>;
    "#;
    parse_grammar(component).unwrap();
}

#[test]
fn test_oil_component_with_text_child() {
    let component = r#"
    export component ui_button [name] =
        <button>Hello {{name}}</button>
    ;"#;
    parse_grammar(component).unwrap();
}

#[test]
fn test_oil_component_with_binding_dot_dot() {
    let component = r#"
    export component hello [player] =
        Hello {{player.name}}!
    ;"#;
    parse_grammar(component).unwrap();
}

#[test]
fn test_oil_component_with_binding_dot_dot_dot() {
    let component = r#"
    export component window_info [settings] =
        Screen ratio {{settings.window.ratio}}
    ;"#;
    parse_grammar(component).unwrap();
}

#[test]
fn test_oil_component_with_many_child() {
    let component = r#"
    export component ui_menu [game_name, btn_class] =
        <group>
            {{game_name}}
            <button class=btn_class>Play!</button>
            <button class={.btn_class;}>Settings</button>
            <button class={.btn_class;}>Quit</button>
        </group>
    ;"#;
    parse_grammar(component).unwrap();
}

#[test]
fn test_oil_component_style_anonymous_class() {
    let component = r#"
    export component ui_menu [game, btn_class] =
        <group class={
            .dark_theme                 if game.dark_theme;
            width: 400                  if game.window.width > 300;
            width: game.window.width    if game.window.width <= 300;
            // Equivalent as above: the unit is inferred based on the property.
            // As `height` accept only length and `px` is the default unit,
            // this is equivalent to "game.window.height px"
            height: game.window.height;
        }>
            {{game.name}}
            <button class={.btn_class;}>Play!</button>
            <button class={.btn_class;}>Settings</button>
            <button class={.btn_class;}>Quit</button>
        </group>
    ;"#;
    parse_grammar(component).unwrap();
}

#[test]
fn test_oil_component_static_argument() {
    let component = r#"
    component ui_button =
        <button [gotoview]={foo}></button>;
    "#;
    parse_grammar(component).unwrap();
}

#[test]
fn test_oil_component_object_argument_simple_expr() {
    let component = r#"
    component ui_button =
        <el [arg1]={foo}></el>;
    "#;
    parse_grammar(component).unwrap();
}

#[test]
fn test_oil_component_object_argument_complex_expr() {
    let component = r#"
    component ui_button =
        <el [arg1]={foo + foo ^ 2}></el>;
    "#;
    parse_grammar(component).unwrap();
}

#[test]
fn test_oil_component_object_argument_property_renaming() {
    let component = r#"
    component ui_button =
        <el [arg1]={{settings: { foo: foo + foo ^ 2 }, bar: 23, test: "text"}}></el>;
    "#;
    parse_grammar(component).unwrap();
}

#[test]
fn test_oil_component_object_crazy_nesting() {
    let component = r#"
    component something =
        <el [a]={{test: {
            aa: "foo",
            bb: 12 + 35
        }, test1: "set", test1: { crazy: "bar" } }}></el>;
    "#;
    parse_grammar(component).unwrap();
}

#[test]
#[ignore]
fn test_oil_component_with_prelude() {
    let component = r#"
    component something = {
        let obj = {
            test: {
                aa: "foo",
                bb: 12 + 35
            },
            test1: "set",
            test1.crazy: "bar"
        };
        <el [a]={obj}></el>
    }"#;
    parse_grammar(component).unwrap();
}

#[test]
#[should_panic]
fn test_oil_component_invalid_arg_anonymous_class() {
    let component = r#"
    component ui_button =
        <e [arg1]={foo @if b}></e>;
    "#;
    parse_grammar(component).unwrap();
}

#[test]
#[should_panic]
fn test_oil_component_not_matching() {
    let component = r#"
    component john_doe = <button></btton>;
    "#;
    parse_grammar(component).unwrap();
}

#[test]
#[should_panic]
fn test_oil_component_not_matching2() {
    let component = r#"
    component john_doe = <button><button>;
    "#;
    parse_grammar(component).unwrap();
}

#[test]
#[should_panic]
fn test_oil_component_wrong_nesting() {
    let component = r#"
    component john_doe = <button><group></button></group>;
    "#;
    parse_grammar(component).unwrap();
}

#[test]
fn test_oil_component_no_keyword_in_component_or_view() {
    { let component = r#"export component a01 = export;"#; parse_grammar(component).unwrap(); }
    { let component = r#"export component a01 = import;"#; parse_grammar(component).unwrap(); }
    { let component = r#"export component a01 = component;"#; parse_grammar(component).unwrap(); }
    { let component = r#"export component a01 = view;"#; parse_grammar(component).unwrap(); }
}

// Weirdy cases
#[test]
fn test_oil_component_text_node_edge_cases() {
    { let component = r#"component a01 = +;"#; parse_grammar(component).unwrap(); }
    { let component = r#"component a02 = -;"#; parse_grammar(component).unwrap(); }
    { let component = r#"component a03 = &;"#; parse_grammar(component).unwrap(); }
    { let component = r#"component a04 = %;"#; parse_grammar(component).unwrap(); }
    { let component = r#"component a05 = ^;"#; parse_grammar(component).unwrap(); }
    { let component = r#"component a05 = !;"#; parse_grammar(component).unwrap(); }
    { let component = r#"component a05 = ?;"#; parse_grammar(component).unwrap(); }
    { let component = r#"component a05 = @;"#; parse_grammar(component).unwrap(); }
    { let component = r#"component a06 = *;"#; parse_grammar(component).unwrap(); }
    { let component = r#"component a07 = #;"#; parse_grammar(component).unwrap(); }
    { let component = r#"component a08 = ';"#; parse_grammar(component).unwrap(); }
    { let component = r#"component a09 = ";"#; parse_grammar(component).unwrap(); }
    { let component = r#"component a10 = $;"#; parse_grammar(component).unwrap(); }
    { let component = r#"component a11 = :;"#; parse_grammar(component).unwrap(); }
    { let component = r#"component a12 = .;"#; parse_grammar(component).unwrap(); }
    { let component = r#"component a13 = =;"#; parse_grammar(component).unwrap(); }
    { let component = r#"component a14 = [;"#; parse_grammar(component).unwrap(); }
    { let component = r#"component a15 = ];"#; parse_grammar(component).unwrap(); }
    { let component = r#"component a16 = (;"#; parse_grammar(component).unwrap(); }
    { let component = r#"component a17 = );"#; parse_grammar(component).unwrap(); }
}
