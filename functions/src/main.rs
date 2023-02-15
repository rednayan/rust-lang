fn main() {
    another_function(5);
    let x = return_value();
    println!("This is the return value {x}");
    let y = return_value2(10);
    println!("value of y = {y}");
}

fn another_function(x: u32){
    println!("{x}");
}

fn return_value() -> i32 {
    5
}

fn return_value2(x : u32) -> u32 {
    return x+1; 
}
