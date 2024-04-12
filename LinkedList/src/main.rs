use std::mem;

#[derive(PartialEq)]
struct Node{
    elem: i32,
    next: Option<Box<Node>>
}

struct LinkedList{
    head: Option<Box<Node>>
}

impl LinkedList{
    pub fn new() -> Self{
        LinkedList{head: None}
    }

    pub fn is_empty(&self) -> bool{
        self.head == None
    }

    pub fn push(&mut self, elem: i32){
        let temp = Box::new(Node{elem, next: self.head.take()});
        self.head = Some(temp);
    }

    pub fn pop(&mut self) -> Option<i32>{
        match mem::replace(&mut self.head, None) {
            None => None,
            Some(elem) => {
                self.head = elem.next;
                Some(elem.elem)
            }
        }
    }

    pub fn peek(&mut self) -> Option<&i32>{
        self.head.as_ref().map(|node|{
            &node.elem
        })
    }

    pub fn print(&mut self){
        let mut curr = &self.head.take();
        while curr.is_some() {
            println!("{}", curr.as_ref().unwrap().elem);
            curr = &curr.as_ref().unwrap().next;
        }
    }

    pub fn clear(&mut self){
        self.head = None;
    }
}
fn main() {

    ///easy linked_list
    // let mut s1 = Box::new(Node{elem: 1, next: None});
    // let mut s2 = Box::new(Node{elem: 2, next: None});
    // let mut s3 = Box::new(Node{elem: 3, next: None});
    //
    // s2.next = Some(s3);
    // s1.next = Some(s2);
    //
    // let mut curr = Some(&s1);
    // while curr.is_some() {
    //     println!("{}", curr.unwrap().elem);
    //     curr = (&curr.unwrap().next).as_ref();
    // }

    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);

    println!("{:?}", list.peek().unwrap());

    list.clear();

    list.print();

}
