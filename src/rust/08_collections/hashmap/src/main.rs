use std::collections::HashMap;

fn main() {
    {
        println!("Hello, world!");
        let mut scores = HashMap::new();

        scores.insert("Blue".to_string(), 10);

        scores.entry("Yellow".to_string()).or_insert(50);
        scores.entry("Blue".to_string()).or_insert(25);

        let team_name = "Blue".to_string();
        // let score = scores.get(&team_name).copied().unwrap_or(0);
        let score = scores.get(&team_name).copied().unwrap_or(0);

        println!("{:?}", scores);
        for (key, value) in &scores {
            println!("{key}: {value}");
        }
    }

    {
        let mut map = HashMap::new();
        let key = "Favorite color".to_string();
        let value =  "blue".to_string();
        map.insert(key, value);
        println!("{:?}", map); // forbidden
    }
    
    let text = "hello world wonderful world";
    
    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;   
    }

    println!("{:?}", map);

}
