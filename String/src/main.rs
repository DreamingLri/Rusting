fn main() {
    let data = "hello, world";
    let string = data.to_string();

    let mut s1 = string.clone();
    let mut s2 = String::new();
    for i in s1.chars().rev() {
        s2.push(i);
    }

    println!("{}", string);
    println!("{:?}", s2);
}
