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

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.length > other.length
    }

    fn square(size: u32) -> Rectangle{
        Rectangle{
            width: size,
            length: size,
        }
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

    let rect1 = Rectangle{
        width: 30,
        length: 50,
    };

    let rect2 = Rectangle{
        width: 40,
        length: 60,
    };

    let square = Rectangle::square(20);

    println!("{}", rect1.get_area());

    println!("{}", rect1.can_hold(&rect2));

    println!("{}", square.get_area());

}
