extern crate json;

use json::Json;

#[test]
fn test_parse_string() {
    assert_eq!(json::parse_str(r#""hello""#).unwrap(), Json::String("hello".to_string()))
}

#[test]
fn test_parse_true() {
    assert_eq!(json::parse_str("true").unwrap(), Json::Boolean(true));
}

#[test]
fn test_parse_false() {
    assert_eq!(json::parse_str("false").unwrap(), Json::Boolean(false));
}

#[test]
fn test_parse_null() {
    assert_eq!(json::parse_str("null").unwrap(), Json::Null);
}

#[test]
fn test_parse_null_with_bad_suffix() {
    assert!(json::parse_str("nullpo").is_err());
}

#[test]
fn test_parse_top_level_value_with_whitespaces() {
    assert_eq!(json::parse_str("  null  ").unwrap(), Json::Null);
}
