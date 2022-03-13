use std::collections::HashMap;

pub fn new_hash_map_test() {
    println!("=== new_hash_map_test ===");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    match scores.get(&String::from("Blue")) {
        Some(v) => println!("v = {}", v),
        None => println!("none"),
    }
}

pub fn iterate_hash_map_test() {
    println!("=== iterate_hash_map_test ===");

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    for (k, v) in &scores {
        println!("key = {}, value = {}", k, v);
    }
}

pub fn update_test() {
    println!("=== update_test ===");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // entry 函数的返回值是一个枚举
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}
