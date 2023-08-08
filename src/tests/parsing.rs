use num::{BigInt, Num};
use crate::parser::ionpath_parser;
use crate::{Path, Segment, Key};


#[test]
fn test_parsing_keys() {
    // symbol keys
    assert_eq!(ionpath_parser::path("a"), Ok(Path {
        absolute: false,
        segments: vec![Segment::new(false, Key::Symbol("a".into()))].into()
    }));
    assert_eq!(ionpath_parser::path("/a"), Ok(Path {
        absolute: true,
        segments: vec![Segment::new(false, Key::Symbol("a".into()))].into()
    }));
    assert_eq!(ionpath_parser::path("/a_b_$c3$_4_"), Ok(Path {
        absolute: true,
        segments: vec![Segment::new(false, Key::Symbol("a_b_$c3$_4_".into()))].into()
    }));
    assert_eq!(ionpath_parser::path("/'  a b $c3$ 4_ ?&%'"), Ok(Path {
        absolute: true,
        segments: vec![Segment::new(false, Key::Symbol("  a b $c3$ 4_ ?&%".into()))].into()
    }));
    assert_eq!(ionpath_parser::path("/''"), Ok(Path {
        absolute: true,
        segments: vec![Segment::new(false, Key::Symbol("".into()))].into()
    }));
    assert_eq!(ionpath_parser::path("/a/b/c/d"), Ok(Path {
        absolute: true,
        segments: vec![
            Segment::new(false, Key::Symbol("a".into())),
            Segment::new(false, Key::Symbol("b".into())),
            Segment::new(false, Key::Symbol("c".into())),
            Segment::new(false, Key::Symbol("d".into())),
        ].into()
    }));
    assert_eq!(ionpath_parser::path("a/b/c/d"), Ok(Path {
        absolute: false,
        segments: vec![
            Segment::new(false, Key::Symbol("a".into())),
            Segment::new(false, Key::Symbol("b".into())),
            Segment::new(false, Key::Symbol("c".into())),
            Segment::new(false, Key::Symbol("d".into())),
        ].into()
    }));
    assert_eq!(ionpath_parser::path("a_b_$c3$_4_/'  a b $c3$ 4_ ?&%'/''/$"), Ok(Path {
        absolute: false,
        segments: vec![
            Segment::new(false, Key::Symbol("a_b_$c3$_4_".into())),
            Segment::new(false, Key::Symbol("  a b $c3$ 4_ ?&%".into())),
            Segment::new(false, Key::Symbol("".into())),
            Segment::new(false, Key::Symbol("$".into())),
        ].into()
    }));

    // string keys
    assert_eq!(ionpath_parser::path("/\"abc\""), Ok(Path {
        absolute: true,
        segments: vec![Segment::new(false, Key::String("abc".into()))].into()
    }));
    assert_eq!(ionpath_parser::path("/\"a : b $c \\t\\x25\""), Ok(Path {
        absolute: true,
        segments: vec![Segment::new(false, Key::String("a : b $c \t%".into()))].into()
    }));
    assert_eq!(ionpath_parser::path(r#"/"123.456 \\0 \f\x0C""#), Ok(Path {
        absolute: true,
        segments: vec![Segment::new(false, Key::String("123.456 \\0 \x0C\x0C".into()))].into()
    }));
    assert_eq!(ionpath_parser::path("/\"\""), Ok(Path {
        absolute: true,
        segments: vec![Segment::new(false, Key::String("".into()))].into()
    }));
    assert_eq!(ionpath_parser::path(r#""a : b $c \t\x25"/"123.456 \\0 \f\x0C"/"""#), Ok(Path {
        absolute: false,
        segments: vec![
            Segment::new(false, Key::String("a : b $c \t%".into())),
            Segment::new(false, Key::String("123.456 \\0 \x0C\x0C".into())),
            Segment::new(false, Key::String("".into())),
        ].into()
    }));

    // both
    assert_eq!(ionpath_parser::path(r#"a_b_$c3$_4_/"a : b $c \t\x25"/'  a b $c3$ 4_ ?&%'/"123.456 \\0 \f\x0C"/''/""/$"#), Ok(Path {
        absolute: false,
        segments: vec![
            Segment::new(false, Key::Symbol("a_b_$c3$_4_".into())),
            Segment::new(false, Key::String("a : b $c \t%".into())),
            Segment::new(false, Key::Symbol("  a b $c3$ 4_ ?&%".into())),
            Segment::new(false, Key::String("123.456 \\0 \x0C\x0C".into())),
            Segment::new(false, Key::Symbol("".into())),
            Segment::new(false, Key::String("".into())),
            Segment::new(false, Key::Symbol("$".into())),
        ].into()
    }));

    // index keys
    assert_eq!(ionpath_parser::path("/3"), Ok(Path {
        absolute: true,
        segments: vec![Segment::new(false, Key::Index(BigInt::from(3)))].into()
    }));
    assert_eq!(ionpath_parser::path("/123"), Ok(Path {
        absolute: true,
        segments: vec![Segment::new(false, Key::Index(BigInt::from(123)))].into()
    }));
    assert_eq!(ionpath_parser::path("/12345678901234567890123456789012345678901234567890"), Ok(Path {
        absolute: true,
        segments: vec![Segment::new(false, Key::Index(BigInt::from_str_radix("12345678901234567890123456789012345678901234567890", 10).unwrap()))].into()
    }));

    // slice keys
    assert_eq!(ionpath_parser::path("/1:2"), Ok(Path {
        absolute: true,
        segments: vec![Segment::new(false, Key::Slice(Some(1), Some(2), None))].into()
    }));
    assert_eq!(ionpath_parser::path("/1:2:3"), Ok(Path {
        absolute: true,
        segments: vec![Segment::new(false, Key::Slice(Some(1), Some(2), Some(3)))].into()
    }));
    assert_eq!(ionpath_parser::path("/-1:-2"), Ok(Path {
        absolute: true,
        segments: vec![Segment::new(false, Key::Slice(Some(-1), Some(-2), None))].into()
    }));
    assert_eq!(ionpath_parser::path("/-1:-2:-3"), Ok(Path {
        absolute: true,
        segments: vec![Segment::new(false, Key::Slice(Some(-1), Some(-2), Some(-3)))].into()
    }));
    assert_eq!(ionpath_parser::path("/3:"), Ok(Path {
        absolute: true,
        segments: vec![Segment::new(false, Key::Slice(Some(3), None, None))].into()
    }));
    assert_eq!(ionpath_parser::path("/:4"), Ok(Path {
        absolute: true,
        segments: vec![Segment::new(false, Key::Slice(None, Some(4), None))].into()
    }));
    assert_eq!(ionpath_parser::path("/5::6"), Ok(Path {
        absolute: true,
        segments: vec![Segment::new(false, Key::Slice(Some(5), None, Some(6)))].into()
    }));
    assert_eq!(ionpath_parser::path("/:7:8"), Ok(Path {
        absolute: true,
        segments: vec![Segment::new(false, Key::Slice(None, Some(7), Some(8)))].into()
    }));
    assert_eq!(ionpath_parser::path("/-3:"), Ok(Path {
        absolute: true,
        segments: vec![Segment::new(false, Key::Slice(Some(-3), None, None))].into()
    }));
    assert_eq!(ionpath_parser::path("/:-4"), Ok(Path {
        absolute: true,
        segments: vec![Segment::new(false, Key::Slice(None, Some(-4), None))].into()
    }));
    assert_eq!(ionpath_parser::path("/-5::-6"), Ok(Path {
        absolute: true,
        segments: vec![Segment::new(false, Key::Slice(Some(-5), None, Some(-6)))].into()
    }));
    assert_eq!(ionpath_parser::path("/:-7:-8"), Ok(Path {
        absolute: true,
        segments: vec![Segment::new(false, Key::Slice(None, Some(-7), Some(-8)))].into()
    }));
    assert_eq!(ionpath_parser::path("/1234567890:-987654321:1357924680"), Ok(Path {
        absolute: true,
        segments: vec![Segment::new(false, Key::Slice(Some(1234567890), Some(-987654321), Some(1357924680)))].into()
    }));
    assert_eq!(ionpath_parser::path("/-1234567890:987654321:-1357924680"), Ok(Path {
        absolute: true,
        segments: vec![Segment::new(false, Key::Slice(Some(-1234567890), Some(987654321), Some(-1357924680)))].into()
    }));

    // all together
    assert_eq!(ionpath_parser::path(r#"a_b_$c3$_4_/"a : b $c \t\x25"\
/123456789098765432101234567890987654321/'  a b $c3$ 4_ ?&%'/3:2:-1/"123.456 \\0 \f\x0C"\
/:12:34/''/-123::-456/""/$/:567:-89"#), Ok(Path {
        absolute: false,
        segments: vec![
            Segment::new(false, Key::Symbol("a_b_$c3$_4_".into())),
            Segment::new(false, Key::String("a : b $c \t%".into())),
            Segment::new(false, Key::Index(BigInt::from_str_radix("123456789098765432101234567890987654321", 10).unwrap())),
            Segment::new(false, Key::Symbol("  a b $c3$ 4_ ?&%".into())),
            Segment::new(false, Key::Slice(Some(3), Some(2), Some(-1))),
            Segment::new(false, Key::String("123.456 \\0 \x0C\x0C".into())),
            Segment::new(false, Key::Slice(None, Some(12), Some(34))),
            Segment::new(false, Key::Symbol("".into())),
            Segment::new(false, Key::Slice(Some(-123), None, Some(-456))),
            Segment::new(false, Key::String("".into())),
            Segment::new(false, Key::Symbol("$".into())),
            Segment::new(false, Key::Slice(None, Some(567), Some(-89))),
        ].into()
    }));
}


#[test]
fn test_parsing_annotations() {
    // TODO
}


#[test]
fn test_parsing_subquery_predicates() {
    // TODO
}


#[test]
fn test_parsing_comparison_predicates() {
    // TODO
}


#[test]
fn test_parsing_all() {
    // TODO
}