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

#[cfg(test)]
mod tests {
    use super::*;

    fn diverse_json() -> JSON {
        let mut map = HashMap::new();

        map.insert("null".to_string(), JSON::Null);
        map.insert("boolean".to_string(), JSON::Bool(true));
        map.insert("integer".to_string(), JSON::Int(123));
        map.insert("float".to_string(), JSON::Float(std::f64::consts::PI));
        map.insert(
            "string".to_string(),
            JSON::String("Hello, world!".to_string()),
        );

        let nested_map = HashMap::from([("nested_object".to_string(), JSON::Object(map.clone()))]);
        map.insert("object".to_string(), JSON::Object(nested_map));

        let array = map.clone().into_values().collect();
        map.insert("array".to_string(), JSON::Array(array));

        println!("{:?}", map);
        JSON::Object(map)
    }

    #[test]
    fn smoke_test() {
        let json = diverse_json();

        match json {
            JSON::Object(json) => {
                assert_eq!(json.len(), 7);
            }
            _ => panic!("Test JSON should be JSON Object at the top level!"),
        }
    }
}
