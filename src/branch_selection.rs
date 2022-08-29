pub fn is_key_included(include_patterns: &[&str], key: &str) -> bool {
    for p in include_patterns {
        // TODO: Allow * in all elements (compare element wise)
        if key.starts_with(p) || p.starts_with(format!("{}.", key).as_str()) || *p == "*" {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_key_included() {
        assert!(!is_key_included(&vec!["key-with-list"], "nested-key"));
        assert!(is_key_included(&vec!["main"], "main.nested-key"));
        assert!(is_key_included(&vec!["main.nested-key"], "main.nested-key"));
        assert!(is_key_included(&vec!["main.nested-key"], "main"));
    }
    //
    // use pretty_assertions::assert_eq;
    // use indoc::indoc;
    // use ys::yaml_parsing::*;
    // const EXAMPLE_YAML: &str = indoc! {"
    //     key-with-string: 'string'
    //     key-with-float: 3.14
    //     inline-mapping: {'a': 1, 'b': 2}
    //     inline-list: [1, 2]
    //     key-with-subtree:
    //         nested-key-with-string: \"another string\"
    //     key-with-list:
    //     - item 1
    //     - item 2
    //     - item 3
    // "};
    //
    // #[test]
    // fn test_filter_documents_without_filter() {
    //     assert_eq!(
    //         filter_documents(EXAMPLE_YAML.to_owned(), &|_: &str| true),
    //         Ok(EXAMPLE_YAML.to_owned())
    //     );
    // }
    //
    // #[test]
    // fn test_filter_documents_for_single_key() {
    //     assert_eq!(
    //         filter_documents(EXAMPLE_YAML.to_owned(), &|s: &str| s
    //             .starts_with("key-with-string")),
    //         Ok("key-with-string: 'string'\n".to_owned())
    //     );
    // }
    //
    // #[test]
    // fn test_filter_documents_for_key_with_list() {
    //     assert_eq!(
    //         filter_documents(EXAMPLE_YAML.to_owned(), &|s: &str| s
    //             .starts_with("key-with-list")),
    //         Ok(indoc! {"
    //             key-with-list:
    //             - item 1
    //             - item 2
    //             - item 3
    //         "}
    //         .to_owned())
    //     );
    // }
    //
    // #[test]
    // fn test_filter_documents_for_blocklist() {
    //     assert_eq!(
    //         filter_documents(
    //             indoc! {"
    //                 date: 2012-08-06
    //                 items:
    //                 - part_no: A4786
    //                 - part_no: E1628
    //             "}
    //             .to_owned(),
    //             &|s: &str| s.starts_with("date")
    //         ),
    //         Ok("date: 2012-08-06\n".to_owned())
    //     );
    // }
}
