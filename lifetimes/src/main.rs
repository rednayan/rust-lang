struct ImportantExcerpt<'a> {
    part : &'a str,
}

fn main() {
    let string1 = String::from("abcd");
    let result : &str;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(),string2.as_str());
    }
    println!("The longest string is {}",result);
    
}

fn longest<'a>(x:&'a str,y:&str) -> &'a str{
    // if x.len() > y.len(){
    //     x
    // } else {
    //     y
    // }
    x
}


