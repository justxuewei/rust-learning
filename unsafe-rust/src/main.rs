use std::{slice, str::from_utf8_unchecked};


fn main() {
   unsafe_test();
   access_unsafe_memory_test();
   smart_pointer_to_raw_pointer_test();
}

fn unsafe_test() {
    println!("=== unsafe_test ===");

    let mut num = 5;

    // 知识点
    // 1. as 是强制转换，这里是将一个引用强制转换为 raw pointer。
    // 2. 创建裸指针是安全的行为，而解引用裸指针才是不安全的行为，所以不需要 unsafe 关键词。
    // r1 指向一个不可变 i32 常量
    let r1 = &num as *const i32;
    // r2 指向一个可变 i32 变量
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 = {}, r2 = {}", *r1, *r2);
    }
}

// 这个地方我有个疑问：string 变量在 get_memory_location 结束之后应该就被回收了，为什么还可以在 get_str_at_location 中被访问？
// A: https://course.rs/advance/unsafe/superpowers.html 在这个介绍，裸指针没有实现任何自动的回收 (drop)
fn get_memory_location() -> (usize, usize) {
    let string = "Hello World!";
    let pointer = string.as_ptr() as usize;
    let length = string.len();
    (pointer, length)
}

fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
    unsafe {
        from_utf8_unchecked(slice::from_raw_parts(pointer as *const u8, length))
    }
}

fn access_unsafe_memory_test() {
    println!("=== access_unsafe_memory_test ===");

    let (pointer, length) = get_memory_location();
    let message = get_str_at_location(pointer, length);
    println!("The {} bytes at 0x{:X} stored: {}", length, pointer, message);

    // let message = get_str_at_location(1000, 10);
    // println!("message = {}", message);
}

fn smart_pointer_to_raw_pointer_test() {
    println!("=== smart_pointer_to_raw_pointer_test ===");

    let a = Box::new(10);
    // 需要先解引用a
    let b: *const i32 = &*a;
    let c: *const i32 = Box::into_raw(a);

    unsafe {
        println!("b = {}, c = {}", *b, *c)
    }
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    // ptr 是 slice 首地址的 raw pointer
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    // compilation error
    // (&mut slice[..mid], &mut slice[mid..])

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len-mid),
        )
    }
}
