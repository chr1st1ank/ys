use range_collections::range_set;
use std::ops::{Bound, Range};
use yaml_rust;
use yaml_rust::scanner::TokenType;

enum ParserState {
    ReadListEntry,
    ReadMappingKey,
    ReadMappingValue,
    Empty,
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

pub fn join_path(elements: &Vec<PathElement>) -> String {
    elements
        .iter()
        .map(|path_element| path_element.to_string())
        .collect::<Vec<String>>()
        .join(".")
}

pub fn filter_documents(input: String, is_key_whitelisted: &dyn Fn(&str) -> bool) -> String {
    let scanner = yaml_rust::scanner::Scanner::new(input.chars());

    let mut state = ParserState::Empty;
    let mut current_path: Vec<PathElement> = Vec::new();
    let mut skip_ranges: Vec<Range<usize>> = Vec::new();
    let mut output = String::new();
    let mut skip_from: Option<usize> = None;

    for yaml_rust::scanner::Token(marker, token_type) in scanner {
        println!(
            "{:>3},{:>3},{:>3} Start: {:?}\n            Path 1: {:}\n            Token: {:?}",
            marker.line(),
            marker.col(),
            marker.index(),
            current_path.last(),
            join_path(&current_path),
            token_type,
        );

        if let Some(from) = skip_from {
            extend_last_skiprange(&mut skip_ranges, from, marker.index());
            skip_from = None;
        }
        match token_type {
            TokenType::StreamStart(_e) => (),
            TokenType::DocumentStart => {
                state = ParserState::Empty;
                current_path.clear();
            }
            TokenType::BlockMappingStart | TokenType::FlowMappingStart => {
                state = ParserState::Empty;
            }
            TokenType::Key => state = ParserState::ReadMappingKey,
            TokenType::Value => state = ParserState::ReadMappingValue,
            TokenType::BlockSequenceStart | TokenType::FlowSequenceStart => {
                current_path.push(PathElement::Index(0, marker.index()));
                state = ParserState::Empty;
            }
            TokenType::BlockEntry | TokenType::FlowEntry => state = ParserState::ReadListEntry,
            TokenType::Scalar(_scalar_style, value) => match state {
                ParserState::ReadMappingKey => {
                    current_path.push(PathElement::Key(value, marker.index()));
                    if !is_key_whitelisted(join_path(&current_path).as_str()) {
                        skip_ranges.push(marker.index()..marker.index());
                        println!("      - {} (skipped)", join_path(&current_path));
                    } else {
                        println!("      - {}", join_path(&current_path));
                    }
                    state = ParserState::ReadMappingValue
                }
                ParserState::ReadMappingValue => {
                    if !is_key_whitelisted(join_path(&current_path).as_str()) {
                        skip_from = Some(current_path.last().unwrap().start());
                    }
                    current_path.pop();
                    state = ParserState::Empty;
                }
                ParserState::ReadListEntry => {
                    if !is_key_whitelisted(join_path(&current_path).as_str()) {
                        skip_from = Some(current_path.last().unwrap().start());
                    }
                    let index = current_path.pop().unwrap();
                    if let PathElement::Index(i, _) = index {
                        current_path.push(PathElement::Index(i + 1, marker.index()));
                    }
                    state = ParserState::Empty;
                }
                _ => {}
            },
            TokenType::BlockEnd => {
                if !current_path.is_empty() {
                    let _ = current_path.pop();
                }
            }
            TokenType::FlowSequenceEnd | TokenType::FlowMappingEnd => {
                if !is_key_whitelisted(join_path(&current_path).as_str()) {
                    if let Some(last) = current_path.last() {
                        skip_from = Some(last.start());
                    }
                }
                if !current_path.is_empty() {
                    let _ = current_path.pop();
                }
            }
            TokenType::DocumentEnd | TokenType::StreamEnd => {
                println!("{:?}", skip_ranges);
                let mut selected_ranges = range_set::RangeSet2::from(output.len()..marker.index());
                for range in skip_ranges.iter() {
                    selected_ranges.difference_with(&range_set::RangeSet2::from(range.clone()));
                }

                for (start_bound, stop_bound) in selected_ranges.iter() {
                    if let (Bound::Included(start), Bound::Excluded(stop)) =
                        (start_bound, stop_bound)
                    {
                        output.push_str(&input[Range::from(*start..*stop)]);
                    } else {
                        panic!("Something is wrong with the bounds!");
                    }
                }
            }
            TokenType::NoToken => (),
            TokenType::VersionDirective(_u1, _u2) => (),
            TokenType::Alias(_s) | TokenType::Anchor(_s) => (),
            TokenType::Tag(_s1, _s2) | TokenType::TagDirective(_s1, _s2) => (),
        }

        println!("            Path 2: {:}", join_path(&current_path));
        if !is_key_whitelisted(join_path(&current_path).as_str()) {
            if let Some(last) = current_path.last() {
                skip_from = Some(last.start());
            }
        }
    }

    output
}

fn extend_last_skiprange(skip_ranges: &mut Vec<Range<usize>>, new_start: usize, new_stop: usize) {
    skip_ranges.pop();
    skip_ranges.push(new_start..new_stop);
}
