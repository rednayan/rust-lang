struct Point<X,Y> {
    x: X,
    y: Y,
}
impl <X,Y> Point<X,Y>{
    fn mixup<X1,Y1>(self,other : Point<X1,Y1>) -> Point<X,Y1> {
        return Point { x: self.x, y: other.y };
    }
}

fn main() {
    let p1 = Point {x:5,y:9};
    let p2 = Point {x:"Hello",y:'d'};

    let p3 = p1.mixup(p2);
    println!("{} : {}",p3.x,p3.y);
}