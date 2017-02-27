use super::{TokState, Tokenizer, Tok};


#[test]
fn test_tok_state() {
    let txt = "component = a<a>;a{c.d}";
    let mut tok = Tokenizer::new(txt, 0);
    assert_eq!(tok.state, TokState::Normal);
    assert_eq!(tok.next().unwrap().unwrap().1, Tok::Component);
    assert_eq!(tok.state, TokState::WaitingForEq);
    assert_eq!(tok.next().unwrap().unwrap().1, Tok::Equals);
    assert_eq!(tok.state, TokState::InComponentOrView);
    assert_variant!(tok.next().unwrap().unwrap().1, Tok::TextNode);
    assert_eq!(tok.next().unwrap().unwrap().1, Tok::LessThan);
    assert_eq!(tok.state, TokState::InTag);
    assert_variant!(tok.next().unwrap().unwrap().1, Tok::Id);
    assert_eq!(tok.next().unwrap().unwrap().1, Tok::GreaterThan);
    assert_eq!(tok.state, TokState::InComponentOrView);
    assert_eq!(tok.next().unwrap().unwrap().1, Tok::Semi);
    assert_eq!(tok.state, TokState::Normal);
    assert_variant!(tok.next().unwrap().unwrap().1, Tok::Id);
    assert_eq!(tok.state, TokState::Normal);
    assert_eq!(tok.next().unwrap().unwrap().1, Tok::LeftBrace);
    assert_eq!(tok.state, TokState::Normal);
    assert_variant!(tok.next().unwrap().unwrap().1, Tok::Id);
    assert_eq!(tok.state, TokState::Normal);
    assert_eq!(tok.next().unwrap().unwrap().1, Tok::Dot);
    assert_eq!(tok.state, TokState::Normal);
    assert_variant!(tok.next().unwrap().unwrap().1, Tok::Id);
    assert_eq!(tok.state, TokState::Normal);
    assert_eq!(tok.next().unwrap().unwrap().1, Tok::RightBrace);
    assert_eq!(tok.state, TokState::Normal);
}
