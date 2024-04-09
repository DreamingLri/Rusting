enum Result{
    Ok,
    Warning{
        code: i32,
        message: String,
    },
    Err(String),
}

// match make_result() {
//     Result::Ok =>
//     println!("Ok"),
//     Result::Warning {code, message}=>
//     println!("Warning: {}", message),
//     Result::Err(s)=>
//     println!("Failed: {}", s),
// }

enum Option<T>{
    Some(T),
    None,
}

// enum IpAddrKind{
//     V4,
//     V6,
// }
//
// struct IpAddr{
//     kind: IpAddrKind,
//     address: String,
// }

enum IpAddr{
    V4(u8, u8, u8, u8),
    V6(String)
}


fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => Option::None,
        Some(i) => Option::Some(i+1),
    }
}


fn main() {
    // let home = IpAddr{
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // let y: i8 = 5;
    // let x = Some(5);
    // let z = x + y;

    let five = Option::Some(5);
    let six = plus_one(five);
    let none = plus_one(Option::None);
}
