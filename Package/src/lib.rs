mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){}
        pub fn seat_at_table(){}
    }

    mod serving{
        fn take_order(){}
        fn serve_order(){}
        fn take_payment(){}
    }
}

pub fn eat_at_restaurant(){
    let mut meal = back_of_house::Breakfast::summer("Rya");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast, please!", meal.toast);
    //meal.seasonal_fruit = String::from("strawberries");
}

mod back_of_house{
    fn fix_incorrect_order(){
        cook_order();
        super::test();
    }

    fn cook_order(){}

    pub struct Breakfast{
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast{
        pub fn summer(toast: &str) -> Breakfast{
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

fn test(){}
