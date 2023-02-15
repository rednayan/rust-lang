#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

enum Message {
    Quit,
    Move {x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

enum IpAddrKind {
    V4(i32,i32,i32,i32),
    V6(String),
}

impl Message {
    fn call(&self){
        // method body 
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("You got a penny");
            return 1;
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}",state);
            return 25;
        },
    }
}

fn some_operation(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) =>  {
            println!("{}",i);
            return Some(i + 1);
        },
    }
}


fn main(){
    let localhost : IpAddrKind =  IpAddrKind::V4(127,0,0,1);
    let m: Message = Message::Write(String::from("hello"));
    m.call();
    let coin : Coin = Coin::Quarter(UsState::Alaska);
    value_in_cents(coin);
    some_operation(Some(5));
    some_operation(None);
}
