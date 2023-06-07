struct Item(String, i64);

fn main() {
    let banana = Item("바나나".to_string(), 300);
    let items = vec![banana];
    println!("총합은 {}", print_and_sum_items(&items))
}

fn print_and_sum_items(items: &Vec<Item>) -> i64 {
    let mut total = 0;
    for it in items {
        total += it.1;
    }
    return total;
}
