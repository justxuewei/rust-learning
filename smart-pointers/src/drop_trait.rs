struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomerSmartPointer with data {}!", self.data);
    }
}

pub fn drop_trait_test() {
    println!("=== drop_trait_test ===");

    let c = CustomSmartPointer{
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created, c = {}, d = {}.", c.data, d.data);
}

pub fn drop_manually_test() {
    println!("=== drop_manually_test ===");

    let c = CustomSmartPointer{
        data: String::from("my stuff"),
    };
    println!("CustomSmartPointers created, c = {}.", c.data);
    drop(c);
    println!("CustomSmartPointer dropped before the end of function.");
}
