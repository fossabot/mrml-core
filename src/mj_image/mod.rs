use crate::prelude::hash::Map;

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub const NAME: &str = "mj-image";

#[derive(Debug, Default)]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintComponent))]
#[cfg_attr(feature = "print", mrml_print(tag = "NAME"))]
#[cfg_attr(feature = "parse", derive(mrml_parse_macros::MrmlParseComponent))]
pub struct MJImage {
    attributes: Map<String, String>,
}