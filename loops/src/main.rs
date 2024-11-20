fn main() {

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter *2;
        }
    };
    println!("Result value is: {}", result);
    println!("--------------------");
    println!("");
    
    let mut count = 0;
    
    'counting_up: loop {
        println!("count value is: {count}");
        let mut remaining = 10;
        
        loop {
            println!("remaining value is {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        
        count += 1;
    }
    
    println!("End count value is: {count}");
    println!("--------------------");
    println!("");
    
    let mut number = 3;
    
    while number != 0 {
        println!("number valueis: {number}");
        number -= 1;
    }
    
    println!("KONIEC while!");
    println!("--------------------");
    println!("");
    
    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;
    
    while index < arr.len() {
        println!("element array to index {index} value is: {}", arr[index]);
        
        index += 1;
    }
    println!("--------------------");
    println!("");
    
    let mut cnt: i8 = 0;
    for element in arr {
        println!("element {cnt} in array have value is: {element}");
        cnt += 1;
    }
    println!("--------------------");
    println!("");

    for number in (1..4).rev() {
        println!("Number value is: {number}");
    }
    println!("KONIEC!!")
}
