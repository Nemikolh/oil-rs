use super::parse_grammar;


#[test]
fn test_oil_style_class_def() {
    let style = r#"
    .some-class {
    }
    "#;
    parse_grammar(style).unwrap();
}

#[test]
fn test_oil_style_hexadecimal_value() {
    let style = r#"
    .some-class {
        background-color: #FF0A0E;
        background-color: #ffffff;
    }
    "#;
    parse_grammar(style).unwrap();
}

#[test]
#[should_panic]
fn test_oil_style_invalid_hexadecimal_value() {
    let style = r#"
    .some-class {
        background-color: #+FF0A0E;
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
        background-image: $img;
        font: $font;
    }
    "#;
    parse_grammar(style).unwrap();
}

#[test]
fn test_oil_style_sub_img_explicit() {
    let style = r#"
    .btn {
        background-image: $img[y: 172, x: 0, w: 400, h: 40];
    }
    "#;
    parse_grammar(style).unwrap();
}

#[test]
fn test_oil_style_sub_img_implicit() {
    let style = r#"
    .btn {
        background-image: $img[172, 0, 400, 40];
    }
    "#;
    parse_grammar(style).unwrap();
}

#[test]
fn test_oil_style_entire_img_alternatives() {
    { let style = r#".btn { background-image: $img[]; }"#; parse_grammar(style).unwrap(); }
    { let style = r#".btn { background-image: $img; }"#; parse_grammar(style).unwrap(); }
}

#[test]
#[should_panic]
fn test_oil_style_incorrect_property() {
    let style = r#"
    .some-class {
        width: 100px
    }
    "#;
    parse_grammar(style).unwrap();
}

#[test]
#[should_panic]
fn test_oil_style_incorrect_class_def1() {
    let style = r#"
    . some-class {
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
