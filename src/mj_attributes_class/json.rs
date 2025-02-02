use super::{MJAttributesClass, NAME};
use serde::de::{Error, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

const FIELDS: [&str; 3] = ["type", "name", "attributes"];

impl Serialize for MJAttributesClass {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("type", NAME)?;
        map.serialize_entry("name", self.name())?;
        if !self.attributes.is_empty() {
            map.serialize_entry("attributes", &self.attributes)?;
        }
        map.end()
    }
}

#[derive(Default)]
struct MJAttributesClassVisitor;

impl<'de> Visitor<'de> for MJAttributesClassVisitor {
    type Value = MJAttributesClass;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an map with properties type, name and attributes")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut result = MJAttributesClass::default();
        while let Some(key) = access.next_key::<String>()? {
            if key == "type" {
                if access.next_value::<String>()? != NAME {
                    return Err(M::Error::custom(format!("expected type to equal {NAME}")));
                }
            } else if key == "name" {
                result.name = access.next_value()?;
            } else if key == "attributes" {
                result.attributes = access.next_value()?;
            } else {
                return Err(M::Error::unknown_field(&key, &FIELDS));
            }
        }
        Ok(result)
    }
}

impl<'de> Deserialize<'de> for MJAttributesClass {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MJAttributesClassVisitor::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_attributes_class::MJAttributesClass;

    #[test]
    fn serialize() {
        let mut elt = MJAttributesClass {
            name: "classname".into(),
            ..Default::default()
        };
        elt.attributes.insert("margin-bottom".into(), "20px".into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-class","name":"classname","attributes":{"margin-bottom":"20px"}}"#
        );
    }

    #[test]
    fn deserialize() {
        let elt = MJAttributesClass {
            name: "classname".into(),
            ..Default::default()
        };
        let json = serde_json::to_string(&elt).unwrap();
        let _res: MJAttributesClass = serde_json::from_str(&json).unwrap();
    }
}
