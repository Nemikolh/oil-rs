use std::io;
use lalrpop_util::ParseError;
use tok::{self, Tok};

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    ParseError,
}

pub use self::symbols::pass_resolve_symbols;
pub use self::paths::pass_resolve_packages_from_root;

mod symbols;
mod paths;

#[cfg(test)]
mod test;


impl From<io::Error> for Error {

    fn from(from: io::Error) -> Error {
        Error::IoError(from)
    }
}

impl<'input> From<ParseError<usize, Tok<'input>, tok::Error>> for Error {

    fn from(_from: ParseError<usize, Tok<'input>, tok::Error>) -> Error {
        Error::ParseError
    }
}