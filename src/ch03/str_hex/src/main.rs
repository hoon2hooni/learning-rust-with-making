fn main() {
    let pr = "성공하는 사람은 송곳처럼 어느 한 점을 향아여 일한다.";
    hex_dump(pr);
    str_substr2(pr);
}

fn hex_dump(str: &'static str) {
    for (i, c) in str.bytes().enumerate() {
        print!("{:02x} ", c);
        if i % 16 == 0 {
            print!("{:08x}|", i);
        }
        if i % 4 == 3 {
            print!("{:02x}|", c);
        } else {
            print!("{:02x}", c);
        }

        if i % 16 == 15 {
            println!("");
        }
    }
    println!("")
}

fn str_substr2(pr: &str) {
    let sub3: String = pr.chars().take(2).collect();
    println!("앞 두글자 {}", sub3);

    let pr_chars: Vec<char> = pr.chars().collect();
    let sub_chars = &pr_chars[4..=5];
    let sub4: String = sub_chars.into_iter().collect();
    println!("4-5번째 글자 {}", sub4);
}
