use std::cell::RefCell;
use std::rc::{Rc, Weak};
use List::{Cons, Nil};

const DISABLE_CODE: bool = true;

#[derive(Debug)]
enum List {
    // 这样设计后，可变的是 Rc<List>
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

pub fn reference_cycle_tests() {
    println!("=== reference_cycle_tests (DISABLED: {}) ===", DISABLE_CODE);

    if DISABLE_CODE {
        return;
    }

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // println!("a next item = {:?}", a.tail());
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    // 解释一下为什么这里的类型是 RefCell<Vec<Rc<Node>>>：
    // - Rc<Node>：一个子节点可能有多个父节点，换句话说多个父节点持有一个子节点。
    // - RefCell<Vec>>：我们希望可以动态修改 Vec 中的内容。
    // 疑问：
    // Q1: 这里必须使用 RefCell 指针吗？直接使用 Vec 可不可以呢？
    // A1: 因为在使用的时候一般是将 Node 包裹在 Rc 中，而 Rc 是不可变的，所以必须使用 RefCell 来对其进行包裹。
    children: RefCell<Vec<Rc<Node>>>,
}

pub fn building_tree_test() {
    println!("=== building_tree_test ===");

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // 在为空的时候使用 upgrade() 会获得一个 None
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // downgrade 创建一个 weak counter
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // println!("branch = {:?}", branch);
    // 在不为空的时候使用 upgrade() 会获得一个 Some(value)
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

pub fn count_visibility_test() {
    println!("=== count_visibility_test ===");

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // 1, 0
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        // 1, 1
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        // 2, 0
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    // None
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // 1, 0
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
