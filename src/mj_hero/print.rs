#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_hero::MJHero::default();
        assert_eq!("<mj-hero />", item.dense_print());
    }
}
