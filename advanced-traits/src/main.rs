use std::ops::Add;

pub trait Iterator {
    type Item; //associated type

    fn next(&mut self) -> Option<Self::Item>
}

pub trait GenericIterator<T> {
    fn next(&mut self) -> Option<T>;
}

struct Counter {}

impl GenericIterator<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        Some(0);
    }
}

impl Iterator<u16> for Counter {
    fn next(&mut self) -> Option<u16> {
        Some(0)
    }
}

#[derive(Debug,PartialEq)]
struct Point {
    x:i32,
    y:i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self,other:Point) -> Point {
        Point {
            x:self.x + other.x,
            y: self.y + other.y,
        }
    }
}
fn main(){
    assert_eq!(
        Point {x:1,y:0} + Point {x:2,y:3},
        Point {x:3,y:3}
    )
} 