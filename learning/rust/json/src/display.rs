use crate::JSON;
use std::fmt;
use std::fmt::Display;

impl Display for JSON {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JSON::Null => write!(f, "null"),
            JSON::Bool(bool) => write!(f, "{}", bool.to_string()),
            JSON::Int(int) => write!(f, "{}", int.to_string()),
            JSON::Float(float) => write!(f, "{}", float.to_string()),
            JSON::String(s) => write!(f, r#""{}""#, s),

            JSON::Object(map) => {
                
                let comma_separated = map.iter()
                    .map(|(key, value)| format!(r#""{key}": {value}"#))
                    .collect::<Vec<_>>()
                    .join(", ");
                
                write!(f, "{{{comma_separated}}}")
            }

            JSON::Array(jsons) => {

                let comma_separated = jsons.iter()
                    .map(|json| json.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                
                write!(f, "[{comma_separated}]")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::JSON;
    use std::collections::HashMap;

    #[test]
    fn json_null_to_string() {
        let json = JSON::Null;

        assert_eq!(json.to_string(), "null");
    }

    #[test]
    fn json_bool_to_string() {
        let json_true = JSON::Bool(true);
        let json_false = JSON::Bool(false);

        assert_eq!(json_true.to_string(), "true");
        assert_eq!(json_false.to_string(), "false");
    }

    #[test]
    fn json_int_to_string() {
        let json_int = JSON::Int(123);
        let json_neg_int = JSON::Int(-321);

        assert_eq!(json_int.to_string(), "123");
        assert_eq!(json_neg_int.to_string(), "-321");
    }

    #[test]
    fn json_float_to_string() {
        let json_float = JSON::Float(-123.45);

        assert_eq!(json_float.to_string(), "-123.45");
    }

    #[test]
    fn json_object_to_string() {
        // string: int
        let mut map = HashMap::new();
        map.insert("int key".to_string(), JSON::Int(123));

        let json_obj = JSON::Object(map);

        assert_eq!(json_obj.to_string(), r#"{"int key": 123}"#);
        
        // string: string
        let mut map = HashMap::new();
        map.insert("string key".to_string(), JSON::String("value".to_string()));

        let json_obj = JSON::Object(map);

        assert_eq!(json_obj.to_string(), r#"{"string key": "value"}"#);
        
        // nested
        let mut map = HashMap::new();
        map.insert("string key".to_string(), JSON::String("value".to_string()));

        let mut nested_map = HashMap::new();
        nested_map.insert("nested".to_string(), JSON::Object(map));
        
        let json_obj = JSON::Object(nested_map);
        
        assert_eq!(json_obj.to_string(), r#"{"nested": {"string key": "value"}}"#);

    }
}
