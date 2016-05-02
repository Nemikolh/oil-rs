extern crate phf_codegen;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("known_properties.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    write!(&mut file, "pub static STYLE_PROPERTIES: phf::Map<&'static str, PropertyName> = ").unwrap();
    phf_codegen::Map::new()
        // Absolute positioning properties
        .entry("left", "PropertyName::LEFT")
        .entry("right", "PropertyName::RIGHT")
        .entry("top", "PropertyName::TOP")
        .entry("bottom", "PropertyName::BOTTOM")
        .entry("height", "PropertyName::HEIGHT")
        .entry("width", "PropertyName::WIDTH")
        // Margin properties
        .entry("margin", "PropertyName::MARGIN")
        .entry("margin_left", "PropertyName::MARGIN_LEFT")
        .entry("margin_right", "PropertyName::MARGIN_RIGHT")
        .entry("margin_top", "PropertyName::MARGIN_TOP")
        .entry("margin_bottom", "PropertyName::MARGIN_BOTTOM")
        // Padding properties
        .entry("padding", "PropertyName::PADDING")
        .entry("padding_left", "PropertyName::PADDING_LEFT")
        .entry("padding_right", "PropertyName::PADDING_RIGHT")
        .entry("padding_top", "PropertyName::PADDING_TOP")
        .entry("padding_bottom", "PropertyName::PADDING_BOTTOM")
        // Border properties
        .entry("border", "PropertyName::BORDER")
        .entry("border_left", "PropertyName::BORDER_LEFT")
        .entry("border_right", "PropertyName::BORDER_RIGHT")
        .entry("border_top", "PropertyName::BORDER_TOP")
        .entry("border_bottom", "PropertyName::BORDER_BOTTOM")
        // Layout mode (absolute / rtl / ltr)
        .entry("layout", "PropertyName::LAYOUT_MODE")
        // Background image
        .entry("background_image", "PropertyName::BACKGROUND_IMAGE")
        .entry("background_image_rule", "PropertyName::BACKGROUND_IMAGE_RULE")
        .build(&mut file)
        .unwrap();

    write!(&mut file, ";\n").unwrap();
}
