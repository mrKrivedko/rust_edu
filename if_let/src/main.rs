fn main() {
    {
        let config_max = Some(3u8);
        match config_max {
            Some(max) => println!("max value config is: {max}"),
            _ => (),
        }
        
        {
            let config_max: Option<u8> = None; // Some(89u8);
            if let Some(num) = config_max {  // Синтаксис if let принимает шаблон и выражение,
                                                 // разделённые знаком равенства. Он работает так же, как match,
                                                 // когда в него на вход передадут выражение
                                                 // и подходящим шаблоном для этого выражения окажется первая ветка
                println!("max value config is: {num}");
            }
            else {
                println!("Tyt nichego net!");
            }
        }
    }

    println!("Hello, world!");
}
