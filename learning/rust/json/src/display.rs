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
        assert_eq!(JSON::Null.to_string(), "null");
    }

    #[test]
    fn json_bool_to_string() {
        assert_eq!(JSON::Bool(true).to_string(), "true");
        assert_eq!(JSON::Bool(false).to_string(), "false");
    }

    #[test]
    fn json_int_to_string() {
        assert_eq!(JSON::Int(123).to_string(), "123");
        assert_eq!(JSON::Int(-321).to_string(), "-321");
    }

    #[test]
    fn json_float_to_string() {
        assert_eq!(JSON::Float(123.45).to_string(), "123.45");
        assert_eq!(JSON::Float(-54.321).to_string(), "-54.321");
    }

    #[test]
    fn json_object_to_string() {
        // string: int
        let json_obj = JSON::Object(HashMap::from(
            [("int key".to_string(), JSON::Int(123))]
        ));
        assert_eq!(json_obj.to_string(), r#"{"int key": 123}"#);
        
        // string: string
        let json_obj = JSON::Object(HashMap::from(
            [("string key".to_string(), JSON::String("value".to_string()))]
        ));
        assert_eq!(json_obj.to_string(), r#"{"string key": "value"}"#);
        
        // nested
        let json_obj = JSON::Object(HashMap::from(
            [("nested".to_string(), JSON::Object(HashMap::from(
                [("string key".to_string(), JSON::String("value".to_string()))]
            )))]
        ));
        
        assert_eq!(json_obj.to_string(), r#"{"nested": {"string key": "value"}}"#);

    }
}
