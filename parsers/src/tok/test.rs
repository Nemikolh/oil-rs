use super::{StrView, TokState, Tokenizer, Tok};

#[test]
fn test_strview_should_eq_str() {
    let some_str = "hello &é)*ù!";
    let view = StrView::from(some_str, 0, some_str.len());
    assert_eq!(view.as_str(), some_str);
}

#[test]
fn test_strview_partial_should_eq() {
    let some_str = "hello &é)*ù!";
    let view = StrView::from(some_str, 6, some_str.len() - 1);
    assert_eq!(view.as_str(), "&é)*ù");
}

#[test]
fn test_strview_merge() {
    let some_str = "hello &é)*ù!";
    let view1 = StrView::from(some_str, 0, 1);
    let view2 = StrView::from(some_str, 4, 5);
    assert_eq!(view1.as_str(), "h");
    assert_eq!(view2.as_str(), "o");
    assert_eq!(view1.merge(view2.clone()).as_str(), "hello");
    assert_eq!(view2.merge(view1.clone()).as_str(), "hello");
}

#[test]
fn test_to_string() {
    let some_str = "hello &é)*ù!";
    assert_eq!(StrView::from(some_str, 0, 5).to_string(), "hello".to_string());
}

#[test]
fn test_tok_state() {
    let txt = "template = a<a>;a{c.d}";
    let mut tok = Tokenizer::new(txt, 0);
    assert_eq!(tok.state, TokState::Normal);
    assert_eq!(tok.next().unwrap().unwrap().1, Tok::Template);
    assert_eq!(tok.state, TokState::WaitingForEq);
    assert_eq!(tok.next().unwrap().unwrap().1, Tok::Equals);
    assert_eq!(tok.state, TokState::InTemplateOrView);
    assert_variant!(tok.next().unwrap().unwrap().1, Tok::TextNode);
    assert_eq!(tok.next().unwrap().unwrap().1, Tok::LessThan);
    assert_eq!(tok.state, TokState::InTag);
    assert_variant!(tok.next().unwrap().unwrap().1, Tok::Id);
    assert_eq!(tok.next().unwrap().unwrap().1, Tok::GreaterThan);
    assert_eq!(tok.state, TokState::InTemplateOrView);
    assert_eq!(tok.next().unwrap().unwrap().1, Tok::Semi);
    assert_eq!(tok.state, TokState::Normal);
    assert_variant!(tok.next().unwrap().unwrap().1, Tok::Id);
    assert_eq!(tok.state, TokState::Normal);
    assert_eq!(tok.next().unwrap().unwrap().1, Tok::LeftBrace);
    assert_eq!(tok.state, TokState::Normal);
    assert_variant!(tok.next().unwrap().unwrap().1, Tok::Id);
    assert_eq!(tok.state, TokState::Normal);
    assert_variant!(tok.next().unwrap().unwrap().1, Tok::DotId);
    assert_eq!(tok.state, TokState::Normal);
    assert_eq!(tok.next().unwrap().unwrap().1, Tok::RightBrace);
    assert_eq!(tok.state, TokState::Normal);
}
