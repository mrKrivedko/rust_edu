fn main() {
    println!("Hello, world!");
    any_func();
    args_func(99);
    more_args_func(-25, 'f');

    let y = {
        let x = 3;
        x + 1
    };

    println!("Y value  is: {}", y);

    println!("Returned func return is: {}", returned_func(4))
}

fn any_func() {
    println!{"Hello, python"};
}

fn args_func(num: i32) {
    println!("argumetn func is: {num}");
}

fn more_args_func(num: i32, symbol: char) {
    println!("first arg is: {}, second arg is {}", num, symbol);
}

fn returned_func(num: i32) -> i32 {
    num
}