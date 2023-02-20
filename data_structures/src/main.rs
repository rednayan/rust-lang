enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn vectors() {
    let v : Vec<SpreadSheetCell> = vec![SpreadSheetCell::Int(32),SpreadSheetCell::Float(2.56),SpreadSheetCell::Text(String::from("here"))];
    for i  in &v {
        match i {
            SpreadSheetCell::Int(x) => println!("{}",x),
            _ => println!("None"),
        }
    }
}

fn main() {
    let vec = vectors();
}
