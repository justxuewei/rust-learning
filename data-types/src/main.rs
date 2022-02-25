fn main() {
    tuple();
}

fn tuple() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("x: {}, y: {}, z: {}", x, y, z);

    let x1 = tup.0;
    let y1 = tup.1;
    let z1 = tup.2;

    println!("x: {}, y: {}, z: {}", x1, y1, z1);
}

fn array() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let b = [3; 5];
}
