fn main() {

    let some_number = Some(5);
    let some_char = Some('z');

    let absent_number: Option<i32> = None;

    let x: u8 = 5;
    let y: Option<u8> = None;  // Some(8);

    println!("sum is: {}", x + y.unwrap_or_default());
}
