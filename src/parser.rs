use std::collections::VecDeque;
use regex::Regex;
use num::{BigInt, Num};
use base64::Engine;
use ion_rs::IonType;
use super::{Path, Segment, Key, Literal, Predicate, CompareOp};

fn unescape(s: &str) -> String {
    let string = s.replace(r"\0","\0")
        .replace(r"\r","\r")
        .replace(r"\n","\n")
        .replace(r"\t","\t")
        .replace(r"\'","'")
        .replace(r#"\""#,"\"")
        .replace(r"\a","\x07")
        .replace(r"\b","\x08")
        .replace(r"\v","\x0B")
        .replace(r"\f","\x0C")
        .replace(r"\?","?")
        .replace(r"\/","/")
        .replace(r"\\","\\")
        .replace("\\\r", "")
        .replace("\\\n", "");
    let re = Regex::new(r#"(\\x(?<h1>[0-9a-fA-F]{2}))|(\\u(?<h2>[0-9a-fA-F]{4}))|(\\U000(?<h3>[0-9a-fA-F]{6}))|(\\U0010(?<h4>[0-9a-fA-F]{4}))"#).unwrap();
    re.replace_all(&string, |caps: &regex::Captures<'_>| {
        let codepoint;
        if let Some(mat) = caps.name("h1") {
            codepoint = u32::from_str_radix(mat.as_str(), 16).unwrap();
        }
        else if let Some(mat) = caps.name("h2") {
            codepoint = u32::from_str_radix(mat.as_str(), 16).unwrap();
        }
        else if let Some(mat) = caps.name("h3") {
            codepoint = u32::from_str_radix(mat.as_str(), 16).unwrap();
        }
        else if let Some(mat) = caps.name("h4") {
            codepoint = u32::from_str_radix(mat.as_str(), 16).unwrap();
        }
        else {
            unreachable!();
        }
        let mut result = String::new();
        result.push(char::from_u32(codepoint).unwrap());
        result
    }).to_string()
}

peg::parser!{
    pub grammar ionpath_parser() for str {
        rule ws() = quiet!{[' ' | '\t' | '\x0B' | '\x0C' | '\r' | '\n']*}

        // nulls

        rule typename() -> &'input str
            = $("null" / "bool" / "int" / "float" / "decimal" / "timestamp"
              / "string" / "symbol" / "blob" / "clob" / "struct" / "list" / "sexp")

        rule null_type() -> IonType = "." ty:typename()
        {
            match ty {
                "null" => IonType::Null,
                "bool" => IonType::Bool,
                "int" => IonType::Int,
                "float" => IonType::Float,
                "decimal" => IonType::Decimal,
                "timestamp" => IonType::Timestamp,
                "string" => IonType::String,
                "symbol" => IonType::Symbol,
                "blob" => IonType::Blob,
                "clob" => IonType::Clob,
                "struct" => IonType::Struct,
                "list" => IonType::List,
                "sexp" => IonType::SExp,
                _ => unreachable!()
            }
        }

        pub rule null() -> Literal
            = "null" ty:null_type()? { Literal::Null(ty.unwrap_or(IonType::Null)) }

        // bool

        pub rule boolean() -> Literal
            = s:$("true" / "false") { Literal::Boolean(s == "true") }

        // numerics

        rule decimal_unsigned_int() -> &'input str
            = $("0") / $(['1'..='9'] ("_"? ['0'..='9'])*)

        rule decimal_frac() -> &'input str
            = $("." (['0'..='9'] ("_"? ['0'..='9'])*)?)

        rule integer_b10() -> BigInt
            = n:$("-"? decimal_unsigned_int()) {? n.parse().or(Err("integer")) }

        rule hex_digit() -> char = ['0'..='9' | 'a'..='f' | 'A'..='F']

        rule integer_b16() -> BigInt
            = n:$("-"? ("0x" / "0X") hex_digit()+ ("_"? hex_digit())*)
        {?
            BigInt::from_str_radix(&n.replace("_", "").replace("0x","").replace("0X",""), 16)
                .or(Err("valid hexadecimal digit"))
        }

        rule integer_b2() -> BigInt
            = n:$("-"? ("0b" / "0B") ['0'..='1']+ ("_"? ['0'..='1'])*)
        {?
            BigInt::from_str_radix(&n.replace("_", "").replace("0b","").replace("0B",""), 2)
                .or(Err("valid binary digit"))
        }

        pub rule integer() -> Literal = i:(integer_b16() / integer_b2() / integer_b10()) {
            Literal::Integer(i)
        }

        rule float_exp() -> &'input str
            = $(("E" / "e") ("+" / "-")? ['0'..='9']+)

        rule decimal_exp() -> &'input str
            = $(("D" / "d") ("+" / "-")? ['0'..='9']+)

        pub rule float() -> Literal
            = s:($("-"? decimal_unsigned_int() decimal_frac()? float_exp()) / $(("+" / "-")? "inf") / $("nan") / $("NaN"))
        {?
            s.replace("_", "").parse().map(|f| Literal::Float(f)).map_err(|_| "float")
        }

        pub rule decimal() -> Literal
            = s:($("-"? decimal_unsigned_int() decimal_frac()? decimal_exp()?) / $(("+" / "-")? "inf") / $("nan") / $("NaN"))
        {?
            match ion_rs::element::Element::read_one(s.replace("_", "")) {
                Ok(e) => e.as_decimal().map(|d| Literal::Decimal(d.clone())).ok_or("decimal"),
                Err(_) => Err("decimal")
            }
        }

        // strings

        rule symbol_text_allowed() -> &'input str
            = $(['\x20'..='\x26' | '\x28'..='\x5B' | '\x5D'..='\u{FFFF}' | ' ' | '\t' | '\x0B' | '\x0C']+)

        rule unicode_escape() -> &'input str
            = $("\\x" hex_digit() hex_digit())
              / $("\\u" hex_digit() hex_digit() hex_digit() hex_digit())
              / $("\\U000" hex_digit() hex_digit() hex_digit() hex_digit() hex_digit() hex_digit())
              / $("\\U0010" hex_digit() hex_digit() hex_digit() hex_digit())

        rule escape_seq() -> &'input str
            = $("\\a" / "\\b" / "\\t" / "\\n" / "\\f" / "\\r" / "\\v" / "\\0" / "\\?" / "\\\"" / "\\'" / "\\/" / "\\\\")

        rule quoted_symbol() -> Literal
            = s:$("'" (symbol_text_allowed() / unicode_escape() / escape_seq())* "'")
        {
            Literal::Symbol(unescape(&s[1..(s.len()-1)]))
        }

        rule ident_symbol() -> Literal
            = s:$(['*' | '$' | '_' | 'a'..='z' | 'A'..='Z'] ['$' | '_' | 'a'..='z' | 'A'..='Z' | '0'..='9']*)
        {
            Literal::Symbol(s.to_string())
        }

        pub rule symbol() -> Literal
            = quoted_symbol() / ident_symbol()

        rule string_text_allowed() -> &'input str
            = $(['\x20'..='\x21' | '\x23'..='\x5B' | '\x5D'..='\u{FFFF}' | ' ' | '\t' | '\x0B' | '\x0C']+)

        rule quoted_string() -> String
            = s:$("\"" (unicode_escape() / escape_seq() / string_text_allowed())* "\"")
        {
            s[1..(s.len()-1)].to_string()
        }

        rule string_long_text_allowed_char() -> char
            = ['\x20'..='\x26' | '\x28'..='\x5B' | '\x5D'..='\u{FFFF}' | ' ' | '\t' | '\x0B' | '\x0C' | '\r' | '\n']

        rule string_long_text_allowed() -> String
            = s:$((string_long_text_allowed_char()+) / ("'" string_long_text_allowed_char()+) / ("''" string_long_text_allowed_char()+)) { s.to_string() }

        rule long_quoted_string_single() -> String
            = ws() s:$("'''" (unicode_escape() / escape_seq() / string_long_text_allowed())* "'''") ws() { s.to_string() }

        rule long_quoted_string() -> String
            = ss:(long_quoted_string_single()+)
        {
            let string = ss.join("");
            let string = string.replace("''''''", "");
            if string.is_empty() || string.as_str() == "''''''" {
                "".to_string()
            }
            else {
                assert!(string.starts_with("'''"));
                assert!(string.ends_with("'''"));
                string[3..(string.len()-3)].to_string()
            }
        }

        pub rule string() -> Literal
            = s:(long_quoted_string() / quoted_string())
        {
            Literal::String(unescape(&s))
        }

        // blob

        pub rule blob() -> Literal
            = s:$(ws() "{{" ws() base64_quartet()* base64_pad()? ws() "}}" ws())
        {?
            let string = s.replace(&[' ', '\t', '\r', '\n', '\x0B', '\x0C'], "").replace("{{", "").replace("}}", "");
            base64::engine::general_purpose::STANDARD.decode(&string)
                .map(|b| Literal::Blob(b))
                .or(Err("valid base64"))
        }

        rule base64_pad() -> &'input str = base64_pad1() / base64_pad2()

        rule base64_pad1() -> &'input str = $(b64_char() ws() b64_char() ws() b64_char() ws() "=")
        rule base64_pad2() -> &'input str = $(b64_char() ws() b64_char() ws() "=" ws() "=")

        rule base64_quartet() -> &'input str = $(b64_char() ws() b64_char() ws() b64_char() ws() b64_char() ws())

        rule b64_char() -> char = ['0'..='9' | 'a'..='z' | 'A'..='Z' | '+' | '/']

        // clob

        pub rule clob() -> Literal = s:(short_quoted_clob() / long_quoted_clob()) {
            let string = unescape(&s);
            Literal::Clob(string.encode_utf16().map(|long| (long & 0xFF) as u8).collect())
        }

        rule short_quoted_clob() -> String
            = s:$("{{" ws() "\"" ($(escape_seq()) / $("\\x" hex_digit() hex_digit()) / $(clob_short_text_allowed()))* "\"" ws() "}}")
        {
            let mut string = String::new();
            let mut in_string = false;
            let mut prev_char = '\0';
            for c in s.chars() {
                if c == '"' && prev_char != '\\' { in_string = !in_string; }
                if in_string || ![' ', '\t', '\r', '\n', '\x0B', '\x0C'].contains(&c) {
                    string.push(c);
                }
                prev_char = c;
            }
            assert!(!in_string);
            assert!(string.starts_with("{{\""));
            assert!(string.ends_with("\"}}"));
            string[3..(string.len()-3)].to_string()
        }

        rule long_quoted_clob() -> String
            = s:$("{{" (ws() "'''" clob_long_text()* "'''")+ ws() "}}")
        {
            let mut string = String::new();
            let mut quote_count = 0;
            let mut in_string = false;
            for c in s.chars() {
                if c == '\'' { quote_count += 1; }
                else { quote_count = 0; }
                if quote_count == 3 {
                    in_string = !in_string;
                    quote_count = 0;
                }
                if in_string || ![' ', '\t', '\r', '\n', '\x0B', '\x0C'].contains(&c) {
                    string.push(c);
                }
            }
            assert!(!in_string);
            let string = string.replace("''''''", "");
            if string.as_str() != "{{}}" {
                assert!(string.starts_with("{{'''"));
                assert!(string.ends_with("'''}}"));
                string[5..(string.len()-5)].to_string()
            }
            else {
                string[2..(string.len()-2)].to_string()
            }
        }

        rule clob_long_text_no_quote() -> &'input str
            = $($(clob_long_text_allowed()) / $("\\\n") / $(escape_seq()) / $("\\x" hex_digit() hex_digit()))

        rule clob_long_text() -> &'input str
            = $(clob_long_text_no_quote() / ("'" clob_long_text_no_quote()) / ("''" clob_long_text_no_quote()))

        rule clob_short_text_allowed() -> &'input str
            = $(['\x20'..='\x21' | '\x23'..='\x5B' | '\x5D'..='\x7F' | '\t']+)

        rule clob_long_text_allowed() -> &'input str
            = $(['\x20'..='\x26' | '\x28'..='\x5B' | '\x5D'..='\x7F' | '\t' | '\r' | '\n']+)

        // timestamps

        pub rule timestamp() -> Literal
            = s:$((ts_date() ("T" ts_time()?)?)
              / (ts_year() "-" ts_month() "T")
              / (ts_year() "T"))
        {?
            match ion_rs::element::Element::read_one(s) {
                Ok(e) => e.as_timestamp().map(|ts| Literal::Timestamp(ts.clone())).ok_or("timestamp"),
                Err(_) => Err("timestamp")
            }
        }

        rule ts_date() -> &'input str
            = $(ts_year() "-" ts_month() "-" ts_day())

        rule ts_year() -> &'input str
            = $("000" ['1'..='9'])
            / $("00" ['1'..='9'] ['0'..='9'])
            / $("0" ['1'..='9'] ['0'..='9'] ['0'..='9'])
            / $(['1'..='9'] ['0'..='9'] ['0'..='9'] ['0'..='9'])

        rule ts_month() -> &'input str
            = $("0" ['1'..='9']) / $("1" ['0'..='2'])

        rule ts_day() -> &'input str
            = $("0" ['1'..='9']) / $(['1'..='2'] ['0'..='9']) / $("3" ['0'..='1'])

        rule ts_time() -> &'input str
            = $(ts_hour() ":" ts_minute() (":" ts_second())? ts_offset())

        rule ts_hour() -> &'input str
            = $(['0'..='1'] ['0'..='9']) / $("2" ['0'..='3'])

        rule ts_minute() -> &'input str
            = $(['0'..='5'] ['0'..='9'])

        rule ts_second() -> &'input str
            = $(['0'..='5'] ['0'..='9'] ("." ['0'..='9']+)?)

        rule ts_offset() -> &'input str
            = $("Z") / $(("+" / "-") ts_hour() ":" ts_minute())

        // literal rule

        pub rule literal() -> Literal
            = (timestamp() / string() / null() / float() / decimal() / integer() / boolean() / symbol() / clob() / blob())

        // path syntax rules

        rule slice_step() -> i32
            = ":" ws() int:$("-"? decimal_unsigned_int()) ws() {? int.parse().or(Err("integer")) }

        rule slice_open_start() -> (Option<i32>, Option<i32>)
            = ws() ":" ws() int:$("-"? decimal_unsigned_int()) ws()
        {?
            match int.parse() {
                Ok(i) => Ok((None, Some(i))),
                Err(_) => Err("integer")
            }
        }

        rule slice_closed_start() -> (Option<i32>, Option<i32>)
            = a:$("-"? decimal_unsigned_int()) ws() ":" ws() b:$("-"? decimal_unsigned_int())? ws()
        {?
            if let Ok(ia) = a.parse() {
                if let Some(b) = b {
                    match b.parse() {
                        Ok(ib) => Ok((Some(ia), Some(ib))),
                        Err(_) => Err("integer"),
                    }
                }
                else {
                    Ok((Some(ia), None))
                }
            }
            else {
                Err("integer")
            }
        }

        rule key_slice() -> Key
            = slice:(slice_open_start() / slice_closed_start()) step:slice_step()?
        {
            let (a, b) = slice;
            Key::Slice(a, b, step)
        }

        rule key_literal() -> Key
            = lit:(symbol() / string() / integer())
        {
            match lit {
                Literal::Symbol(s) => Key::Symbol(s),
                Literal::String(s) => Key::String(s),
                Literal::Integer(i) => Key::Index(i),
                _ => unreachable!()
            }
        }

        rule key() -> Key = key_slice() / key_literal()

        rule cmp() -> CompareOp
            = s:$("==" / "=" / "!=" / ">=" / "<=" / ">" / "<")
        {
            match s {
                "==" | "=" => CompareOp::Equal,
                "!=" => CompareOp::NotEqual,
                ">=" => CompareOp::GreaterOrEqual,
                "<=" => CompareOp::LessOrEqual,
                ">" => CompareOp::GreaterThan,
                "<" => CompareOp::LessThan,
                _ => unreachable!(),
            }
        }

        rule annotation_single() -> Vec<String>
            = sym:(symbol() / string()) ws() "::" ws()
        {
            vec![match sym {
                Literal::Symbol(s) => s,
                Literal::String(s) => s,
                _ => unreachable!()
            }]
        }

        rule annotation_choice_list() -> Vec<String>
            = "(" ws() first:(symbol() / string()) rest:(annotation_choice()*) ws() ")" ws() "::" ws()
        {
            let mut results = Vec::new();
            results.push(match first {
                Literal::Symbol(s) => s,
                Literal::String(s) => s,
                _ => unreachable!()
            });
            for sy in rest {
                results.push(sy);
            }
            results
        }

        rule annotation_choice() -> String
            = ws() "|" ws() sy:(symbol() / string())
        {
            match sy {
                Literal::Symbol(s) => s,
                Literal::String(s) => s,
                _ => unreachable!()
            }
        }

        rule predicate_OR_list() -> Vec<Predicate>
            = ws() "[" first:(pred_cmp() / pred_single_path()) rest:(or_predicate())* "]"
        {
            let mut all = vec![first];
            for p in rest {
                all.push(p);
            }
            all
        }

        rule pred_single_path() -> Predicate =  ws() p:path()  {
            Predicate::Path(Box::new(p))
        }

        rule pred_cmp() -> Predicate = ws() p:path()? ws() c:cmp()  ws() l:literal() {
            Predicate::Compare {
                path: p.map(|p| Box::new(p)),
                op: c,
                value: l
            }
        }

        rule or_predicate() -> Predicate
            = ws() ("or"/"OR"/"oR"/"Or") ws() p:(pred_cmp() / pred_single_path())  { p }

        rule first_segment() -> (Segment, bool /* is_absolute */)
            = first:"/"? second:"/"? ws() annotation_lists:(annotation_choice_list() / annotation_single())* k:key() pred_lists:(predicate_OR_list()*)
        {
            (Segment {
                recursive: first.is_some() && second.is_some(),
                annotation_lists,
                key: k,
                predicate_lists: pred_lists,
            }, first.is_some())
        }

        rule other_segment() -> (Segment, bool /* is_absolute */)
            = "/" second:"/"? ws() annotation_lists:(annotation_choice_list() / annotation_single())* k:key() pred_lists:(predicate_OR_list()*)
        {
            (Segment {
                recursive: second.is_some(),
                annotation_lists,
                key: k,
                predicate_lists: pred_lists,
            }, true)
        }

        pub rule path() -> Path = first:first_segment() rest:(other_segment()*) {
            let mut segments = VecDeque::new();
            segments.push_back(first.0);
            for seg in rest {
                segments.push_back(seg.0);
            }
            Path { absolute: first.1, segments }
        }
    }
}