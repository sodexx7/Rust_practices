struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 5.1, y: 10.2 };
    let p3 = Point { x: "a", y: "b" };
    let p4 = Point { x: String::from("This is the first coordinate"), y: String::from("This is the second coordinate") };

    println!("p1.x = {}", p1.x());
    println!("p2.x = {}", p2.x());
    println!("p3.x = {}", p3.x());
    println!("p4.x = {}", p4.x());
}