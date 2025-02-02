#[cfg(test)]
mod tests {
    use crate::mj_carousel::MJCarousel;

    #[test]
    fn serialize() {
        let mut elt = MJCarousel::default();
        elt.attributes.insert("margin".into(), "42px".into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-carousel","attributes":{"margin":"42px"}}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-carousel","attributes":{"margin":"42px"},"children":[{"type":"mj-carousel-image","attributes":{"src":"https://jolimail.io"}}]}"#;
        let res: MJCarousel = serde_json::from_str(json).unwrap();
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.children.len(), 1);
        let next = serde_json::to_string(&res).unwrap();
        assert_eq!(next, json);
    }
}
