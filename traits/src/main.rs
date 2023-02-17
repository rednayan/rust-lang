pub struct NewsArticle {
    pub author : String,
    pub headline :String,
    pub content : String,
}


impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} . by {}",self.headline, self.author)
    }
}

pub struct Tweet {
    pub username : String,
    pub content  : String,
    pub reply : bool,
    pub retweet : bool,
} 

impl Summary for Tweet{
    fn summarize(&self) -> String {
        format!("{} : {}",self.username,self.content)
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
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

 println!("Tweet Summary : {}",tweet.summarize());
 println!("Article Summary : {}",article.summarize());
}