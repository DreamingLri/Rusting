// enum IpAddKind{
//     V4,
//     V6,
// }

enum IpAddrKind{
    V4(u8, u8, u8, u8),
    V6(String),
}
// struct IpAddr{
//     kind: IpAddKind,
//     address: String,
// }

enum Message{
    Quit,
    Move{x: u32, y: u32},
    Write(String),
    ChangeColor(u32, u32, u32),
}

impl Message{
    fn call(&self){}
}

fn main() {
    // let four = IpAddKind::V4;
    // let six = IpAddKind::V6;
    //
    // let home = IpAddr{
    //     kind: IpAddKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    let home = IpAddrKind::V4(127, 0, 0, 0);
    let loopback = IpAddrKind::V6(String::from("::1"));

    let q = Message::Quit;
    let m = Message::Move {x: 10, y: 20};
    let w = Message::Write(String::from("hello"));
    let c = Message::ChangeColor(20, 30, 40);

    m.call();
    // let loopback = IpAddr{
    //     kind: IpAddKind::V6,
    //     address: String::from("::1"),
    // };
}
