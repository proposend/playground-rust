use std::collections::HashMap;

// Storing Keys with Associated Values in Hash Maps
fn main() {
    {
        // Creating a New Hash Map
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        println!("{:?}", scores);
    }
    {
        // Accessing Values in a Hash Map
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name).copied().unwrap_or(0);

        println!("{}", score);

        for (key, value) in &scores {
            println! {"{key}:{value}"};
        }
    }
    {
        // Hash Maps and Ownership
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);

        // println!("{}, {}", field_name, field_value);
    }
    {
        // Updating a Hash Map
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);

        println!("{:?}", scores);

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{:?}", scores);
    }
    {
        // Updating a Value Based on the Old Value
        let text = "hello world wonderful world";
        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map);
    }
}
