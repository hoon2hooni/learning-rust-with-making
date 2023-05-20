fn main() {
    for i in 1..101 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", i);
        }
    }
    anotherFizzBuzz();
}

fn anotherFizzBuzz() {
    for i in 1..51 {
        if i % 3 == 0 && i % 10 == 3 {
            println!("A");
            continue;
        }
        if i >= 30 && i <= 39 {
            println!("A");
            continue;
        }
        println!("{}", i);
    }
}
