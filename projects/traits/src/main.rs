pub struct NewArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}.", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}

fn main() {
    let tweet = Tweet {
        username: String::from("@petrostrak"),
        content: String::from("Hello rust!"),
        reply: false,
        retweet: false,
    };

    let article = NewArticle {
        author: String::from("Petros Trak"),
        content: String::from("The sky is falling"),
        headline: String::from("not actually falling (:")
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());
}