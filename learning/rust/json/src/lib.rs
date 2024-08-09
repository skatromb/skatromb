use std::collections::HashMap;
use std::ops::Index;

#[derive(Clone, Debug)]
pub enum Obj {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Object(HashMap<String, Obj>),
    Array(Vec<Obj>),
}

pub struct JSON {
    pub json: Obj,
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
    pub fn parse(string: String) -> Obj {
        for line in string.lines() {
        }
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    fn make_json() -> Obj {
        let mut map = HashMap::new();

        map.insert("null".to_string(), Obj::Null);
        map.insert("boolean".to_string(), Obj::Bool(true));
        map.insert("integer".to_string(), Obj::Int(1));
        map.insert("float".to_string(), Obj::Float(std::f64::consts::PI));
        map.insert("string".to_string(), Obj::String("Hello, world!".to_string()));

        let nested_map = HashMap::from([
            ("nested_object".to_string(), Obj::Object(map.clone()))
        ]);
        map.insert("map".to_string(), Obj::Object(nested_map));

        let array= map.clone().into_values().collect();
        map.insert("array".to_string(), Obj::Array(array));
        
        println!("{:?}", map);
        Obj::Object(map)
    }
    
    #[test]
    fn smoke_test() {
        let json = make_json();
        match json {
            Obj::Object(json) => {
                assert_eq!(json.len(), 7);
                assert_eq!(Obj::Null, json);
            } 
            _ => panic!("Test JSON should be JSON Object at the top level!"),
        }
    }
}
