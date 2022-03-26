use macrorules::{create_function, find_min, test, calculate};

create_function!(foo);
create_function!(bar);

fn main() {
    foo();
    bar();

    macrorules::print_result!(1u32 + 1);

    // 回想一下，代码块也是表达式！
    macrorules::print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });

    macrorules::test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    macrorules::test!(true; or false);

    println!("{}", macrorules::find_min!(1u32));
    println!("{}", macrorules::find_min!(1u32 + 2 , 2u32));
    println!("{}", macrorules::find_min!(5u32, 2u32 * 3, 4u32));

    calculate!(eval 1 + 1);
    calculate!(eval 1 + 1, eval 3 + 4, eval (2 * 3) + 1);
}
