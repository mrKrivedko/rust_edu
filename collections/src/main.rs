use std::vec;

use collections::vectors;
use collections::strings;
use collections::hashmap;

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

    {
        let mut hashmapa = hashmap::get_new_hashmap();

        hashmapa.insert(String::from("Sinie"), -5);
        hashmapa.insert(String::from("Krasnie"), 55);

        let team_name = String::from("Sinie");
        let score = hashmapa.get(&team_name).copied().unwrap_or(0);
        println!("{score}");
        let score = hashmapa.get("hhh").copied().unwrap_or(-999);
        println!("{score}");
        
        for (key, value) in &hashmapa {  // пкчатает содержимое в произвольном порядке
            println!("{key}: {value}");
        }
        println!("{score}");

        let mut hashmapa2 = hashmap::get_new_hashmap();

        let field = String::from("field");
        let value = 5;

        hashmapa2.insert(field, value);
        hashmapa2.insert(String::from("1"), value);
        
        println!("{:?}", hashmapa2);
        // println!("{}", field);
        println!("{}", value);

        // перезапись значения если ключ совпадает
        hashmapa2.insert(String::from("1"), 7);
        println!("{:?}", hashmapa2);
        
        // вставка значения только если ключа нет
        hashmapa2.entry(String::from("field")).or_insert(10);
        hashmapa2.entry(String::from("new")).or_insert(9);
        println!("{:?}", hashmapa2);

        use std::collections::HashMap;

        let text = "hello fucking fucking word";

        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{map:?}");
    }
}