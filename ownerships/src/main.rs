fn main() {
    {
        let hello = "hello";
        let mut hello: String = String::from(hello);
        hello.push_str(", fucking world!");

        println!("{hello}");
    }
    {
        let num = 5;
        let mut number = num + 1;

        number += 1;

        println!("num value: {num}, number value: {}, number: {}", number, num);
    }

    {
        let strings: String = String::from("hello");
        println!("clean string: {strings}");
        
        let mut other_string = strings.clone();  // deepcopy pointer to String
        println!("other string: {other_string}");        
        other_string.push_str(", fucking world!");
        println!("push other string: {other_string}");
        println!("clean string: {strings}");

    }

    {
        let s = String::from("hello");  // s входит в область видимости
    
        takes_ownership(s);  // значение s перемещается в функцию...
                                         // ... поэтому оно больше недействительно здесь

        let x = 5;  // x входит в область видимости
    
        makes_copy(x);  // x был бы передан в функцию,
                                     // но поскольку i32 является типом Copy, то x всё ещё
                                     // можно безопасно использовать дальше
    
    } // Здесь x выходит из области видимости, затем s. Но поскольку значение s было перемещено,
      // ничего особенного не происходит.
    
    {
        let s1 = gives_ownership();  // gives_ownership передаёт своё возвращаемое значение в s1
    
        let s2 = String::from("hello");  // s2 входит в область видимости
    
        let s3 = takes_and_gives_back(s2);  // s2 перемещается в функцию takes_and_gives_back, которая также
                                                             // возвращает значение и перемещает его в s3
    } // Здесь s3 выходит из области видимости и уничтожается. Так как s2 была перемещена, ничего
      // не происходит. Затем s1 выходит из области видимости и тоже уничтожается.
    {
        let strings = String::from("hello");

        let (other_string, len) = calculate_length(strings);

        println!("Other string {} has length {}", other_string, len);

    }
    // println!("{hello}");
    }

fn calculate_length(stroka: String) -> (String, usize) {
    let length = stroka.len();
    (stroka, length)
}
    
fn takes_ownership(some_string: String) { // some_string входит в область видимости
    println!("{some_string}");
} // Здесь some_string выходит из области видимости и вызывается `drop`. Память освобождается.
    
fn makes_copy(some_integer: i32) { // some_integer входит в область видимости
    println!("{some_integer}");
} // Здесь some_integer выходит из области видимости. Ничего особенного не происходит.

fn gives_ownership() -> String {  // gives_ownership передаст своё возвращаемое значение
                                  // в вызывающую функцию
    
    let some_string = String::from("yours"); // some_string входит в область видимости
    
    some_string  // some_string возвращается и перемещается в вызывающую функцию
}
    
// Эта функция принимает строку типа String и возвращает её
fn takes_and_gives_back(a_string: String) -> String { // a_string входит в область видимости
    a_string  // a_string возвращается и перемещается обратно в вызывающую функцию
}