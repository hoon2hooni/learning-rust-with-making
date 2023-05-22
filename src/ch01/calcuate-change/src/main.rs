// 숫자 단위 ㄱㅖ산
fn main() {
    let price: i64 = 3950;
    for i500 in 0..11 {
        for i100 in 0..4 {
            for i50 in 0..11 {
                let total = i500 * 500 + i100 * 100 + i50 * 50;
                if total == price {
                    println!("{}개 {}개 {}개", i500, i100, i50);
                }
            }
        }
    }
    println!("{}", i128::MAX);
    println!("최대:{} 최소:{}", i16::MAX, i16::MIN);
    println!("최대:{} 최소:{}", u16::MAX, u16::MIN);
    println!("최대:{} 최소:{}", f64::MAX, f64::MIN);
    println!("{}", u128::MAX);
}
