use std::vec;

use collections::vectors;
use collections::strings;

fn main() {
    {
        let mut vector1 = vectors::VECTOR1;
        vector1.push(1);
        vector1.push(3);
        println!("{vector1:?}");
    
        let mut vector2 = vectors::get_vector_123();
        println!("{vector2:?}");
        
        let third: &i32 = &vector2[3];  // при выходе за пределы вектора, будет panic
        
        println!("{third}");
        vector2.push(6);
        let fourth: Option<&i32> = vector2.get(9);  // через get мы не получим panic если выйдем за диапазон вектора
        match fourth {
            Some(1) => println!("this first element"),
            Some(fourth) => println!("this fourth element: {}", fourth),
            None => println!("tyt pusto ("),
        }
        
        for num in &mut vector2 {
            *num += 20;
            println!("{num}");
        }
        println!("{vector2:?}");
        
        for num in &vector2 {
            println!("{num}"); 
        }
        println!("{vector2:?}");

        let row = vec![
            vectors::SpreadSheetsCell::Int(3),
            vectors::SpreadSheetsCell::Float(5.56),
            vectors::SpreadSheetsCell::Text(String::from("fuck!"))
        ];

        println!("{row:?}");
    }

    {
        let data = "initial data";

        let stroka = data.to_string();
        println!("{}", stroka);
        
        let stroka_also = "initial data1".to_string();
        println!("{}", stroka_also);
        
        let also_stroka = String::from("Initial data");
        println!("{}", also_stroka);

        println!("{:?}", strings::hello());

        let mut stroka1 = String::from("fuck");
        stroka1.push_str(&stroka);

        println!("{}", stroka1);
        stroka1.push('Z');
        println!("{}", stroka1);

        // Adding strings

        let stroka3 = String::from("Hello, ");
        let stroka4 = String::from("fucking world");
        let stroka5 = stroka3 + &stroka4;

        println!("{}, {}, {}", stroka1, stroka4, stroka5);
        
        let stroka2 = format!("{}-{}-{}", stroka1, stroka4, stroka5);
        println!("{}", stroka2);
        println!("{}, {}, {}", stroka1, stroka4, stroka5);

        println!("{}", &stroka2[1..=4]);
        let ucki = &stroka2[1..=4];
        for ch in ucki.chars() {  // будет перебирать символы
            println!("{ch}")
        }

        for byte in ucki.bytes() {
            println!("{byte}");
        }
    }

}
