use std::io;

fn main() {    
    println!("Enter nth number");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n : u32 = n.trim().parse().expect("Enter valid number");
    let mut i : u32 = 0;
    let mut _p : u32 = 0;
    let mut _n : u32 = 1;

    while i != n {
       let mut fib_no = _p + _n;
       _p = _n;
       _n = fib_no;
       println!("{fib_no}");
       i+=1; 
    }

}
