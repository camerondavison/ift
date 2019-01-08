use pest_derive::*;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct IfTParser;

#[cfg(test)]
use pest::Parser;

#[cfg(test)]
macro_rules! assert_rule {
    ($rule:expr, $in:expr) => {
        assert_eq!(
            IfTParser::parse($rule, $in)
                .unwrap()
                .last()
                .unwrap()
                .into_span()
                .end(),
            $in.len()
        );
    };
}

#[cfg(test)]
macro_rules! assert_not_rule {
    ($rule:expr, $in:expr) => {
        assert!(
            IfTParser::parse($rule, $in).is_err()
                || IfTParser::parse($rule, $in)
                    .unwrap()
                    .last()
                    .unwrap()
                    .into_span()
                    .end()
                    != $in.len()
        );
    };
}

#[test]
fn test_filter_ipv4() {
    let s = "FilterIPv4";
    assert_rule!(Rule::filter, s);
}

#[test]
fn test_filter_flags() {
    let s = r#"FilterFlags "up""#;
    assert_rule!(Rule::filter, s);
}

#[test]
fn test_filter_flags_missing() {
    let s = r#"FilterFlags"#;
    assert_not_rule!(Rule::filter, s);
}

#[test]
fn test_sort_by() {
    let s = r#"SortBy "default""#;
    assert_rule!(Rule::sort, s);
}


#[test]
fn test_producer() {
    let s = "GetAllInterfaces";
    assert_rule!(Rule::producer, s);
}