use std::collections::HashMap;
use std::ops::Index;

#[derive(Clone)]
pub enum Object {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Object(HashMap<String, Object>),
    Array(Vec<Object>),
}

pub struct JSON {
    pub json: Object,
}

// impl Index<Object> for JSON {
//     type Output = Object;
// 
//     fn index(&self, index: Object) -> &Self::Output {
//         
//     }
//     fn 
// }

impl JSON {
    pub fn parse(string: String) -> Object {
        for line in string.lines() {

        }
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_json() -> Object {
        let mut map = HashMap::new();

        map.insert("null".to_string(), Object::Null);
        map.insert("boolean".to_string(), Object::Bool(true));
        map.insert("integer".to_string(), Object::Int(1));
        map.insert("float".to_string(), Object::Float(std::f64::consts::PI));
        map.insert("string".to_string(), Object::String("Hello, world!".to_string()));

        let nested_map = HashMap::from([
            ("nested_object".to_string(), Object::Object(map.clone()))
        ]);
        map.insert("map".to_string(), Object::Object(nested_map));

        let array= map.clone().into_values().collect();
        map.insert("array".to_string(), Object::Array(array));

        Object::Object(map)
    }
    
    #[test]
    fn smoke_test() {
        let json = make_json();
        match json {
            Object::Object(x) => {
                print!("{:?}", x.len());
                assert_eq!(x.len(), 7);
            },
            _ => panic!("Test JSON should be JSON Object at the top level!"),
        }
    }
}
