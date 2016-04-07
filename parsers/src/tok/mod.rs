use std::str::CharIndices;
use unicode_xid::UnicodeXID;

use self::ErrorCode::*;
use self::Tok::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    pub location: usize,
    pub code: ErrorCode
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorCode {
    UnrecognizedToken,
    UnterminatedEscape,
    UnterminatedStringLiteral,
    // ExpectedStringLiteral,
}

fn error<T>(c: ErrorCode, l: usize) -> Result<T,Error> {
    Err(Error { location: l, code: c })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tok<'input> {
    // Keywords;
    Import,
    From,
    View,
    Template,
    Export,

    // Special keywords: these are accompanied by a series of
    // uninterpreted strings representing imports and stuff.
    StringLiteral(&'input str),
    Number(&'input str), // No unit

    TextNode(&'input str),

    // Identifiers of various kinds:
    DotId(&'input str), // excludes the '.'
    ViewId(&'input str), // excludes the '!'
    Id(&'input str),

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
    in_template: bool,
    in_tag: bool,
}

macro_rules! eof {
    ($x:expr) => {
        match $x { Some(v) => v, None => { return None; } }
    }
}

pub type Spanned<T> = (usize, T, usize);

const KEYWORDS: &'static [(&'static str, Tok<'static>)] = &[
    ("import", Import),
    ("export", Export),
    ("from", From),
    ("view", View),
    ("template", Template),
    ];

impl<'input> Tokenizer<'input> {
    pub fn new(text: &'input str, shift: usize) -> Tokenizer<'input> {
        let mut t = Tokenizer {
            text: text,
            chars: text.char_indices(),
            lookahead: None,
            shift: shift,
            in_template: false,
            in_tag: false,
        };
        t.bump();
        t
    }

    fn next_unshifted(&mut self) -> Option<Result<Spanned<Tok<'input>>, Error>> {
        loop {
            return match self.lookahead {
                Some((idx0, '-')) => {
                    match self.bump() {
                        Some((idx1, '>')) => {
                            self.bump();
                            Some(Ok((idx0, Arrow, idx1+1)))
                        }
                        Some((idx1, d)) if d.is_digit(10) => {
                            Some(self.number(idx0))
                        }
                        _ => {
                            Some(Ok((idx0, Minus, idx0+1)))
                        }
                    }
                }
                Some((idx0, '$')) => {
                    // TODO: merge this with identifier branch.
                    self.bump();
                    let (_, word, end) = self.word(idx0);
                    Some(Ok((idx0, Id(word), end)))
                }
                Some((idx0, '<')) => {
                    if self.in_template {
                        self.in_tag = true;
                    }
                    match self.bump() {
                        Some((idx1, '/')) => {
                            self.bump();
                            Some(Ok((idx0, LessThanSlash, idx1+1)))
                        }
                        _ => {
                            Some(Ok((idx0, LessThan, idx0+1)))
                        }
                    }
                }
                Some((idx0, d)) if d.is_digit(10) => {
                    Some(self.number(idx0))
                }
                Some((idx0, ':')) => {
                    self.bump();
                    Some(Ok((idx0, Colon, idx0+1)))
                }
                Some((idx0, ',')) => {
                    self.bump();
                    Some(Ok((idx0, Comma, idx0+1)))
                }
                Some((idx0, '.')) if !self.in_tag && !self.in_template => {
                    self.bump();
                    let (start, word, end) = self.word(idx0);
                    Some(Ok((start, DotId(word), end)))
                }
                Some((idx0, '=')) => {
                    self.bump();
                    Some(Ok((idx0, Equals, idx0+1)))
                }
                Some((idx0, '>')) => {
                    if self.in_template {
                        self.in_tag = false;
                    }
                    self.bump();
                    Some(Ok((idx0, GreaterThan, idx0+1)))
                }
                Some((idx0, '{')) => {
                    self.bump();
                    Some(Ok((idx0, LeftBrace, idx0+1)))
                }
                Some((idx0, '[')) => {
                    self.bump();
                    Some(Ok((idx0, LeftBracket, idx0+1)))
                }
                Some((idx0, '(')) => {
                    self.bump();
                    Some(Ok((idx0, LeftParen, idx0+1)))
                }
                Some((idx0, '+')) => {
                    match self.bump() {
                        Some((idx1, d)) if d.is_digit(10) => {
                            Some(self.number(idx0))
                        }
                        _ => Some(Ok((idx0, Plus, idx0+1)))
                    }
                }
                Some((idx0, '}')) => {
                    self.bump();
                    Some(Ok((idx0, RightBrace, idx0+1)))
                }
                Some((idx0, ']')) => {
                    self.bump();
                    Some(Ok((idx0, RightBracket, idx0+1)))
                }
                Some((idx0, ')')) => {
                    self.bump();
                    Some(Ok((idx0, RightParen, idx0+1)))
                }
                Some((idx0, ';')) => {
                    self.in_template = false;
                    self.bump();
                    Some(Ok((idx0, Semi, idx0+1)))
                }
                Some((idx0, '*')) => {
                    self.bump();
                    Some(Ok((idx0, Star, idx0+1)))
                }
                Some((idx0, c)) if c == '"' || c == '\'' => {
                    self.bump();
                    Some(self.string_literal(idx0, c))
                }
                Some((idx0, '/')) => {
                    match self.bump() {
                        Some((_, '/')) => {
                            self.take_until(|c| c == '\n');
                            continue;
                        }
                        _ => {
                            Some(error(UnrecognizedToken, idx0))
                        }
                    }
                }
                Some((idx0, c)) if is_identifier_start(c) => {
                    Some(self.identifierish(idx0))
                    // if c == 'r' {
                    //     // watch out for r"..." or r#"..."# strings
                    //     self.bump();
                    //     match self.lookahead {
                    //         Some((_, '#')) |
                    //         Some((_, '"')) => {
                    //             Some(self.regex_literal(idx0))
                    //         }
                    //         _ => {
                    //             // due to the particulars of how identifierish works,
                    //             // it's ok that we already consumed the 'r', because the
                    //             // identifier will run from idx0 (the 'r') to the end
                    //             Some(self.identifierish(idx0))
                    //         }
                    //     }
                    // } else {
                    //     Some(self.identifierish(idx0))
                    // }
                }
                Some((_, c)) if c.is_whitespace() => {
                    self.bump();
                    continue;
                }
                Some((idx, _)) => {
                    Some(error(UnrecognizedToken, idx))
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

    // fn code(&mut self, idx0: usize, open_delims: &str, close_delims: &str) -> Result<usize, Error> {
    //     // This is the interesting case. To find the end of the code,
    //     // we have to scan ahead, matching (), [], and {}, and looking
    //     // for a suitable terminator: `,`, `;`, `]`, `}`, or `)`.
    //     let mut balance = 0; // number of unclosed `(` etc
    //     loop {
    //         if let Some((idx, c)) = self.lookahead {
    //             if c == '"' {
    //                 self.bump();
    //                 try!(self.string_literal(idx)); // discard the produced token
    //                 continue;
    //             } else if c == 'r' {
    //                 self.bump();
    //                 if let Some((idx, '#')) = self.lookahead {
    //                     try!(self.regex_literal(idx));
    //                 }
    //                 continue;
    //             } else if c == '/' {
    //                 self.bump();
    //                 if let Some((_, '/')) = self.lookahead {
    //                     self.take_until(|c| c == '\n');
    //                 }
    //                 continue;
    //             } else if open_delims.find(c).is_some() {
    //                 balance += 1;
    //             } else if balance > 0 {
    //                 if close_delims.find(c).is_some() {
    //                     balance -= 1;
    //                 }
    //             } else {
    //                 debug_assert!(balance == 0);
    //
    //                 if c == ',' || c == ';' || close_delims.find(c).is_some() {
    //                     // Note: we do not consume the
    //                     // terminator. The code is everything *up
    //                     // to but not including* the terminating
    //                     // `,`, `;`, etc.
    //                     return Ok(idx);
    //                 }
    //             }
    //         } else if balance > 0 {
    //             // the input should not end with an
    //             // unbalanced number of `{` etc!
    //             return error(UnterminatedCode, idx0);
    //         } else {
    //             debug_assert!(balance == 0);
    //             return Ok(self.text.len());
    //         }
    //
    //         self.bump();
    //     }
    // }

    // fn escape(&mut self, idx0: usize) -> Result<Spanned<Tok<'input>>, Error> {
    //     match self.take_until(|c| c == '`') {
    //         Some(idx1) => {
    //             self.bump(); // consume the '`'
    //             let text: &'input str = &self.text[idx0+1..idx1]; // do not include the `` in the str
    //             Ok((idx0, Escape(text), idx1+1))
    //         }
    //         None => {
    //             error(UnterminatedEscape, idx0)
    //         }
    //     }
    // }

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

        if self.in_template && self.in_tag {
            return match self.take_until(|c| c == ';' || c == '<') {
                Some(idx1) => {
                    self.bump(); // consume the ';' or '<'
                    let text: &'input str = &self.text[idx0..idx1];
                    Ok((idx0, TextNode(text), idx1+1))
                }
                None => {
                    error(UnterminatedEscape, idx0)
                }
            }
        }

        let (start, word, end) = self.word(idx0);
        // if word == "use" {
        //     let code_end = try!(self.code(idx0, "([{", "}])"));
        //     let code = &self.text[end..code_end];
        //     return Ok((start, Tok::Use(code), code_end));
        // }
        //
        // if word == "where" {
        //     let mut wcs = vec![];
        //     let mut wc_start = end;
        //     let mut wc_end;
        //     loop {
        //         // Note: do not include `{` as a delimeter here, as
        //         // that is not legal in the trait/where-clause syntax,
        //         // and in fact signals start of the fn body. But do
        //         // include `<`.
        //         wc_end = try!(self.code(wc_start, "([<", ">])"));
        //         let wc = &self.text[wc_start..wc_end];
        //         wcs.push(wc);
        //
        //         // if this ended in a comma, maybe expect another where-clause
        //         if let Some((_, ',')) = self.lookahead {
        //             self.bump();
        //             wc_start = wc_end + 1;
        //         } else {
        //             break;
        //         }
        //     }
        //
        //     return Ok((start, Tok::Where(wcs), wc_end));
        // }

        let tok =
            // search for a keyword first; if none are found, this is
            // either a ViewId or a Id, depending on whether there
            // is a `!` immediately afterwards
            KEYWORDS.iter()
                    .filter(|&&(w, _)| w == word)
                    .map(|&(_, ref t)| t.clone())
                    .next()
                    .unwrap_or_else(|| {
                        match self.lookahead {
                            Some((_, '!')) => ViewId(word),
                            _ => Id(word),
                        }
                    });

        if tok == Template {
            self.in_template = true;
        }

        Ok((start, tok, end))
    }

    fn word(&mut self, idx0: usize) -> Spanned<&'input str> {
        match self.take_while(is_identifier_continue) {
            Some(end) => (idx0, &self.text[idx0..end], end),
            None => (idx0, &self.text[idx0..], self.text.len()),
        }
    }

    fn number(&mut self, idx0: usize) -> Result<Spanned<Tok<'input>>, Error> {
        // TODO: consume digits
        // Note: The text might start with '-' or '+'
        //       Ideally we want to accept floating point
        //       value as well.
        unimplemented!()
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
            Some(Ok((l, t, r))) =>
                Some(Ok((l+self.shift, t, r+self.shift))),
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
    UnicodeXID::is_xid_continue(c) || c == '-'
}
