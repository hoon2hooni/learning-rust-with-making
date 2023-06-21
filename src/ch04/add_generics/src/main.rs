fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn x2<T: std::ops::Add<Output = T> + Copy>(a: T) -> T {
    a + a
}

fn add2<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    a + b
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T>
where
    T: std::ops::AddAssign,
{
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    fn add(&mut self, other: Point<T>) {
        self.x += other.x;
        self.y += other.y;
    }
}

fn main() {
    add(1, 2);
    add(1.0, 2.0);
    add::<i32>(1, 2);
    let pt_i = Point { x: 1, y: 2 };
    let pt_f = Point { x: 1.0, y: 2.0 };
}
