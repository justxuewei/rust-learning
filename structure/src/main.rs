fn main() {
    rectangle_test();
    print_rectangle_test();
    rectangle_method_test();
    rectangle_associated_function_test();
}

fn rectangle_test() {
    println!("=== rectangle_test ===");
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", area(&rect));
}

fn print_rectangle_test() {
    println!("=== print_rectangle_test ===");
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect is {:?}", rect);
    println!("rect is {:#?}", rect);
    dbg!(&rect);
}

fn rectangle_method_test() {
    println!("=== rectangle_impl_test ===");
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", rect.area());

    let rect1 = Rectangle {
        width: 20,
        height: 20,
    };
    println!("Can rect hold rect1? {}", rect.can_hold(&rect1));
}

fn rectangle_associated_function_test() {
    println!("=== rectangle_associated_function_test ===");
    let rect = Rectangle::square(20);
    println!("The area of the rectangle is {} square pixels.", rect.area());
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
