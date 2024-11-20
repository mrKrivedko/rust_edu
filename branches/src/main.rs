fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divizible by 4");
    }
    else if number % 3 == 0{
        println!("number is divizible by 3");
    }
    else if number % 2 == 0 {
        println!("number is divizible by 2")
    }
    else {
        println!("number not divizible by 4, 3, 2.. :(")
    }

    let condition = true;
    let number = if condition {5} else {7};

    println!("Value number is: {}", number);
}
