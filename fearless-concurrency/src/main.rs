use std::sync::mpsc;
use std::thread;

fn main() {
    channel_test();
}

fn channel_test() {
    println!("=== channel_test ===");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // 阻塞等待
    let received = rx.recv().unwrap();
    // 非阻塞等待
    // let received = rx.try_recv().unwrap();
    println!("Got: {}", received);
}
