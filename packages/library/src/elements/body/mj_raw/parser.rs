use super::MJRaw;
use crate::elements::body::raw::RawElement;
use crate::elements::error::Error;
use crate::parser::Node;
use crate::util::header::Header;

impl MJRaw {
    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJRaw, Error> {
        let mut children = vec![];
        for child in node.children.iter() {
            children.push(RawElement::conditional_parse(&child, header, true)?.into());
        }
        Ok(MJRaw {
            context: None,
            children,
        })
    }
}