use blog::{Post};

fn main() {
    let mut post = Post::new();
    
    post.add_text("I ate a salad for lunch today");
    
    let mut post = post.request_review();
    
    post.approve();
    let mut post = match post.try_publish() {
        Ok(_) => panic!("Shouldn't transform yet after only 1 approval"),
        Err(post) => post,
    };
    
    post.approve();
    
    let post = match post.try_publish() {
        Ok(post) => post,
        Err(_) =>  panic!("Shouldn't error after 2 approvals"),
    };
    
    assert_eq!("I ate a salad for lunch today", post.content());
}
