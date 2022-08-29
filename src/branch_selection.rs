
pub fn key_matches_any_pattern(include_patterns: &[&str], key: &str) -> bool {
    for include_pattern in include_patterns {
        if key_matches_pattern(key, include_pattern) {
            return true
        }
    }
    false
}

fn key_matches_pattern(key: &str, include_pattern: &str) -> bool {
    for (k, p) in key.split(".").zip(include_pattern.split(".")) {
        if p != "*" && k != p {
            return false
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_key_included() {
        assert!(!key_matches_any_pattern(&vec!["key-with-list"], "nested-key"));
        assert!(!key_matches_any_pattern(&vec!["key-with-list", "nested"], "nested-key"));
        assert!(key_matches_any_pattern(&vec!["main"], "main.nested-key"));
        assert!(key_matches_any_pattern(&vec!["*.nested-key"], "main.nested-key"));
        assert!(key_matches_any_pattern(&vec!["main.*"], "main.nested-key"));
        assert!(key_matches_any_pattern(&vec!["*"], "main.nested-key"));
        assert!(!key_matches_any_pattern(&vec!["nested-key"], "main.nested-key"));
        assert!(key_matches_any_pattern(&vec!["main.nested-key"], "main.nested-key"));
        assert!(key_matches_any_pattern(&vec!["main.nested-key"], "main"));
        assert!(key_matches_any_pattern(&vec!["main"], "main.1"));
        assert!(key_matches_any_pattern(&vec!["main.1"], "main.1"));
        assert!(key_matches_any_pattern(&vec!["main.*"], "main.1"));
        assert!(!key_matches_any_pattern(&vec!["main.0"], "main.1"));
        assert!(!key_matches_any_pattern(&vec!["*.0"], "main.1"));
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
