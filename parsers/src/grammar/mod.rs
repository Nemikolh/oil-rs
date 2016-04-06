//
// // Dependencies
// use self::parser::Parser;
// use xml::attribute::OwnedAttribute;
// use std::io::BufRead;
// use ErrorReporter;
// use oil_shared::markup::Node;
//
// pub use self::lib::Library;
//
// // Name for the "main" view.
// pub const MAIN_VIEW_NAME: &'static str = "main";
//
// // Tag list
// const TEMPLATE_TAG: &'static str = "template";
// const VIEW_TAG: &'static str = "view";
// const GROUP_TAG: &'static str = "group";
// const BUTTON_TAG: &'static str = "button";
// const LINE_INPUT_TAG: &'static str = "line-input";
// const PROGRESS_BAR_TAG: &'static str = "progress-bar";
// const REPEAT_TAG: &'static str = "repeat";
//
// mod parser;
// mod tags;
// mod lib;
mod oil;

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



use lalrpop_util;
use tok;

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

//
// /// Parse the given BufRead.
// ///
// /// # Example:
// ///
// /// ```
// /// use oil_parsers::StdOutErrorReporter;
// /// use oil_parsers::markup;
// ///
// /// let reader = std::io::BufReader::new(
// ///     "<view name=\"toto\">\
// ///     </view>\
// /// ".as_bytes());
// /// markup::parse(StdOutErrorReporter, reader);
// /// ```
// pub fn parse<E, B>(reporter: E, reader: B) -> Library<E>
//     where E: ErrorReporter,
//           B: BufRead
// {
//     let mut parser = Parser::new(reporter, reader);
//     parser.parse()
// }
//
//
// trait HasNodeChildren {
//     fn add(&mut self, maybe_child: Option<Node>);
// }
//
//
// // ======================================== //
// //                  HELPERS                 //
// // ======================================== //
//
// fn lookup_name<'a>(name: &'a str,
//                    attributes: &Vec<OwnedAttribute>)
//                    -> Option<String>
// {
//     attributes.iter()
//         .find(|ref attribute| attribute.name.local_name == name)
//         .map(|ref attribute| attribute.value.clone())
// }
//
// enum ErrorStatus {
//     NotReported(&'static str),
//     Reported,
// }
//
// enum ErrorType {
//     Fatal,
//     Warning,
// }
//
// type ParseError = (ErrorType, ErrorStatus);
