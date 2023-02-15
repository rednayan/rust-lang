mod back_of_house{
    pub struct Breakfast{
        pub toast : String,
        seasonal_fruit: String,
    }

    impl Breakfast{
        pub fn summer(toast: &str) -> Breakfast  {
            Breakfast{
                toast : String::from(toast),
                seasonal_fruit : String::From("peaches"),
            }
        }
    }
}

pub eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I would like {} toast",meal.toast);
}
