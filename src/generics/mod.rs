pub mod r#trait;

use std::fmt::Display;
use std::ops::Add;

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
    // fn distance(&self, p: &Point<T>) -> T {
    //     ((self.x() - p.x()).powi(2) + (self.y() - p.y()).powi(2)).sqrt()
    // }
}

pub fn test_point() {
    let p_0: Point<i32> = Point { x: 10, y: 20 };
    let p_1: Point<i32> = Point { x: 10, y: 25 };
    let p_2: Point<f32> = Point { x: 10.1, y: 25.2 };
    // println!("distance form:{:?} to:{:?} is {:?}", &p_0, &p_1, p_0.distance(&p_1))
}

pub fn add<T: Display + Add<Output = T>>(a: T, b: T) -> T {
    return a + b;
}

pub fn test_number_add() {
    println!("10 + 11 equals:{}", add(10, 11));
    println!("11.5 + 0.6 equals:{}", add(11.5, 0.6));
}

pub fn test_larget_number() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}

/// # rust 函数参数和返回值生命周期的问题:
/// 规则:
/// 由于引用的生命周期是在编译时就需要确定的(不是在运行时确定的),
/// 所以对于类似的函数与:
/// ```
/// fn longest_str(s1 :&str, s2 :&str) -> &str {
///     if x.len() > y.len() {
///         x
///     } else {
///         y
///     }
/// }
/// ```
/// 函数在编译时就需要确定返回值引用的生命周期用于错误的检查.
/// 但是,由于返回值有可能是s1, 也有可能是s2, 并且
/// s1和s2的生命周期有可能是不同的,
/// 编译器没办法确定返回值的生命周期是什么.
/// 就会出现编译错误.
fn longest<'b>(x: &'b str, y: &'b str) -> &'b str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn sub_string(x: &str, f: usize, t: usize) -> &str {
    &x[f..t]
}


#[allow(unused)]
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

/// # error example:
/// ```
/// let result;
/// let string1 = String::from("long string is long");
/// {
///     let string2 = String::from("xyz");
///     result = longest(string1.as_str(), string2.as_str());
/// }
/// println!("The longest string is {}", result);
/// ```
pub fn test_lifetime() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = "xyzwwww";
        // why following will be an error
        // let string2 = String::from("xyzwwww");
        // because all static string have a static lifetime,
        // let s: &'static str = "I have a static lifetime.";
        result = longest(&string2, &string1);
    }
    println!("The longest string is {}", result);

    let string1 = String::from("long string is long");

    println!("The sub string is {}", sub_string(string1.as_str(), 0, 4));

    println!("The frist word is {}", first_word(string1.as_str()));

    let impe = ImportantExcerpt::new("hello");
    {
        let annoncement = impe.announce_and_return_part("abcdef");
    }
}


fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: std::fmt::Display {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn new(part: &str) -> ImportantExcerpt {
        ImportantExcerpt {
            part,
        }
    }

    fn level(&self) -> i32 {
        3
    }

    // 返回值被赋予了&self的生命周期
    fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        &announcement[2..5]
    }

    fn announce_and_return_part2(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}