#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_section::MJSection::default();
        assert_eq!("<mj-section />", item.dense_print());
    }
}
