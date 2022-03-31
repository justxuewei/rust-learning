fn main() {
    println!("Hello, world!");
}

#[allow(unused_variables)]
struct Socket {

}

impl Socket {
    #[allow(dead_code)]
    fn read_buf() {}
    #[allow(dead_code)]
    fn set_readble_callback(callback: fn()) {}
    #[allow(dead_code)]
    fn set_readble_callback(callback: fn()) {}
}

trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Poll,
}

struct SocketRead<'a> {
    socket: &'a Socket,
}

impl SimpleFuture for SocketRead<'_> {

}
