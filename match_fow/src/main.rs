#[derive(Debug)]
enum Orel {
    Gagarin,
    Pushkin,
}

enum Coin {
    Rouble,
    Pyatak,
    Chervonets(Orel),
    Chetvertak,
}

fn main() {
    println!("{}", value_in_roubles(Coin::Rouble));
    println!("{}", value_in_roubles(Coin::Pyatak));
    println!("{}", value_in_roubles(Coin::Chervonets(Orel::Gagarin)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{}", six.unwrap_or_default());
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(number) => Some(number + 1),
    }
}

fn value_in_roubles(coin: Coin) -> u8 {
    match coin {
        Coin::Rouble => { // 1,  // если выполняется несколько строк в ветке, то ставим {} в конце запятая необязательна
            println!("Lucku roublik");
            1
        }
        Coin::Pyatak => 5,
        Coin::Chervonets(orel) => {
            println!("Na orle narisovan: {orel:?}.");
            10
        }
        Coin::Chetvertak => 25,
    }
}
