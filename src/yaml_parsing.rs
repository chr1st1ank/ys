use yaml_rust::{self};

pub fn filter_yaml(
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
        .filter(|(_i,_y, p)| is_key_whitelisted(p))
        .map(|(_i, y, p)| filter_yaml(y, &p, &is_key_whitelisted))
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
        .map(|(k, v, p)| (k.to_owned(), filter_yaml(v, &p, &is_key_whitelisted)))
        .collect();
    Yaml::Hash(map2)
}

fn concat_path(path: &str, k: &str) -> String {
    if path == "" {
        k.to_owned()
    } else {
        (path.to_owned() + "." + k).to_owned()
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
