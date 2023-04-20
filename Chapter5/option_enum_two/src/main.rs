fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // If let Syntax

    let some_value = Some(None);
    match some_value {
        Some(3) => println!("three"),
        _ => (),
    }
    if Some(3) == some_value {
        println!("three");
    }
    if let Some(3) = some_value {
        println!("three");
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn values_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}