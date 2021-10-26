use std::fmt::Display;

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        self.summarize_author()
    }
}

#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{:?}", self)
    }
}

// display 是在std::fmt::中定义的一个trait, 实现了display
// 可以在println!中通过"{}"的方式,参数直接对应对象获取对象的格式化字符串.
impl Display for NewsArticle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(auth:{}, headline:{}, content:{}, location:{})",
               self.author, self.headline, self.content, self.location)
    }
}


pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// pub fn notify(item: impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

/// # 如果有两个impl Summary的参数, 且必须为强制为相同类型, 则可以用下面的定义方法.
/// ```
/// pub fn notify<T: Summary>(item: &T, item1: &T) {}
/// ```


// 如果函数的一个参数被要求实现同时实现 Summary 和 Display , 可以用下面的方法:
pub fn notify_v1<T: Summary + Display>(item: &T) {
    println!("bound trait notify: \nsummary:{}\ndisplay:{}", item.summarize(), item);
}

/// there is an another way to define function 'notify_v1'
/// # Examples
/// ```
/// pub fn notify_v1<T>(item: &T)
/// where T:Summary + Display {
///     println!("bound trait notify: \nsummary:{}\ndisplay:{}", item.summarize(), item);
/// }
///
/// # Or following:
///
/// pub fn notify_v1(item :&(impl Summary + std::fmt::Display)){
///     println!("bound trait notify: \nsummary:{}\ndisplay:{}", item.summarize(), item);
/// }
/// ```


pub fn test_trait() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
    hockey team in the NHL."),
    };

    notify(&article);
    notify_v1(&article);
    // println!("New article available! {}", article.summarize());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    notify(&tweet);
    // println!("1 new tweet: {}", tweet.summarize());
}