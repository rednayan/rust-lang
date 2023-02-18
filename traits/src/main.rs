use std::fmt::Display;


struct Pair<T>{
    x:T,
    y:T,
}

impl<T> Pair<T> {
    fn new (x:T,y:T) -> Self {
        Self{x,y}
        
    }
}

impl<T:Display + PartialOrd> Pair<T>{
    fn cmp_display(&self){
        if self.x >= self.y {
            println!("The largest member is x = {}",self.x);
        }
        else {
            println!("The largest member is y = {}",self.y);
        }
    }
}

pub struct NewsArticle {
    pub author : String,
    pub headline :String,
    pub content : String,
}


impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{} . by {}",self.headline, self.author)
    // }

    fn summarize_author(&self) -> String {
        format!("{}",self.author)
    }
}

pub struct Tweet {
    pub username : String,
    pub content  : String,
    pub reply : bool,
    pub retweet : bool,
} 

impl Summary for Tweet{
    fn summarize_author(&self) -> String {
        format!("{} : {}",self.username,self.content)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        String::from("Read more .... ")
    }
}

pub fn notify<T: Summary>(item1 : &T, item2 : &T){
    println!("Breaking news {}",item1.summarize());
    println!("Breaking news {}",item2.summarize());
}

fn returns_summarizable() -> impl Summary{
    Tweet {
        username:String::from("horse ebboks"),
        content:String::from("Of course, as you probably already know peole"),
        reply:false,
        retweet:false
    }
}

fn main() {
 let tweet : Tweet  = Tweet{
    username : String::from("@john"),
    content:String::from("Hello World"),
    reply:false,
    retweet:false
 };

 let article : NewsArticle = NewsArticle { author: String::from("John Doe"),
  headline: String::from("The Sky is Falling"),
   content: String::from("This is the end of the world and me")
 };

 println!("{}",returns_summarizable().summarize_author());

 println!("Tweet Summary : {}",tweet.summarize_author());
 println!("Article Summary : {}",article.summarize());
}