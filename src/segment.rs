use ion_rs::element::{Element, Sequence, Struct};
use num::ToPrimitive;
use crate::{Key, Predicate};


#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
    pub recursive: bool,
    pub annotation_lists: Vec<Vec<String>>,
    pub key: Key,
    pub predicate_lists: Vec<Vec<Predicate>>,
}

impl Segment {
    pub fn new(recursive: bool, key: Key) -> Self {
        Segment {
            recursive,
            annotation_lists: Vec::new(),
            key,
            predicate_lists: Vec::new(),
        }
    }

    pub fn with_annotation_list(mut self, list: Vec<String>) -> Self {
        self.annotation_lists.push(list);
        self
    }

    pub fn with_predicate_list(mut self, list: Vec<Predicate>) -> Self {
        self.predicate_lists.push(list);
        self
    }

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
