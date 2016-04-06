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
fn test_oil_style_properties() {
    let style = r#"
    .btn {
        width: 100px;
        height: 40px;
        background-image: $img
        font: $font
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
