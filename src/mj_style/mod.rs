#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

pub const NAME: &str = "mj-style";

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintAttributes))]
pub struct MjStyleAttributes {
    pub inline: Option<String>,
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintComponent))]
#[cfg_attr(feature = "print", mrml_print(tag = "NAME", indent_children = false))]
#[cfg_attr(feature = "json", derive(mrml_json_macros::MrmlJsonComponent))]
#[cfg_attr(feature = "json", mrml_json(tag = "NAME"))]
pub struct MJStyle {
    pub attributes: MjStyleAttributes,
    pub children: String,
}

impl MJStyle {
    pub fn children(&self) -> &str {
        &self.children
    }
}

impl From<String> for MJStyle {
    fn from(children: String) -> Self {
        Self {
            attributes: MjStyleAttributes::default(),
            children,
        }
    }
}

impl From<&str> for MJStyle {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}
