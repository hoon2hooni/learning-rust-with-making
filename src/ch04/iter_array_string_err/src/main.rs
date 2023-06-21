struct PrimeIterator {
    n: u8,
}
impl PrimeIterator {
    fn new() -> PrimeIterator {
        PrimeIterator { n: 1 }
    }

    fn is_prime(&self) -> bool {
        for i in 2..self.n {
            if self.n % i == 0 {
                return false;
            }
        }
        return true;
    }
}

impl Iterator for PrimeIterator {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.n += 1;
            if std::u8::MAX == self.n {
                return None;
            }
            if self.is_prime() {
                return Some(self.n);
            }
        }
    }
}

struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { curr: 1, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let temp = self.curr;
        self.curr = self.next;
        self.next = temp + self.next;
        return Some(self.curr as usize);
    }
}
fn main() {
    let array = ["Hello".to_string(), "World".to_string(), "!".to_string()];
    let prime_iter = PrimeIterator::new();
    for p in prime_iter {
        println!("{}", p);
    }

    // for a in array {
    //     println!("{}", a);
    // }
    for a in array.iter() {
        println!("{}", a);
    }
    let fib_iter = Fibonacci::new();
    for (i, n) in fib_iter.enumerate() {
        if i >= 10 {
            break;
        }
        println!("{}", n)
    }
    let fib_iter = Fibonacci::new();
    fib_iter.take(10).for_each(|f| println!("{}", f));
    print!("\n");
    
    println!("length {}", array.len())
}
