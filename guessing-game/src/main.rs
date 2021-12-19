use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 使用 shadow 用新值隐藏旧值
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You gussed: {}", guess);

        // 1. Ordering 是一个枚举类型，成员包括 Less, Greater 和 Equal
        // 2. match 表达式由分支组成，一个分支用于一个匹配模式
        // 3. cmp 方法根据传入的数字字符串的不同，返回不同的 Ordering 枚举
        // 类型
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // 退出程序当“猜测”结果正确时
                break;
            }
        }
    }

}
