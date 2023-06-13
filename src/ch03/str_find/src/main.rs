fn main() {
    let s = "제주도의 특산품 은 귤이걸랑요~";
    match s.find('귤') {
        Some(idx) => println!("찾는 문자의 위치는 {}입니다.", idx),
        None => println!("찾는 문자가 없습니다."),
    }

    match s.find("바나나") {
        Some(idx) => println!("찾는 문자의 위치는 {}입니다.", idx),
        None => println!("찾는 문자가 없습니다."),
    }
    closure_find();
}

fn closure_find() {
    let s = format!(
        "{}{}",
        "
    there is more happiness in giving than in receiving.",
        "
    It's better to give than to receive."
    );
    match s.find(|ch: char| ch.to_ascii_uppercase() == 'I') {
        Some(idx) => println!("찾는 문자의 위치는 {}입니다.", idx),
        None => println!("찾는 문자가 없습니다."),
    }
}
