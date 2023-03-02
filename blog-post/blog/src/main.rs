use post::add;

fn main() { 
    let mut post = Post::new();

    post.add_text("new blog post");
    assert_eq!("",post.content());
    
    post.request_review();
    assert_eq!("",post.content());

    post.approve();
    assert_eq!("new blog post",post.content());
}
