mod display;
mod from_str;

use std::collections::HashMap;
use std::ops::Index;

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

/// Implement straightforward and inconvenient `Index` first.
/// It would panic if a value is absent, so that's not what you want usually.
impl Index<&str> for JSON {
    type Output = JSON;

    fn index(&self, key: &str) -> &Self::Output {
        match self {
            JSON::Object(map) => &map[key],
            _ => panic!(),
        }
    }
}

impl Index<usize> for JSON {
    type Output = JSON;

    fn index(&self, key: usize) -> &Self::Output {
        match self {
            JSON::Array(array) => &array[key],
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_map_happy() {
        let map = HashMap::from([
            ("key1".to_string(), JSON::Int(10)),
            ("key2".to_string(), JSON::Bool(true)),
        ]);
        let json = JSON::Object(map);

        assert_eq!(json["key1"], JSON::Int(10));
        assert_eq!(json["key2"], JSON::Bool(true));
    }

    #[test]
    fn index_array_happy() {
        let array = vec![
            JSON::Int(10),
            JSON::Bool(true),
            JSON::String("test".to_string()),
        ];
        let json = JSON::Array(array);

        assert_eq!(json[0], JSON::Int(10));
        assert_eq!(json[1], JSON::Bool(true));
        assert_eq!(json[2], JSON::String("test".to_string()));
    }

    #[test]
    #[should_panic]
    fn index_map_panic() {
        let map = HashMap::from([("key1".to_string(), JSON::Int(10))]);
        let json = JSON::Object(map);

        let _ = json["nonexistent_key"];
    }

    #[test]
    #[should_panic]
    fn index_array_panic() {
        let array = vec![JSON::Int(10)];
        let json = JSON::Array(array);

        let _ = json[10];
    }
}
