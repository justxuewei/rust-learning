use std::fmt::Display;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn explicit_lifetimes_test() {
    println!("=== explicit_lifetimes_test ===");

    let str = "hahaha";
    let string = String::from("haha");
    println!("longest string is \"{}\"", longest(str, &string))
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // i32 不是指针，所以不需要 lifetimes
    fn level(&self) -> i32 {
        3
    }

    // announcement 和返回的 string slice 具有相同的生命周期
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// 我有个问题是 T 会不会是一个指针类型呢？
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
    where T: Display {
    println!("Announcement: {}!", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

f
