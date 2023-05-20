fn main() {
    for i in 1..10 {
        for j in 1..10 {
            print!("{:3},", i * j)
        }
        println!("");
    }
    withoutCommaAtEnd()
}

fn withoutCommaAtEnd() {
    for i in 1..10 {
        let s = (1..10)
            .map(|j| format!("{:3}", i * j))
            .collect::<Vec<String>>()
            .join(",");
        println!("{}", s);
    }
}
