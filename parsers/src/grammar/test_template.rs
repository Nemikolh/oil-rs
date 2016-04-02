use super::oil::{parse_ui_package};


#[test]
fn test_oil_basic_template_no_args() {
    let template = r#"
    template ui-el =
        <el></el>
    "#;
    parse_ui_package(template).unwrap();
}

#[test]
fn test_oil_basic_template_no_args1() {
    let template = r#"
    template ui-el [] =
        <el></el>
    "#;
    parse_ui_package(template).unwrap();
}

#[test]
fn test_oil_basic_template_unused_arg() {
    let template = r#"
    template ui-el [a] =
        <el></el>
    "#;
    parse_ui_package(template).unwrap();
}

#[test]
fn test_oil_basic_template_no_args_export() {
    let template = r#"
    export template ui-el =
        <el></el>
    "#;
    parse_ui_package(template).unwrap();
}

#[test]
fn test_oil_basic_template_with_args() {
    let template = r#"
    template ui-el [el-class] =
        <el class={el-class}></el>
    "#;
    parse_ui_package(template).unwrap();
}

#[test]
fn test_oil_basic_template_with_args_export() {
    let template = r#"
    export template ui-el [el-class] =
        <el class={el-class}></el>
    "#;
    parse_ui_package(template).unwrap();
}

#[test]
fn test_oil_basic_template_with_args_return() {
    let template = r#"
    template ui-el [el-class] -> event =
        <el class={el-class} (click)={event}></el>
    "#;
    parse_ui_package(template).unwrap();
}

#[test]
fn test_oil_template_with_text_child() {
    let template = r#"
    export template ui-button [name] =
        <button>Hello {name}</button>
    "#;
    parse_ui_package(template).unwrap();
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
    "#;
    parse_ui_package(template).unwrap();
}

#[test]
fn test_oil_template_static_attribute() {
    let template = r#"
    template ui-button =
        <button [gotoview]="foo"></button>
    "#;
    parse_ui_package(template).unwrap();
}

#[test]
#[should_panic]
fn test_oil_template_not_matching() {
    let template = r#"
    template john_doe = <button></btton>
    "#;
    parse_ui_package(template).unwrap();
}

#[test]
#[should_panic]
fn test_oil_template_not_matching2() {
    let template = r#"
    template john_doe = <button><button>
    "#;
    parse_ui_package(template).unwrap();
}

#[test]
#[should_panic]
fn test_oil_template_wrong_nesting() {
    let template = r#"
    template john_doe = <button><group></button></group>
    "#;
    parse_ui_package(template).unwrap();
}
