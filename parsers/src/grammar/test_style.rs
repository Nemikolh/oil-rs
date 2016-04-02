use super::oil::{parse_ui_package};


#[test]
fn test_oil_style_class_def() {
    let style = r#"
    .some-class {
    }
    "#;
    parse_ui_package(style).unwrap();
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
    parse_ui_package(style).unwrap();
}

#[test]
#[should_panic]
fn test_oil_style_incorrect_class_def1() {
    let style = r#"
    . some-class {
    }
    "#;
    parse_ui_package(style).unwrap();
}

#[test]
#[should_panic]
fn test_oil_style_incorrect_class_def2() {
    let style = r#"
    .$img {
    }
    "#;
    parse_ui_package(style).unwrap();
}
