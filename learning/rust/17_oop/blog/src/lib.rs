pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            given_approvals: 0,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
    given_approvals: u8,
}

impl PendingReviewPost {
    pub fn approve(&mut self) {
        self.given_approvals += 1;
    }

    pub fn try_publish(self) -> Result<Post, Self> {
        if self.given_approvals >= 2 {
            Ok(Post {
                content: self.content,
            })
        } else {
            Err(self)
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}
