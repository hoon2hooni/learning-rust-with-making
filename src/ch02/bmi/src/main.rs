fn main() {
    let s = "365";
    let i = match s.parse() {
        Ok(v) => v,
        Err(_) => 0,
    };
    println!("{}", i);

    let test =  "3.1415a";
    // let num = test.parse::<f64>().expect("변환 실패");
    let num2 = test.parse::<f64>();
    match num2 {
        Ok(result) => println!("{}",result),
        Err(e) => println!("에러 발생 이유 = '{:?}'",e)
    };
    // println!("{}", num)
}
