#[cfg(test)]
mod tests {
    use crate::mj_wrapper::MJWrapper;

    #[test]
    fn serialize() {
        let mut elt = MJWrapper::default();
        elt.attributes.insert("margin".into(), "42px".into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-wrapper","attributes":{"margin":"42px"}}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-wrapper","attributes":{"margin-bottom":"20px"},"children":[{"type":"comment","children":"Hello World!"},"Hello World!"]}"#;
        let res: MJWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.children.len(), 2);
    }
}
