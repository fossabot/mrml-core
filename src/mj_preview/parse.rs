#[cfg(test)]
mod tests {
    #[test]
    fn success() {
        let res = crate::mjml::MJML::parse(
            r#"<mjml><mj-head><mj-preview>Hello World!</mj-preview></mj-head></mjml>"#,
        );
        assert!(res.is_ok());
    }
}
