use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    println!("{}", acronym(&s));
}

fn acronym(input: &str) -> String {
    let mut res = String::new();
    let binding = input.replace("-", " ");
    let ss = binding.split_whitespace();
    for s in ss {
        // let c = s.chars()
        //     .filter(|c| !c.is_ascii_punctuation())
        //     .next()
        //     .unwrap();
        let c = s.get(0..1).unwrap();
        if c.chars().next().unwrap().is_ascii_alphabetic() {
            res.push(c.chars().next().unwrap());
        } else {
            res.push(c.parse().unwrap());
        }

    }
    res.to_uppercase()
}
