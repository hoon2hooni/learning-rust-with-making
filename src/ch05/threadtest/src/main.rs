use std::{thread, time};

fn sleep_print(word: &str) {
    for i in 1..3 {
        println!("{}: i= {}", word, i);
        thread::sleep(time::Duration::from_millis(1000));
    }
}

fn main() {
    println!("쓰레드 없음");
    sleep_print("쓰레드 없음");

    println!("쓰레드 이용");
    thread::spawn(|| {
        sleep_print("토마토");
    });
    thread::spawn(|| {
        sleep_print("스위스");
    });
    thread::spawn(|| {
        sleep_print("별동별");
    });
    sleep_print("기러기");
}
