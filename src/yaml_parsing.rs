use yaml_rust::{self};

pub fn filter_yaml(
    yaml_element: &yaml_rust::Yaml,
    is_key_whitelisted: &dyn Fn(&str) -> bool,
) -> yaml_rust::Yaml {
    filter_yaml_on_subpath(yaml_element, "", is_key_whitelisted)
}

fn filter_yaml_on_subpath(
    yaml_element: &yaml_rust::Yaml,
    path: &str,
    is_key_whitelisted: &dyn Fn(&str) -> bool,
) -> yaml_rust::Yaml {
    use yaml_rust::Yaml;

    match yaml_element {
        Yaml::Real(_) | Yaml::Integer(_) | Yaml::String(_) | Yaml::Boolean(_) | Yaml::Null => {
            yaml_element.clone()
        }
        Yaml::Array(arr) => filter_array(arr, path, &is_key_whitelisted),
        Yaml::Hash(arr) => filter_map(arr, path, &is_key_whitelisted),
        _ => yaml_element.clone(),
    }
}

fn filter_array(
    yaml_array: &Vec<yaml_rust::Yaml>,
    path: &str,
    is_key_whitelisted: &dyn Fn(&str) -> bool,
) -> yaml_rust::Yaml {
    use yaml_rust::Yaml;
    println!("filter_array({:?})", yaml_array);
    let vec2 = yaml_array
        .iter()
        .enumerate()
        .map(|(i, y)| (i, y, concat_path(path, &i.to_string())))
        .filter(|(_i, _y, p)| is_key_whitelisted(p))
        .map(|(_i, y, p)| filter_yaml_on_subpath(y, &p, &is_key_whitelisted))
        .collect();
    Yaml::Array(vec2)
}

fn filter_map(
    yaml_map: &yaml_rust::yaml::Hash,
    path: &str,
    is_key_whitelisted: &dyn Fn(&str) -> bool,
) -> yaml_rust::Yaml {
    use yaml_rust::Yaml;
    let map2 = yaml_map
        .iter()
        .map(|(k, v)| (k, v, concat_path(path, &yaml_to_string(k))))
        .filter(|(_, _, p)| is_key_whitelisted(p))
        .map(|(k, v, p)| {
            (
                k.to_owned(),
                filter_yaml_on_subpath(v, &p, &is_key_whitelisted),
            )
        })
        .collect();
    Yaml::Hash(map2)
}

fn concat_path(path: &str, k: &str) -> String {
    if path.is_empty() {
        k.to_owned()
    } else {
        path.to_owned() + "." + k
    }
}

fn yaml_to_string(yaml_element: &yaml_rust::Yaml) -> String {
    use yaml_rust::Yaml;

    match yaml_element {
        Yaml::String(s) => s.clone(),
        Yaml::Boolean(b) => b.to_string(),
        Yaml::Real(n) => n.clone(),
        Yaml::Integer(n) => n.to_string(),
        _ => "".to_owned(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use yaml_rust::Yaml;

    const EXAMPLE_YAML: &str = indoc! {"
        key-with-string: 'string'
        key-with-float: 3.14
        inline-mapping: {'a': 1, 'b': 2}
        inline-list: [1, 2]
        key-with-subtree:
            nested-key-with-string: \"another string\"
        key-with-list:
        - item 1
        - item 2
        - item 3
    "};

    #[test]
    fn test_filter_yaml_without_filter() {
        let yaml_doc = yaml_doc_from_str(EXAMPLE_YAML);
        assert_eq!(filter_yaml(&yaml_doc, &|_: &str| true), yaml_doc);
    }

    #[test]
    fn test_filter_yaml_for_root_key() {
        let yaml_doc = yaml_doc_from_str(EXAMPLE_YAML);
        let expected = yaml_doc_from_str("key-with-string: 'string'\n");
        assert_eq!(
            filter_yaml(&yaml_doc, &|s: &str| s.starts_with("key-with-string")),
            expected
        );
    }

    #[test]
    fn test_filter_yaml_for_nested_key() {
        let yaml_doc = yaml_doc_from_str(EXAMPLE_YAML);
        let expected = yaml_doc_from_str(indoc! {"
            inline-mapping:
                a: 1
        "});
        assert_eq!(
            filter_yaml(&yaml_doc, &|s: &str| s == "inline-mapping"
                || s == "inline-mapping.a"),
            expected
        );
    }

    #[test]
    fn test_filter_yaml_for_key_with_list() {
        let yaml_doc = yaml_doc_from_str(EXAMPLE_YAML);
        let expected = yaml_doc_from_str(indoc! {"
            key-with-list:
            - item 1
            - item 2
            - item 3
        "});
        assert_eq!(
            filter_yaml(&yaml_doc, &|s: &str| s.starts_with("key-with-list")),
            expected
        );
    }

    #[test]
    fn test_filter_yaml_for_list_item() {
        let yaml_doc = yaml_doc_from_str(EXAMPLE_YAML);
        let expected = yaml_doc_from_str(indoc! {"
            key-with-list:
            - item 2
        "});
        assert_eq!(
            filter_yaml(&yaml_doc, &|s: &str| s == "key-with-list"
                || s == "key-with-list.1"),
            expected
        );
    }

    #[test]
    fn test_filter_yaml_key_before_blocklist() {
        let yaml_doc = yaml_doc_from_str(indoc! {"
            date: 2012-08-06
            items:
            - part_no: A4786
            - part_no: E1628
        "});
        let expected = yaml_doc_from_str(indoc! {"
            date: 2012-08-06
        "});
        assert_eq!(
            filter_yaml(&yaml_doc, &|s: &str| s.starts_with("date")),
            expected
        );
    }

    fn yaml_doc_from_str(doc: &str) -> Yaml {
        yaml_rust::YamlLoader::load_from_str(doc)
            .unwrap()
            .first()
            .unwrap()
            .to_owned()
    }
}
