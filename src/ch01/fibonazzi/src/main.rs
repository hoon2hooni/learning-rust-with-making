fn main() {
    let mut a = 1;
    let mut b = 1;
    let mut temp;
    for _ in 0..30 {
        temp = a + b;
        a = b;
        b = temp;
        println!("{}", b);
    }
    Error();
}

fn Error() {
    let a = 3;
    a = 5;
}