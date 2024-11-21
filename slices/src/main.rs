fn main() {
    let mut stroka = String::from("Hello, fucking world!");
    
    let word = first_word(&stroka);
    stroka.clear();
    println!("{word}");
    
    let stroka = String::from("Hello, fucking world!");

    let hello = &stroka[0..5];
    let fucking = &stroka[7..14];
    let world = &stroka[15..20];

    println!("{}, {} {}!", hello, fucking, world);

    let first = new_first_word(&stroka);
    println!("{first}");

    let stroka = "Hello, fucking world!"; // строкавые литералы - это срезы
    println!("{stroka}");

    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[..3];
    assert_eq!(slice, &[1, 2, 3]);

}

fn new_first_word(stroka: &str) -> &str {
    let bytes = stroka.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &stroka[..i];
        }
    }
    &stroka[..]
} 

fn first_word(stroka: &String) -> usize {
    let bytes = stroka.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;  // если хотим вернуть раньше
        }
    }

    stroka.len()
}