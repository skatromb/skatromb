use std::collections::HashMap;

#[macro_export]
macro_rules! hash_map {
    ( $( $key:expr => $value:expr ),* $(,)? ) => ({
        let mut hash_map = HashMap::new();

        $(
            hash_map.insert($key, $value);
        )*

        hash_map
    });
}

fn main() {
    let my_map = hash_map! {
        "key" =>  hash_map!["nested" => "nested_val"],
        "another_key" =>  hash_map!["nested_2" => "nested_val_2"],
    };

    println!("{:#?}", my_map);

    let empty_map: HashMap<&str, &str> = hash_map![];

    println!("Empty map: {empty_map:?}");
}
