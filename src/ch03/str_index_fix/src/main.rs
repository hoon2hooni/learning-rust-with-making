fn main() {
    let s = "안녕하세요";
    let ch = s.chars().nth(0).unwrap();
    println!("{}의 1번째 문자: {}", s, ch);
    // 3번째 문자를 가져오려면?
    let ch = s.chars().nth(2).unwrap();
    println!("{}의 3번째 문자: {}", s, ch);
    with_slice();
}
fn with_slice() {
    let s = "안녕하세요";
    let slice = &s[0..3];
    println!("{}의 1~3번째 문자: {}", s, slice);
    //하를 출력
    let slice = &s[6..9];
    println!("{}의 4~6번째 문자: {}", s, slice);
}

// string과 &str서로 변환하기

// Path: src/main.rs
fn swap_str_string() {
    let slice = "안녕하세요";
    let string = slice.to_string();
    println!("string: {}", string);
    let string = "안녕하세요".to_string();
    println!("string: {}", string);
    let string = String::from("안녕하세요");
    println!("string: {}", string);
    let string = String::from("안녕하세요");
    let str = &string[..];
    let str2 = string.as_str();
    println!("str: {}", str);
    let string = String::from("안녕하세요");
    let str = &string;
    println!("str: {}", str);
}
