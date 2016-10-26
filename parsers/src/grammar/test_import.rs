use super::parse_grammar;


#[test]
fn test_oil_import_all() {
    let import = r#"import * from 'material-oil';"#;
    parse_grammar(import).unwrap();
}

#[test]
fn test_oil_import_all_variant1() {
    let import = r#"import 'material-oil';"#;
    parse_grammar(import).unwrap();
}

#[test]
fn test_oil_import_one() {
    let import = r#"import {btn} from 'material-oil';"#;
    parse_grammar(import).unwrap();
}

#[test]
fn test_oil_import_some() {
    let import = r#"import {btn, prg, g1} from 'material-oil';"#;
    parse_grammar(import).unwrap();
}

#[test]
fn test_oil_import_path1() {
    let import = r#"import './path/to/file1';"#;
    parse_grammar(import).unwrap();
}

#[test]
fn test_oil_import_path2() {
    let import = r#"import '.././path/../path/to/./file1';"#;
    parse_grammar(import).unwrap();
}

#[test]
fn test_oil_import_image1() {
    let import = r#"import $img from './logo.png';"#;
    parse_grammar(import).unwrap();
}

#[test]
fn test_oil_import_image2() {
    let import = r#"import logo from './logo.png';"#;
    parse_grammar(import).unwrap();
}

#[test]
fn test_oil_valid_module_import_that_look_like_image_import() {
    // Should be rejected by later pass but is accepted
    // by the parser.
    let import = r#"import {img} from './logo.png';"#;
    parse_grammar(import).unwrap();
}

#[test]
fn test_oil_import_font() {
    let import = r#"import $font from 'material-oil';"#;
    parse_grammar(import).unwrap();
}

#[test]
fn test_oil_import_font2() {
    let import = r#"import font from 'material-oil';"#;
    parse_grammar(import).unwrap();
}
