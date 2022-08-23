use range_collections::range_set;
use std::ops::{Bound, Range};
use yaml_rust::scanner::ScanError;
use yaml_rust::scanner::TokenType;
use yaml_rust::{self};

#[derive(Debug)]
enum ParserState {
    Empty,
    ExpectAnyToken,
    ReadListEntry(usize, bool),
    ReadMapping,
    ReadMappingKey,
    ReadMappingValue,
    EndDocument,
}

#[derive(Debug)]
pub enum PathElement {
    /// key, start
    Key(String, usize),
    /// index, start
    Index(usize, usize),
}

impl PathElement {
    fn to_string(&self) -> String {
        match self {
            PathElement::Key(s, _) => s.clone(),
            PathElement::Index(i, _) => i.to_string(),
        }
    }
    fn start(&self) -> usize {
        match self {
            PathElement::Key(_, start) => *start,
            PathElement::Index(_, start) => *start,
        }
    }
}

pub fn join_path(elements: &[PathElement]) -> String {
    elements
        .iter()
        .map(|path_element| path_element.to_string())
        .collect::<Vec<String>>()
        .join(".")
}

pub fn join_path_and_append(elements: &[PathElement], last: String) -> String {
    join_path(elements) + "." + last.as_str()
}

// pub fn filter_documents(
//     input: String,
//     is_key_whitelisted: &dyn Fn(&str) -> bool,
// ) -> Result<String, ScanError> {
//     let scanner = yaml_rust::scanner::Scanner::new(input.chars());

//     for yaml_rust::scanner::Token(marker, token_type) in scanner {
//         println!(
//             "{:>3},{:>3},{:>3} Token: {:?}",
//             marker.line(),
//             marker.col(),
//             marker.index(),
//             token_type,
//         );
//     }
//     Ok("".to_owned())
// }

pub fn filter_documents(
    input: String,
    is_key_whitelisted: &dyn Fn(&str) -> bool,
) -> Result<String, ScanError> {
    let scanner = yaml_rust::scanner::Scanner::new(input.chars());

    let mut state_stack = Vec::new();
    let mut current_path: Vec<PathElement> = Vec::new();
    let mut skip_ranges: Vec<Range<usize>> = Vec::new();
    let mut output = String::new();
    let mut skip_from: Option<usize> = None;

    for yaml_rust::scanner::Token(marker, token_type) in scanner {
        let state = state_stack.last().unwrap_or(&ParserState::Empty);
        println!("{:?}", state_stack);
        println!(
            "{:>3},{:>3},{:>3} Token: {:?}
            State: {:?}
            Start: {:?}
            Path 1: {:}",
            marker.line(),
            marker.col(),
            marker.index(),
            token_type,
            state,
            current_path.last(),
            join_path(&current_path)
        );

        if let Some(from) = skip_from {
            extend_last_skiprange(&mut skip_ranges, from, marker.index());
            skip_from = None;
        }

        match state {
            ParserState::Empty => match token_type {
                TokenType::StreamStart(_e) => {
                    state_stack.push(ParserState::ExpectAnyToken);
                    current_path.clear();
                }
                TokenType::DocumentStart => {
                    state_stack.push(ParserState::ExpectAnyToken);
                    current_path.clear();
                }
                _ => {
                    unreachable!();
                }
            },
            ParserState::ExpectAnyToken => match token_type {
                TokenType::DocumentStart => {
                    state_stack.clear();
                    state_stack.push(ParserState::ExpectAnyToken);
                    current_path.clear();
                }
                TokenType::BlockMappingStart | TokenType::FlowMappingStart => {
                    state_stack.push(ParserState::ReadMapping);
                }
                TokenType::Key => state_stack.push(ParserState::ReadMappingKey),
                TokenType::Value => state_stack.push(ParserState::ReadMappingValue),
                TokenType::BlockSequenceStart | TokenType::FlowSequenceStart => {
                    current_path.push(PathElement::Index(0, marker.index()));
                    state_stack.push(ParserState::ReadListEntry(0, false));
                }
                TokenType::BlockEntry | TokenType::FlowEntry => {} // => state = ParserState::ReadListEntry,
                TokenType::BlockEnd => {
                    current_path.pop();
                }
                TokenType::FlowSequenceEnd | TokenType::FlowMappingEnd => {
                    if !is_key_whitelisted(join_path(&current_path).as_str()) {
                        if let Some(last) = current_path.last() {
                            skip_from = Some(last.start());
                        }
                    }
                    current_path.pop();
                    current_path.pop();
                }
                TokenType::DocumentEnd | TokenType::StreamEnd => {
                    state_stack.pop();
                    state_stack.push(ParserState::EndDocument);
                }
                TokenType::NoToken => (),
                TokenType::VersionDirective(_u1, _u2) => (),
                TokenType::Alias(_s) | TokenType::Anchor(_s) => (),
                TokenType::Tag(_s1, _s2) | TokenType::TagDirective(_s1, _s2) => (),
                _ => {
                    unreachable!();
                }
            },
            ParserState::ReadMapping => match token_type {
                TokenType::Key => state_stack.push(ParserState::ReadMappingKey),
                TokenType::BlockEnd | TokenType::FlowMappingEnd => {
                    current_path.pop();
                    state_stack.pop();
                }
                TokenType::FlowEntry => {}
                _ => {
                    unreachable!();
                }
            },
            ParserState::ReadMappingKey => match token_type {
                TokenType::Key => {}
                TokenType::Scalar(_scalar_style, value) => {
                    current_path.push(PathElement::Key(value, marker.index()));
                    if !is_key_whitelisted(join_path(&current_path).as_str()) {
                        skip_ranges.push(marker.index()..marker.index());
                        println!("      - {} (skipped)", join_path(&current_path));
                    } else {
                        println!("      - {}", join_path(&current_path));
                    }
                    state_stack.pop();
                    state_stack.push(ParserState::ReadMappingValue);
                }
                _ => {
                    unreachable!();
                }
            },
            ParserState::ReadMappingValue => match token_type {
                TokenType::Value => {}
                TokenType::Scalar(_scalar_style, _value) => {
                    if !is_key_whitelisted(join_path(&current_path).as_str()) {
                        skip_from = Some(
                            current_path
                                .last()
                                .unwrap_or(&PathElement::Key("".to_owned(), marker.index()))
                                .start(),
                        );
                    }
                    current_path.pop();
                    state_stack.pop();
                    // state_stack.push(ParserState::ReadMapping);
                }
                TokenType::BlockMappingStart | TokenType::FlowMappingStart => {
                    state_stack.pop();
                    state_stack.push(ParserState::ReadMappingKey)
                }
                TokenType::BlockSequenceStart | TokenType::FlowSequenceStart => {
                    state_stack.pop();
                    state_stack.push(ParserState::ReadListEntry(0, false))
                }
                TokenType::BlockEntry | TokenType::FlowEntry => {
                    state_stack.pop();
                    state_stack.push(ParserState::ReadListEntry(0, false))
                }
                TokenType::DocumentEnd | TokenType::StreamEnd => {
                    state_stack.pop();
                    state_stack.push(ParserState::EndDocument);
                }
                _ => {
                    unreachable!();
                }
            },
            ParserState::ReadListEntry(i, path_pushed) => match token_type {
                TokenType::BlockEntry | TokenType::FlowEntry => {}
                TokenType::Scalar(_scalar_style, _value) => {
                    if !is_key_whitelisted(
                        join_path_and_append(&current_path, i.to_string()).as_str(),
                    ) {
                        skip_from = Some(
                            current_path
                                .last()
                                .unwrap_or(&PathElement::Key("".to_owned(), marker.index()))
                                .start(),
                        );
                    }
                    current_path.pop();
                    let next_index = i + 1;
                    state_stack.pop();
                    state_stack.push(ParserState::ReadListEntry(next_index, false));
                }
                TokenType::BlockMappingStart | TokenType::FlowMappingStart => {
                    current_path.push(PathElement::Index(*i, marker.index()));
                    let next_index = i + 1;
                    state_stack.pop();
                    state_stack.push(ParserState::ReadListEntry(next_index, false));
                    state_stack.push(ParserState::ReadMapping)
                }
                TokenType::BlockEnd | TokenType::FlowSequenceEnd => {
                    if *path_pushed {
                        current_path.pop();
                    }
                    state_stack.pop();
                    println!("Block End, popping state.")
                }
                TokenType::DocumentEnd | TokenType::StreamEnd => {
                    if *path_pushed {
                        current_path.pop();
                    }
                    state_stack.pop();
                    state_stack.push(ParserState::EndDocument);
                }
                _ => {
                    unreachable!();
                }
            },
            ParserState::EndDocument => {
                assert!(state_stack.is_empty());
            }
        };

        println!("            Path 2: {:}", join_path(&current_path));
        println!(
            "            State 2: {:?}",
            state_stack.last().unwrap_or(&ParserState::Empty)
        );
        if !is_key_whitelisted(join_path(&current_path).as_str()) {
            if let Some(last) = current_path.last() {
                skip_from = Some(last.start());
            }
        }
    }
    println!("{:?}", skip_ranges);
    let selected_ranges = selected_ranges(&skip_ranges, output.len(), input.len());
    copy_ranges(selected_ranges, &input, &mut output);

    Ok(output)
}

fn copy_ranges(
    selected_ranges: range_collections::RangeSet<[usize; 2]>,
    input: &str,
    output: &mut String,
) {
    for (start_bound, stop_bound) in selected_ranges.iter() {
        if let (Bound::Included(start), Bound::Excluded(stop)) = (start_bound, stop_bound) {
            output.push_str(&input[Range::from(*start..*stop)]);
        } else {
            unreachable!("Something is wrong with the bounds!");
        }
    }
}

fn selected_ranges(
    skip_ranges: &Vec<Range<usize>>,
    offset: usize,
    marker_index: usize,
) -> range_collections::RangeSet<[usize; 2]> {
    let mut selected_ranges = range_set::RangeSet2::from(offset..marker_index);
    for range in skip_ranges.iter() {
        selected_ranges.difference_with(&range_set::RangeSet2::from(range.clone()));
    }
    selected_ranges
}

fn extend_last_skiprange(skip_ranges: &mut Vec<Range<usize>>, new_start: usize, new_stop: usize) {
    skip_ranges.pop();
    skip_ranges.push(new_start..new_stop);
}
#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_join_path_empty() {
        assert_eq!(join_path(&vec![]), "");
    }

    #[test]
    fn test_join_path() {
        assert_eq!(
            join_path(&vec![
                PathElement::Key("key".to_owned(), 1),
                PathElement::Index(4, 0),
                PathElement::Key("me".to_owned(), 0),
            ]),
            "key.4.me"
        );
    }

    const EXAMPLE_YAML: &str = indoc! {"
        key-with-string: 'string'
        key-with-float: 3.14
        inline-mapping: {'a': 1, 'b': 2}
        inline-list: {1, 2}
        key-with-subtree:
            nested-key-with-string: \"another string\"
        key-with-list:
        - item 1
        - item 2
        - item 3
    "};

    #[test]
    fn test_filter_documents_without_filter() {
        assert_eq!(
            filter_documents(EXAMPLE_YAML.to_owned(), &|_: &str| true),
            Ok(EXAMPLE_YAML.to_owned())
        );
    }

    #[test]
    fn test_filter_documents_for_single_key() {
        assert_eq!(
            filter_documents(EXAMPLE_YAML.to_owned(), &|s: &str| s
                .starts_with("key-with-string")),
            Ok("key-with-string: 'string'".to_owned())
        );
    }

    #[test]
    fn test_filter_documents_for_key_with_list() {
        assert_eq!(
            filter_documents(EXAMPLE_YAML.to_owned(), &|s: &str| s
                .starts_with("key-with-list")),
            Ok(indoc! {"
                key-with-list:
                - item 1
                - item 2
                - item 3
            "}
            .to_owned())
        );
    }

    #[test]
    fn test_filter_documents_for_blocklist() {
        assert_eq!(
            filter_documents(
                indoc! {"
                    date: 2012-08-06
                    items:
                    - part_no: A4786
                    - part_no: E1628
                "}
                .to_owned(),
                &|s: &str| s.starts_with("date")
            ),
            Ok("date: 2012-08-06".to_owned())
        );
    }
}
