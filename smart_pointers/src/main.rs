use std::ops::Deref;
use std::mem::drop;
use std::rc::Rc;
use List::{Cons,Nil};

enum List{
    Cons(i32,Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T>{
    fn new(x:T) -> MyBox<T> {
        return MyBox(x);
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
 }

 struct CustomSmartPointer{
    data: String,
 }

 impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with the data {}",self.data);
    }
 }

fn main() {
    let list = Cons(1,Box::new(Cons(2,Box::new(Cons(3,Box::new(Nil))))));
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5,x);
    assert_eq!(5,*y);

    let m = MyBox::new(String::from("rust"));
    hello(&m);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data:String::from("other stuff"),
    };

    println!("CustomSmartPointers created");
    drop(c);
    println!("CustomerSmartPointers dropped before main ")
}

fn hello(name:&str) {
    println!("Hello, {}!",name);
}
