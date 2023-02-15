use std::io;

fn main() {
        
    while true{
        println!("Enter 1 to convert C to F");
        println!("Enter 2 to convert F to C");
 
        let mut selection = String::new();
        io::stdin().read_line(&mut selection).expect("Failed to read line");
        let selection : u32 = selection.trim().parse().expect("Please type a number");
        println!("You selected {selection}");

        if selection == 1{
            println!("Enter temperature in Celcius");
            let mut temperature_c = String::new();
            io::stdin().read_line(&mut temperature_c).expect("Failed to read line");
            let temperature_c : f64  = temperature_c.trim().parse().expect("Please enter valid temperature");
            let temperature_f : f64 = temperature_c * 9.0/5.0 + 32.0; 
            println!("{temperature_c} degree Celcius in Farenheit is {temperature_f}");
            continue;
        }
        else if selection == 2{
            println!("Enter temperature in Farenheit");
            let mut temperature_f = String::new();
            io::stdin().read_line(&mut temperature_f).expect("Failed to read line");
            let temperature_f: f64 = temperature_f.trim().parse().expect("Please enter valid temperature");
            let temperature_c: f64 = (temperature_f - 32.0) * 5.0/9.0;
            println!("{temperature_f} in degree Celcius is {temperature_c}");
            continue;
            }
 
         }
    }
