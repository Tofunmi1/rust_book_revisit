#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

//fill the remaining parts in a match scope
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        // Some(i) => Some(i + 1),
        _ => None,
    }
}

fn main() {
    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{}", rect1.area());
}
