#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_group::MJGroup::default();
        assert_eq!("<mj-group />", item.dense_print());
    }
}
