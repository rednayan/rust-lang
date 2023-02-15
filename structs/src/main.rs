
struct User {
    active : bool,
    user_name: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email : String::from("nayan@gmail.com"),
        user_name : String::from("nayan"),
        active: true,
        sign_in_count :1,
    };
    let user2 = build_user(user1.email,user1.user_name);
    println!("{}",user2.sign_in_count);
    println!("{}",user1.sign_in_count);
}
fn build_user(email:String, user_name:String) -> User {
    return User {
        email,
        user_name,
        active: true,
        sign_in_count : 3,
    }
}
