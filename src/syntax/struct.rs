#![allow(dead_code)]

#[derive(Debug)]
struct Rectangle { width: u32, height: u32 }

impl Rectangle {
    fn area(&mut self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

pub fn rectangle_area() {
    let mut rect = Rectangle {
        width: 30,
        height: 20,
    };

    // this would be an compile error
    // println!("rect is {:#?}\nit's area is : {}", rect, rect.area());

    let area = (&mut rect).area();
    println!("rect is {:#?}\nit's area is : {}", rect, area);

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    let max_rect = Rectangle::square(100);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can max_rect hold rect1? {}", max_rect.can_hold(&rect1));
}

