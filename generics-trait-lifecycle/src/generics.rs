fn largest<T>(list: &[T]) -> T
    where T: PartialOrd + Copy {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn largest_test() {
    println!("=== largest_test ===");

    let v = vec![1, 2, 3, 4, 5];
    println!("the largest value of the v = {}", largest(&v))
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// 在 impl 后面加 <T> 表示又创建了一个新的 <T>，Point 后面的 <T> 则是表示我要使用这个 T 类型的泛型
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 这里是与上面例子的对比，这里不在创建一个新的泛型了，而是使用一个 f32 具体类型
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub fn impl_generics_test() {
    println!("=== impl_generics_test ===");

    let p = Point { x: 5, y: 10 };
    println!("p = {:?}", p);
    println!("p.x() = {}", p.x());

    let p = Point { x: 5.0, y: 10.0 };
    println!("p = {:?}", p);
    println!("p.distance_from_origin() = {}", p.distance_from_origin())
}
