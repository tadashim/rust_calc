use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Alice"), 10);
    scores.insert(String::from("Bob"), 20);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
