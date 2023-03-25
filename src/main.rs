#![allow(dead_code)]

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
    fn can_hold(&self, another: &Self) -> bool {
        if self.width < another.width { return false; }
        if self.height < another.height { return false; }
        return true;
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 40,
        ..rect1
    };

    println!("rect1 can hold rect2 ? {} ", rect1.can_hold(&rect2));
}
