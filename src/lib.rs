use std::cmp::Ordering;
use std::collections::VecDeque;
use bigdecimal::BigDecimal;
use ion_rs::{Decimal, Int, IonType, Symbol, Timestamp};
use ion_rs::element::{Element, Sequence, Struct, Value};
use ion_rs::types::{Bytes, IntAccess};
use num::{BigInt, FromPrimitive, ToPrimitive};


#[cfg(test)]
mod tests;


pub mod parser;


#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    absolute: bool,
    segments: VecDeque<Segment>
}

impl Path {
    pub fn next(&mut self) -> Option<Segment> {
        self.segments.pop_front()
    }

    pub fn match_element(&self, root_element: Element) -> Vec<Element> {
        let mut path = self.clone();
        let mut context: Vec<Element> = vec![root_element];
        while let Some(seg) = path.next() {
            println!("[\n{}\n]", context.iter().map(|e| format!("    {}", e.to_string())).collect::<Vec<_>>().join("\n"));
            let mut next_context = Vec::new();
            for e in context.iter() {
                let mut result_set = seg.match_key(e);
                result_set.retain(|elem| seg.match_annotations(elem));
                for or_list in seg.predicate_lists.iter() {
                    result_set.retain(|e| {
                        for pred in or_list.iter() {
                            if pred.filter(e) { return true; }
                        }
                        false
                    });
                }
                next_context.append(&mut result_set);
            }
            context = next_context;
        }
        context
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
    recursive: bool,
    annotation_lists: Vec<Vec<String>>,
    key: Key,
    predicate_lists: Vec<Vec<Predicate>>,
}

impl Segment {
    pub fn match_annotations(&self, elem: &Element) -> bool {
        // must match ALL lists, where each list must match ANY annotation
        for ann_list in self.annotation_lists.iter() {
            let mut matched_option = false;
            for option in ann_list.iter() {
                if elem.annotations().contains(option) {
                    matched_option = true;
                    break;
                }
            }
            if !matched_option { return false; }
        }
        true
    }

    /// input: a single element of any type. may be a sequence, struct, or value.
    /// output: results of matching that element against this segment's key.
    ///         for sequences: child elements that match the key
    ///         for structs: values of fields that match the key
    ///         for values: never matches (TODO: does it ever?)
    pub fn match_key(&self, element: &Element) -> Vec<Element> {
        if let Some(sequence) = element.as_sequence() {
            self.match_sequence_against_key(sequence)
        }
        else if let Some(st) = element.as_struct() {
            self.match_struct_against_key(st)
        }
        else {
            Vec::new()
        }
    }

    fn match_sequence_against_key(&self, sequence: &Sequence) -> Vec<Element> {
        if sequence.is_empty() { return Vec::new(); }
        match &self.key {
            Key::Index(i) => {
                if let Some(mut small) = i.to_i32() {
                    while small < 0 {
                        small += sequence.len() as i32;
                    }
                    if let Some(e) = sequence.get(small as usize) {
                        return vec![e.clone()];
                    }
                }
            },
            Key::Slice(start, end, step) => {
                let mut start = start.unwrap_or(0);
                let mut end = end.unwrap_or(sequence.len() as i32 - 1);
                let step = step.unwrap_or(1);
                // sequence.len() is known to be > 0 at this point
                while start < 0 { start += sequence.len() as i32; }
                while end < 0 { end += sequence.len() as i32; }
                if start == end {
                    return match sequence.get(start as usize) {
                        Some(e) => vec![e.clone()],
                        None => vec![]
                    }
                }
                let mut results = Vec::new();
                if step > 0 {
                    if end > start {
                        let mut i = start;
                        while i <= end {
                            if let Some(elem) = sequence.get(i as usize) {
                                results.push(elem.clone());
                            }
                            i += step;
                        }
                    }
                    // else step positive & end < start -> set of indices is empty,
                    // fall through and return empty set
                }
                else if step < 0 {
                    if start > end {
                        let mut i = start;
                        while i >= end {
                            if let Some(elem) = sequence.get(i as usize) {
                                results.push(elem.clone());
                            }
                            i += /* negative */step;
                        }
                    }
                    // else step negative & start < end -> set of indices is empty,
                    // fall through and return empty set
                }
                // step == 0 -> empty set; return results without adding any elements
                return results;
            },
            Key::String(s) | Key::Symbol(s) => {
                if s.as_str() == "*" {
                    return sequence.elements().map(|r| r.clone()).collect();
                }
                // non-wildcard string keys never match sequence elements
            }
        }
        Vec::new()
    }

    fn match_struct_against_key(&self, st: &Struct) -> Vec<Element> {
        match &self.key {
            Key::String(s) | Key::Symbol(s) =>  {
                let mut results = Vec::new();
                for (key, val) in st.fields() {
                    if wildmatch::WildMatch::new(&s).matches(key.text().unwrap_or("$0")) {
                        results.push(val.clone());
                    }
                }
                results
            },
            _ => Vec::new()
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum Key {
    String(String),
    Symbol(String),
    Index(BigInt),
    Slice(Option<i32>,Option<i32>,Option<i32>),
}


#[derive(Debug, Clone, PartialEq)]
pub enum Predicate {
    Path(Box<Path>),
    Compare {
        path: Option<Box<Path>>,
        op: CompareOp,
        value: Literal
    },
}

impl Predicate {
    pub fn filter(&self, element: &Element) -> bool {
        match self {
            Predicate::Path(path) => {
                !path.match_element(element.clone()).is_empty()
            }
            Predicate::Compare { path, op , value } => {
                let subquery_res = match path {
                    Some(p) => p.match_element(element.clone()),
                    None => vec![element.clone()]
                };
                for sub in subquery_res {
                    let sub_lit: Literal = sub.try_into().unwrap();
                    match op {
                        CompareOp::Equal => {
                            if &sub_lit == value {
                                return true;
                            }
                        },
                        CompareOp::NotEqual => {
                            if &sub_lit != value {
                                return true;
                            }
                        },
                        CompareOp::LessThan => {
                            if &sub_lit < value {
                                return true;
                            }
                        },
                        CompareOp::GreaterThan => {
                            if &sub_lit > value {
                                return true;
                            }
                        },
                        CompareOp::LessOrEqual => {
                            if &sub_lit <= value {
                                return true;
                            }
                        },
                        CompareOp::GreaterOrEqual => {
                            if &sub_lit >= value {
                                return true;
                            }
                        },
                    }
                }
                false
            }
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum CompareOp {
    Equal, NotEqual, LessThan, GreaterThan, LessOrEqual, GreaterOrEqual
}


#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Boolean(bool),
    Integer(BigInt),
    Float(f64),
    Decimal(Decimal),
    String(String),
    Symbol(String),
    Null(IonType),
    Blob(Vec<u8>),
    Clob(Vec<u8>),
    Timestamp(Timestamp),
}

impl Into<Value> for Literal {
    fn into(self) -> Value {
        match self {
            Literal::Boolean(b) => Value::Bool(b),
            Literal::Integer(i) => Value::Int(Int::BigInt(i)),
            Literal::Float(f) => Value::Float(f),
            Literal::Decimal(d) => Value::Decimal(d.clone()),
            Literal::String(s) => Value::String(s.into()),
            Literal::Symbol(s) => Value::Symbol(Symbol::from(s.as_str())),
            Literal::Null(ty) => Value::Null(ty),
            Literal::Blob(b) => Value::Blob(Bytes::from(b)),
            Literal::Clob(b) => Value::Clob(Bytes::from(b)),
            Literal::Timestamp(ts) => Value::Timestamp(ts)
        }
    }
}

impl TryFrom<Element> for Literal {
    type Error = ();

    fn try_from(value: Element) -> Result<Self, ()> {
        Literal::try_from(value.value().clone())
    }
}

impl TryFrom<Value> for Literal {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, ()> {
        match value {
            Value::Null(ty) => Ok(Literal::Null(ty)),
            Value::Bool(b) => Ok(Literal::Boolean(b)),
            Value::Int(i) => Ok(Literal::Integer(i.as_big_int().map(|r| r.clone()).unwrap_or(BigInt::from(i.as_i64().unwrap())))),
            Value::Float(f) => Ok(Literal::Float(f)),
            Value::Decimal(d) => Ok(Literal::Decimal(d)),
            Value::Timestamp(ts) => Ok(Literal::Timestamp(ts)),
            Value::Symbol(s) => Ok(Literal::Symbol(s.text().unwrap_or("$0").to_string())),
            Value::String(s) => Ok(Literal::String(s.text().to_string())),
            Value::Clob(b) => Ok(Literal::Clob(Vec::from(b.as_ref()))),
            Value::Blob(b) => Ok(Literal::Blob(Vec::from(b.as_ref()))),
            _ => Err(()),
        }
    }
}

impl PartialEq<Element> for Literal {
    fn eq(&self, other: &Element) -> bool {
        match self {
            Literal::Boolean(b) => other.as_bool().map(|b2| *b == b2) == Some(true),
            Literal::Integer(i) => other.as_int().map(|int| int.as_big_int().map(|r| r.clone()).unwrap_or(BigInt::from(int.as_i64().unwrap())) == *i) == Some(true),
            Literal::Float(f) => other.as_float().map(|f2| *f == f2) == Some(true),
            Literal::Decimal(d) => other.as_decimal().map(|d2| d == d2) == Some(true),
            Literal::String(s) => other.as_string().map(|s2| s.as_str() == s2) == Some(true),
            Literal::Symbol(s) => other.as_symbol().map(|sym| s.as_str() == sym.text().unwrap_or("$0")) == Some(true),
            // true if both have types that match, or if either one is of unspecified type
            Literal::Null(ty) => other.is_null()
                && (other.value().ion_type() == *ty
                || *ty == IonType::Null
                || other.value().ion_type() == IonType::Null),
            Literal::Blob(bytes) => other.as_blob().map(|bytes2| &bytes[..] == bytes2) == Some(true),
            Literal::Clob(bytes) => other.as_clob().map(|bytes2| &bytes[..] == bytes2) == Some(true),
            Literal::Timestamp(ts) => other.as_timestamp().map(|ts2| ts == ts2) == Some(true),
        }
    }
}

impl PartialOrd<Literal> for Literal {
    fn partial_cmp(&self, other: &Literal) -> Option<Ordering> {
        match self {
            Literal::Boolean(_) | Literal::Integer(_) | Literal::Float(_) | Literal::Decimal(_)=> {
                let value = if let Literal::Boolean(b) = self {
                    BigDecimal::from_f64(if *b { 1.0 } else { 0.0 }).unwrap()
                }
                else if let Literal::Integer(i) = self {
                    BigDecimal::from(i.clone())
                }
                else if let Literal::Float(f) = self {
                    match BigDecimal::from_f64(*f) {
                        Some(dec) => dec,
                        None => return None
                    }
                }
                else if let Literal::Decimal(d) = self {
                    // should only fail on negative zero, convert to positive zero in that case
                    BigDecimal::try_from(d.clone()).unwrap_or(BigDecimal::from(0))
                }
                else { unreachable!() };

                let other_value = match other {
                    Literal::Boolean(b) => BigDecimal::from_f64(if *b { 1.0 } else { 0.0 }).unwrap(),
                    Literal::Integer(i) => BigDecimal::from(i.clone()),
                    Literal::Float(f) => match BigDecimal::from_f64(*f) {
                        Some(dec) => dec,
                        None => return None
                    }
                    Literal::Decimal(d) => BigDecimal::try_from(d.clone()).unwrap_or(BigDecimal::from(0)),
                    _ => return None
                };
                Some(value.cmp(&other_value))
            }
            Literal::String(s) => {
                if let Literal::String(s2) = other {
                    Some(s.cmp(s2))
                }
                else { None }
            }
            Literal::Symbol(s) => {
                if let Literal::Symbol(s2) = other {
                    Some(s.cmp(s2))
                }
                else { None }
            }
            Literal::Null(_) => None,
            Literal::Blob(b) | Literal::Clob(b) => {
                match other {
                    Literal::Blob(b2) | Literal::Clob(b2) => {
                        Some(b.cmp(b2))
                    },
                    _ => None
                }
            },
            Literal::Timestamp(ts) => {
                if let Literal::Timestamp(ts2) = other {
                    Some(ts.cmp(ts2))
                }
                else { None }
            }
        }
    }
}

impl PartialOrd<Element> for Literal {
    fn partial_cmp(&self, other: &Element) -> Option<Ordering> {
        let as_lit: Literal = (other.value().clone()).try_into().unwrap();
        self.partial_cmp(&as_lit)
    }
}