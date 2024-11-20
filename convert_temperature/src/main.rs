use std::io;

fn main() {
    
    let measure = loop {
        let mut measure: String = String::new();
        println!("Select measure! (F | C)");

        io::stdin().read_line(&mut measure).expect("Failed");
        
        measure = match measure.trim().parse() {
            Ok(measure) => measure,
            Err(_) => continue,
        };
        if measure == "F" {
            break measure;
        }
        else if measure == "C" {
            break measure;
        }
        else {
            println!("Only F or C");
            continue;
        }
    };
    
    println!("Select '{}'", measure);
    
    if measure == "F" {
        println!("Converted Farenheit to Celsius");
    }
    else if measure == "C" {
        println!("Converted Celsius to Fahrengeit");
    }
    
    let degrees: i32 = loop {
        let mut degrees: String = String::new();
        io::stdin().read_line(&mut degrees).expect("Failed");

        let degrees: i32 = match degrees.trim().parse() {
            Ok(deg) => deg,
            Err(_) => continue,
        };
        println!("Degrees value is: {degrees}");
        break degrees;
    };

    let result = match &measure[..] {
        "F" => fahrengeit_to_celsius(degrees),
        "C" => celsius_to_fahrengeit(degrees),
        _ => 0.0,
    };

    println!("{result} {measure}");

}

fn fahrengeit_to_celsius(how: i32) -> f64{
    let how: f64 = how as f64;
    (5.0 / 9.0) * (how - 32.0)
}

fn celsius_to_fahrengeit(how: i32) -> f64 {
    let how = how as f64;
    (9.0 / 5.0) * how + 32.0
}