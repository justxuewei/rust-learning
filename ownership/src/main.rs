fn main() {
    move_test();
    clone_test();
    ownership_with_function();
    ownership_with_return_value();
}

// s1 不可用，因为 s1 已经移动到 s2，因此 s1 已经失效了。
fn move_test() {
    println!("=== move_test ===");

    let s1 = String::from("hello");
    let s2 = s1;

    println!("s2: {}", s2);
}

// 克隆同样也会复制堆上的内容
fn clone_test() {
    println!("=== clone_test ===");
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1: {}, s2: {}", s1, s2)
}

fn ownership_with_function() {
    println!("=== ownership_with_function ===");
    let s = String::from("hello"); // s 进入作用域
    takes_ownership(s); // s 的值移动到函数里
    // 所以到这里 s 不再有效

    let x = 5; // x 进入作用域
    makes_copy(x); // x 被拷贝到函数中
    // x 可以继续使用
} // x 移出了作用域，s 被忽略

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_int: i32) {
    println!("{}", some_int);
}

fn ownership_with_return_value() {
    println!("=== ownership_with_return_value ===");

    let s1 = gives_ownership(); // gives_ownership 将返回值转移给 s1
    let s2 = String::from("hello"); // s2 进入作用域
    let s3 = takes_and_gives_back(s2); // s2 被移动到 takes_and_gives_back 中，它也将返回值移给 s3

    println!("s1: {}, s3: {}", s1, s3);
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，所以什么也不会发生，s1 离开作用域并被丢弃。

// gives_ownership 会将返回值移动给调用它的函数
fn gives_ownership() -> String {
    let some_string = String::from("yours"); // some_string 进入作用域
    some_string // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(some_string: String) -> String { // a_string 进入作用域
    some_string // 返回 a_string 并移出给调用的函数
}
