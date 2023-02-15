fn main() {
    let y = if true { 5 } else {4};
    println!("The value is {y}");
    loops();
    disambiguate_loops();
    for_loop();
    countdown();
}

fn loops(){
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    println!(" {result}");
}

fn disambiguate_loops(){
   let mut count = 0;
   'counting_up : loop {
        println!("The value of count : {count}");
        let mut remaining = 10;
        loop{
            println!("The value of remaining : {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
   }
        println!("End count : {count}");
}

fn for_loop() {
    let arr : [u32;4] = [1,2,3,4];
    let tup: (u32,f32,i32) = (2,3.4,-45);

    for element in arr {
        println!("element : {element}");
    }
    println!("{}",tup.2)
}

fn countdown(){
    for i in (1..10).rev(){
        println!("{i}!");
    }
    println!("LIFTOFF!!!");
}
