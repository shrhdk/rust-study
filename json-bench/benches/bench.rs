#![feature(test)]

extern crate fuka;
extern crate hayashi;
extern crate horiuchi;
extern crate kondo;
extern crate shiro;
extern crate test;

mod simple_string {
    use std::str::FromStr;

    static JSON: &str = r#""hello""#;

    #[bench]
    fn fuka(b: &mut test::Bencher) {
        b.iter(|| fuka::Value::from_str(JSON))
    }

    #[bench]
    fn hayashi(b: &mut test::Bencher) {
        b.iter(|| hayashi::parse(JSON))
    }

    //    #[bench]
    //    fn horiuchi(b: &mut test::Bencher) {
    //        b.iter(|| horiuchi::json::value(JSON))
    //    }

    #[bench]
    fn kondo(b: &mut test::Bencher) {
        b.iter(|| kondo::parse(JSON))
    }

    #[bench]
    fn shiro(b: &mut test::Bencher) {
        b.iter(|| shiro::parse_str(JSON));
    }
}

mod simple_number {
    use std::str::FromStr;

    static JSON: &str = "123";

    #[bench]
    fn fuka(b: &mut test::Bencher) {
        b.iter(|| fuka::Value::from_str(JSON))
    }

    #[bench]
    fn hayashi(b: &mut test::Bencher) {
        b.iter(|| hayashi::parse(JSON))
    }

    //    #[bench]
    //    fn horiuchi(b: &mut test::Bencher) {
    //        b.iter(|| horiuchi::json::value(JSON))
    //    }

    #[bench]
    fn kondo(b: &mut test::Bencher) {
        b.iter(|| kondo::parse(JSON))
    }

    #[bench]
    fn shiro(b: &mut test::Bencher) {
        b.iter(|| shiro::parse_str(JSON));
    }
}

mod simple_object {
    use std::str::FromStr;

    static JSON: &str = r#"
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
    "#;

    #[bench]
    fn fuka(b: &mut test::Bencher) {
        b.iter(|| fuka::Value::from_str(JSON))
    }

    #[bench]
    fn hayashi(b: &mut test::Bencher) {
        b.iter(|| hayashi::parse(JSON))
    }

    //    #[bench]
    //    fn horiuchi(b: &mut test::Bencher) {
    //        b.iter(|| horiuchi::json::value(JSON))
    //    }

    #[bench]
    fn kondo(b: &mut test::Bencher) {
        b.iter(|| kondo::parse(JSON))
    }

    #[bench]
    fn shiro(b: &mut test::Bencher) {
        b.iter(|| shiro::parse_str(JSON));
    }
}
