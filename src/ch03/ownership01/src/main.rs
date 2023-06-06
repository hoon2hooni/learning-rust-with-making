fn main() {
    let g1 = 30;
    let g2 = g1;
    //소유권 시스템을 적용받지 않음
    //숫자타입, boolean 타입
    println!("{}", g1);
    copy();
}

fn copy() {
    let g1 = String::from("무야호");
    // let g2 = g1;
    //borrow of moved value: `g1`
    let g2 = g1.clone();
    println!("{}", g1);
    println!("{}", g2)
}
