pub fn closure_ownership_test() {
    println!("=== closure_ownership_test ===");

    let x = vec![1, 2, 3];

    let equal_to_x = move |z| x == z;

    // x 不能再使用了，因为闭包使用 move 关键字定义，获取了 x 的所有权。
    // println!("x = {:?}", x);

    let y = vec![1, 2, 3];

    println!("result = {}", equal_to_x(y));

    // 同样 y 也不能只用了，因为 y 的所有权已经转移给闭包了。
    // println!("y = {:?}", y);
}
