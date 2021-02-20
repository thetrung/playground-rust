use rand::Rng;

// mod front_of_house;

// pub use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }

enum IpAddrKind {
    V4, V6
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

fn route(ip_kind: IpAddrKind){}

enum Message {
    Quit,                  // no assoc data
    Move {x: i32, y: i32}, // anonymous struct
    Write(String),         // assoc String
    ChangeColor(i32, i32, i32), // 3x i32
}

impl Message {
    fn call(&self) {
        // method body define here
    }
}

enum Option<T> {
    Some(T),
    None,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(i32),
    Dont_Give_A_Shit
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(0) => 25,
        _ => 0, // anything else would be zero
    }
}


fn main() {
    
    // test enum
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello, World !"));
    
    m.call();

    let some_number = Some(10);
    let some_string = Some("Some String");

    let test = value_in_cents(Coin::Dont_Give_A_Shit);

    println!("Value in cents if you don't give a shit is {}", test);

    let some_u8_value = Some(10);
    //
    // matching and execute code with "if let"
    //
    if let Some(3) = some_u8_value {
        println!("three");
    } else { 
        println!("others") 
    }

    let secret_number = rand::thread_rng().gen_range(1, 100);
    println!("example of randomize function: {}", secret_number);
}
