use std::fmt::Display;

trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more...)")
    }
}

struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
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

struct DefaultSummary {}

impl Summary for DefaultSummary {}

pub fn trait_test() {
    println!("=== trait_test ===");

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    println!("article summary = {}", article.summarize());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    println!("tweet summary = {}", tweet.summarize());

    let summary = DefaultSummary {};
    println!("default summary = {}", summary.summarize());
}

pub fn trait_as_parameter_test() {
    println!("=== trait_as_parameter_test ===");

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    notify(&article);
    notify_1(&article);
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn notify_1<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    // Self 是一个 impl 块类型的类型别名，Self == Pair<T>
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmd_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
