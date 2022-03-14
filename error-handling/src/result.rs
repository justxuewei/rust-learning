use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

pub fn open_file() {
    println!("=== open_file ===");

    let result = File::open("hello.txt");
    let f = match result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        }
    };
}

pub fn unwrap_expect_test() {
    println!("=== unwrap_expect_test ===");

    // unwrap 函数返回内容，如果是 Ok，反之则会调用 panic!
    let f = File::open("hello.txt").unwrap();
    // expect 与 unwrap 的行为相似，区别在于 expect 可以定制 panic! 输出的内容
    let f = File::open("hello1.txt").expect("Failed to open hello1.txt");
}

pub fn propagating_error_test() {
    println!("=== propagating_error_test ===");

    let s = read_username_from_file().unwrap();
    println!("the returned value from read_username_from_file() = {}", s);

    let s = read_username_from_file_1().unwrap();
    println!("the returned value from read_username_from_file_1() = {}", s);

    let s = read_username_from_file_2().unwrap();
    println!("the returned value from read_username_from_file_2() = {}", s);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_1() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

pub fn option_question_test() {
    println!("=== option_question_test ===");

    let text = String::from("haha\nxuewei");
    println!("the last char of the first line of the \"{}\" is {}", text, last_char_of_first_line(&text).unwrap())
}

pub fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
