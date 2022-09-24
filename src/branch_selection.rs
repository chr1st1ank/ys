pub fn key_matches_any_pattern(include_patterns: &[&str], key: &str) -> bool {
    for include_pattern in include_patterns {
        if key_matches_pattern(key, include_pattern) {
            return true;
        }
    }
    false
}

fn key_matches_pattern(key: &str, include_pattern: &str) -> bool {
    for (k, p) in key.split('.').zip(include_pattern.split('.')) {
        if p != "*" && k != p {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_key_matches_any_pattern() {
        assert!(!key_matches_any_pattern(
            &vec!["key-with-list"],
            "nested-key"
        ));
        assert!(!key_matches_any_pattern(
            &vec!["key-with-list", "nested"],
            "nested-key"
        ));
        assert!(key_matches_any_pattern(&vec!["main"], "main.nested-key"));
        assert!(key_matches_any_pattern(
            &vec!["*.nested-key"],
            "main.nested-key"
        ));
        assert!(key_matches_any_pattern(&vec!["main.*"], "main.nested-key"));
        assert!(key_matches_any_pattern(&vec!["*"], "main.nested-key"));
        assert!(!key_matches_any_pattern(
            &vec!["nested-key"],
            "main.nested-key"
        ));
        assert!(key_matches_any_pattern(
            &vec!["main.nested-key"],
            "main.nested-key"
        ));
        assert!(key_matches_any_pattern(&vec!["main.nested-key"], "main"));
        assert!(key_matches_any_pattern(&vec!["main"], "main.1"));
        assert!(key_matches_any_pattern(&vec!["main.1"], "main.1"));
        assert!(key_matches_any_pattern(&vec!["main.*"], "main.1"));
        assert!(!key_matches_any_pattern(&vec!["main.0"], "main.1"));
        assert!(!key_matches_any_pattern(&vec!["*.0"], "main.1"));
        assert!(key_matches_any_pattern(
            &vec!["X", "main"],
            "main.nested-key"
        ));
        assert!(key_matches_any_pattern(&vec!["X", "main.1"], "main.1"));
        assert!(key_matches_any_pattern(&vec!["X", "main.*"], "main.1"));
    }
}
