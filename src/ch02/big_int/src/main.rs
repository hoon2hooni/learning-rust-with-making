use num_bigint::BigInt;

fn main() {
    //     let v = num_bigint::BigInt::from(1234); 이것도 가능
    let v = BigInt::from(1234);
    println!("{}", v.pow(5678))
}
