mod parse;
mod print;
mod render;

use crate::mj_raw::MJRawChild;
use std::collections::HashMap;

pub const NAME: &str = "mj-accordion-text";

#[derive(Debug, Default)]
pub struct MJAccordionText {
    attributes: HashMap<String, String>,
    children: Vec<MJRawChild>,
}