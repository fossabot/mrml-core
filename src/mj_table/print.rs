#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_table::MJTable::default();
        assert_eq!("<mj-table />", item.dense_print());
    }
}
