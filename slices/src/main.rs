fn main() {
    first_word_test();
}

fn first_word_test() {
    println!("=== first_word_test ===");

    let my_string = String::from("hello world");
    let word = first_word(&my_string[0..6]);
    println!("The first word is {}", word);
    let word = first_word(&my_string[..]);
    println!("The first word is {}", word);
    let word = first_word(&my_string);
    println!("The first word is {}", word);

    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal[0..6]);
    println!("The first word is {}", word);
    let word = first_word(&my_string_literal[..]);
    println!("The first word is {}", word);
    // 因为字符串字面值已经是字符串 slice 了
    let word = first_word(my_string_literal);
    println!("The first word is {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
