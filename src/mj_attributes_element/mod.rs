use crate::prelude::hash::Map;

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

#[derive(Debug, Default)]
pub struct MJAttributesElement {
    name: String,
    attributes: Map<String, String>,
}

impl MJAttributesElement {
    pub fn new(name: String) -> Self {
        Self {
            name,
            attributes: Map::new(),
        }
    }
}

impl MJAttributesElement {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn attributes(&self) -> &Map<String, String> {
        &self.attributes
    }
}
