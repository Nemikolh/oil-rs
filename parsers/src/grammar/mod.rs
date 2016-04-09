use lalrpop_util;
use tok;


mod oil;
mod ast;


pub type ParseError<'input> = lalrpop_util::ParseError<usize, tok::Tok<'input>, tok::Error>;

pub fn parse_grammar<'input>(input: &'input str)
                             -> Result<(), ParseError<'input>>
{
    let tokenizer = tok::Tokenizer::new(input, 0);
    oil::parse_ui_package(tokenizer)
    // let mut grammar = try!(oil::parse_ui_package(input, tokenizer));

    // // find a unique prefix that does not appear anywhere in the input
    // while input.contains(&grammar.prefix) {
    //     grammar.prefix.push('_');
    // }
    //
    // Ok(grammar)
}


#[cfg(test)]
mod test_mixed;
#[cfg(test)]
mod test_import;
#[cfg(test)]
mod test_style;
#[cfg(test)]
mod test_template;
#[cfg(test)]
mod test_view;
