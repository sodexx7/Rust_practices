struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1: Clone, Y1: Clone> Point<X1, Y1> {
    fn mixup<X2, Y2: Clone>(&self, other: &Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x.clone(),
            y: other.y.clone(),
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(&p2);
    let p4 = p2.mixup(&p1);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    println!("p4.x = {}, p4.y = {}", p4.x, p4.y);
}