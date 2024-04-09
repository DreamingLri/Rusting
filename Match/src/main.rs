use crate::UsState::Alabama;

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
}

fn value_in_coin(coin: Coin) -> u8{
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State Quarter is from {:?}", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn main() {
    let c = Coin::Quarter(Alabama);
    println!("{}", value_in_coin(c));

    let four = Some(4);
    let five = plus_one(four);
    let none = plus_one(None);

    let v = Some(0u8);
    match v {
        Some(0) => println!("0"),
        _ => (),
    }

    //or

    if let Some(0) = v{
        println!("0");
    } else {
        println!("others");
    }
}
