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

fn data_type() {
    let spaces = "    ";
    println!("the spaces is: \"{}\"", spaces);
    
    let spaces = spaces.len();
    println!("the spaces is: \"{}\"", spaces);
}
