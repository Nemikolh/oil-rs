use super::parse_grammar;
use lalrpop_util::ParseError;
use tok;


macro_rules! assert_err_code {
    ($text:ident, $code:path) => (
        match parse_grammar($text) {
            Err(ParseError::User {
                error: tok::Error {
                    code: $code,
                    ..
                }
            }) => assert!(true),
            c => {
                println!("Found: {:?}", c);
                println!("Expected: Err({:?})", $code);
                assert!(false)
            }
        }
    );
}

#[test]
fn test_error_import_img_or_font() {
    let import = r#"import img from './logo.png';"#;
    assert_err_code!(import, tok::ErrorCode::OnlyFontOrImg)
}

#[test]
fn test_error_unmatchintag_in_template_or_view() {
    let template = r#"template tmp = <test></tst>;"#;
    assert_err_code!(template, tok::ErrorCode::UnmatchingTag)
}

#[test]
fn test_error_invalid_select() {
    let template = r#"template tmp = <why:we/>;"#;
    assert_err_code!(template, tok::ErrorCode::InvalidSelect)
}

#[test]
fn test_error_invalid_query() {
    let template = r#"template tmp = <select:we/>;"#;
    assert_err_code!(template, tok::ErrorCode::InvalidQuery)
}

#[test]
fn test_error_invalid_style_property() {
    let style = r#".some { test: 234zef;}"#;
    assert_err_code!(style, tok::ErrorCode::InvalidUnit)
}
