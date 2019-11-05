extern crate json;
#[macro_use]
extern crate maplit;

use json::Json;

#[test]
fn test_parse_complex_object() {
    let obj = Json::Object(Box::new(hashmap! {
        "array".to_string() => Json::Array(vec![
            Json::Object(Box::new(hashmap! {
                "num".to_string() => Json::Number(1.0),
                "str".to_string() => Json::String("hello".to_string()),
            })),
            Json::Number(2.0),
            Json::String("world".to_string()),
        ]),
    }));
    assert_eq!(
        json::parse_str(
            r#"
    {
        "array": [
            {
                "num": 1,
                "str": "hello"
            },
            2,
            "world"
        ]
    }
    "#
        )
        .unwrap(),
        obj
    );
}

#[test]
fn test_parse_object_with_1_pair() {
    let obj = Json::Object(Box::new(hashmap! {
        "hello".to_string() => Json::String("world".to_string()),
    }));
    assert_eq!(json::parse_str(r#"{"hello":"world"}"#).unwrap(), obj);
}

#[test]
fn test_parse_object_with_2_pairs() {
    let obj = Json::Object(Box::new(hashmap! {
        "hello".to_string() => Json::String("world".to_string()),
        "foo".to_string() => Json::String("bar".to_string()),
    }));
    assert_eq!(
        json::parse_str(r#"{"hello":"world","foo":"bar"}"#).unwrap(),
        obj
    );
}

#[test]
fn test_parse_array_with_1_value() {
    let array = Json::Array(vec![Json::String("hello".to_string())]);
    assert_eq!(json::parse_str(r#"["hello"]"#).unwrap(), array);
}

#[test]
fn test_parse_array_with_2_values() {
    let array = Json::Array(vec![
        Json::String("hello".to_string()),
        Json::String("world".to_string()),
    ]);
    assert_eq!(json::parse_str(r#"["hello","world"]"#).unwrap(), array);
}

#[test]
fn test_parse_string() {
    assert_eq!(
        json::parse_str(r#""hello""#).unwrap(),
        Json::String("hello".to_string())
    );
}

#[test]
fn test_parse_string_with_lf() {
    assert_eq!(
        json::parse_str(r#""hello\nworld""#).unwrap(),
        Json::String("hello\nworld".to_string())
    );
}

#[test]
fn test_parse_string_with_codepoint() {
    assert_eq!(
        json::parse_str(r#""hello \u0052\u0075\u0073\u0074""#).unwrap(),
        Json::String("hello Rust".to_string())
    );
}

#[test]
fn test_parse_string_with_surrogate_pair_codepoint() {
    assert_eq!(
        json::parse_str(r#""hello \u0001\uF980""#).unwrap(),
        Json::String("hello 🦀".to_string())
    );
}

#[test]
fn test_parse_string_with_unexpected_eof() {
    assert!(json::parse_str(r#""hel"#).is_err());
}

#[test]
fn test_parse_number() {
    assert_eq!(json::parse_str("123").unwrap(), Json::Number(123f64));
}

#[test]
fn test_parse_number_with_positive_exponent() {
    assert_eq!(json::parse_str("123e+3").unwrap(), Json::Number(123e+3f64));
}

#[test]
fn test_parse_number_with_negative_exponent() {
    assert_eq!(json::parse_str("123e-3").unwrap(), Json::Number(123e-3f64));
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
