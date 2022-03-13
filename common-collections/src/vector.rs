pub fn vector_test() {
    println!("=== vector_test ===");

    let mut v: Vec<i32> = Vec::new();
    let _v1 = vec![1, 2, 3];

    v.push(4);
    v.push(5);
    v.push(6);
    v.push(7);

    // 这里如果使用 v[2] 那是否表明会获取这个元素的所有权呢？
    // 我理解这个事情是不会的，这里的作用域没有改变，所以所有元素的所有权还是在这个大括号之下的，其次这样做应该是值拷贝
    // 工作，参见 vector_copy_test() 方法。
    let third = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    if let Some(third) = v.get(2) {
        println!("The third element is {}", third)
    }
}

pub fn vector_borrowing_test() {
    println!("=== vector_borrowing_test ===");

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    println!("First time the first = {}", first);
    v.push(6);
    // 这里再使用 first 是不被允许的，因为 v 已经被修改了。
    // println!("{}", first);
    let first = &v[0];
    println!("Reacquire first = {}", first);
}

pub fn vector_iteration_test() {
    println!("=== vector_iteration_test ===");

    let v = vec![1, 2, 3, 4, 5];
    for i in &v {
        println!("{}", i);
    }

    let mut mutv = vec![100, 32, 57];
    for i in &mut mutv {
        *i += 50
    }
    println!("after mutated");
    for i in &mutv {
        println!("{}", i);
    }
}

pub fn enum_store_multiple_types_test() {
    println!("=== enum_store_multiple_types_test ===");

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let rows = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for row in &rows {
        match row {
            SpreadsheetCell::Int(i) => println!("int = {}", i),
            SpreadsheetCell::Float(f) => println!("float = {}", f),
            SpreadsheetCell::Text(s) => println!("text = {}", s),
        }
    }
}

pub fn vector_copy_test() {
    println!("=== vector_copy_test ===");
}
