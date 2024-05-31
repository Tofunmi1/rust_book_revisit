struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let integer: Point<f32> = Point { x: 5.1, y: 6.1 };
    println!("{}", integer.distance_from_origin());

    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: "x" };

    let p3 = p1.mixup(p2);
}
