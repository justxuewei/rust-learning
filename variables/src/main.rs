fn main() {
    shadowing();
    data_type();
}

fn shadowing() {
    let x = 5;

    let x = x + 5;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}

// 隐藏（shadowing）可以改变变量的类型，本质上它新建了一个新变量。
// 加入 mut 关键字的则相反，是在同一个内存地址上编辑内容，所以它的类型是无法改变的。
fn data_type() {
    let spaces = "    ";
    println!("the spaces is: \"{}\"", spaces);
    
    let spaces = spaces.len();
    println!("the spaces is: \"{}\"", spaces);
}
