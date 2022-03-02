fn main() {
    option_test();
    match_test();
    if_let_test();
}

enum IpAddrKind {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

struct Ipv4Addr {
    // -- snip --
}

struct Ipv6Addr {
    // -- snip --
}

fn option_test() {
    println!("=== option_test ===");
    let some_number = Some(5);
    let some_string = Some(String::from("a string"));
    let absent_number: Option<i32> = None;
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn match_test() {
    println!("=== match_test ===");
    println!("{}", value_in_cents(Coin::Dime));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));
}

fn if_let_test() {
    println!("=== if_let_test ===");
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => {},
    }
    // if let 等价操作
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
