use crate::comment::Comment;
use crate::mj_social_element::MJSocialElement;

#[derive(Debug, mrml_macros::MrmlChildren)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "json", serde(untagged))]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintChildren))]
#[cfg_attr(feature = "parse", derive(mrml_parse_macros::MrmlParseChildren))]
pub enum MJSocialChild {
    Comment(Comment),
    MJSocialElement(MJSocialElement),
}
