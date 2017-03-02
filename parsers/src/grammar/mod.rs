use lalrpop_util;
use tok;
use ast::{Package, Expr};
use ast::builder::{ASTBuilder, ASTFullSpan};

mod oil;


pub type Error<'input> = lalrpop_util::ParseError<usize, tok::Tok<'input>, tok::Error>;

pub fn parse_grammar<'input>(input: &'input str)
                            -> Result<Package, Error<'input>>
{
    parse_grammar_with_builder(ASTFullSpan, input)
}

pub fn parse_grammar_with_builder<'input, B: ASTBuilder>(builder: B, input: &'input str)
                            -> Result<Package, Error<'input>>
{
    let tokenizer = tok::Tokenizer::new(input, 0);
    oil::parse_ui_package(&builder, tokenizer)
}

pub fn parse_expression_with_builder<'input, B: ASTBuilder>(builder: B, input: &'input str)
                            -> Result<Expr, Error<'input>>
{
    let tokenizer = tok::Tokenizer::new(input, 0);
    oil::parse_expression(&builder, tokenizer)
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
