use std::rc::Rc;
use List::{Cons, Nil};

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

pub fn rc_pointer_test() {
    println!("=== rc_pointer_test ===");

    // 如果这里 a 是一种 Box<T> 类型的智能指针，但是所有权只能同时被一个变量所有
    // 在下面 b 和 c 之中，a 的所有权被转移了两次，所以在这种情况下 b 不可用，c 可用
    // 为了让 b 和 c 同时可用，所以需要使用 Rc<T>

    // 比如一个 Rc<T> 离开了作用域，那么计数器就会 -1，如果计数器变为 0 那么内存就会
    // 被回收，所以还是与作用域息息相关的。
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
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
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
