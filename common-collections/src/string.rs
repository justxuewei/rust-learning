pub fn new_string_test() {
    println!("=== new_string_test ===");

    let mut s = String::new();

    let data = "initial contents";
    s = data.to_string();

    println!("s = {}", s);
}

pub fn push_pop_test() {
    println!("=== push_pop_test ===");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s = {}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 = {}, s2 = {}", s1, s2);

    let mut s3 = String::from("lo");
    s3.push('l');
    println!("s3 = {}", s3);
}

pub fn concatenate_test() {
    println!("=== concatenate_test ===");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // + 后面需要一个 &str 参数，而非 &String，但是这个代码还可以正常运转下去的主要原因是强转（coerced）成 &str，
    // Rust 使用了一个被称为 Deref 强制转换（deref coercion）的技术，你可以将其理解为它把 &s2 变成了 &s2[..]。
    let s3 = s1 + &s2;
    // s1 不能够再被使用了，因为 s1 的所有权被转移给 s3
    // println!("s1 = {}", s1);
    println!("s2 = {}, s3 = {}", s2, s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // format 不会使用任何所用权
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s = {}", s);
}

pub fn string_index_test() {
    println!("=== string_index_test ===");

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("s = {}", s);
}

pub fn iterate_string_test() {
    println!("=== iterate_string_test ===");

    for c in "नमस्ते".chars() {
        println!("c = {}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("b = {}", b)
    }
}
