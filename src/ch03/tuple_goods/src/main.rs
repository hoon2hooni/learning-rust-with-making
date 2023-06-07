fn main() {
    let banana = ("바나나", 300);
    let apple = ("사과", 300);
    let total = banana.1 + apple.1;
    print_tuple(&apple);
    println!("{}", total);
}

fn print_tuple(item: &(&str, i32)) {
    println!("{}를 {}원에 구입", item.0, item.1)
}
