use std::str::CharIndices;
use std::slice;
use std::str;
use std::cmp::{min, max};
use std::ops::Deref;
use std::marker::PhantomData;
use std::fmt;
use unicode_xid::UnicodeXID;

use self::ErrorCode::*;
use self::Tok::*;

#[cfg(test)]
mod test;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    pub location: usize,
    pub code: ErrorCode
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorCode {
    UnrecognizedToken,
    UnterminatedTextNode,
    UnterminatedStringLiteral,
    OnlyFontOrImg,
    UnmatchingTag,
    InvalidVarName,
    InvalidUnit,
    InvalidRange,
    InvalidQuery,
    InvalidSelect,
}

fn error<T>(c: ErrorCode, l: usize) -> Result<T,Error> {
    Err(Error { location: l, code: c })
}

#[derive(Clone, PartialEq, Eq)]
pub struct StrView<'input> {
    ptr: *const u8,
    begin_at: usize,
    finish_at: usize,
    phantom: PhantomData<&'input str>
}

impl<'input> fmt::Debug for StrView<'input> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'input> StrView<'input> {
    fn from(text: &'input str, start: usize, end: usize) -> StrView<'input> {
        assert!(start < text.len());
        assert!(end <= text.len());
        StrView {
            ptr: text.as_ptr(),
            begin_at: start,
            finish_at: end,
            phantom: PhantomData,
        }
    }

    pub fn merge(&self, other: StrView<'input>) -> StrView<'input> {
        assert_eq!(self.ptr, other.ptr);
        StrView {
            ptr: self.ptr,
            begin_at: min(self.begin_at, other.begin_at),
            finish_at: max(self.finish_at, other.finish_at),
            phantom: PhantomData,
        }
    }

    pub fn as_str(&self) -> &'input str {
        unsafe {
            let len = self.finish_at - self.begin_at;
            let ptr = self.ptr.offset(self.begin_at as isize);
            let slice = slice::from_raw_parts(ptr, len);
            // TODO: Use 'str::from_utf8_unchecked' instead
            str::from_utf8(slice).unwrap()
        }
    }
}

impl<'input> Deref for StrView<'input> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tok<'input> {
    // Keywords;
    Import,
    From,
    View,
    Template,
    Export,
    If,

    // Special keywords: these are accompanied by a series of
    // uninterpreted strings representing imports and stuff.
    StringLiteral(&'input str),
    Number(&'input str),
    Hex(&'input str),

    TextNode(&'input str),

    // Identifiers of various kinds:
    DotId(StrView<'input>),
    Id(StrView<'input>),

    // Symbols:
    Arrow,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    LessThan,
    LessThanSlash,
    GreaterThan,
    SlashGreaterThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    EqualsEquals,
    BangEquals,
    And,
    Or,
    Div,
    Mod,
    Pipe,
    Bang,
    Caret,
    Equals,
    Plus,
    Minus,
    Semi,
    Star,
    Colon,
    Comma,
}

pub struct Tokenizer<'input> {
    text: &'input str,
    chars: CharIndices<'input>,
    lookahead: Option<(usize, char)>,
    shift: usize,
    state: TokState,
    prev_state: TokState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum TokState {
    Normal,
    WaitingForEq,
    InTemplateOrView,
    InTag,
    InBrace,
}

pub type Spanned<T> = (usize, T, usize);

const KEYWORDS: &'static [(&'static str, Tok<'static>)] = &[
    ("import", Import),
    ("export", Export),
    ("from", From),
    ("view", View),
    ("template", Template),
    ("if", If),
    ];

impl<'input> Tokenizer<'input> {
    pub fn new(text: &'input str, shift: usize) -> Tokenizer<'input> {
        let mut t = Tokenizer {
            text: text,
            chars: text.char_indices(),
            lookahead: None,
            shift: shift,
            state: TokState::Normal,
            prev_state: TokState::Normal,
        };
        t.bump();
        t
    }

    #[inline]
    fn not_text(&self) -> bool {
        self.state != TokState::InTemplateOrView
    }

    #[inline]
    fn go_in_template(&mut self) -> bool {
        if self.state == TokState::WaitingForEq {
            self.state = TokState::InTemplateOrView;
            true
        } else {
            false
        }
    }

    #[inline]
    fn go_out_of_tag(&mut self) {
        if self.state == TokState::InTag {
            self.state = TokState::InTemplateOrView;
        }
    }

    #[inline]
    fn go_in_tag(&mut self) {
        if self.state == TokState::InTemplateOrView {
            self.state = TokState::InTag;
        }
    }

    #[inline]
    fn go_in_brace(&mut self) {
        match &self.state {
            &TokState::InTemplateOrView => {
                self.prev_state = TokState::InTemplateOrView;
                self.state = TokState::InBrace;
            }
            &TokState::InTag => {
                self.prev_state = TokState::InTag;
                self.state = TokState::InBrace;
            }
            _ => (),
        }
    }

    #[inline]
    fn go_out_of_brace(&mut self) {
        match (&self.prev_state, &self.state) {
            (&TokState::InTag, &TokState::InBrace) => {
                self.state = TokState::InTag
            }
            (_, &TokState::InBrace) => {
                self.state = TokState::InTemplateOrView;
            }
            _ => (),
        }
    }

    #[inline]
    fn go_out_of_template(&mut self) {
        if self.state == TokState::InTemplateOrView {
            self.state = TokState::Normal;
        }
    }

    fn next_unshifted(&mut self) -> Option<Result<Spanned<Tok<'input>>, Error>> {
        loop {
            return match self.lookahead {
                // ====================================================
                //              Not ignored in text mode
                Some((idx0, '<')) => {
                    self.go_in_tag();
                    match self.bump() {
                        Some((idx1, '/')) => {
                            self.bump();
                            Some(Ok((idx0, LessThanSlash, idx1+1)))
                        }
                        Some((idx1, '=')) => {
                            self.bump();
                            Some(Ok((idx0, LessThanOrEqual, idx1+1)))
                        }
                        _ => Some(Ok((idx0, LessThan, idx0+1)))
                    }
                }
                Some((idx0, '>')) => {
                    self.go_out_of_tag();
                    match self.bump() {
                        Some((idx1, '=')) => {
                            self.bump();
                            Some(Ok((idx0, GreaterThanOrEqual, idx1+1)))
                        }
                        _ => Some(Ok((idx0, GreaterThan, idx0+1)))
                    }
                }
                Some((idx0, '{')) => {
                    self.go_in_brace();
                    self.bump();
                    Some(Ok((idx0, LeftBrace, idx0+1)))
                }
                Some((idx0, '}')) => {
                    self.go_out_of_brace();
                    self.bump();
                    Some(Ok((idx0, RightBrace, idx0+1)))
                }
                Some((idx0, ';')) => {
                    self.go_out_of_template();
                    self.bump();
                    Some(Ok((idx0, Semi, idx0+1)))
                }
                // TODO: Can't be parsed as beginning of a TextNode..
                Some((idx0, '/')) => {
                    match self.bump() {
                        Some((_, '/')) => {
                            self.take_until(|c| c == '\n');
                            continue;
                        }
                        Some((idx1, '>')) => {
                            self.bump();
                            Some(Ok((idx0, SlashGreaterThan, idx1+1)))
                        }
                        _ if self.not_text() => Some(Ok((idx0, Div, idx0+1))),
                        _ => Some(error(UnrecognizedToken, idx0)),
                    }
                }
                // ====================================================
                //              Ignored in text mode
                Some((idx0, '-')) if self.not_text() => {
                    match self.bump() {
                        Some((idx1, '>')) => {
                            self.bump();
                            Some(Ok((idx0, Arrow, idx1+1)))
                        }
                        // Some((_, d)) if d.is_digit(10) => {
                        //     Some(self.number(idx0))
                        // }
                        _ => Some(Ok((idx0, Minus, idx0+1))),
                    }
                }
                Some((idx0, '+')) if self.not_text() => {
                    match self.bump() {
                        // Some((_, d)) if d.is_digit(10) => {
                        //     Some(self.number(idx0))
                        // }
                        _ => Some(Ok((idx0, Plus, idx0+1)))
                    }
                }
                Some((idx0, d)) if d.is_digit(10) && self.not_text() => {
                    Some(self.number(idx0))
                }
                Some((_, '#')) if self.not_text() => {
                    match self.bump() {
                        Some((idx1, _)) => {
                            Some(Ok(self.hexadecimal(idx1)))
                        }
                        None => None
                    }
                }
                Some((idx0, '$')) if self.not_text() => {
                    // TODO: merge this with identifier branch.
                    self.bump();
                    let (_, _, end) = self.word(idx0);
                    Some(Ok((idx0, Id(StrView::from(self.text, idx0, end)), end)))
                }
                Some((idx0, '!')) if self.not_text() => {
                    match self.bump() {
                        Some((idx1, '=')) => {
                            self.bump();
                            Some(Ok((idx0, BangEquals, idx1+1)))
                        }
                        _ => Some(Ok((idx0, Bang, idx0+1)))
                    }
                }
                Some((idx0, '%')) if self.not_text() => {
                    self.bump();
                    Some(Ok((idx0, Mod, idx0+1)))
                }
                Some((idx0, '^')) if self.not_text() => {
                    self.bump();
                    Some(Ok((idx0, Caret, idx0+1)))
                }
                Some((idx0, ':')) if self.not_text() => {
                    self.bump();
                    Some(Ok((idx0, Colon, idx0+1)))
                }
                Some((idx0, ',')) if self.not_text() => {
                    self.bump();
                    Some(Ok((idx0, Comma, idx0+1)))
                }
                Some((idx0, '.')) if self.not_text() => {
                    self.bump();
                    let (start, _, end) = self.word(idx0);
                    Some(Ok((start, DotId(StrView::from(self.text, start, end)), end)))
                }
                Some((idx0, '=')) if self.not_text() => {
                    match (self.bump(), self.go_in_template()) {
                        (Some((idx1, '=')), false) => {
                            self.bump();
                            Some(Ok((idx0, EqualsEquals, idx1+1)))
                        }
                        _ => Some(Ok((idx0, Equals, idx0+1))),
                    }
                }
                Some((idx0, '&')) if self.not_text() => {
                    match self.bump() {
                        Some((idx1, '&')) => {
                            self.bump();
                            Some(Ok((idx0, And, idx1+1)))
                        }
                        _ => Some(error(UnrecognizedToken, idx0)),
                    }
                }
                Some((idx0, '|')) if self.not_text() => {
                    match self.bump() {
                        Some((idx1, '|')) => {
                            self.bump();
                            Some(Ok((idx0, Or, idx1+1)))
                        }
                        _ => Some(Ok((idx0, Pipe, idx0+1))),
                    }
                }
                Some((idx0, '[')) if self.not_text() => {
                    self.bump();
                    Some(Ok((idx0, LeftBracket, idx0+1)))
                }
                Some((idx0, '(')) if self.not_text() => {
                    self.bump();
                    Some(Ok((idx0, LeftParen, idx0+1)))
                }
                Some((idx0, ']')) if self.not_text() => {
                    self.bump();
                    Some(Ok((idx0, RightBracket, idx0+1)))
                }
                Some((idx0, ')')) if self.not_text() => {
                    self.bump();
                    Some(Ok((idx0, RightParen, idx0+1)))
                }
                Some((idx0, '*')) if self.not_text() => {
                    self.bump();
                    Some(Ok((idx0, Star, idx0+1)))
                }
                Some((idx0, c)) if (c == '"' || c == '\'') && self.not_text() => {
                    self.bump();
                    Some(self.string_literal(idx0, c))
                }
                Some((idx0, c)) if is_identifier_start(c) && self.not_text() => {
                    Some(self.identifierish(idx0))
                }
                Some((_, c)) if c.is_whitespace() => {
                    self.bump();
                    continue;
                }
                Some((idx0, _)) => {
                    if self.state == TokState::InTemplateOrView {
                        return Some(match self.take_until(|c| c == ';' || c == '<' || c == '{') {
                            Some(idx1) => {
                                let text: &'input str = &self.text[idx0..idx1];
                                Ok((idx0, TextNode(text), idx1+1))
                            }
                            None => {
                                error(UnterminatedTextNode, idx0)
                            }
                        })
                    }

                    println!("Not matched!");
                    Some(error(UnrecognizedToken, idx0))
                }
                None => {
                    None
                }
            };
        }
    }

    fn bump(&mut self) -> Option<(usize, char)> {
        self.lookahead = self.chars.next();
        self.lookahead
    }

    fn string_literal(&mut self, idx0: usize, match_against: char) -> Result<Spanned<Tok<'input>>, Error> {
        let mut escape = false;
        let terminate = |c: char| {
            if escape {
                escape = false;
                false
            } else if c == '\\' {
                escape = true;
                false
            } else if c == match_against {
                true
            } else {
                false
            }
        };
        match self.take_until(terminate) {
            Some(idx1) => {
                self.bump(); // consume the '"'
                let text = &self.text[idx0+1..idx1]; // do not include the "" in the str
                Ok((idx0, StringLiteral(text), idx1+1))
            }
            None => {
                error(UnterminatedStringLiteral, idx0)
            }
        }
    }

    fn identifierish(&mut self, idx0: usize) -> Result<Spanned<Tok<'input>>, Error> {

        let (start, word, end) = self.word(idx0);

        let tok =
            // search for a keyword first; if none are found, this is
            // an Id
            KEYWORDS.iter()
                    .filter(|&&(w, _)| w == word)
                    .map(|&(_, ref t)| t.clone())
                    .next()
                    .unwrap_or_else(|| Id(StrView::from(self.text, start, end)));

        if tok == Template || tok == View {
            self.state = TokState::WaitingForEq;
        }

        Ok((start, tok, end))
    }

    fn word(&mut self, idx0: usize) -> Spanned<&'input str> {
        match self.take_while(is_identifier_continue) {
            Some(end) => (idx0, &self.text[idx0..end], end),
            None => (idx0, &self.text[idx0..], self.text.len()),
        }
    }

    fn hexadecimal(&mut self, idx0: usize) -> Spanned<Tok<'input>> {
        match self.take_while(|c| c.is_digit(16)) {
            Some(end) => (idx0, Hex(&self.text[idx0..end]), end),
            None => (idx0, Hex(&self.text[idx0..]), self.text.len()),
        }
    }

    fn number(&mut self, idx0: usize) -> Result<Spanned<Tok<'input>>, Error> {
        let mut seen_dot = false;
        let valid_number = |c: char| {
            if c.is_digit(10) {
                true
            } else if c == '.' && !seen_dot {
                seen_dot = true;
                true
            } else {
                false
            }
        };
        let res = match self.take_while(valid_number) {
            Some(end) => (idx0, Number(&self.text[idx0..end]), end),
            None => (idx0, Number(&self.text[idx0..]), self.text.len()),
        };
        Ok(res)
    }

    fn take_while<F>(&mut self, mut keep_going: F) -> Option<usize>
        where F: FnMut(char) -> bool
    {
        self.take_until(|c| !keep_going(c))
    }

    fn take_until<F>(&mut self, mut terminate: F) -> Option<usize>
        where F: FnMut(char) -> bool
    {
        loop {
            match self.lookahead {
                None => {
                    return None;
                }
                Some((idx1, c)) => {
                    if terminate(c) {
                        return Some(idx1);
                    } else {
                        self.bump();
                    }
                }
            }
        }
    }
}

impl<'input> Iterator for Tokenizer<'input> {
    type Item = Result<Spanned<Tok<'input>>, Error>;

    fn next(&mut self) -> Option<Result<Spanned<Tok<'input>>, Error>> {
        match self.next_unshifted() {
            None =>
                None,
            Some(Ok((l, t, r))) => {
                println!("{:?}", t);
                Some(Ok((l+self.shift, t, r+self.shift)))
            }
            Some(Err(Error { location, code })) =>
                Some(Err(Error { location: location+self.shift, code: code })),
        }
    }
}

fn is_identifier_start(c: char) -> bool {
    UnicodeXID::is_xid_start(c)
}

fn is_identifier_continue(c: char) -> bool {
    // TODO: Is the last check necessary?
    UnicodeXID::is_xid_continue(c) || c == '_'
}
