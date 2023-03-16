struct Point<T> {
    x: T,
    y: T,
}

impl Point<i32> {
    fn x(&self) -> &i32 {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
