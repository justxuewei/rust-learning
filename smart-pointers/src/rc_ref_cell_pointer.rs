use crate::rc_ref_cell_pointer::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

pub fn rc_ref_cell_test() {
    println!("=== rc_ref_cell_test ===");

    // value 是
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    // b 的所有权没有变化
    match &b {
        Cons(valueb, rc_a) => {
            println!("b.value = {:?}", valueb);
            match &**rc_a {
                Cons(valuea, _) => {
                    println!("a.value = {:?}", valuea);
                    *valuea.borrow_mut() += 10;
                }
                _ => {
                    panic!("a should have next list");
                }
            }
        }
        _ => {
            panic!("b should have next list");
        }
    }

    println!("b = {:?}", b)
}
