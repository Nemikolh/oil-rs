use lalrpop_util;
use tok;
use ast;

mod oil;


pub type Error<'input> = lalrpop_util::ParseError<usize, tok::Tok<'input>, tok::Error>;

pub fn parse_grammar<'input>(input: &'input str)
                             -> Result<ast::Package, Error<'input>>
{
    let tokenizer = tok::Tokenizer::new(input, 0);
    oil::parse_ui_package(tokenizer)
}


#[cfg(test)]
mod test_errors;
#[cfg(test)]
mod test_mixed;
#[cfg(test)]
mod test_import;
#[cfg(test)]
mod test_style;
#[cfg(test)]
mod test_component;
#[cfg(test)]
mod test_view;
