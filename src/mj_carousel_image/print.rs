#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let mut item = crate::mj_carousel_image::MJCarouselImage::default();
        item.attributes
            .insert("src".to_string(), "http://localhost".into());
        assert_eq!(
            "<mj-carousel-image src=\"http://localhost\" />",
            item.dense_print()
        );
    }
}
