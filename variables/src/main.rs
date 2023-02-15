const THIS_IS_A_CONSTANT : u32 = 60 * 30 * 30;

// shadowing
fn shadowing(){
    let y = 10;

    let y = y + 10;

    {
        let y = y + 20;
        println!("This is the value of y in the inner scope {y}");
    }

    println!("This is the value of y in outside {y}");
    
}

fn main() {
    let mut x = 5;
    println!("{}",x);
    x = 6;
    println!("{},this is the value of x after mutation {}",THIS_IS_A_CONSTANT,x);
    shadowing();
    let string : &str= "This is perhaps a string i guess";
    let string =  string.len();
    println!("length of string {string}");
} 

