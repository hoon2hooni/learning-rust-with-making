fn main() {
    println!("Hello, world!");
    let car1 = CarSpec {
        model:1,
        cc:1,
        color:0xFF000,
    }
}

struct CarSpec {
    model: i32,
    cc: i32,
    color:i32,
}

