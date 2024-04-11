enum SpreadSheetShell{
    Int(i32),
    Float(f32),
    Text(String)
}

fn main() {
    let v1:Vec<i32> = Vec::new();
    //or
    let mut v2 = vec![0, 1, 2];
    v2.push(3);

    let third = &v2[2];
    println!("The third element is {}", third);

    match v2.get(2) {
        None => {
            println!("There is no third element")
        }
        Some(third) => {
            println!("The third element is {}", third)
        }
    }

    let mut v3 = vec![0, 1, 2, 3, 4, 5, 6];
    let first = &v3[0];
    // v3.push(7); //error 添加元素时可能导致vec重新分配内存，这是first会失效

    println!("first is {}", first);

    let row = vec![
                   SpreadSheetShell::Float(3.4),
                   SpreadSheetShell::Int(3),
                   SpreadSheetShell::Text(String::from("hello"))];

}
