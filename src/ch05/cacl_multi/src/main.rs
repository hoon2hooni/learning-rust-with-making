use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let request_nums = [43, 42, 20, 39, 37, 35, 30];
    let start_time = Instant::now();
    let (tx, rx) = mpsc::channel::<(i64,i64)>();
    for num in request_nums {
        let sender = tx.clone();
        thread::spawn(move || {
            let answer = fib(num);
            sender.send((num,answer)).unwrap();
        });
    }
    let mut job = request_nums.len();
    loop {
        if let Ok((arg,answer)) = rx.recv() {
            job -= 1;
            println!("fib({} 번째 수 ) = {} 남은 계산={}", arg, answer, job);
            if job <= 0 {
                show_time(start_time);
                break;
            }
        }
    }
}

fn fib(n: i64) -> i64 {
    if n <= 2 {
        return n-1;
    }
    return fib(n-1) + fib(n-2);
}

fn show_time(startTime: Instant) {
    let elapsed = startTime.elapsed();
    println!("소요시간: {}.{:03}초", elapsed.as_secs(), elapsed.subsec_millis());
}

