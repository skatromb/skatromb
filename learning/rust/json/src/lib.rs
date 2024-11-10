mod display;
mod from_str;

use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum JSON {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Object(HashMap<String, JSON>),
    Array(Vec<JSON>),
}
