fn main() {
    let v1 = vec![1,2,4];
    let v2:Vec<i32> = vec![1,5,6,7,7];

    let total: i32 = v1.iter().sum();
    println!("{total}");

    let v3: Vec<i32>= v2.iter().map(|x| x+1).collect();
}
