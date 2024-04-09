fn hello(){
    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";
    let chinese: &str = "世界您好！";
    let regions: [&str; 3] = [southern_germany, japan, chinese];
    for region in regions.iter() {
        println!("{}", region);
    }
}

fn main() {
    hello()
}
