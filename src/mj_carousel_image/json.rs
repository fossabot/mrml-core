#[cfg(test)]
mod tests {
    use crate::mj_carousel_image::MJCarouselImage;

    #[test]
    fn serialize() {
        let mut elt = MJCarouselImage::default();
        elt.attributes
            .insert("src".into(), "https://jolimail.io".into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-carousel-image","attributes":{"src":"https://jolimail.io"}}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-carousel-image","attributes":{"src":"https://jolimail.io"}}"#;
        let res: MJCarouselImage = serde_json::from_str(json).unwrap();
        assert_eq!(res.attributes.len(), 1);
        let next = serde_json::to_string(&res).unwrap();
        assert_eq!(next, json);
    }
}
