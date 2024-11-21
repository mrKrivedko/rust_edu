fn main() {
    let stroka: String = String::from("hello");

    let len = calculate_len(&stroka);
    println!("The length of '{stroka}' is {len}");


    let mut stroka: String = String::from("hello");
    change_str(&mut stroka);    
    println!("{}", stroka);

    let stroka1 = &stroka; // no problem (read link)
    let stroka2 = &stroka; // no problem (read link)
    println!("{stroka1}, {stroka2}");
    
    let stroka3 = &mut stroka;  // no problem (scope s1 & s2 closed)
    println!("{stroka3}");

    let reference_do_nothing = dangle();
    println!("{}", reference_do_nothing);
}

fn dangle() -> String {
    let stroka = String::from("hello");
    // &stroka // строка перестанет существовать после закрытия скоупа, нельзя возвращать ссылку
    stroka
}

fn change_str(strings: &mut String) {
    strings.push_str(", fucking world!");
}

fn calculate_len(strings: &String) -> usize { // s - это ссылка на String
    strings.len()
}  // Здесь s выходит из области видимости. Однако, поскольку она не владеет тем,
   // на что ссылается, она не удаляется.

