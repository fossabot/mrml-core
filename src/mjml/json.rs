use super::MJMLChildren;
use crate::mj_body::MJBody;
use crate::mj_head::MJHead;
use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum MJMLChild {
    MJHead(MJHead),
    MJBody(MJBody),
}

impl MJMLChildren {
    pub fn is_empty(&self) -> bool {
        self.head.is_none() && self.body.is_none()
    }
}

impl Serialize for MJMLChildren {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_seq(Some(2))?;
        if let Some(ref head) = self.head {
            map.serialize_element(head)?;
        }
        if let Some(ref body) = self.body {
            map.serialize_element(body)?;
        }
        map.end()
    }
}

#[derive(Default)]
struct MJMLChildrenVisitor;

impl<'de> Visitor<'de> for MJMLChildrenVisitor {
    type Value = MJMLChildren;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence with title and text elements")
    }

    fn visit_seq<S>(self, mut access: S) -> Result<Self::Value, S::Error>
    where
        S: SeqAccess<'de>,
    {
        let mut result = MJMLChildren::default();
        while let Some(value) = access.next_element::<MJMLChild>()? {
            match value {
                MJMLChild::MJHead(head) => result.head = Some(head),
                MJMLChild::MJBody(body) => result.body = Some(body),
            };
        }
        Ok(result)
    }
}

impl<'de> Deserialize<'de> for MJMLChildren {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(MJMLChildrenVisitor::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::mjml::MJML;

    #[test]
    fn serialize() {
        let elt = MJML::default();
        assert_eq!(serde_json::to_string(&elt).unwrap(), r#"{"type":"mjml"}"#);
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mjml","attributes":{"lang":"fr"},"children":[{"type":"mj-head"},{"type":"mj-body","children":["Hello World!"]}]}"#;
        let res: MJML = serde_json::from_str(json).unwrap();
        assert!(res.children.head.is_some());
        assert!(res.children.body.is_some());
        let next = serde_json::to_string(&res).unwrap();
        assert_eq!(next, json);
    }
}
