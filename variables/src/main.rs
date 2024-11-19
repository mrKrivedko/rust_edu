use std::io;

fn main() {
    let mut variable: u32 = 5;
    println!("The value variable is: {}!", variable);
    variable = 6;
    {
        let variable = variable + 1;
        println!("The value of x in the inner scope is: {variable}");
        
    }
    println!("The value variable is: {}!", variable);

    // float
    {
        let variable = 2.1;
        println!("Float (double) is: {variable}");
    }
    {
        let variable: f32 = 3.6;
        println!("Float (single) is: {variable}");
    }

    // numeric types and operators
    {
        // addition
        let sum = 5 + 10;
        println!("Sun is: {sum}");

        // subtraction
        let difference = 10 - 5;
        println!("Difference is: {}", difference);

        // multiplication
        let product = 4 * 10;
        println!("Product is: {}", product);

        // division
        let quotient = 56.7 / 32.2;
        println!("Quotient is: {quotient}");
        let truncated = -5 / 3;
        println!("Truncated rust: {truncated}");

        // remainder
        let remainder = 43 % 5;
        println!("Remainder is: {}", remainder);
    }

    // logic type
    {
        let bool_t = true;
        println!("Bool true is: {}", bool_t);

        let bool_f = false;
        println!("Bool false is: {}", bool_f);
    }

    // symbol type
    {
        let symbol = 'z';
        println!("Char is: {}", symbol);

        let symbol: char = 'ℤ';
        println!("Char is: {}", symbol);

        let heart_eyed_cat = '😻';
        println!("Heart eyed cat is: {}", heart_eyed_cat);
    }

    // tuple
    {
        let tuple: (i32, f64, char) = (-5, 7.9, 'f');
        let (x, y, z ) = tuple;
        println!("X is: {x}, Y is: {y}, Z is: {z}");

        let int = tuple.0;
        let symbol = tuple.2;
        println!("tuple[0] is: {int}, tuple[2] is: {symbol}, tuple[1] is: {}", tuple.1);
    }

    // array
    {
        let nums = [1, 2, 3, 4, 5];
        println!("nums[0] is: {}", nums[0]);

        let mut three = [3; 5];
        three[1] = 5;
        println!("Three is: [{}, {}, {}, {}, {}]", three[0], three[1], three[2], three[3], three[4]);
    }

    {
        let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
    }

    println!("KONIEC!");
}
