struct Node<'a>{
    elem: u32,
    next: Option<Box<& 'a Node<'a>>>
}
fn main() {
    let s1 = Node{elem: 1, next: None};
    let s2 = Node{elem: 2, next: Option::Some(Box::new(&s1))};

    println!("{}", s1.elem);
    println!("{}", s2.elem);
}
