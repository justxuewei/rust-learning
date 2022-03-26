use std::process::Output;
#[macro_export]
macro_rules! create_function {
    // 此宏接受一个 `ident` 指示符表示的参数，并创建一个名为 `$func_name` 的函数。
    // `ident` 指示符用于变量名或函数名
    ($func_name: ident) => (
        fn $func_name() {
            println!("you called {:?}()", stringify!($func_name));
        }
    )
}

#[macro_export]
macro_rules! print_result {
    // 此宏接受一个 `expr` 类型的表达式，并将它作为字符串，连同其结果一起
    // 打印出来。
    // `expr` 指示符表示表达式。
    ($expression:expr) => {
        println!("{:?} = {:?}",
                 stringify!($expression),
                 $expression)
    };
}

// 重载
#[macro_export]
macro_rules! test {
    ($left:expr; and $right:expr) => {
        println!("{:?} and {:?} is {:?}", stringify!($left), stringify!($right), $left && $right);
    };

    ($left:expr; or $right:expr) => {
        println!("{:?} or {:?} is {:?}", stringify!($left), stringify!($right), $left || $right);
    }
}

// 重复（展开）
// $(...),+ 包围起来，就可以匹配一个或多个用逗号隔开 的表达式。
#[macro_export]
macro_rules! find_min {
    ($x:expr) => {$x};
    // `$x` 后面跟着至少一个 `$y,`
    ($x:expr, $($y:expr),+) => {
        // 对 `$x` 后面的 `$y` 们调用 `find_min!`
        std::cmp::min($x, find_min!($($y), +))
    };
}

#[macro_export]
macro_rules! assert_equal_len {
    // `tt`（token tree，标记树）指示符表示运算符和标记
    ($a:ident, $b:ident, $func:ident, $op:tt) => {
        assert!($a.len() == $b.len(),
        "{:?}: dimension mismatch: {:?} {:?} {:?}",
        stringify!($func), ($a.len(),), stringify!($op), ($b.len(),));
    }
}

#[macro_export]
macro_rules! op {
    ($func:ident, $bound:ident, $op:tt, $method:ident) => {
        // $bound<T, Output=T> 中的 Output 是 Add trait 中的关联类型
        fn $func<T: $bound<T, Output=T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);

            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
            }
        }
    }
}

#[macro_export]
macro_rules! calculate {
    // DSL 语言
    (eval $e: expr) => {
        // 强制转换为整型
        let val: usize = $e;
        println!("{} = {}", stringify!{$e}, val);
    };

    // 可变参数
    (eval $e: expr, $(eval $es:expr),+) => {
        calculate!(eval $e);
        calculate!($(eval $es), +);
    };
}
