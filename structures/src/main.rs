use std::any::Any;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,

}

// кортежные структуры
struct Point(i64, u32, i64);
struct Color(u32, u32, u32);

// единично-подобные структуры
struct Krivedko;

fn main() {
    let mut user1 = User { // весь экземпляр должен быть изменяемым
        active: false,
        username: String::from("mrkrivedko"),
        email: String::from("krivedko@mail.com"),
        sign_in_count: 1,
    };
    
    println!("My username: {}", user1.username);
    
    user1.username = String::from("mr.krivedko");
    
    println!("My username: {}", user1.username);
    
    let user2 = build_user(user1.username, user1.email);
    println!("{} has email: {}", user2.username, user2.email);
    
    let user3 = User {
        username: String::from("krevedko"),
        ..user2
    };
    println!("{}, {}", user3.username, user1.active);
    
    let point = Point(0, 3, 5);
    let color = Color(255, point.1, 0);
    
    println!("{}, {}", point.2, color.1);
    println!("{}", color.0);
    
    

}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 5,
    }
}
