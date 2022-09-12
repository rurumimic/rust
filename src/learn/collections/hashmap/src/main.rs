use std::collections::HashMap; // SipHash

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    if let Some(score) = score {
        println!("{}: {}", &team_name, score);
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    println!("{:?}", scores); // {"Blue": 25, "Yellow": 50}

    let mut map = HashMap::new();
    {
        let field_name = String::from("Count");
        let field_value = 3;
        map.insert(&field_name, field_value); // borrowed value does not live long enough
    }

    let mut map = HashMap::new();
    let field_name = String::from("Count");
    let field_value = 3;
    map.insert(field_name, field_value); // String moved here, i32 copied

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // or_insert() -> &mut V
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map); // {"hello": 1, "world": 2, "wonderful": 1}
}
