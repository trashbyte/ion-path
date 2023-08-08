use std::ops::Neg;
use ion_rs::{Decimal, IonType, Timestamp};
use ion_rs::external::bigdecimal::BigDecimal;
use num::{BigInt, Num};
use num::bigint::Sign;
use crate::{parser::ionpath_parser, Literal};

#[test]
fn test_bool() {
    assert_eq!(ionpath_parser::boolean("true"),  Ok(Literal::Boolean(true)));
    assert_eq!(ionpath_parser::boolean("false"), Ok(Literal::Boolean(false)));
    assert!(ionpath_parser::null("True").is_err());
    assert!(ionpath_parser::null("TRUE").is_err());
    assert!(ionpath_parser::null("'true'").is_err());
    assert!(ionpath_parser::null("\"true\"").is_err());
    assert!(ionpath_parser::null("False").is_err());
    assert!(ionpath_parser::null("FALSE").is_err());
    assert!(ionpath_parser::null("'false'").is_err());
    assert!(ionpath_parser::null("\"false\"").is_err());
    assert!(ionpath_parser::null("tru").is_err());
    assert!(ionpath_parser::null("fals").is_err());
    assert!(ionpath_parser::null("truee").is_err());
    assert!(ionpath_parser::null("falsee").is_err());
    assert!(ionpath_parser::null("truetrue").is_err());
    assert!(ionpath_parser::null("falsefalse").is_err());
}

#[test]
fn test_nulls_positive() {
    assert_eq!(ionpath_parser::null("null"),           Ok(Literal::Null(IonType::Null)));
    assert_eq!(ionpath_parser::null("null.null"),      Ok(Literal::Null(IonType::Null)));
    assert_eq!(ionpath_parser::null("null.bool"),      Ok(Literal::Null(IonType::Bool)));
    assert_eq!(ionpath_parser::null("null.int"),       Ok(Literal::Null(IonType::Int)));
    assert_eq!(ionpath_parser::null("null.float"),     Ok(Literal::Null(IonType::Float)));
    assert_eq!(ionpath_parser::null("null.decimal"),   Ok(Literal::Null(IonType::Decimal)));
    assert_eq!(ionpath_parser::null("null.timestamp"), Ok(Literal::Null(IonType::Timestamp)));
    assert_eq!(ionpath_parser::null("null.string"),    Ok(Literal::Null(IonType::String)));
    assert_eq!(ionpath_parser::null("null.symbol"),    Ok(Literal::Null(IonType::Symbol)));
    assert_eq!(ionpath_parser::null("null.blob"),      Ok(Literal::Null(IonType::Blob)));
    assert_eq!(ionpath_parser::null("null.clob"),      Ok(Literal::Null(IonType::Clob)));
    assert_eq!(ionpath_parser::null("null.struct"),    Ok(Literal::Null(IonType::Struct)));
    assert_eq!(ionpath_parser::null("null.list"),      Ok(Literal::Null(IonType::List)));
    assert_eq!(ionpath_parser::null("null.sexp"),      Ok(Literal::Null(IonType::SExp)));
}

#[test]
fn test_nulls_negative() {
    assert!(ionpath_parser::null("nul").is_err());
    assert!(ionpath_parser::null("nulll").is_err());
    assert!(ionpath_parser::null("nullbool").is_err());
    assert!(ionpath_parser::null("null..bool").is_err());
    assert!(ionpath_parser::null("null.integer").is_err());
    assert!(ionpath_parser::null(".null").is_err());
    assert!(ionpath_parser::null("null.").is_err());
    assert!(ionpath_parser::null("_null").is_err());
    assert!(ionpath_parser::null("'null'").is_err());
    assert!(ionpath_parser::null("'null.bool'").is_err());
    assert!(ionpath_parser::null("\"null\"").is_err());
    assert!(ionpath_parser::null("'''null'''").is_err());
}

#[test]
fn test_ints_positive() {
    assert_eq!(ionpath_parser::integer("0"),
               Ok(Literal::Integer(BigInt::new(Sign::NoSign, vec![0]))));
    assert_eq!(ionpath_parser::integer("-0"),
               Ok(Literal::Integer(BigInt::new(Sign::Minus, vec![0]))));
    assert_eq!(ionpath_parser::integer("123"),
               Ok(Literal::Integer(BigInt::new(Sign::Plus, vec![123]))));
    assert_eq!(ionpath_parser::integer("1_1"),
               Ok(Literal::Integer(BigInt::new(Sign::Plus, vec![11]))));
    assert_eq!(ionpath_parser::integer("0xbeef"),
               Ok(Literal::Integer(BigInt::new(Sign::Plus, vec![48879]))));
    assert_eq!(ionpath_parser::integer("0xC0FE"),
               Ok(Literal::Integer(BigInt::new(Sign::Plus, vec![49406]))));
    assert_eq!(ionpath_parser::integer("0XCAFE_BABE"),
               Ok(Literal::Integer(BigInt::new(Sign::Plus, vec![3405691582]))));
    assert_eq!(ionpath_parser::integer("0b0"),
               Ok(Literal::Integer(BigInt::new(Sign::NoSign, vec![0]))));
    assert_eq!(ionpath_parser::integer("0b1"),
               Ok(Literal::Integer(BigInt::new(Sign::Plus, vec![1]))));
    assert_eq!(ionpath_parser::integer("0b1111"),
               Ok(Literal::Integer(BigInt::new(Sign::Plus, vec![15]))));
    assert_eq!(ionpath_parser::integer("1234567890"),
               Ok(Literal::Integer(BigInt::new(Sign::Plus, vec![1234567890]))));
    assert_eq!(ionpath_parser::integer("1_234_567_890"),
               Ok(Literal::Integer(BigInt::new(Sign::Plus, vec![1234567890]))));
    assert_eq!(ionpath_parser::integer("1_2_345678_90"),
               Ok(Literal::Integer(BigInt::new(Sign::Plus, vec![1234567890]))));
    assert_eq!(ionpath_parser::integer("1_2_3_4_5_6_7_8_9_0"),
               Ok(Literal::Integer(BigInt::new(Sign::Plus, vec![1234567890]))));
    assert_eq!(ionpath_parser::integer("12345678901234567890123456789012345678901234567890"),
               Ok(Literal::Integer(BigInt::from_str_radix(
                   "12345678901234567890123456789012345678901234567890", 10).unwrap())));
    assert_eq!(ionpath_parser::integer("-12345678901234567890123456789012345678901234567890"),
               Ok(Literal::Integer(BigInt::from_str_radix(
                   "12345678901234567890123456789012345678901234567890", 10).unwrap().neg())));
}

#[test]
fn test_ints_negative() {
    assert!(ionpath_parser::integer("").is_err());
    assert!(ionpath_parser::integer("001").is_err());
    assert!(ionpath_parser::integer("1.0").is_err());
    assert!(ionpath_parser::integer("1e0").is_err());
    assert!(ionpath_parser::integer("1d0").is_err());
    assert!(ionpath_parser::integer("1.").is_err());
    assert!(ionpath_parser::integer("_1").is_err());
    assert!(ionpath_parser::integer("1_").is_err());
    assert!(ionpath_parser::integer("1__1").is_err());
    assert!(ionpath_parser::integer("0o123").is_err());
    assert!(ionpath_parser::integer("1_2_3_").is_err());
    assert!(ionpath_parser::integer("0x12_34_").is_err());
    assert!(ionpath_parser::integer("0x12__34").is_err());
    assert!(ionpath_parser::integer("0x_12_34").is_err());
}

#[test]
fn test_floats_positive() {
    assert!(if let Ok(Literal::Float(f)) = ionpath_parser::float("nan") {
        f.is_nan()
    } else { false });
    assert_eq!(ionpath_parser::float("inf"),                Ok(Literal::Float(f64::INFINITY)));
    assert_eq!(ionpath_parser::float("-inf"),               Ok(Literal::Float(f64::NEG_INFINITY)));
    assert_eq!(ionpath_parser::float("+inf"),               Ok(Literal::Float(f64::INFINITY)));
    assert_eq!(ionpath_parser::float("123456.0e0"),         Ok(Literal::Float(123456.0)));
    assert_eq!(ionpath_parser::float("123456e0"),           Ok(Literal::Float(123456.0)));
    assert_eq!(ionpath_parser::float("123456e1"),           Ok(Literal::Float(1234560.0)));
    assert_eq!(ionpath_parser::float("123456e3"),           Ok(Literal::Float(123456000.0)));
    assert_eq!(ionpath_parser::float("123456e42"),          Ok(Literal::Float(123456.0e+42)));
    assert_eq!(ionpath_parser::float("123456e-0"),          Ok(Literal::Float(123456.0)));
    assert_eq!(ionpath_parser::float("123456e-1"),          Ok(Literal::Float(12345.6)));
    assert_eq!(ionpath_parser::float("123456e-3"),          Ok(Literal::Float(123.456)));
    assert_eq!(ionpath_parser::float("123456e-42"),         Ok(Literal::Float(123456.0e-42)));
    assert_eq!(ionpath_parser::float("0.123456e0"),         Ok(Literal::Float(0.123456)));
    assert_eq!(ionpath_parser::float("1.23456e0"),          Ok(Literal::Float(1.23456)));
    assert_eq!(ionpath_parser::float("12345.6e0"),          Ok(Literal::Float(12345.6)));
    assert_eq!(ionpath_parser::float("12345.60e0"),         Ok(Literal::Float(12345.6)));
    assert_eq!(ionpath_parser::float("12345.600e0"),        Ok(Literal::Float(12345.6)));
    assert_eq!(ionpath_parser::float("12300456.0e0"),       Ok(Literal::Float(12300456.0)));
    assert_eq!(ionpath_parser::float("123.00456e0"),        Ok(Literal::Float(123.00456)));
    assert_eq!(ionpath_parser::float("1230.0456e0"),        Ok(Literal::Float(1230.0456)));
    assert_eq!(ionpath_parser::float("12300.456e0"),        Ok(Literal::Float(12300.456)));
    assert_eq!(ionpath_parser::float("123.456e42"),         Ok(Literal::Float(123.456e+42)));
    assert_eq!(ionpath_parser::float("123.456e+42"),        Ok(Literal::Float(123.456e+42)));
    assert_eq!(ionpath_parser::float("123.456e-42"),        Ok(Literal::Float(123.456e-42)));
    assert_eq!(ionpath_parser::float("77777.7e0007"),       Ok(Literal::Float(777777000000.0)));
    assert_eq!(ionpath_parser::float("77777.7e-0007"),      Ok(Literal::Float(0.00777777)));
    assert_eq!(ionpath_parser::float("77777.7e+0007"),      Ok(Literal::Float(777777000000.0)));
    assert_eq!(ionpath_parser::float("12_34.56_78e0"),      Ok(Literal::Float(1234.5678)));
    assert_eq!(ionpath_parser::float("12_34e56"),           Ok(Literal::Float(1234.0e+56)));
    assert_eq!(ionpath_parser::float("1_2_3_4.5_6_7_8E90"), Ok(Literal::Float(1234.5678e+90)));

    for s in [
        "0e0", "0E0", "0.0e0", "0e-0", "0E-0", "0e-42", "0E-313", "0e+103",
        "0E+99", "0E666", "0.0e99", "0.000e-87", "0.0000E45",
        "-0e0", "-0E0", "-0.0e0", "-0e-0", "-0E-0", "-0e-42", "-0E-313",
        "-0e+103", "-0E+99", "-0E666", "-0.0e99", "-0.000e-87", "-0.0000E45"
    ] {
        assert_eq!(ionpath_parser::float(s), Ok(Literal::Float(0.0)));
    }

    for s in [
        "2.2250738585072012e-308",
        "0.00022250738585072012e-304",
        "2.225073858507201200000e-308",
        "2.2250738585072012e-00308",
        "2.2250738585072012997800001e-308",
    ] {
        assert_eq!(ionpath_parser::float(s), Ok(Literal::Float(f64::MIN_POSITIVE)));
    }
}

#[test]
fn test_float_negative() {
    assert!(ionpath_parser::float("00e0").is_err());
    assert!(ionpath_parser::float("0e.3").is_err());
    assert!(ionpath_parser::float("003e4").is_err());
    assert!(ionpath_parser::float("03.4e0").is_err());
    assert!(ionpath_parser::float("3.4ea").is_err());
    assert!(ionpath_parser::float("3.4e4.3").is_err());
    assert!(ionpath_parser::float("3.4ee4").is_err());
    assert!(ionpath_parser::float("3.4.4-3").is_err());
    assert!(ionpath_parser::float("3.4e3-3").is_err());
    assert!(ionpath_parser::float("0e0-3").is_err());
    assert!(ionpath_parser::float("0e-3-4").is_err());
    assert!(ionpath_parser::float("+123e0").is_err());
    assert!(ionpath_parser::float("0_0e0").is_err());
    assert!(ionpath_parser::float("1_.123e0").is_err());
    assert!(ionpath_parser::float("1._123e0").is_err());
    assert!(ionpath_parser::float("1._e0").is_err());
    assert!(ionpath_parser::float("1.e_0").is_err());
    assert!(ionpath_parser::float("12e4\\").is_err());
    assert!(ionpath_parser::float("12e4\\\n").is_err());
}

macro_rules! test_dec_eq {
    ($s:expr, $d:expr) => {
        assert_eq!(ionpath_parser::decimal($s),
               Ok(Literal::Decimal($d)))
    }
}
macro_rules! test_dec_eq_bigint {
    ($s:expr) => {
        test_dec_eq!(($s), Decimal::from(BigDecimal::from_str_radix($s, 10).unwrap()))
    };
    ($s1:expr, $s2:literal) => {
        test_dec_eq!(($s1), Decimal::from(BigDecimal::from_str_radix($s2, 10).unwrap()))
    };
}

#[test]
fn test_decimals_positive() {
    test_dec_eq!("123456.0",        Decimal::new(1234560, -1));
    test_dec_eq!("123456d0",        Decimal::new(123456, 0));
    test_dec_eq!("123456d1",        Decimal::new(123456, 1));
    test_dec_eq!("123456d3",        Decimal::new(123456, 3));
    test_dec_eq!("123456d42",       Decimal::new(123456, 42));
    test_dec_eq!("123456d-0",       Decimal::new(123456, 0));
    test_dec_eq!("123456d-4",       Decimal::new(123456, -4));
    test_dec_eq!("123456d-42",      Decimal::new(123456, -42));
    test_dec_eq!("0.123456",        Decimal::new(123456, -6));
    test_dec_eq!("1.23456",         Decimal::new(123456, -5));
    test_dec_eq!("12345.6",         Decimal::new(123456, -1));
    test_dec_eq!("12345.60",        Decimal::new(1234560, -2));
    test_dec_eq!("12345.600",       Decimal::new(12345600, -3));
    test_dec_eq!("12300456.0",      Decimal::new(123004560, -1));
    test_dec_eq!("123.00456",       Decimal::new(12300456, -5));
    test_dec_eq!("1230.0456",       Decimal::new(12300456, -4));
    test_dec_eq!("12300.456",       Decimal::new(12300456, -3));
    test_dec_eq!("123.456d42",      Decimal::new(123456, 39));
    test_dec_eq!("123.456d+42",     Decimal::new(123456, 39));
    test_dec_eq!("123.456d-42",     Decimal::new(123456, -45));
    test_dec_eq!("77777.7d0007",    Decimal::new(777777, 6));
    test_dec_eq!("77777.7d-0007",   Decimal::new(777777, -8));
    test_dec_eq!("77777.7d+0007",   Decimal::new(777777, 6));
    test_dec_eq!("77777.7d00700",   Decimal::new(777777, 699));
    test_dec_eq!("77777.7d-00700",  Decimal::new(777777, -701));
    test_dec_eq!("77777.7d+00700",  Decimal::new(777777, 699));
    test_dec_eq!("-123456.0",       Decimal::new(-1234560, -1));
    test_dec_eq!("-123456d0",       Decimal::new(-123456, 0));
    test_dec_eq!("-123456d1",       Decimal::new(-123456, 1));
    test_dec_eq!("-123456d3",       Decimal::new(-123456, 3));
    test_dec_eq!("-123456d42",      Decimal::new(-123456, 42));
    test_dec_eq!("-123456d-0",      Decimal::new(-123456, 0));
    test_dec_eq!("-123456d-1",      Decimal::new(-123456, -1));
    test_dec_eq!("-123456d-4",      Decimal::new(-123456, -4));
    test_dec_eq!("-123456d-42",     Decimal::new(-123456, -42));
    test_dec_eq!("-0.123456",       Decimal::new(-1234560, -7));
    test_dec_eq!("-1.23456",        Decimal::new(-123456, -5));
    test_dec_eq!("-12345.6",        Decimal::new(-123456, -1));
    test_dec_eq!("-12345.60",       Decimal::new(-1234560, -2));
    test_dec_eq!("-12345.600",      Decimal::new(-12345600, -3));
    test_dec_eq!("-12300456.0",     Decimal::new(-123004560, -1));
    test_dec_eq!("-123.00456",      Decimal::new(-12300456, -5));
    test_dec_eq!("-1230.0456",      Decimal::new(-12300456, -4));
    test_dec_eq!("-12300.456",      Decimal::new(-12300456, -3));
    test_dec_eq!("-123.456d42",     Decimal::new(-123456, 39));
    test_dec_eq!("-123.456d+42",    Decimal::new(-123456, 39));
    test_dec_eq!("-123.456d-42",    Decimal::new(-123456, -45));
    test_dec_eq!("-77777.7d0007",   Decimal::new(-777777, 6));
    test_dec_eq!("-77777.7d-0007",  Decimal::new(-777777, -8));
    test_dec_eq!("-77777.7d+0007",  Decimal::new(-777777, 6));
    test_dec_eq!("-77777.7d00700",  Decimal::new(-777777, 699));
    test_dec_eq!("-77777.7d-00700", Decimal::new(-777777, -701));
    test_dec_eq!("-77777.7d+00700", Decimal::new(-777777, 699));
    test_dec_eq!("-1.28",           Decimal::new(-128, -2));
    test_dec_eq!("12_34.56_78",     Decimal::new(12345678, -4));
    test_dec_eq!("12_34.",          Decimal::new(1234, 0));
    test_dec_eq!("1_2_3_4.5_6_7_8", Decimal::new(12345678, -4));

    test_dec_eq_bigint!("18446744073709551615.");
    test_dec_eq_bigint!("-18446744073709551615.");
    test_dec_eq_bigint!("18446744073709551616.");
    test_dec_eq_bigint!("-18446744073709551616.");

    for s in [
        "2.718281828459045", "2.718281828459045d0", "2.718281828459045d+0", "2.718281828459045d-0",
        "2718281828459045d-15", "27182818284590450000000000d-25",
        "0.000000027182818284590450000000000d+8", "0.000000027182818284590450000000000d8",
        "0.00000002718281828459045d+8", "0.00000002718281828459045d8", "2.718281828459045D0",
        "2.718281828459045D+0", "2.718281828459045D-0", "2718281828459045D-15",
        "27182818284590450000000000D-25", "0.000000027182818284590450000000000D+8",
        "0.000000027182818284590450000000000D8", "0.00000002718281828459045D+8",
        "0.00000002718281828459045D8",
    ] {
        test_dec_eq_bigint!(s, "2.718281828459045");
    }
    for s in [
        "-2.718281828459045", "-2.718281828459045d0", "-2.718281828459045d+0", "-2.718281828459045d-0",
        "-2718281828459045d-15", "-27182818284590450000000000d-25",
        "-0.000000027182818284590450000000000d+8", "-0.000000027182818284590450000000000d8",
        "-0.00000002718281828459045d+8", "-0.00000002718281828459045d8", "-2.718281828459045D0",
        "-2.718281828459045D+0", "-2.718281828459045D-0", "-2718281828459045D-15",
        "-27182818284590450000000000D-25", "-0.000000027182818284590450000000000D+8",
        "-0.000000027182818284590450000000000D8", "-0.00000002718281828459045D+8",
        "-0.00000002718281828459045D8",
    ] {
        test_dec_eq_bigint!(s, "-2.718281828459045");
    }


    for s in [
        "0.", "0.d0", "0.D0", "0d0", "0D0", "0.0", "0d-0", "0D-0", "0d-42",
        "0D-313", "0d+103", "0D+99", "0D666", "0.0d99", "0.000d-87", "0.0000",
        "-0.", "-0d0", "-0D0", "-0.0", "-0d-0", "-0D-0", "-0d-42", "-0D-313",
        "-0d+103", "-0D+99", "-0D666", "-0.0d99", "-0.000d-87", "-0.0000",
    ] {
        assert_eq!(ionpath_parser::decimal(s), Ok(Literal::Decimal(Decimal::new(0, 0))));
    }
}

#[test]
fn test_decimals_negative() {
    for s in [
        "0d0-3", "0d-3-4", "3.4d3-3", "0.3-4", "0.3.4", "0d.3", "3.4.4-3", "3.4d4.3", "3.4dd4",
        "3.4a", "3.4+43.4+43.4+43.4+4", "04.3", "007d4", "00d0", "123._456", "-_123.456", "123_.456",
        "123_._456", "_123.456", "12__34.56", "+123d0", "123.456_", "12.47\\", "12.47\\\n",
    ] {
        assert!(ionpath_parser::decimal(s).is_err());
    }
}

macro_rules! assert_timestamp_eq {
    ($s:literal = $ts:expr) => {
        assert_eq!(ionpath_parser::timestamp($s), Ok(Literal::Timestamp($ts)))
    }
}

#[test]
fn test_timestamps_positive() {
    assert_timestamp_eq!("0001T"            = Timestamp::with_year(1).build().unwrap());
    assert_timestamp_eq!("0001-01T"          = Timestamp::with_year(1).with_month(1).build().unwrap());
    assert_timestamp_eq!("0001-01-01"        = Timestamp::with_ymd(1, 1, 1).build().unwrap());
    assert_timestamp_eq!("0001-01-01T"       = Timestamp::with_ymd(1, 1, 1).build().unwrap());
    assert_timestamp_eq!("0001-01-01T00:00Z" = Timestamp::with_ymd(1, 1, 1).with_hour_and_minute(0, 0).build_at_offset(0).unwrap());

    let ts_01_prec_ymd_hms = Timestamp::with_ymd_hms(1, 1, 1, 0, 0, 0).build_at_offset(0).unwrap();
    assert_timestamp_eq!("0001-01-01T00:00+00:00"    = ts_01_prec_ymd_hms.clone());
    assert_timestamp_eq!("0001-01-01T00:00-00:00"    = ts_01_prec_ymd_hms.clone());
    assert_timestamp_eq!("0001-01-01T00:00:00Z"      = ts_01_prec_ymd_hms.clone());
    assert_timestamp_eq!("0001-01-01T00:00:00+00:00" = ts_01_prec_ymd_hms.clone());
    assert_timestamp_eq!("0001-01-01T00:00:00-00:00" = ts_01_prec_ymd_hms.clone());

    let ts_01_prec_ymd_hms_ms = Timestamp::with_ymd_hms(1, 1, 1, 0, 0, 0).with_fractional_seconds(Decimal::new(0, -3)).build_at_offset(0).unwrap();
    assert_timestamp_eq!("0001-01-01T00:00:00.0Z"          = ts_01_prec_ymd_hms_ms.clone());
    assert_timestamp_eq!("0001-01-01T00:00:00.0+00:00"     = ts_01_prec_ymd_hms_ms.clone());
    assert_timestamp_eq!("0001-01-01T00:00:00.0-00:00"     = ts_01_prec_ymd_hms_ms.clone());
    assert_timestamp_eq!("0001-01-01T00:00:00.00Z"         = ts_01_prec_ymd_hms_ms.clone());
    assert_timestamp_eq!("0001-01-01T00:00:00.000Z"        = ts_01_prec_ymd_hms_ms.clone());
    assert_timestamp_eq!("0001-01-01T00:00:00.0000Z"       = ts_01_prec_ymd_hms_ms.clone());
    assert_timestamp_eq!("0001-01-01T00:00:00.00000Z"      = ts_01_prec_ymd_hms_ms.clone());
    assert_timestamp_eq!("0001-01-01T00:00:00.00000+00:00" = ts_01_prec_ymd_hms_ms.clone());
    assert_timestamp_eq!("0001-01-01T00:00:00.00000-00:00" = ts_01_prec_ymd_hms_ms.clone());

    assert_timestamp_eq!("1970-01-01" = Timestamp::with_ymd(1970, 1, 1).build().unwrap());
    assert_timestamp_eq!("1970-01-01T" = Timestamp::with_ymd(1970, 1, 1).build().unwrap());
    assert_timestamp_eq!("2046-11-30T23:46Z" = Timestamp::with_ymd(2046, 11, 30).with_hour_and_minute(23, 46).build_at_offset(0).unwrap());
    assert_timestamp_eq!("2004-02-29T10:20Z" = Timestamp::with_ymd(2004, 02, 29).with_hour_and_minute(10, 20).build_at_offset(0).unwrap());
    assert_timestamp_eq!("1970-06-06T03:19+08:00" = Timestamp::with_ymd(1970, 6, 6).with_hour_and_minute(3, 19).build_at_offset(8*60).unwrap());
    assert_timestamp_eq!("1835-03-31T10:50-06:15" = Timestamp::with_ymd(1835, 3, 31).with_hour_and_minute(10, 50).build_at_offset(-(6*60+15)).unwrap());
    assert_timestamp_eq!("0001-01-01T08:49:00Z" = Timestamp::with_ymd(1, 1, 1).with_hms(8, 49, 0).build_at_offset(0).unwrap());
    assert_timestamp_eq!("0001-01-01T08:49:00+08:49" = Timestamp::with_ymd(1, 1, 1).with_hms(8, 49, 0).build_at_offset(8*60+49).unwrap());
    assert_timestamp_eq!("0001-01-01T08:49:00-08:49" = Timestamp::with_ymd(1, 1, 1).with_hms(8, 49, 0).build_at_offset(-(8*60+49)).unwrap());
    assert_timestamp_eq!("1999-06-30T09:16:24.3Z" = Timestamp::with_ymd_hms(1999, 6, 30, 9, 16, 24).with_fractional_seconds(Decimal::new(3, -1)).build_at_offset(0).unwrap());
    assert_timestamp_eq!("6060-07-31T07:04:19.9Z" = Timestamp::with_ymd_hms(6060, 7, 31, 7, 4, 19).with_fractional_seconds(Decimal::new(9, -1)).build_at_offset(0).unwrap());
    assert_timestamp_eq!("1857-05-30T19:24:59.1+23:59" = Timestamp::with_ymd_hms(1857, 5, 30, 19, 24, 59).with_fractional_seconds(Decimal::new(1, -1)).build_at_offset(23*60+59).unwrap());
    assert_timestamp_eq!("0001-01-01T23:59:59.9-23:59" = Timestamp::with_ymd_hms(1, 1, 1, 23, 59, 59).with_fractional_seconds(Decimal::new(9, -1)).build_at_offset(-(23*60+59)).unwrap());
    assert_timestamp_eq!("2000-09-11T08:01:21.98Z" = Timestamp::with_ymd_hms(2000, 9, 11, 8, 1, 21).with_fractional_seconds(Decimal::new(98, -2)).build_at_offset(0).unwrap());
    assert_timestamp_eq!("2000-09-11T08:01:21.987Z" = Timestamp::with_ymd_hms(2000, 9, 11, 8, 1, 21).with_fractional_seconds(Decimal::new(987, -3)).build_at_offset(0).unwrap());
    assert_timestamp_eq!("2000-09-11T08:01:21.9876Z" = Timestamp::with_ymd_hms(2000, 9, 11, 8, 1, 21).with_fractional_seconds(Decimal::new(9876, -4)).build_at_offset(0).unwrap());
    assert_timestamp_eq!("2000-09-11T08:01:21.98765Z" = Timestamp::with_ymd_hms(2000, 9, 11, 8, 1, 21).with_fractional_seconds(Decimal::new(98765, -5)).build_at_offset(0).unwrap());
    assert_timestamp_eq!("2010-10-01T15:15:16.12345Z" = Timestamp::with_ymd_hms(2010, 10, 1, 15, 15, 16).with_fractional_seconds(Decimal::new(12345, -5)).build_at_offset(0).unwrap());
    assert_timestamp_eq!("2001-08-01T19:19:49.00600+01:01" = Timestamp::with_ymd_hms(2001, 8, 1, 19, 19, 49).with_fractional_seconds(Decimal::new(600, -5)).build_at_offset(61).unwrap());
    assert_timestamp_eq!("2100-04-01T22:22:34.06060-10:10" = Timestamp::with_ymd_hms(2100, 4, 1, 22, 22, 34).with_fractional_seconds(Decimal::new(6060, -5)).build_at_offset(-610).unwrap());
    assert_timestamp_eq!("9999T"                     = Timestamp::with_year(9999).build().unwrap());
    assert_timestamp_eq!("9999-12T"                  = Timestamp::with_year(9999).with_month(12).build().unwrap());
    assert_timestamp_eq!("9999-12-31"                = Timestamp::with_ymd(9999, 12, 31).build().unwrap());
    assert_timestamp_eq!("9999-12-31T"               = Timestamp::with_ymd(9999, 12, 31).build().unwrap());
    assert_timestamp_eq!("9999-12-31T23:59:59Z"      = Timestamp::with_ymd_hms(9999, 12, 31, 23, 59, 59).build_at_offset(0).unwrap());
    assert_timestamp_eq!("2008-02-29"                = Timestamp::with_ymd(2008, 02, 29).build().unwrap());
    assert_timestamp_eq!("2008-02-29T"               = Timestamp::with_ymd(2008, 02, 29).build().unwrap());
    assert_timestamp_eq!("2008-02-29T00:00Z"         = Timestamp::with_ymd(2008, 02, 29).with_hour_and_minute(0, 0).build_at_offset(0).unwrap());
    assert_timestamp_eq!("2008-02-29T00:00:00Z"      = Timestamp::with_ymd_hms(2008, 02, 29, 0, 0, 0).build_at_offset(0).unwrap());
    assert_timestamp_eq!("2008-02-29T00:00:00.0000Z" = Timestamp::with_ymd_hms(2008, 02, 29, 0, 0, 0).with_fractional_seconds(Decimal::new(0, -4)).build_at_offset(0).unwrap());
}

#[test]
fn test_timestamps_negative() {
    assert!(ionpath_parser::timestamp("2011-01-32").is_err());
    assert!(ionpath_parser::timestamp("2011-02-29").is_err());
    assert!(ionpath_parser::timestamp("2011-04-31").is_err());
    assert!(ionpath_parser::timestamp("2011-07-32").is_err());
    assert!(ionpath_parser::timestamp("2011-09-31").is_err());
    assert!(ionpath_parser::timestamp("0000T").is_err());
    assert!(ionpath_parser::timestamp("0000-01T").is_err());
    assert!(ionpath_parser::timestamp("0000-00T").is_err());
    assert!(ionpath_parser::timestamp("0000-00-00").is_err());
    assert!(ionpath_parser::timestamp("0000-00-00T").is_err());
    assert!(ionpath_parser::timestamp("0000-00-01").is_err());
    assert!(ionpath_parser::timestamp("0000-00-01T").is_err());
    assert!(ionpath_parser::timestamp("0000-01-00").is_err());
    assert!(ionpath_parser::timestamp("0000-01-00T").is_err());
    assert!(ionpath_parser::timestamp("0000-01-01").is_err());
    assert!(ionpath_parser::timestamp("0000-01-01T").is_err());
    assert!(ionpath_parser::timestamp("0000-00-00T00:00Z").is_err());
    assert!(ionpath_parser::timestamp("0000-00-00T00:00:00Z").is_err());
    assert!(ionpath_parser::timestamp("0000-00-00T00:00:00.0000Z").is_err());
    assert!(ionpath_parser::timestamp("0000-12-31").is_err());
    assert!(ionpath_parser::timestamp("0001-00-00").is_err());
    assert!(ionpath_parser::timestamp("0001-00-00T").is_err());
    assert!(ionpath_parser::timestamp("0001-00-01").is_err());
    assert!(ionpath_parser::timestamp("0001-00-01T").is_err());
    assert!(ionpath_parser::timestamp("0001-01-00").is_err());
    assert!(ionpath_parser::timestamp("0001-01-00T").is_err());
    assert!(ionpath_parser::timestamp("0001-00T").is_err());
    assert!(ionpath_parser::timestamp("1969-02-23T00.00Z").is_err());
    assert!(ionpath_parser::timestamp("1969-02-23T00:00.00Z").is_err());
    assert!(ionpath_parser::timestamp("1969-02-23T00:00:00.1234567890123456789012345676890z").is_err());
    assert!(ionpath_parser::timestamp("97-1-1").is_err());
    assert!(ionpath_parser::timestamp("2001-01").is_err());
    assert!(ionpath_parser::timestamp("2001-01-1").is_err());
    assert!(ionpath_parser::timestamp("1997-2-4").is_err());
    assert!(ionpath_parser::timestamp("2006-02-29").is_err());
    assert!(ionpath_parser::timestamp("2001-01-32").is_err());
    assert!(ionpath_parser::timestamp("2000-04-31").is_err());
    assert!(ionpath_parser::timestamp("1999-06-31").is_err());
    assert!(ionpath_parser::timestamp("2000-13-01").is_err());
    assert!(ionpath_parser::timestamp("2000-00-01").is_err());
    assert!(ionpath_parser::timestamp("2000-01-00").is_err());
    assert!(ionpath_parser::timestamp("97-02-01").is_err());
    assert!(ionpath_parser::timestamp("2007/02/01").is_err());
    assert!(ionpath_parser::timestamp("2007:02:01").is_err());
    assert!(ionpath_parser::timestamp("2005-01-01+08:00").is_err());
    assert!(ionpath_parser::timestamp("2005-01-01-08:00").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T1").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12z").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12Z").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12+08:00").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12-08:00").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10+8").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10+8:0").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10+8:00").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10+08").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10+08:").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10+08:1").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10+100:10").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10+10:100").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10-8").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10-8:0").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10-8:00").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10-08").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10-08:").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10-08:1").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10-100:10").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10-10:100").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10:1").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10:1z").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10:1Z").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10:1+08:00").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10:1-08:00").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10:11").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10:111").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10:11g").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10:11+8").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10:11+8:1").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10:11+08").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10:11+08:").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10:11+08:1").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10:11+8:10").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10:11+888:10").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10:11+88:110").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10:11-8").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10:11-8:1").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10:11-08").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10:11-08:").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10:11-08:1").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10:11-8:10").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10:11-888:10").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10:11-88:110").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10:11.1").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10:11.19987").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10:11.19987x").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T24:10:11Z").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:60:11Z").is_err());
    assert!(ionpath_parser::timestamp("2004-12-11T12:10:60Z").is_err());
    assert!(ionpath_parser::timestamp("2010-11-17T1:30Z").is_err());
    assert!(ionpath_parser::timestamp("2010-11-17T12:3Z").is_err());
    assert!(ionpath_parser::timestamp("2010-11-17U12:30Z").is_err());
    assert!(ionpath_parser::timestamp("2010-11-17T12:34:56.Z").is_err());
}

#[test]
fn test_strings_positive() {
    assert_eq!(ionpath_parser::string(r#""a b c d e f g h i j k l m n o p q r s t u v w x y z""#),
        Ok(Literal::String("a b c d e f g h i j k l m n o p q r s t u v w x y z".into())));
    assert_eq!(ionpath_parser::string(r#""A B C D E F G H I J K L M N O P Q R S T U V W X Y Z""#),
               Ok(Literal::String("A B C D E F G H I J K L M N O P Q R S T U V W X Y Z".into())));
    assert_eq!(ionpath_parser::string(r#""1 2 3 4 5 6 7 8 9 0""#),
               Ok(Literal::String("1 2 3 4 5 6 7 8 9 0".into())));
    assert_eq!(ionpath_parser::string(r#"", . ; / [ ' ] \\ = - 0 9 8 7 6 5 4 3 2 1 ` ~ ! @ # $ % ^ & * ( ) _ + | : < > ?""#),
               Ok(Literal::String(", . ; / [ ' ] \\ = - 0 9 8 7 6 5 4 3 2 1 ` ~ ! @ # $ % ^ & * ( ) _ + | : < > ?".into())));
    assert_eq!(ionpath_parser::string(r#""\0 \a \b \t \n \f \r \v \" \' \? \\\\ \/ \0\a\b\t\n\f\r\v\"\'\?\\\\\/""#),
               Ok(Literal::String("\0 \x07 \x08 \t \n \x0C \r \x0B \" \' ? \\\\ / \0\x07\x08\t\n\x0C\r\x0B\"\'?\\\\/".into())));
    assert_eq!(ionpath_parser::string(r#""\uabcd \uffff \u1234 \u4e6a \ud37b\uf4c2\u0000\x00\xff""#),
               Ok(Literal::String("\u{abcd} \u{ffff} \u{1234} \u{4e6a} \u{d37b}\u{f4c2}\0\0\u{00ff}".into())));
    assert_eq!(ionpath_parser::string(r#""\uABCD \ucFFF \u1234 \u4E6A \uD37B\uF4C2\u0000\x00\xff""#),
               Ok(Literal::String("\u{abcd} \u{cfff} \u{1234} \u{4e6a} \u{d37b}\u{f4c2}\0\0\u{00ff}".into())));
    assert_eq!(ionpath_parser::string(r#""\uaBcD \ucffF \u1234 \u4E6a \ud37B\uF4c2\u0000\x00\xff""#),
               Ok(Literal::String("\u{abcd} \u{cfff} \u{1234} \u{4e6a} \u{d37b}\u{f4c2}\0\0\u{00ff}".into())));
    assert_eq!(ionpath_parser::string(r#""\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987""#),
               Ok(Literal::String("\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}".into())));
    assert_eq!(ionpath_parser::string(r#"".\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987""#),
               Ok(Literal::String(".\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}".into())));
    assert_eq!(ionpath_parser::string(r#""..\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987""#),
               Ok(Literal::String("..\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}".into())));
    assert_eq!(ionpath_parser::string(r#""...\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987\uF987""#),
               Ok(Literal::String("...\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}\u{F987}".into())));
    assert_eq!(ionpath_parser::string("'''Stuff to write on '''\n'''multiple lines '''\n'''if you want to'''"),
               Ok(Literal::String("Stuff to write on multiple lines if you want to".into())));
    assert_eq!(ionpath_parser::string("''''''"), Ok(Literal::String("".into())));
    assert_eq!(ionpath_parser::string("'''concatenated'''  ''' from '''   '''a single line'''"),
               Ok(Literal::String("concatenated from a single line".into())));
    assert_eq!(ionpath_parser::string(r#"'''a b c d e f g h i j k l m n o p q r s t u v w x y z '''
'''A B C D E F G H I J K L M N O P Q R S T U V W X Y Z '''
''', . ; / [ ' ] \\ = - 0 9 8 7 6 5 4 3 2 1 ` ~ ! @ # $ % ^ & * ( ) _ + | : < > ? '''
'''\0 \a \b \t \n \f \r \v \" \' \? \\\\ \/ \0\a\b\t\n\f\r\v\"\'\?\\\\\/'''
'''\uabcd \ucfff \u1234 \u4e6a \ud37b\uf4c2\u0000\x00\xff'''
'''\uABCD \uCFFF \u1234 \u4E6A \uD37B\uF4C2\u0000\x00\xFF'''
'''\uaBcD \uCffF \u1234 \u4E6a \ud37B\uF4c2\u0000\x00\xfF'''"#),
               Ok(Literal::String("a b c d e f g h i j k l m n o p q r s t u v w x y z A B C D E F G H I J K L M N O P Q R S T U V W X Y Z , . ; / [ ' ] \\ = - 0 9 8 7 6 5 4 3 2 1 ` ~ ! @ # $ % ^ & * ( ) _ + | : < > ? \0 \x07 \x08 \t \n \x0C \r \x0B \" \' ? \\\\ / \0\x07\x08\t\n\x0C\r\x0B\"\'?\\\\/\u{abcd} \u{cfff} \u{1234} \u{4e6a} \u{d37b}\u{f4c2}\0\0\u{00ff}\u{abcd} \u{cfff} \u{1234} \u{4e6a} \u{d37b}\u{f4c2}\0\0\u{00ff}\u{abcd} \u{cfff} \u{1234} \u{4e6a} \u{d37b}\u{f4c2}\0\0\u{00ff}".into())));
    assert_eq!(ionpath_parser::string(r#"'''multi-line string
with embedded\nnew line
characters'''"#), Ok(Literal::String("multi-line string
with embedded
new line
characters".into())));
   assert_eq!(ionpath_parser::string(r#""	""#),
              Ok(Literal::String("	".into())));
   assert_eq!(ionpath_parser::string(r#""""#),
              Ok(Literal::String("".into())));
   assert_eq!(ionpath_parser::string(r#""""#),
              Ok(Literal::String("".into())));
   assert_eq!(ionpath_parser::string(r#"" ""#),
              Ok(Literal::String(" ".into())));
   assert_eq!(ionpath_parser::string(r#"'''	'''
''''''
''''''
''' '''"#), Ok(Literal::String("	 ".into())));
}

#[test]
fn test_strings_negative() {
    assert!(ionpath_parser::string("\"\\8\"").is_err());
    assert!(ionpath_parser::string("\"\\900\"").is_err());
    assert!(ionpath_parser::string("\"\\xgg\"").is_err());
    assert!(ionpath_parser::string("\"\\ugggg\"").is_err());
    assert!(ionpath_parser::string("\"\\z\"").is_err());
    assert!(ionpath_parser::string("\"\\d\"").is_err());
    assert!(ionpath_parser::string("\"\\@\"").is_err());
    assert!(ionpath_parser::string("\"\\$\"").is_err());
    assert!(ionpath_parser::string("\"\"").is_err());
    assert!(ionpath_parser::string("\"abc\n\"").is_err());
    assert!(ionpath_parser::string("\"abc").is_err());
    assert!(ionpath_parser::string("\"\\e\"").is_err());
}

#[test]
fn test_symbols_positive() {
    assert_eq!(ionpath_parser::symbol(r#"'a b c d e f g h i j k l m n o p q r s t u v w x y z'"#),
               Ok(Literal::Symbol("a b c d e f g h i j k l m n o p q r s t u v w x y z".into())));
    assert_eq!(ionpath_parser::symbol(r#"'A B C D E F G H I J K L M N O P Q R S T U V W X Y Z'"#),
               Ok(Literal::Symbol("A B C D E F G H I J K L M N O P Q R S T U V W X Y Z".into())));
    assert_eq!(ionpath_parser::symbol(r#"'1 2 3 4 5 6 7 8 9 0'"#),
               Ok(Literal::Symbol("1 2 3 4 5 6 7 8 9 0".into())));
    assert_eq!(ionpath_parser::symbol(r#"', . ; / [ \' ] \\ = - 0 9 8 7 6 5 4 3 2 1 ` ~ ! @ # $ % ^ & * ( ) _ + | : < > ?'"#),
               Ok(Literal::Symbol(r#", . ; / [ ' ] \ = - 0 9 8 7 6 5 4 3 2 1 ` ~ ! @ # $ % ^ & * ( ) _ + | : < > ?"#.into())));
    assert_eq!(ionpath_parser::symbol(r#"'\0 \a \b \t \n \f \r \v \" \' \? \\\\ \/ \0\a\b\t\n\f\r\v\"\'\?\\\\\/'"#),
               Ok(Literal::Symbol("\0 \x07 \x08 \t \n \x0C \r \x0B \" \' ? \\\\ / \0\x07\x08\t\n\x0C\r\x0B\"\'?\\\\/".into())));
    assert_eq!(ionpath_parser::symbol(r#"'\uabcd \ud7ff \uffff \u1234 \u4e6a \ud37b\uf4c2\u0000\x00\xff'"#),
               Ok(Literal::Symbol("\u{abcd} \u{d7ff} \u{ffff} \u{1234} \u{4e6a} \u{d37b}\u{f4c2}\u{0000}\0\u{00ff}".into())));
    assert_eq!(ionpath_parser::symbol(r#"'\uABCD \uD7FF \uFFFF \u1234 \u4E6A \uD37B\uF4C2\u0000\x00\xff'"#),
               Ok(Literal::Symbol("\u{abcd} \u{d7ff} \u{ffff} \u{1234} \u{4e6a} \u{d37b}\u{f4c2}\u{0000}\0\u{00ff}".into())));
    assert_eq!(ionpath_parser::symbol(r#"'\uaBcD \uD7ff \uFffF \u1234 \u4E6a \ud37B\uF4c2\u0000\x00\xff'"#),
               Ok(Literal::Symbol("\u{abcd} \u{d7ff} \u{ffff} \u{1234} \u{4e6a} \u{d37b}\u{f4c2}\u{0000}\0\u{00ff}".into())));
    assert_eq!(ionpath_parser::symbol(r#"bareSymbol"#), Ok(Literal::Symbol("bareSymbol".into())));
    assert_eq!(ionpath_parser::symbol(r#"BareSymbol"#), Ok(Literal::Symbol("BareSymbol".into())));
    assert_eq!(ionpath_parser::symbol(r#"$bare"#), Ok(Literal::Symbol("$bare".into())));
    assert_eq!(ionpath_parser::symbol(r#"_bare"#), Ok(Literal::Symbol("_bare".into())));
    assert_eq!(ionpath_parser::symbol(r#"zzzzz"#), Ok(Literal::Symbol("zzzzz".into())));
    assert_eq!(ionpath_parser::symbol(r#"aaaaa"#), Ok(Literal::Symbol("aaaaa".into())));
    assert_eq!(ionpath_parser::symbol(r#"ZZZZZ"#), Ok(Literal::Symbol("ZZZZZ".into())));
    assert_eq!(ionpath_parser::symbol(r#"AAAAA"#), Ok(Literal::Symbol("AAAAA".into())));
    assert_eq!(ionpath_parser::symbol(r#"z"#), Ok(Literal::Symbol("z".into())));
    assert_eq!(ionpath_parser::symbol(r#"Z"#), Ok(Literal::Symbol("Z".into())));
    assert_eq!(ionpath_parser::symbol(r#"a"#), Ok(Literal::Symbol("a".into())));
    assert_eq!(ionpath_parser::symbol(r#"A"#), Ok(Literal::Symbol("A".into())));
    assert_eq!(ionpath_parser::symbol(r#"_"#), Ok(Literal::Symbol("_".into())));
    assert_eq!(ionpath_parser::symbol(r#"$"#), Ok(Literal::Symbol("$".into())));
    assert_eq!(ionpath_parser::symbol(r#"_9876543210"#), Ok(Literal::Symbol("_9876543210".into())));
    assert_eq!(ionpath_parser::symbol(r#"$3"#), Ok(Literal::Symbol("$3".into())));
    assert_eq!(ionpath_parser::symbol(r#"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789$_"#),
               Ok(Literal::Symbol("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789$_".into())));
    assert_eq!(ionpath_parser::symbol(r#"'$99'"#),
               Ok(Literal::Symbol("$99".into())));
    assert_eq!(ionpath_parser::symbol(r#"''"#), Ok(Literal::Symbol("".into())));
    assert_eq!(ionpath_parser::symbol(r#"'	'"#), Ok(Literal::Symbol("	".into())));
    assert_eq!(ionpath_parser::symbol(r#"''"#), Ok(Literal::Symbol("".into())));
    assert_eq!(ionpath_parser::symbol(r#"''"#), Ok(Literal::Symbol("".into())));
    assert_eq!(ionpath_parser::symbol(r#"''"#), Ok(Literal::Symbol("".into())));
    assert_eq!(ionpath_parser::symbol(r#"$0"#), Ok(Literal::Symbol("$0".into())));
    assert_eq!(ionpath_parser::symbol(r#"'\\\n'"#), Ok(Literal::Symbol("".into())));
    assert_eq!(ionpath_parser::symbol(r#"'\\\r\\\n'"#), Ok(Literal::Symbol("".into())));
    assert_eq!(ionpath_parser::symbol(r#"'\\\r'"#), Ok(Literal::Symbol("".into())));
}

#[test]
fn test_symbols_negative() {
    assert!(ionpath_parser::symbol("1symbol").is_err());
    assert!(ionpath_parser::symbol("9symbol").is_err());
    assert!(ionpath_parser::symbol(":symbol").is_err());
    assert!(ionpath_parser::symbol("'\\877'").is_err());
    assert!(ionpath_parser::symbol("'\\900'").is_err());
    assert!(ionpath_parser::symbol("'sym\\8bol'").is_err());
    assert!(ionpath_parser::symbol("'\\z'").is_err());
    assert!(ionpath_parser::symbol("'\\ugggg'").is_err());
    assert!(ionpath_parser::symbol("'\\xgg'").is_err());
    assert!(ionpath_parser::symbol("a.b").is_err());
    assert!(ionpath_parser::symbol("'\\e'").is_err());
    //assert!(ionpath_parser::symbol("$99").is_err());
}

#[test]
fn test_blobs_positive() {
    assert_eq!(ionpath_parser::blob("{{\n		YSBiIGMgZCBlIGYgZyBoIGkgaiBrIGwgbSBuIG8gcCBxIHIgcyB0IHUgdiB3IHggeSB6\n}}"),
               Ok(Literal::Blob(vec![0x61,0x20,0x62,0x20,0x63,0x20,0x64,0x20,0x65,0x20,0x66,0x20,0x67,
                                     0x20,0x68,0x20,0x69,0x20,0x6a,0x20,0x6b,0x20,0x6c,0x20,0x6d,0x20,
                                     0x6e,0x20,0x6f,0x20,0x70,0x20,0x71,0x20,0x72,0x20,0x73,0x20,0x74,
                                     0x20,0x75,0x20,0x76,0x20,0x77,0x20,0x78,0x20,0x79,0x20,0x7a])));
    assert_eq!(ionpath_parser::blob("{{  QSB CIEM  gRC B F IEYg Ry BI IEk gSi BLIE w   gTS B OI       E8 g UC BRI FIgUy BU IF Ug ViB XI F gg WS B a}}"),
               Ok(Literal::Blob(vec![0x41,0x20,0x42,0x20,0x43,0x20,0x44,0x20,0x45,0x20,0x46,0x20,0x47,
                                     0x20,0x48,0x20,0x49,0x20,0x4a,0x20,0x4b,0x20,0x4c,0x20,0x4d,0x20,
                                     0x4e,0x20,0x4f,0x20,0x50,0x20,0x51,0x20,0x52,0x20,0x53,0x20,0x54,
                                     0x20,0x55,0x20,0x56,0x20,0x57,0x20,0x58,0x20,0x59,0x20,0x5a])));
    assert_eq!(ionpath_parser::blob("{{MSAyIDMgNCA1IDYgNyA4IDkgMA\n==}}"),
               Ok(Literal::Blob(vec![0x31,0x20,0x32,0x20,0x33,0x20,0x34,0x20,0x35,0x20,0x36,0x20,0x37,
                                     0x20,0x38,0x20,0x39,0x20,0x30])));
    assert_eq!(ionpath_parser::blob("{{\n\n			LCAuIDsgLyBbICcgXSBcID0gLSAwIDkgOCA3IDYgNSA0IDMgMiAxIGAgfiAhIEAgIyAkICUgXiAmICogKCApIF8gKyB8IDogPCA+ID8=\n\n      }}"),
               Ok(Literal::Blob(vec![0x2c,0x20,0x2e,0x20,0x3b,0x20,0x2f,0x20,0x5b,0x20,0x27,0x20,0x5d,
                                     0x20,0x5c,0x20,0x3d,0x20,0x2d,0x20,0x30,0x20,0x39,0x20,0x38,0x20,
                                     0x37,0x20,0x36,0x20,0x35,0x20,0x34,0x20,0x33,0x20,0x32,0x20,0x31,
                                     0x20,0x60,0x20,0x7e,0x20,0x21,0x20,0x40,0x20,0x23,0x20,0x24,0x20,
                                     0x25,0x20,0x5e,0x20,0x26,0x20,0x2a,0x20,0x28,0x20,0x29,0x20,0x5f,
                                     0x20,0x2b,0x20,0x7c,0x20,0x3a,0x20,0x3c,0x20,0x3e,0x20,0x3f])));
    assert_eq!(ionpath_parser::blob("{{OiBTIKUgTyAASb8=}}"),
               Ok(Literal::Blob(vec![0x3a,0x20,0x53,0x20,0xa5,0x20,0x4f,0x20,0x00,0x49,0xbf])));
    assert_eq!(ionpath_parser::blob("{{  //79/PsAAQIDBAU=  }}"),
               Ok(Literal::Blob(vec![0xff,0xfe,0xfd,0xfc,0xfb,0x00,0x01,0x02,0x03,0x04,0x05])));
    assert_eq!(ionpath_parser::blob("						      {{ QSBWZXJ5IFZlcnkgVmVyeSBWZXJ5IExhcmdlIFRlc3QgQmxvYg== }}"),
               Ok(Literal::Blob(vec![0x41,0x20,0x56,0x65,0x72,0x79,0x20,0x56,0x65,0x72,0x79,0x20,0x56,
                                     0x65,0x72,0x79,0x20,0x56,0x65,0x72,0x79,0x20,0x4c,0x61,0x72,0x67,
                                     0x65,0x20,0x54,0x65,0x73,0x74,0x20,0x42,0x6c,0x6f,0x62])));
    assert_eq!(ionpath_parser::blob("{{\nA\n R E\nZ H i\n w 3 P\nE h R Y 2\n d 1 f Y u\nO n K W x t\n c b M 0 9 /\nv 9 v 8 A\n}}"),
               Ok(Literal::Blob(vec![0x01,0x11,0x19,0x1e,0x2c,0x37,0x3c,0x48,0x51,0x63,0x67,0x75,0x7d,
                                     0x8b,0x8e,0x9c,0xa5,0xb1,0xb5,0xc6,0xcc,0xd3,0xdf,0xef,0xf6,0xff,
                                     0x00])));
}

#[test]
fn test_blobs_negative() {
    for s in [
        "{{ 12345 }}",
        "{{ 'nonsens' }}",
        "{{ nonsense= }}",
        "{{ ==== }}",
        "{{ SSBhbSBhIGJsb2I== }}",
        "{{ YSBiIGMgZCBlIGYgZyBoIGkgaiBrI_GwgbSBuIG8gcCBxIHIgcyB0IHUgdiB3IHggeSB6 }}",
        "{{ YSBiIGMgZCBlIGYgZyBoIGkgaiBrI.GwgbSBuIG8gcCBxIHIgcyB0IHUgdiB3IHggeSB6 }}",
        "{{ .YSBiIGMgZCBlIGYgZyBoIGkgaiBrIGwgbSBuIG8gcCBxIHIgcyB0IHUgdiB3IHggeSB6 }}",
        "{{ _YSBiIGMgZCBlIGYgZyBoIGkgaiBrIGwgbSBuIG8gcCBxIHIgcyB0IHUgdiB3IHggeSB6 }}",
        "{{ YSBiIGMgZCBlIGYgZyBoIGkgaiBrIGwgbSBuIG8gcCBxIHIgcyB0IHUgdiB3IHggeSB6. }}",
        "{{ YSBiIGMgZCBlIGYgZyBoIGkgaiBrIGwgbSBuIG8gcCBxIHIgcyB0IHUgdiB3IHggeSB6_ }}",
        "{{ YSBiIGMgZCBlIGYgZyBoIGkgaiBrIGwgbSBuIG8gcCBxIHIgcyB0IHUgdiB3IHggeSB6=== }}",
        "{{aaaa}\n}",
        "{{aaaa}\\\n}"
    ] {
        assert!(ionpath_parser::blob(s).is_err());
    }
}

#[test]
fn test_clobs_positive() {
    assert_eq!(ionpath_parser::clob(r##"{{"a b c d e f g h i j k l m n o p q r s t u v w x y z"}}"##),
               Ok(Literal::Clob(vec![0x61,0x20,0x62,0x20,0x63,0x20,0x64,0x20,0x65,0x20,0x66,0x20,
                                     0x67,0x20,0x68,0x20,0x69,0x20,0x6a,0x20,0x6b,0x20,0x6c,0x20,
                                     0x6d,0x20,0x6e,0x20,0x6f,0x20,0x70,0x20,0x71,0x20,0x72,0x20,
                                     0x73,0x20,0x74,0x20,0x75,0x20,0x76,0x20,0x77,0x20,0x78,0x20,
                                     0x79,0x20,0x7a])));
    assert_eq!(ionpath_parser::clob(r##"{{
        "A B C D E F G H I J K L M N O P Q R S T U V W X Y Z"
}}"##),
               Ok(Literal::Clob(vec![0x41,0x20,0x42,0x20,0x43,0x20,0x44,0x20,0x45,0x20,0x46,0x20,
                                     0x47,0x20,0x48,0x20,0x49,0x20,0x4a,0x20,0x4b,0x20,0x4c,0x20,
                                     0x4d,0x20,0x4e,0x20,0x4f,0x20,0x50,0x20,0x51,0x20,0x52,0x20,
                                     0x53,0x20,0x54,0x20,0x55,0x20,0x56,0x20,0x57,0x20,0x58,0x20,
                                     0x59,0x20,0x5a])));
    assert_eq!(ionpath_parser::clob(r##"{{            "1 2 3 4 5 6 7 8 9 0"              }}"##),
               Ok(Literal::Clob(vec![0x31,0x20,0x32,0x20,0x33,0x20,0x34,0x20,0x35,0x20,0x36,0x20,
                                     0x37,0x20,0x38,0x20,0x39,0x20,0x30])));
    assert_eq!(ionpath_parser::clob(r##"{{   ", . ; / [ ' ] \\ = - 0 9 8 7 6 5 4 3 2 1 ` ~ ! @ # $ % ^ & * ( ) _ + | : < > ?"

}}"##),
               Ok(Literal::Clob(vec![0x2c,0x20,0x2e,0x20,0x3b,0x20,0x2f,0x20,0x5b,0x20,0x27,0x20,
                                     0x5d,0x20,0x5c,0x20,0x3d,0x20,0x2d,0x20,0x30,0x20,0x39,0x20,
                                     0x38,0x20,0x37,0x20,0x36,0x20,0x35,0x20,0x34,0x20,0x33,0x20,
                                     0x32,0x20,0x31,0x20,0x60,0x20,0x7e,0x20,0x21,0x20,0x40,0x20,
                                     0x23,0x20,0x24,0x20,0x25,0x20,0x5e,0x20,0x26,0x20,0x2a,0x20,
                                     0x28,0x20,0x29,0x20,0x5f,0x20,0x2b,0x20,0x7c,0x20,0x3a,0x20,
                                     0x3c,0x20,0x3e,0x20,0x3f])));
    assert_eq!(ionpath_parser::clob(r##"{{                   "\0 \a \b \t \n \f \r \v \" \' \? \\\\ \/ \0\a\b\t\n\f\r\v\"\'\?\\\\\/"}}"##),
               Ok(Literal::Clob(vec![0x00,0x20,0x07,0x20,0x08,0x20,0x09,0x20,0x0a,0x20,0x0c,0x20,
                                     0x0d,0x20,0x0b,0x20,0x22,0x20,0x27,0x20,0x3f,0x20,0x5c,0x5c,
                                     0x20,0x2f,0x20,0x00,0x07,0x08,0x09,0x0a,0x0c,0x0d,0x0b,0x22,
                                     0x27,0x3f,0x5c,0x5c,0x2f])));
    assert_eq!(ionpath_parser::clob(r##"{{"\x7f \x66 \x00 \x5a\x5b\x00\x1c\x2d\x3f\xFf"}}"##),
               Ok(Literal::Clob(vec![0x7f,0x20,0x66,0x20,0x00,0x20,0x5a,0x5b,0x00,0x1c,0x2d,0x3f,0xff])));
    assert_eq!(ionpath_parser::clob(r##"{{"\x7F \x66 \x00 \x5A\x5B\x00\x1C\x2D\x3F\xfF"}}"##),
               Ok(Literal::Clob(vec![0x7f,0x20,0x66,0x20,0x00,0x20,0x5a,0x5b,0x00,0x1c,0x2d,0x3f,0xff])));
    assert_eq!(ionpath_parser::clob(r##"{{'''Stuff to write on '''
  '''multiple lines '''
  '''if you want to'''}}"##),
               Ok(Literal::Clob(vec![0x53,0x74,0x75,0x66,0x66,0x20,0x74,0x6f,0x20,0x77,0x72,0x69,
                                     0x74,0x65,0x20,0x6f,0x6e,0x20,0x6d,0x75,0x6c,0x74,0x69,0x70,
                                     0x6c,0x65,0x20,0x6c,0x69,0x6e,0x65,0x73,0x20,0x69,0x66,0x20,
                                     0x79,0x6f,0x75,0x20,0x77,0x61,0x6e,0x74,0x20,0x74,0x6f])));
    assert_eq!(ionpath_parser::clob(r##"{{""}}"##),
               Ok(Literal::Clob(vec![])));
    assert_eq!(ionpath_parser::clob(r##"{{''''''}}"##),
               Ok(Literal::Clob(vec![])));
    assert_eq!(ionpath_parser::clob(r##"{{
""
}}"##),
               Ok(Literal::Clob(vec![])));
    assert_eq!(ionpath_parser::clob(r##"{{  '''concatenated'''  ''' from '''   '''a single line'''  }}"##),
               Ok(Literal::Clob(vec![0x63,0x6f,0x6e,0x63,0x61,0x74,0x65,0x6e,0x61,0x74,0x65,0x64,
                                     0x20,0x66,0x72,0x6f,0x6d,0x20,0x61,0x20,0x73,0x69,0x6e,0x67,
                                     0x6c,0x65,0x20,0x6c,0x69,0x6e,0x65])));
    assert_eq!(ionpath_parser::clob(r##"{{ ""}}"##),
               Ok(Literal::Clob(vec![])));
    assert_eq!(ionpath_parser::clob(r##"{{
        '''a b c d e f g h i j k l m n o p q r s t u v w x y z '''
        '''A B C D E F G H I J K L M N O P Q R S T U V W X Y Z '''
        ''', . ; / [ ' ] \\ = - 0 9 8 7 6 5 4 3 2 1 ` ~ ! @ # $ % ^ & * ( ) _ + | : < > ? '''
        '''\0 \a \b \t \n \f \r \v \" \' \? \\\\ \/ \0\a\b\t\n\f\r\v\"\'\?\\\\\/'''
        '''\x7f \x66 \x00 \x5a\x5b\x00\x1c\x2d\x3f'''
        '''\x7F \x66 \x00 \x5A\x5B\x00\x1C\x2D\x3F'''
}}"##),
               Ok(Literal::Clob(vec![0x61,0x20,0x62,0x20,0x63,0x20,0x64,0x20,0x65,0x20,0x66,0x20,
                                     0x67,0x20,0x68,0x20,0x69,0x20,0x6a,0x20,0x6b,0x20,0x6c,0x20,
                                     0x6d,0x20,0x6e,0x20,0x6f,0x20,0x70,0x20,0x71,0x20,0x72,0x20,
                                     0x73,0x20,0x74,0x20,0x75,0x20,0x76,0x20,0x77,0x20,0x78,0x20,
                                     0x79,0x20,0x7a,0x20,0x41,0x20,0x42,0x20,0x43,0x20,0x44,0x20,
                                     0x45,0x20,0x46,0x20,0x47,0x20,0x48,0x20,0x49,0x20,0x4a,0x20,
                                     0x4b,0x20,0x4c,0x20,0x4d,0x20,0x4e,0x20,0x4f,0x20,0x50,0x20,
                                     0x51,0x20,0x52,0x20,0x53,0x20,0x54,0x20,0x55,0x20,0x56,0x20,
                                     0x57,0x20,0x58,0x20,0x59,0x20,0x5a,0x20,0x2c,0x20,0x2e,0x20,
                                     0x3b,0x20,0x2f,0x20,0x5b,0x20,0x27,0x20,0x5d,0x20,0x5c,0x20,
                                     0x3d,0x20,0x2d,0x20,0x30,0x20,0x39,0x20,0x38,0x20,0x37,0x20,
                                     0x36,0x20,0x35,0x20,0x34,0x20,0x33,0x20,0x32,0x20,0x31,0x20,
                                     0x60,0x20,0x7e,0x20,0x21,0x20,0x40,0x20,0x23,0x20,0x24,0x20,
                                     0x25,0x20,0x5e,0x20,0x26,0x20,0x2a,0x20,0x28,0x20,0x29,0x20,
                                     0x5f,0x20,0x2b,0x20,0x7c,0x20,0x3a,0x20,0x3c,0x20,0x3e,0x20,
                                     0x3f,0x20,0x00,0x20,0x07,0x20,0x08,0x20,0x09,0x20,0x0a,0x20,
                                     0x0c,0x20,0x0d,0x20,0x0b,0x20,0x22,0x20,0x27,0x20,0x3f,0x20,
                                     0x5c,0x5c,0x20,0x2f,0x20,0x00,0x07,0x08,0x09,0x0a,0x0c,0x0d,
                                     0x0b,0x22,0x27,0x3f,0x5c,0x5c,0x2f,0x7f,0x20,0x66,0x20,0x00,
                                     0x20,0x5a,0x5b,0x00,0x1c,0x2d,0x3f,0x7f,0x20,0x66,0x20,0x00,
                                     0x20,0x5a,0x5b,0x00,0x1c,0x2d,0x3f])));
    assert_eq!(ionpath_parser::clob(r##"{{'''\
multi-line string
with embedded\nnew line
characters\
'''}}"##),
               Ok(Literal::Clob(vec![0x6d,0x75,0x6c,0x74,0x69,0x2d,0x6c,0x69,0x6e,0x65,0x20,0x73,
                                     0x74,0x72,0x69,0x6e,0x67,0x0a,0x77,0x69,0x74,0x68,0x20,0x65,
                                     0x6d,0x62,0x65,0x64,0x64,0x65,0x64,0x0a,0x6e,0x65,0x77,0x20,
                                     0x6c,0x69,0x6e,0x65,0x0a,0x63,0x68,0x61,0x72,0x61,0x63,0x74,
                                     0x65,0x72,0x73])));
}

#[test]
fn test_clobs_negative() {
    for s in [
        r#"{{ "\u3000" }}"#,
        r#"{{ "\877" }}"#,
        r#"{{"💩"}}"#,
        r#"{{
// hello
"world
}}"#,
        r#"{{
"hello"
// world
}}"#,
        r#"{{/*hello*/ "world"}}"#,
        r#"{{"hello" /*world*/}}"#,
        r#"{{'''
multiline clob with invalid ASCII
é
'''}}"#,
        r#"{{"€"}}"#,
        r#"{{ // hello
'''world'''}}"#,
        r#"{{ '''hello'''
// world  }}"#,
        r#"{{'''hello'''
/*world*/
// goodbye
'''moon'''
}}"#,
        r#"{{/*hello*/ '''world'''}}"#,
        r#"{{'''hello''' /*world*/}}"#,
        r#"{{ "\U00000080" }}"#,
        r#"{{ "\u0020" }}"#,
        r#"{{ "\U0000013F" }}"#,
        r#"{{ "\U0000003F" }}"#,
        r#"{{ "\u01FF" }}"#,
        r#"{{ "\u00FF" }}"#,
        r#"{{ 'one' }} "#,
        r#"{{ "one" 6 }}"#,
        r#"{{ "one" other stuff }}"#,
        r#"{{ "one" '''another'''}}"#,
        r#"{{ '''one''' "" '''another'''}}"#,
        r#"{{ '''one''' "" }}"#,
        r#"{{ "one" "another" }}"#,
    ] {
        assert!(ionpath_parser::clob(s).is_err());
    }
}

#[test]
fn literal_rule_returns_correct_type() {
    // TODO
}
