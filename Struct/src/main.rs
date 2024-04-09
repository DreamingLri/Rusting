#[derive(Debug)]
struct User{
    username: String,
    active: bool,
    email: String,
    sign_in_count: u64,
}

struct Color(u32, u32, u32); //tuple struct

struct Rectangle{
    width: u32,
    length: u32,
}

impl Rectangle {
    fn get_area(&self) -> u32{
        self.width * self.length
    }
}



fn main() {
    fn create_user(username: String, email: String) -> User{
        User{
            username,
            email,
            active: true,
            sign_in_count: 1,
        }
    }

    let user1 = User{
        username: String::from("bob"),
        email: String::from("123@qq.com"),
        active: true,
        sign_in_count: 2,
    };

    let user2 = create_user(String::from("bob"), String::from("111"));

    let user3 = User{
        username: String::from("alex"),
        ..user2 //语法糖，标记为剩下与user2相同
    };

    println!("{:?}", user1);

    let rect = Rectangle{
        width: 30,
        length: 50,
    };

    println!("{}", rect.get_area());

}
