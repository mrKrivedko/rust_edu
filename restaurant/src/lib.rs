mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        
        fn serve_orders() {}

        fn take_payments() {}
    }
}

/*
Дерево модулей

crate
 └── front_of_house
 ├── hosting
 │   ├── add_to_waitlist
 │   └── seat_at_table
 └── serving
     ├── take_order
     ├── serve_order
     └── take_payment
 */
