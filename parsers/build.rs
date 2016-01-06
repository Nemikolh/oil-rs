extern crate phf_codegen;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("keyword_style_parser_phf_generated.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    write!(&mut file, "pub static KEYWORDS: phf::Map<&'static str, KwValue> = ").unwrap();
    phf_codegen::Map::new()
        .entry("auto", "KwValue::Auto")
        .entry("expand", "KwValue::Expand")
        .entry("absolute", "KwValue::Absolute")
        .entry("fit", "KwValue::Fit")
        .entry("repeat", "KwValue::Repeat")
        .build(&mut file)
        .unwrap();

    write!(&mut file, ";\n").unwrap();

    write!(&mut file, "pub static KEYWORDS_SELECTOR_STATE: phf::Map<&'static str, SelectorState> = ").unwrap();
    phf_codegen::Map::new()
        .entry("focus", "SelectorState::Focus")
        .entry("hover", "SelectorState::Hover")
        .entry("creation", "SelectorState::Creation")
        .build(&mut file)
        .unwrap();

    write!(&mut file, ";\n").unwrap();
}
