use super::parse_grammar;


#[test]
fn test_oil_style_class_def() {
    let style = r#"
    .some_class {
    }
    "#;
    parse_grammar(style).unwrap();
}

#[test]
#[should_panic]
fn test_oil_style_invalid_class_def() {
    let style = r#".some-class {}"#;
    parse_grammar(style).unwrap();
}

#[test]
fn test_oil_style_class_def_with_empty_args() {
    let style = r#".some_class () {}"#;
    parse_grammar(style).unwrap();
}

#[test]
fn test_oil_style_class_def_with_one_arg() {
    let style = r#".some_class (a) {}"#;
    parse_grammar(style).unwrap();
}

#[test]
fn test_oil_style_class_def_with_many_args() {
    let style = r#".some_class (a, b, c) {}"#;
    parse_grammar(style).unwrap();
}

#[test]
fn test_oil_style_class_def_condition_eq() {
    let style = r#".a { .b if c == d; }"#;
    parse_grammar(style).unwrap();
}

#[test]
fn test_oil_style_class_def_condition_neq() {
    let style = r#".a { .b if c != d; }"#;
    parse_grammar(style).unwrap();
}

#[test]
fn test_oil_style_class_def_condition_and() {
    let style = r#".a { .b if c && d; }"#;
    parse_grammar(style).unwrap();
}

#[test]
fn test_oil_style_class_def_condition_or() {
    let style = r#".a { .b if c || d; }"#;
    parse_grammar(style).unwrap();
}

#[test]
fn test_oil_style_class_def_condition_many1() {
    let style = r#".a { .b if (c || d) && d; }"#;
    parse_grammar(style).unwrap();
}

#[test]
fn test_oil_style_class_def_condition_many2() {
    let style = r#".a { .b if c || (d && d); }"#;
    parse_grammar(style).unwrap();
}

#[test]
fn test_oil_style_class_def_conditionally_include() {
    let style = r#".some_class (a, b) {
        .other_class if a == b;
    }"#;
    parse_grammar(style).unwrap();
}

#[test]
fn test_oil_style_class_def_conditionally_include2() {
    let style = r#".some_class (a, b) {
        .other_class(fit, rtl) if a == b;
    }"#;
    parse_grammar(style).unwrap();
}

#[test]
fn test_oil_style_class_def_conditionally_properties() {
    let style = r#".some_class (a, b) {
        background_color: #FFFFFF if a;
        background_image: $img    if b;
    }"#;
    parse_grammar(style).unwrap();
}

#[test]
fn test_oil_style_hexadecimal_value() {
    let style = r#"
    .some_class {
        background_color: #FF0A0E;
        background_color: #ffffff;
    }
    "#;
    parse_grammar(style).unwrap();
}

#[test]
#[should_panic]
fn test_oil_style_invalid_hexadecimal_value() {
    let style = r#"
    .some_class {
        background_color: #+FF0A0E;
    }
    "#;
    parse_grammar(style).unwrap();
}

#[test]
fn test_oil_style_properties() {
    let style = r#"
    .btn {
        width: 100px;
        height: 40px;
        background_image: $img;
        font: $font;
    }
    "#;
    parse_grammar(style).unwrap();
}

#[test]
fn test_oil_style_sub_img_explicit() {
    let style = r#"
    .btn {
        background_image: $img[y: 172, x: 0, w: 400, h: 40];
    }
    "#;
    parse_grammar(style).unwrap();
}

#[test]
fn test_oil_style_sub_img_implicit() {
    let style = r#"
    .btn {
        background_image: $img[172, 0, 400, 40];
    }
    "#;
    parse_grammar(style).unwrap();
}

#[test]
fn test_oil_style_entire_img_alternatives() {
    { let style = r#".btn { background_image: $img[]; }"#; parse_grammar(style).unwrap(); }
    { let style = r#".btn { background_image: $img; }"#; parse_grammar(style).unwrap(); }
}

#[test]
#[should_panic]
fn test_oil_style_incorrect_property() {
    let style = r#"
    .some_class {
        width: 100px
    }
    "#;
    parse_grammar(style).unwrap();
}

#[test]
#[should_panic]
fn test_oil_style_incorrect_class_def1() {
    let style = r#"
    . some_class {
    }
    "#;
    parse_grammar(style).unwrap();
}

#[test]
#[should_panic]
fn test_oil_style_incorrect_class_def2() {
    let style = r#"
    .$img {
    }
    "#;
    parse_grammar(style).unwrap();
}
