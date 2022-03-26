use std::rc::Rc;
use List::{Cons, Nil};

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

pub fn rc_pointer_test() {
    println!("=== rc_pointer_test ===");

    let a = Rc::new(
        Cons(5, Rc::new(
            Cons(10, Rc::new(
                Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    println!("iterate cons list a");
    iterate_cons_list(&a);
    println!("iterate cons list b");
    iterate_cons_list(&b);
    println!("iterate cons list c");
    iterate_cons_list(&c);
}

fn iterate_cons_list(l: &List) {
    let mut node = l;

    loop {
        match node {
            Cons(value, next) => {
                println!("value = {}", value);
                node = next;
            }
            Nil => { return; }
        }
    }
}

pub fn ref_count_test() {
    println!("=== ref_count_test ===");

    let a = Rc::new(Cons(5,
                         Rc::new(Cons(10,
                                      Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
