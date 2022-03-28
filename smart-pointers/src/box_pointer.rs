use std::ops::Deref;
use crate::box_pointer::List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn cons_list_test() {
    println!("=== cons_list_test ===");

    let list = Cons(1, Box::new(
        Cons(2, Box::new(
            Cons(3, Box::new(
                Nil))))));

    let mut list_item = &list;
    loop {
        match list_item {
            Cons(value, next) => {
                println!("value = {}", value);
                // defer 强制转换
                list_item = next;
            }
            Nil => { break }
        }
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    #[allow(dead_code)]
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
