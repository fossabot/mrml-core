#[cfg(test)]
mod tests {
    use crate::mj_divider::MJDivider;

    #[test]
    fn serialize() {
        let mut elt = MJDivider::default();
        elt.attributes.insert("margin-bottom".into(), "20px".into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-divider","attributes":{"margin-bottom":"20px"}}"#
        );
    }

    #[test]
    fn deserialize() {
        let elt = MJDivider::default();
        let json = serde_json::to_string(&elt).unwrap();
        let _res: MJDivider = serde_json::from_str(&json).unwrap();
    }
}
