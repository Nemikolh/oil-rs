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
            }) => (),
            c => {
                println!("Found: {:?}", c);
                panic!("Expected: Err({:?})", $code);
            }
        }
    );
}

// #[test]
// fn test_error_import_img_or_font() {
//     let import = r#"import img from './logo.png';"#;
//     assert_err_code!(import, tok::ErrorCode::OnlyFontOrImg)
// }

#[test]
fn test_error_unmatchintag_in_component_or_view() {
    let component = r#"component tmp = <test></tst>;"#;
    assert_err_code!(component, tok::ErrorCode::UnmatchingTag)
}

// #[test]
// fn test_error_invalid_select() {
//     let component = r#"component tmp = <why:we/>;"#;
//     assert_err_code!(component, tok::ErrorCode::InvalidSelect)
// }

// #[test]
// fn test_error_invalid_query() {
//     let component = r#"component tmp = <select:we/>;"#;
//     assert_err_code!(component, tok::ErrorCode::InvalidQuery)
// }