#![feature(test)]

extern crate fuka;
extern crate hayashi;
extern crate horiuchi;
extern crate kondo;
extern crate shiro;
extern crate test;

macro_rules! benchmark {
    ($name: ident, $json: expr) => {
        mod $name {
            use test::Bencher;
            use std::str::FromStr;

            #[bench]
            fn fuka(b: &mut Bencher) {
                b.iter(|| fuka::Value::from_str($json).is_ok());
            }

            #[bench]
            fn hayashi(b: &mut Bencher) {
                b.iter(|| hayashi::parse($json).is_ok());
            }

            #[bench]
            fn horiuchi(b: &mut Bencher) {
                b.iter(|| horiuchi::parse($json).is_ok());
            }

            #[bench]
            fn kondo(b: &mut Bencher) {
                b.iter(|| kondo::parse($json).is_ok());
            }

            #[bench]
            fn shiro(b: &mut Bencher) {
                b.iter(|| shiro::parse_str($json).is_ok());
            }
        }
    };
}

benchmark!(parse_string, r#""hello""#);

benchmark!(parse_escaped_string, r#""\"\\\/\b\f\n\r\t\u0052""#);

benchmark!(parse_escaped_string_including_surrogate_pair, r#""\"\\\/\b\f\n\r\t\uD83E\uDD80\u0052""#);

benchmark!(parse_number, "123");

benchmark!(parse_number2, "-123.45678e+9");

benchmark!(parse_true, "true");

benchmark!(parse_false, "false");

benchmark!(parse_null, "null");

benchmark!(parse_array, r#"
    [
        123,
        "hello",
        true,
        false,
        null
    ]
"#);

benchmark!(parse_object, r#"
    {
        "number": 123,
        "string": "hello",
        "boolean1": true,
        "boolean2": false,
        "null": null
    }
"#);

benchmark!(parse_complex_object, r#"
    {
        "number": 123,
        "string": "hello",
        "boolean1": true,
        "boolean2": false,
        "null": null
        "array": [
            123,
            "hello",
            true,
            false,
            null
        ]
    }
"#);
