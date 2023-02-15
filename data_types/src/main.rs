fn integers(){
    let integer1 : u8 = 255;
    let integer2 : i8 = -128;
    let integer3 : i8 = 127;

    let integer4 : u16 = 65535;
    let integer5 : i16 = -32768;
    let integer6 : i16 = 32767;

    let integer8 : u32 = 4294967295;
    let integer9 : i32 = -2147483648;
    let integer10 : i32 = 2147483647;

    let integer11 : u64 = 18446744073709551615;
    let integer12 : i64 = -9223372036854775808;
    let integer13 : i64 = 9223372036854775807;

    let integer14 : u128 = 340282366920938463463374607431768211455;
    let integer15 : i128 = -170141183460469231731687303715884105728;
    let integer16 : i128 = 170141183460469231731687303715884105727;
}

fn floating_point(){
    let floating1 : f32 = 12.634;
    let floating2 : f64 = 12.3535234;

    println!("floating point f32 : {} , floating point f64 : {}",floating1 , floating2);
}

fn boolean(){
    let is_true: bool = true;
    let is_false: bool = false;

    println!("{}:{} ",is_true,is_false);
}

fn character_x(){
    let chara = 'a';
    let charz : char = 'z';
    println!("{}:{}",charz,chara);
}

fn tuple_test(){
    let tup : (u32,f32,u64) = (1,1.2,18446744073709551615);    
    println!("{}",tup.2); 
    let (x,y,z) = tup;
    println!("({},{},{})", x,y,z);
}

fn array_type() {
    let arr : [u32;5] = [1,2,3,4,5];
    let arr1 : [f64;4] = [1.234,3.4456,4.67,4.56];
    let arr2  = ["this","that","where"];

    let arr3 : [i32;5] = [1,2,3,4,5];

    let arr4 = [3;5];
    println!("{}",arr2[1]);

    
}

fn main() {
    let guess : u32 = match "43".parse(){
        Ok(num) => num,
        Err(_) => 401,
    };
    println!("{}",guess);
    integers();
    floating_point();
    boolean();
    character_x();
    tuple_test();
    array_type();
}

