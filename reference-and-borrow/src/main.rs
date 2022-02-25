fn main() {
    reference();
    multiple_reference();
}

fn reference() {
    println!("=== reference ===");

    let s = String::from("hello");
    let len = calculate_length(&s);

    println!("s = {}, len = {}", s, len)
}

fn calculate_length(str: &String) -> usize {
    str.len()
}

// s1, s2 以及 s3 作用域没有重叠，所以可以正常编译
fn multiple_reference() {
    println!("=== multiple_reference ===");

    let mut s = String::from("hello");

    let s1 = &s;
    let s2 = &s;
    println!("s1 = {}, s2 = {}", s1, s2);

    let s3 = &mut s;
    s3.push_str(", world");
    println!("s3 = {}", s3);

    // s1 和 s2 将不能再使用
    // println!("s1 = {}", s1);
}
